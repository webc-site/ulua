use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

/// §2 判定注（B 型·记录判别子 cpp 逐字镜像）：对应 cpp `Tarjan.cpp`
/// `TypeId ty = nullptr; TypePackId tp = nullptr;`——ty/tp 恰一为空即结点种类判别子，
/// 空=「非本类结点」的缺席分支由消费方 `is_null` 判读承载，未吞任何 cpp 分支；
/// 收编为 enum 属记录布局重设计，不在本清扫批。
#[derive(Debug, Clone)]
pub struct TarjanNode {
  pub(crate) ty: TypeId,
  pub(crate) tp: TypePackId,
  pub(crate) on_stack: bool,
  pub(crate) dirty: bool,
  pub(crate) lowlink: i32,
}
