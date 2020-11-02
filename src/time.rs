pub fn sleep(seconds: u64) {
  std::thread::sleep(std::time::Duration::from_secs(seconds));
}

pub fn sleep_ms(ms: u64) {
  std::thread::sleep(std::time::Duration::from_millis(ms));
}

