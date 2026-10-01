use ulua_common::records::variant::Variant3;

use crate::{
  records::blocked_constraint_registry::ConstraintId,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// `DenseHashMap` 键面哨兵账（b16-bcid-default 侦查定案）：#30 裁定表写"impl 落
/// analysis 定义包"，本波经 rustc 实证该落点**不可达**——本类型是 common
/// `Variant3` 的纯类型别名，`impl DenseDefault for BlockedConstraintId` 双侧皆为
/// 外部定义，孤儿规则直接 E0117（与 Symbol 案不同：Symbol 是本地 records 结构，
/// 别名不产生本地类型，类型参数含本地项也不豁免）。newtype 化需重写 30+ 文件
/// 的变体构造/匹配面，远超本波白名单，判否。
///
/// 收口预案（归 common 侧独立微波）：在 ulua-common `dense_hash_table.rs` 补
/// `impl<T0: DenseDefault, T1, T2> DenseDefault for Variant3<T0, T1, T2>`，体为
/// `Self::V0(T0::dense_default())`（与 cpp `Variant()` 默认构造首选项同构）；对
/// `T0 = TypeId = *const Type` 即得 `V0(null)`，与现存两调用点
/// （constraint_graph 两 find 依赖表调用点的 `new(V0(null::<Type>()))`）
/// 逐位等价，随后可机械 default() 化并补"哨兵可存取不占用"契约单测。
///
/// 预案已于 b16-variant3-dense 波落地：blanket impl 落 ulua-common
/// `dense_hash_table.rs`（本包不再定义任何 `DenseDefault` impl），两调用点已
/// `default()` 化，哨兵契约单测见 `tests/dense_bcid_keys.rs`。
///
/// §2（裸指针 → Rust 类型）V2 分支：cpp `NotNull<Constraint>` 身份令牌
/// `*const Constraint` 已换成 [`ConstraintId`] u32 句柄（与 `SymDefId`/`BlockId`/
/// `InstrId` 同先例），登记/解析单点收口在
/// [`crate::records::blocked_constraint_registry`]；原 `V2(null())` 键面缺省对应
/// `ConstraintId::NULL`。判等/散列语义与迁移前指针键双射等价。
pub type BlockedConstraintId = Variant3<TypeId, TypePackId, ConstraintId>;
