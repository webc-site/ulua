use alloc::vec::Vec;

pub fn split_path(path: &str) -> Vec<&str> {
  let mut components = Vec::new();

  let mut pos = 0;
  let bytes = path.as_bytes();

  while let Some(next_pos_offset) = bytes[pos..].iter().position(|&b| b == b'\\' || b == b'/') {
    let next_pos = pos + next_pos_offset;
    components.push(&path[pos..next_pos]);
    pos = next_pos + 1;
  }

  components.push(&path[pos..]);

  components
}
