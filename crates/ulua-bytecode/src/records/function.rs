use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Function {
  /// cpp `std::string data`：该函数预序列化的字节码 blob（原始字节，非 UTF-8）。
  pub(crate) data: Vec<u8>,
  pub(crate) maxstacksize: u8,
  pub(crate) numparams: u8,
  pub(crate) numupvalues: u8,
  pub(crate) isvararg: bool,
  pub(crate) debugname: u32,
  pub(crate) debuglinedefined: i32,
  pub(crate) dump: String,
  pub(crate) dumpname: String,
  pub(crate) dumpinstoffs: Vec<i32>,
  /// cpp `std::string typeinfo`：类型编码字节流（LBC_TYPE_* 原始字节）。
  pub(crate) typeinfo: Vec<u8>,
}
