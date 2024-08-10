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
    exec {0} 0 0 -- {1}/magisk --post-fs-data

on property:vold.decrypt=trigger_restart_framework
    exec {0} 0 0 -- {1}/magisk --service
    exec {0} 0 0 -- {1}/magisk --stop

on nonencrypted
    exec {0} 0 0 -- {1}/magisk --service
    exec {0} 0 0 -- {1}/magisk --stop
"#,
    "u:r:magisk:s0", tmp_dir
    )
    .ok();

    mem::forget(file)
}
