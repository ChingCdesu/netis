use libc::{getpwnam, passwd, setuid};

pub fn init_passwd() {
  let pw = getpwnam("n2n") || getpwnam("nobody");
}

pub fn init() {
  init_passwd();
}

pub unsafe fn init_user() {
  if setuid(0) != 0 {
    panic!("unable to become root");
  }
}