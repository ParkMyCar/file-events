use fsevent_sys as fsevent;

bitflags! {
    pub struct FSEventStreamEventFlags: fsevent::FSEventStreamEventFlags {
        const NONE   = 0x00000000;
        const MUSTSCANSUBDIRS = 0x00000001;
        const USERDROPPED = 0x00000002;
        const KERNELDROPPED = 0x00000004;
        const EVENTIDSWRAPPED = 0x00000008;
        const HISTORYDONE = 0x00000010;
        const ROOTCHANGED = 0x00000020;
        const MOUNT  = 0x00000040;
        const UNMOUNT = 0x00000080;
        const ITEMCREATED = 0x00000100;
        const ITEMREMOVED = 0x00000200;
        const ITEMINODEMETAMOD = 0x00000400;
        const ITEMRENAMED = 0x00000800;
        const ITEMMODIFIED = 0x00001000;
        const ITEMFINDERINFOMOD = 0x00002000;
        const ITEMCHANGEOWNER = 0x00004000;
        const ITEMXATTRMOD = 0x00008000;
        const ITEMISFILE = 0x00010000;
        const ITEMISDIR = 0x00020000;
        const ITEMISSYMLINK = 0x00040000;
        const OWNEVENT = 0x00080000;
        const ITEMISHARDLINK = 0x00100000;
        const ITEMISLASTHARDLINK = 0x00200000;
        const ITEMCLONED = 0x00400000;
    }
}