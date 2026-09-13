// 块头 metadata 槽存指针值（所属页或空闲链），按字节语义用 *mut u8
macro_rules! metadata {
  ($block:expr) => {
    *($block as *mut *mut u8)
  };
}

pub(crate) use metadata;
