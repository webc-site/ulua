use alloc::string::String;

/// 纯数据 record，无不变量可护：字段直接公开（对齐仓库 records 惯例），
/// 外部按引用读取，免 getter 整串 clone。
#[derive(Debug, Clone, Default)]
pub struct VfsNavigator {
  pub real_path: String,
  pub absolute_real_path: String,
  pub absolute_path_prefix: String,
  pub module_path: String,
  pub absolute_module_path: String,
}
