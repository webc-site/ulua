use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{type_function_context::TypeFunctionContext, type_once_visitor::TypeOnceVisitor},
  type_aliases::type_id::TypeId,
};
/// cpp `FindUserTypeFunctionBlockers : TypeOnceVisitor`
/// （`Analysis/src/UserDefinedTypeFunction.cpp:49-59`），其成员
/// `NotNull<TypeFunctionContext> ctx` 在 cpp 里只是「本次遍历要问 solver」的别名。
///
/// # 为何是 `&'a mut TypeFunctionContext`（而非 `NonNull`/`Handle` 门面）
/// 本访问者的寿命被调用点严格包住：它在 `user_defined_type_function` 的阻塞检查段
/// 现场构造、遍历完即弃（cpp 亦是同一栈帧的局部 `check`），且全程只经 `ctx->solver`
/// 读一个 `Copy` 裸句柄，不存在跨帧/跨对象持有，故 Rust 的独占借用可以原样表达
/// cpp 的 `NotNull` 而不需要任何句柄包装——`'a` 由构造点唯一决定，编译器直接核验
/// 「遍历期间没有别的路径改写 ctx」。也因此本类型不再是 `Clone`（cpp 侧同样
/// 不可拷贝：`TypeOnceVisitor` 派生自不可复制的访问器状态），原先无人使用的
/// `Clone` derive 随之移除。
#[derive(Debug)]
pub struct FindUserTypeFunctionBlockers<'a> {
  pub base: TypeOnceVisitor,
  pub(crate) ctx: &'a mut TypeFunctionContext,
  pub(crate) blocking_type_map: DenseHashSet<TypeId>,
  pub(crate) blocking_types: Vec<TypeId>,
}

impl<'a> FindUserTypeFunctionBlockers<'a> {
  pub fn new(ctx: &'a mut TypeFunctionContext) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("FindUserTypeFunctionBlockers"), true),
      ctx,
      blocking_type_map: DenseHashSet::default(),
      blocking_types: Vec::new(),
    }
  }
}
