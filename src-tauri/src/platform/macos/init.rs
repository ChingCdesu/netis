use std::ffi::CString;

use libc::{getpwnam, passwd};
use libn2n::{n2n_edge_conf_t, n2n_tuntap_priv_config_t};

pub fn init_passwd() -> *mut passwd {
    let n2n_name = CString::new("n2n").unwrap();
    let nobody_name = CString::new("nobody").unwrap();
    let n2n_pw = unsafe { getpwnam(n2n_name.as_ptr()) };
    let nobody_pw = unsafe { getpwnam(nobody_name.as_ptr()) };
    if !n2n_pw.is_null() {
        return n2n_pw;
    } else if !nobody_pw.is_null() {
        return nobody_pw;
    } else {
        return std::ptr::null_mut();
    }
}

pub unsafe fn init(ec: *mut n2n_tuntap_priv_config_t) {
    let pw = init_passwd();
    if !pw.is_null() {
        (*ec).userid = (*pw).pw_uid;
        (*ec).groupid = (*pw).pw_gid;

        (*ec).tuntap_dev_name =
            unsafe { std::mem::transmute(CString::new("edge0").unwrap().as_bytes()) };
        (*ec).netmask =
            unsafe { std::mem::transmute(CString::new("255.255.255.0").unwrap().as_bytes()) };
        std::mem::forget(pw);
    }
}
