use alloc::string::String;

pub(crate) fn write_byte(ss: &mut String, value: u8) {
  unsafe {
    ss.as_mut_vec().push(value);
  }
}
