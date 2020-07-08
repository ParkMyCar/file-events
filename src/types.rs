use crate::flags::FSEventStreamEventFlags;

#[derive(Debug)]
pub struct EventInfo {
    pub paths: Vec<String>,
    pub flags: FSEventStreamEventFlags,
}