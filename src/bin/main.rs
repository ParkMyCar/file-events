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
    flags::FSEventStreamEventFlags,
    types::EventInfo,
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

static mut CHANNEL: MaybeUninit<(Sender<Option<EventInfo>>, Receiver<Option<EventInfo>>)> = MaybeUninit::uninit();
static CHANNEL_INIT: Once = Once::new();

#[inline]
fn get_channel() -> &'static (Sender<Option<EventInfo>>, Receiver<Option<EventInfo>>) {
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

        unsafe {
            FSEventStreamScheduleWithRunLoop(
                stream_ref,
                cf::runloop::CFRunLoopGetCurrent(),
                cf::runloop::kCFRunLoopDefaultMode,
            );

            let started = FSEventStreamStart(stream_ref);
            
            if started {
                println!("FS Event Stream started!")
            } else {
                panic!("FS Event Stream failed to start!")
            }

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
    eventFlags: *const u32,
    _eventIds: *const u64,
) { 
    let (tx, _rx) = get_channel();

    let cf_paths: ItemMutRef<CFArray<CFString>> = unsafe { CFArray::from_mut_void(eventPaths) };

    let paths: Vec<String> = cf_paths.into_iter().map(|p| p.to_string()).collect();
    let flags = unsafe { FSEventStreamEventFlags::from_bits_unchecked(*eventFlags) };
    let info = EventInfo{
        paths,
        flags,
    };

    tx.send(Some(info)).unwrap();
}