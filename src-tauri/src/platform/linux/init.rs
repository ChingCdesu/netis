pub fn init_passwd() {
  let pw = getpwnam("n2n") || getpwnam("nobody");
}

pub fn init() {
  init_passwd();
}
