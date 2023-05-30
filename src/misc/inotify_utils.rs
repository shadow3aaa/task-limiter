use inotify::{Inotify, WatchMask};
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub fn inotify_block<T: IntoIterator>(path: T) -> Result<(), Box<dyn Error>>
where
    <T as std::iter::IntoIterator>::Item: std::convert::AsRef<std::path::Path>,
{
    let mut inotify = Inotify::init()?;
    path.into_iter().for_each(|p| {
        inotify.add_watch(p, WatchMask::MODIFY).unwrap();
    });

    let _ = inotify.read_events_blocking(&mut []);
    Ok(())
}

pub fn read_block(path: &'static str, timer: Duration) -> Result<(), Box<dyn Error>> {
    let init = fs::read_to_string(path)?;
    loop {
        if init != fs::read_to_string(path)? {
            break;
        }
        sleep(timer);
    }
    Ok(())
}

pub async fn inotify_block_async<T: IntoIterator>(path: T) -> Result<(), Box<dyn Error>>
where
    <T as std::iter::IntoIterator>::Item: std::convert::AsRef<std::path::Path>,
{
    inotify_block(path)
}

pub async fn read_block_async(path: &'static str, timer: Duration) -> Result<(), Box<dyn Error>> {
    read_block(path, timer)
}
