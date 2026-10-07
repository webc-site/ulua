use ulua_common::{
  records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseEqDefault},
  type_aliases::dense_hash_default::DenseHashDefault,
};

use crate::records::string_ref::StringRef;

/// `BytecodeBuilder::stringTable` 的具体类型（cpp `DenseHashMap<StringRef, uint32_t>`）。
///
/// 字符串表要**按插入遍历序落盘**（`write_string_table`/`get_string_table`/`finalize`
/// 里的容量累加依赖它），故保持 FNV 版 `DenseHashDefault`，迭代序即契约；
/// 空槽哨兵是 `StringRef::default()`（= cpp 的 `data == nullptr`），空串键不与之冲突。
pub(crate) type StringTable<'a> =
  DenseHashMap<StringRef<'a>, u32, DenseHashDefault<StringRef<'a>>, DenseEqDefault<StringRef<'a>>>;
