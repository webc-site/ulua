pub extern "C-unwind" fn throwing(arg: i64) {
  assert_eq!(arg, 25);
  panic!("testing");
}
