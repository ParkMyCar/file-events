use core_foundation as cf;
use fsevent_sys as fsevent;

#[macro_use]
extern crate bitflags;

pub mod flags;
pub mod types;

pub type UnsafeMutableRawPointer = *mut std::ffi::c_void;

pub type FSEventStreamRef = *mut ::std::os::raw::c_void;

pub type FSEventStreamCallback = Option<extern "C" fn(
    streamRef: FSEventStreamRef,
    clientCallBackInfo: UnsafeMutableRawPointer,
    numEvents: usize,
    eventPaths: UnsafeMutableRawPointer,
    eventFlags: *const fsevent::FSEventStreamEventFlags,
    eventIds: *const fsevent::FSEventStreamEventId,
)>;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub fn FSEventStreamCreate(
        allocator: cf::base::CFAllocatorRef,
        callback: FSEventStreamCallback,
        context: UnsafeMutableRawPointer,
        pathsToWatch: cf::array::CFArrayRef,
        sinceWhen: fsevent::FSEventStreamEventId,
        latency: cf::date::CFTimeInterval,
        flags: fsevent::FSEventStreamEventFlags,
    ) -> FSEventStreamRef;

    pub fn FSEventStreamScheduleWithRunLoop(
        streamRef: FSEventStreamRef,
        runLoop: cf::runloop::CFRunLoopRef,
        runLoopMode: cf::string::CFStringRef,
    );

    pub fn FSEventStreamStart(
        streamRef: FSEventStreamRef,
    ) -> bool;
}
