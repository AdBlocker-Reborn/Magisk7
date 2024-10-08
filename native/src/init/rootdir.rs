use std::fs::File;
use std::io::Write;
use std::mem;
use std::os::fd::{FromRawFd, RawFd};

use base::{debug, Utf8CStr};

pub fn inject_magisk_rc(fd: RawFd, tmp_dir: &Utf8CStr) {
    debug!("Injecting magisk rc");

    let mut file = unsafe { File::from_raw_fd(fd) };

    write!(
        file,
        r#"
on post-fs-data
    start logd
    umount /system/etc/init/hw/init.zygote64_32.rc
    umount /system/etc/init/hw/init.zygote32.rc
    umount /system/etc/init/hw/init.rc
    mount none none /debug_ramdisk private rec
    mkdir /dev/test1
    mount tmpfs tmpfs /dev/test1
"#)
    .ok();

    mem::forget(file)
}
