use inotify::{Inotify, WatchMask};
use std::error::Error;

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
