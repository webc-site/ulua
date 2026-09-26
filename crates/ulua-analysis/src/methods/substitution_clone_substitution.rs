use crate::{
  functions::{
    get_type_pack, shallow_clone_substitution::shallow_clone_type_id_type_arena_txn_log,
  },
  records::{
    substitution::Substitution, type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_pack::TypePack, type_pack_var::TypePackVar, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Substitution {
  pub(crate) fn clone_type_id(&mut self, ty: TypeId) -> TypeId {
    // Safety: self.arena 对应构造期接线的 `NotNull<TypeArena>`，非空、比 Substitution 长寿；
    // 重建的 &mut 独占借用仅存活于本浅拷贝调用，单线程下此刻 arena 无其他借用故无别名冲突。
    let arena = self.wired_arena_mut();
    // Safety: 被调 unsafe fn 契约要求 dest/log 有效——arena 为上一步独占借用，self.base.log 为
    // Substitution 构造注入的非空 TxnLog（empty 单例或活动 log），ty 为遍历中存活的 arena 句柄。
    unsafe { shallow_clone_type_id_type_arena_txn_log(ty, arena, self.base.log) }
  }

  pub(crate) fn clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    // Safety: self.base.log 为 Substitution 构造注入的非空 TxnLog（empty 单例或会话活动 log），
    // follow_type_pack_id 取 &self 只读，返回遍历期内存活的 arena 类型包句柄。
    let mut tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    // Safety: 同上 log 非空存活；pending_type_pack_id 取 &self 只读，返回指向 log 中存活
    // PendingTypePack 的裸指针（可能为 null，下一行判空）。
    let ptp = unsafe { (*self.base.log).pending_type_pack_id(tp) };
    if !ptp.is_null() {
      // Safety: `!ptp.is_null()` 已判空，ptp 指向 log 中存活的 PendingTypePack；共享读 .pending
      // 后取其地址作为 tp，log 节点在整段克隆遍历期内地址稳定、存活。
      tp = unsafe { &(*ptp).pending as *const TypePackVar };
    }

    if let Some(tpp) = get_type_pack::get::<TypePack>(tp) {
      return self.add_type_pack(TypePack::new(tpp.head.clone(), tpp.tail));
    }

    if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tp) {
      let clone = VariadicTypePack {
        ty: vtp.ty,
        hidden: vtp.hidden,
      };
      return self.add_type_pack(clone);
    }

    if let Some(tfitp) = get_type_pack::get::<TypeFunctionInstanceTypePack>(tp) {
      let clone = TypeFunctionInstanceTypePack {
        function: tfitp.function,
        type_arguments: tfitp.type_arguments.clone(),
        pack_arguments: tfitp.pack_arguments.clone(),
      };
      return self.add_type_pack(clone);
    }

    // Safety: tp 为 follow/pending 解析后的 *const TypePackVar；到达此兜底分支前，上方对
    // TypePack/VariadicTypePack/TypeFunctionInstanceTypePack 的 get_type_pack_id 已按同一地址
    // 成功解引用（RTTI 内部读 class_index），故 tp 非空且指向 arena/log 存活节点，clone 仅只读深拷贝。
    self.add_type_pack(unsafe { (*tp).clone() })
  }
}
