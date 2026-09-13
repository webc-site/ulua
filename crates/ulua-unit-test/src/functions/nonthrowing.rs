pub extern "C-unwind" fn nonthrowing(arg: i64) {
  assert_eq!(arg, 25);
}
