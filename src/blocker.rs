use std::{error::Error, sync::mpsc, thread, time::Duration};

#[derive(Debug)]
pub struct Blocker<T> {
    rx: mpsc::Receiver<T>,
    data: Option<T>,
    thread: thread::JoinHandle<()>,
}

impl<T: Send + Clone + std::cmp::PartialEq + 'static> Blocker<T> {
    // 通过一个堵塞函数更新数据，数据不变化就堵塞
    pub fn new<F>(fun: F, timer: Duration) -> Self
    where
        F: Fn() -> T + Send + 'static,
    {
        let (sx, rx) = mpsc::channel();
        let thread = thread::spawn(move || loop {
            let data = fun();

            if sx.send(data).is_err() {
                return;
            }
            thread::sleep(timer);
        });
        Self {
            rx,
            data: None,
            thread,
        }
    }

    pub fn alive(&self) -> bool {
        self.thread.is_finished()
    }

    pub fn block(&mut self) -> Result<(), Box<dyn Error>> {
        let mut recv_data = self.rx.try_recv().ok();
        while self.data == recv_data {
            recv_data = Some(self.rx.recv()?);
        }
        if let Some(o) = self.rx.try_iter().last() {
            self.data = Some(o);
        } else {
            self.data = recv_data;
        }
        Ok(())
    }

    pub async fn block_async(&mut self) -> Result<(), Box<dyn Error>> {
        self.block()
    }
}
