use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug)]
pub struct InfoSync<T> {
    rx: mpsc::Receiver<T>,
    data: Option<T>,
    thread: thread::JoinHandle<()>,
}

impl<T: Send + Clone + 'static> InfoSync<T> {
    // 通过定时器更新数据
    pub fn new_timer<F>(fun: F, timer: Duration) -> Self
    where
        F: Fn() -> T + Send + 'static,
    {
        let (sx, rx) = mpsc::channel();
        let thread = thread::spawn(move || loop {
            let data = fun();
            // InfoSync drop时send会err，因此不需要自定义Drop自动退出
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

    // 通过一个堵塞函数更新数据，不堵塞就更新
    pub fn new_blocker<F, B>(fun: F, blocker: B) -> Self
    where
        F: Fn() -> T + Send + 'static,
        B: Fn() + Send + 'static,
    {
        let (sx, rx) = mpsc::channel();
        let thread = thread::spawn(move || loop {
            let data = fun();
            // InfoSync drop时send会err，因此不需要自定义Drop自动退出
            if sx.send(data).is_err() {
                return;
            }
            blocker();
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

    pub fn get(&mut self) -> Option<T> {
        if let Some(o) = self.rx.try_iter().last() {
            self.data = Some(o);
        }
        self.data.clone()
    }
}
