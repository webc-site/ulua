use alloc::{boxed::Box, vec};
use core::mem::replace;

use crate::{
  functions::{follow_type, get_type::type_variant_of},
  records::{
    arena_handle::Handle, free_type::FreeType, pending_type::PendingType,
    pending_type_pack::PendingTypePack, txn_log::TxnLog, r#type::Type, type_arena::TypeArena,
    type_pack_var::TypePackVar, union_type::UnionType,
  },
  type_aliases::{
    bound_type::BoundType,
    type_id::TypeId,
    type_variant::{TypeVariant, TypeVariantMember},
  },
};
impl TxnLog {
  /// # Safety
  /// `arena` 为 `Handle` 句柄，须满足其类型级契约：目标 `TypeArena` 在本调用
  /// 全程存活、借用期内无其它并存可变别名（与原裸指针入参同一契约）。
  pub unsafe fn concat_as_union(&mut self, mut rhs: TxnLog, arena: Handle<TypeArena>) {
    /*
     * Check for cycles.
     *
     * We must not combine a log entry that binds 'a to 'b with a log that
     * binds 'b to 'a.
     *
     * Of the two, identify the one with the 'bigger' scope and eliminate the
     * entry that rebinds it.
     */
    // Snapshot rhs's type-var keys; we do not insert into either map during this
    // loop, so the Box-backed pending pointers remain stable.
    let right_keys: vec::Vec<TypeId> = rhs.type_var_changes.iter().map(|(ty, _)| *ty).collect();

    for right_ty in right_keys {
      // `rightRep` lives in rhs's map.
      let right_rep: *mut PendingType = match rhs.type_var_changes.find_mut(&right_ty) {
        Some(rep) => rep.as_mut() as *mut PendingType,
        None => continue,
      };

      unsafe {
        // Safety: right_rep 指向 rhs 日志映射中 Box<PendingType> 的堆对象——容器
        // rehash 只移动 Box 指针、不移动堆内容，且本轮循环不向任一映射插入，堆对象
        // 与两个日志本身（rhs 由 &mut 独占、self 由本方法持有）全程存活；right_ty 是
        // 来自 rhs 键集合的 TypeId，指向类型 arena bump 块中的存活节点，只读其 ty；
        // left_rep/left_ty 同理由 self 的映射与 BoundType.bound_to（arena 节点）取得，
        // 对两个 PendingType 的 dead 写分别落在各自独占的日志条目上，互不别名。
        if (*right_rep).dead {
          continue;
        }

        // We explicitly use get_if here because we do not wish to do anything
        // if the uncommitted type is already bound to something else.
        let rf = FreeType::get_if(type_variant_of(right_ty));
        if rf.is_none() {
          continue;
        }
        let rf = rf.expect("上一 is_none() 分支已 continue");

        let rb = BoundType::get_if(&(*right_rep).pending.ty);
        if rb.is_none() {
          continue;
        }
        let rb = rb.expect("上一 is_none() 分支已 continue");

        let left_ty: TypeId = rb.bound_to;
        let lf = FreeType::get_if(type_variant_of(left_ty));
        if lf.is_none() {
          continue;
        }
        let lf = lf.expect("上一 is_none() 分支已 continue");

        // `leftRep` lives in self's map.
        let left_rep: *mut PendingType = match self.type_var_changes.find_mut(&left_ty) {
          Some(rep) => rep.as_mut() as *mut PendingType,
          None => continue,
        };

        if (*left_rep).dead {
          continue;
        }

        let lb = BoundType::get_if(&(*left_rep).pending.ty);
        if lb.is_none() {
          continue;
        }
        let lb = lb.expect("上一 is_none() 分支已 continue");

        if lb.bound_to == right_ty {
          // leftTy has been bound to rightTy, but rightTy has also been bound
          // to leftTy. We find the one that belongs to the more deeply nested
          // scope and remove it from the log.
          let discard_left = lf.level.subsumes(&rf.level);

          if discard_left {
            (*left_rep).dead = true;
          } else {
            (*right_rep).dead = true;
          }
        }
      }
    }

    // Snapshot rhs's keys again; loop 2 inserts into self's map but never into rhs.
    let right_keys: vec::Vec<TypeId> = rhs.type_var_changes.iter().map(|(ty, _)| *ty).collect();

    for ty in right_keys {
      // Move the rhs box out so we can `std::move` it into self when needed.
      let dead = match rhs.type_var_changes.find(&ty) {
        Some(rep) => rep.dead,
        None => continue,
      };
      if dead {
        continue;
      }

      // Determine whether self already has a live entry for `ty`.
      let left_live = matches!(self.type_var_changes.find(&ty), Some(rep) if !rep.dead);

      if left_live {
        let (left_clone, right_clone) = {
          let left_rep = self
            .type_var_changes
            .find(&ty)
            .expect("left_live 即由同键 find==Some 判出，中间无删改");
          let right_rep = rhs
            .type_var_changes
            .find(&ty)
            .expect("上方 Some(rep) 命中后才未 continue，rhs 键集无删改");
          (left_rep.pending.clone(), right_rep.pending.clone())
        };

        // arena 句柄契约（目标存活、无并存别名）由本函数 # Safety 入参约定保证；
        // left/right clone 均为刚从存活节点/日志条目拷贝出的独立值，add_type 只追加新节点。
        let left_ty: TypeId = arena.get_mut().add_type::<Type>(left_clone);
        // 同上——right_clone 是独立克隆值，add_type 仅追加节点。
        let right_ty: TypeId = arena.get_mut().add_type::<Type>(right_clone);

        if follow_type::follow(left_ty) == follow_type::follow(right_ty) {
          // typeVarChanges[ty] = std::move(rightRep);
          let right_box = take_pending(&mut rhs, ty);
          *self.type_var_changes.get_or_insert(ty) = right_box;
        } else {
          // typeVarChanges[ty]->pending.ty = UnionType{{leftTy, rightTy}};
          let slot = self.type_var_changes.get_or_insert(ty);
          slot.pending.ty = TypeVariant::Union(UnionType {
            options: vec![left_ty, right_ty],
          });
        }
      } else {
        // typeVarChanges[ty] = std::move(rightRep);
        let right_box = take_pending(&mut rhs, ty);
        *self.type_var_changes.get_or_insert(ty) = right_box;
      }
    }

    let pack_keys: vec::Vec<*const TypePackVar> =
      rhs.type_pack_changes.iter().map(|(tp, _)| *tp).collect();
    for tp in pack_keys {
      let rep = take_pending_pack(&mut rhs, tp);
      *self.type_pack_changes.get_or_insert(tp) = rep;
    }

    self.radioactive |= rhs.radioactive;
  }
}

/// Helper mirroring C++ `std::move(rhs.typeVarChanges[ty])`: removes the box from
/// rhs's map by replacing it with a fresh default and returning the original.
fn take_pending(rhs: &mut TxnLog, ty: TypeId) -> Box<PendingType> {
  let slot = rhs.type_var_changes.get_or_insert(ty);
  let placeholder = Box::new(PendingType {
    // Safety: ty 是 rhs 日志的既有键，指向类型 arena（bump 块）存活节点，clone 只读；
    // 此占位条目随即被标记 dead，与 C++ move 后留下的默认值语义一致。
    pending: unsafe { (*ty).clone() },
    dead: true,
  });
  replace(slot, placeholder)
}

fn take_pending_pack(rhs: &mut TxnLog, tp: *const TypePackVar) -> Box<PendingTypePack> {
  let slot = rhs.type_pack_changes.get_or_insert(tp);
  let placeholder = Box::new(PendingTypePack {
    // Safety: tp 是 rhs pack 日志的既有键，指向 pack arena（bump 块）存活节点，
    // clone 只读；占位条目仅用于替换被移走的 Box，堆地址稳定不受影响。
    pending: unsafe { (*tp).clone() },
  });
  replace(slot, placeholder)
}
