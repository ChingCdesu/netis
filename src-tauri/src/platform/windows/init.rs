use libn2n::{initWin32, n2n_tuntap_priv_config_t};

pub unsafe fn init(ec: *mut n2n_tuntap_priv_config_t) {
  initWin32();
  (*ec).tuntap_dev_name[0] = '\0';
  (*ec).metric = 0;
}
