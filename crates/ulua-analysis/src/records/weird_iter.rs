use core::ptr::NonNull;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{get_mutable_txn_log::get_mutable_pending_type_pack, get_mutable_type_pack},
  records::{
    arena_handle::{alias, alias_nn, alias_nn_ref, alias_ref},
    arena_id::ArenaId,
    free_type_pack::FreeTypePack,
    scope::Scope,
    txn_log::TxnLog,
    type_level::TypeLevel,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};

#[derive(Debug, Clone)]
pub struct WeirdIter {
  pub(crate) pack_id: TypePackId,
  pub(crate) log: *mut TxnLog,
  pub(crate) pack: Option<NonNull<TypePack>>,
  pub(crate) index: usize,
  pub(crate) growing: bool,
  pub(crate) level: TypeLevel,
  pub(crate) scope: *mut Scope,
}

impl WeirdIter {
  pub fn weird_iter_good(&self) -> bool {
    self.pack.is_some() && self.index < alias_nn_ref(self.pack.unwrap()).head.len()
  }

  pub fn weird_iter_can_grow(&self) -> bool {
    alias_ref(self.log)
      .txn_log_get::<FreeTypePack, _>(self.pack_id)
      .is_some()
  }

  #[inline]
  pub fn current(&mut self) -> &mut TypeId {
    LUAU_ASSERT!(self.weird_iter_good());
    &mut alias_nn(self.pack.expect("weird_iter_good 判 pack 命中")).head[self.index]
  }

  pub fn weird_iter_type_pack_id_txn_log(&mut self, mut pack_id: TypePackId, log: &mut TxnLog) {
    self.pack_id = pack_id;
    self.log = log as *mut TxnLog;
    self.pack = log.txn_log_get_mutable::<TypePack, TypePackId>(pack_id);
    self.index = 0;
    self.growing = false;
    // 与 C++ WeirdIter 构造一致：沿 tail 链下沉到第一个 head 非空（或无 tail）的 pack。
    while let Some(pack_nn) = self.pack {
      // 判空后解引用；此处只读 `head`/`tail` 两个字段，不写回。
      let pack = alias_nn_ref(pack_nn);
      let Some(next) = pack.tail else {
        break;
      };
      if !pack.head.is_empty() {
        break;
      }
      pack_id = next;
      self.pack = log.txn_log_get_mutable::<TypePack, TypePackId>(pack_id);
    }
  }

  pub fn weird_iter_push_type(&mut self, ty: TypeId) {
    LUAU_ASSERT!(self.pack.is_some());
    // self.log 由 Unifier::try_unify 以 `&mut self.log as *mut _` 取得（Unifier
    // 拥有的 TxnLog 字段），非空且对齐，WeirdIter 生命周期嵌套于该可变借用之内；
    // alias 门面按模块契约借出该字段，queue_type_pack_id 只经 &mut 重建一次借用，
    // 单线程串行遍历此刻无其它存活别名。
    let pending_pack = alias(self.log).queue_type_pack_id(self.pack_id);
    // pending_pack 恒指向 log.type_pack_changes 中 Box<PendingTypePack> 堆节点
    // （Box 地址稳定，log 比本迭代器长寿），Handle 类型编码其非空；
    // get_mutable_pending_type_pack 内部按 RTTI 判别 pendingType 变体，未命中返回
    // None（原 null 哨兵）。Some 分支内 pending 为该 Box 内活着 TypePack 变体的独占
    // 可变借用（Box 不移动，地址稳定）；写 head 仅此一处，单线程串行、无第二别名，
    // 折叠 `NonNull::from` 后存回 self.pack 与原「直接存返回指针」逐位同构。
    let pending = get_mutable_pending_type_pack::<TypePack>(pending_pack);
    if let Some(pending) = pending {
      pending.head.push(ty);
      self.pack = Some(NonNull::from(pending));
    } else {
      LUAU_ASSERT!(false);
    }
  }

  pub fn weird_iter_grow(&mut self, new_tail: TypePackId) {
    LUAU_ASSERT!(self.weird_iter_can_grow());
    LUAU_ASSERT!(get_mutable_type_pack::get_mutable::<TypePack>(new_tail).is_some());

    // 紧邻 LUAU_ASSERT(weird_iter_can_grow()) 即 `get::<FreeTypePack>(pack_id).is_some()`
    // 的判定，下转必命中。
    let free_pack = get_mutable_type_pack::get_mutable::<FreeTypePack>(self.pack_id)
      .expect("紧邻 can_grow 断言即 FreeTypePack 判定，下转必命中");
    self.level = free_pack.level;
    if !free_pack.scope.is_null() {
      self.scope = free_pack.scope;
    }
    alias(self.log).replace_type_pack_id_type_pack_var(
      self.pack_id,
      TypePackVar {
        ty: TypePackVariant::Bound(new_tail),
        persistent: false,
        owning_arena: ArenaId::NONE,
      },
    );
    self.pack_id = new_tail;
    self.pack = get_mutable_type_pack::get_mutable::<TypePack>(new_tail).map(NonNull::from);
    self.index = 0;
    self.growing = true;
  }

  pub fn weird_iter_advance(&mut self) -> bool {
    let Some(pack_nn) = self.pack else {
      return self.weird_iter_good();
    };
    if self.index < alias_nn_ref(pack_nn).head.len() {
      self.index += 1;
    }
    if self.growing || self.index < alias_nn_ref(self.pack.unwrap()).head.len() {
      return self.weird_iter_good();
    }
    if let Some(tail) = alias_nn_ref(self.pack.unwrap()).tail {
      self.pack_id = alias_ref(self.log).follow_type_pack_id(tail);
      self.pack = alias_ref(self.log).txn_log_get_mutable::<TypePack, _>(self.pack_id);
      self.index = 0;
    }
    self.weird_iter_good()
  }
}
