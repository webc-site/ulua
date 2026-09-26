use core::ptr::{from_mut, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{get_mutable_txn_log::get_mutable_pending_type_pack, get_mutable_type_pack},
  records::{
    arena_id::ArenaId, free_type_pack::FreeTypePack, scope::Scope, txn_log::TxnLog,
    type_level::TypeLevel, type_pack::TypePack, type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};

#[derive(Debug, Clone)]
pub struct WeirdIter {
  pub(crate) pack_id: TypePackId,
  pub(crate) log: *mut TxnLog,
  pub(crate) pack: *mut TypePack,
  pub(crate) index: usize,
  pub(crate) growing: bool,
  pub(crate) level: TypeLevel,
  pub(crate) scope: *mut Scope,
}

impl WeirdIter {
  pub fn weird_iter_good(&self) -> bool {
    !self.pack.is_null() && self.index < unsafe { (*self.pack).head.len() }
  }

  pub fn weird_iter_can_grow(&self) -> bool {
    unsafe { (*self.log).txn_log_get::<FreeTypePack, _>(self.pack_id) }.is_some()
  }

  #[inline]
  pub fn current(&mut self) -> &mut TypeId {
    LUAU_ASSERT!(self.weird_iter_good());
    unsafe {
      let pack = &mut *self.pack;
      &mut pack.head[self.index]
    }
  }

  #[inline]
  pub fn weird_iter_operator_deref(&mut self) -> &mut TypeId {
    self.current()
  }

  pub fn weird_iter_type_pack_id_txn_log(&mut self, mut pack_id: TypePackId, log: &mut TxnLog) {
    self.pack_id = pack_id;
    self.log = log as *mut TxnLog;
    self.pack = log.txn_log_get_mutable::<TypePack, TypePackId>(pack_id);
    self.index = 0;
    self.growing = false;
    // 与 C++ WeirdIter 构造一致：沿 tail 链下沉到第一个 head 非空（或无 tail）的 pack。
    loop {
      if self.pack.is_null() {
        break;
      }
      // Safety: 判空后解引用。`self.pack` 由 `log.txn_log_get_mutable::<TypePack, _>` 产出，
      // 其契约为「null 或指向 TxnLog 内 `Box<PendingType>` 槽位、或 TypeArena bump 块内的
      // TypePackVar 节点」；两类存储地址在 log/arena 存活期内恒定（bump 块不搬家、Box 不重
      // 分配），故引用在使用点始终有效。此处只读 `head`/`tail` 两个字段，不写回，单线程串行
      // 遍历中无并发可变别名。
      let pack = unsafe { &*self.pack };
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
    LUAU_ASSERT!(!self.pack.is_null());
    // Safety: self.log 由 Unifier::try_unify 以 `&mut self.log as *mut _` 取得（Unifier
    // 拥有的 TxnLog 字段），非空且对齐，WeirdIter 生命周期嵌套于该可变借用之内；
    // queue_type_pack_id 只经 &mut 重建一次借用，单线程串行遍历此刻无其它存活别名。
    let pending_pack = unsafe { (*self.log).queue_type_pack_id(self.pack_id) };
    // Safety: pending_pack 恒为 log.type_pack_changes 中 Box<PendingTypePack> 堆节点
    // （Box 地址稳定，log 比本迭代器长寿），满足 get_mutable_pending_type_pack 的
    // pending 非空/存活契约；内部按 RTTI 判别 pendingType 变体，未命中返回 None
    // （原 null 哨兵）。Some 分支内 pending 为该 Box 内活着 TypePack 变体的独占
    // 可变借用（Box 不移动，地址稳定）；写 head 仅此一处，单线程串行、无第二别名，
    // 物化 `from_mut` 后的裸句柄存回 self.pack 与原「直接存返回指针」逐位同构。
    let pending = unsafe { get_mutable_pending_type_pack::<TypePack>(pending_pack) };
    if let Some(pending) = pending {
      pending.head.push(ty);
      self.pack = from_mut(pending);
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
    unsafe {
      (*self.log).replace_type_pack_id_type_pack_var(
        self.pack_id,
        TypePackVar {
          ty: TypePackVariant::Bound(new_tail),
          persistent: false,
          owning_arena: ArenaId::NONE,
        },
      );
    }
    self.pack_id = new_tail;
    self.pack =
      get_mutable_type_pack::get_mutable::<TypePack>(new_tail).map_or(null_mut(), from_mut);
    self.index = 0;
    self.growing = true;
  }

  pub fn weird_iter_advance(&mut self) -> bool {
    if self.pack.is_null() {
      return self.weird_iter_good();
    }
    // Safety: 上方 is_null 早退保证 `self.pack` 非空——它由构造期
    // `log.getMutable<TypePack>(packId)` 或后续 tail 跟进取得，指向类型 arena
    // 驻留的 TypePack 节点（bump arena 块地址永不移动，比本迭代器长寿）；
    // 此处仅只读 `head.len()`，无并存的可变借用。
    if self.index < unsafe { (*self.pack).head.len() } {
      self.index += 1;
    }
    // Safety: 同上——pack 非空（函数头守卫）且指向存活 arena TypePack 节点，
    // 本语句仍只读 `head.len()`，index 的更新不触及该节点。
    if self.growing || self.index < unsafe { (*self.pack).head.len() } {
      return self.weird_iter_good();
    }
    // Safety: 同上只读守卫——`pack` 非空存活，`tail` 字段按值拷出
    // （Option<Copy 指针>），拷出后不再依赖该借用。
    if let Some(tail) = unsafe { (*self.pack).tail } {
      // Safety: `self.log` 由构造点 `&mut self.log as *mut _` 取自 Unifier
      // 自有的 TxnLog 字段，迭代器严格短于该 Unifier；此处重建 `&` 共享短
      // 借用调用纯函数式 follow（C++ `log.follow(...)`），语句结束即归还。
      self.pack_id = unsafe { (*self.log).follow_type_pack_id(tail) };
      // Safety: 同一 log 字段的共享短借用——`txn_log_get_mutable` 以 `&self`
      // 查询并返回 arena 驻留节点裸指针（可能为 null，已由调用方 is_null 语义
      // 消费），写回 `self.pack` 仅是句柄替换，与 C++ 同语句同构。
      self.pack = unsafe { (*self.log).txn_log_get_mutable::<TypePack, _>(self.pack_id) };
      self.index = 0;
    }
    self.weird_iter_good()
  }
}
