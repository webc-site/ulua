use alloc::string::String;

pub fn to_utf_8(path: &[u16]) -> String {
  String::from_utf16_lossy(path)
}
