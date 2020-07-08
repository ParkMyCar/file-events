use core_foundation::{
    array::CFArray,
    base::{
        ItemMutRef,
        FromMutVoid,
        TCFType,
    },
    string::CFString,
    self as cf,
};
use crossbeam_channel::{
    self as channel,
    Receiver,
    Sender,
};
use fsevent_sys as fsevent;

use file_events::{
    FSEventStreamCreate,
    FSEventStreamScheduleWithRunLoop,
    FSEventStreamStart,
};

use std::{
    ffi,
    mem::MaybeUninit,
    ptr,
    sync::Once,
    time::Duration,
};

static mut CHANNEL: MaybeUninit<(Sender<Option<String>>, Receiver<Option<String>>)> = MaybeUninit::uninit();
static CHANNEL_INIT: Once = Once::new();

#[inline]
fn get_channel() -> &'static (Sender<Option<String>>, Receiver<Option<String>>) {
    CHANNEL_INIT.call_once(|| unsafe {
        ptr::write(CHANNEL.as_mut_ptr(), channel::unbounded());
    });

    unsafe {
        &*CHANNEL.as_ptr()
    }
}

fn main() {
    std::thread::spawn(|| {
        let stream_flags = fsevent::kFSEventStreamCreateFlagUseCFTypes
            | fsevent::kFSEventStreamCreateFlagFileEvents;

        let paths: Vec<CFString> = vec![CFString::from_static_string("/Volumes/Extreme SSD")];
        println!("Watching paths:\n{:#?}", &paths);

        let cf_paths = CFArray::from_CFTypes(&paths[..]);

        let stream_ref = unsafe {
            println!("creating stream!");
            let stream_ref = FSEventStreamCreate(
                fsevent::core_foundation::kCFAllocatorDefault,
                Some(callback),
                ptr::null_mut(),
                cf_paths.as_concrete_TypeRef(),
                fsevent::kFSEventStreamEventIdSinceNow,
                Duration::from_secs(1).as_secs_f64(),
                stream_flags);
    
            if stream_ref.is_null() {
                panic!("Failed to create FS Event Stream");
            }
    
            stream_ref
        };
        println!("stream created!");

        unsafe {
            println!("shceduling with runloop!");
            FSEventStreamScheduleWithRunLoop(
                stream_ref,
                cf::runloop::CFRunLoopGetCurrent(),
                cf::runloop::kCFRunLoopDefaultMode,
            );

            let started = FSEventStreamStart(stream_ref);
            
            if started {
                println!("FS Event Stream started!")
            } else {
                println!("FS Event Stream failed to start!")
            }

            println!("running runloop!");
            cf::runloop::CFRunLoopRun();
        };
    });


    let (_tx, rx) = get_channel();

    for maybe_string in rx.iter() {
        if let Some(data) = maybe_string {
            println!("Event emitted for: {:#?}", data);
        }
    };
}

#[allow(non_snake_case)]
extern "C" fn callback(
    _streamRef: fsevent::FSEventStreamRef,
    _clientCallbackInfo: *mut ffi::c_void,
    _numEvents: usize,
    eventPaths: *mut ffi::c_void,
    _eventFlags: *const u32,
    _eventIds: *const u64,
) { 
    let (tx, _rx) = get_channel();

    let cf_paths: ItemMutRef<CFArray<CFString>> = unsafe { CFArray::from_mut_void(eventPaths) };
    for path in cf_paths.iter() {
        tx.send(Some(path.to_string())).unwrap();
    }
}