use alloc::vec;
use core::mem::replace;

use crate::{
  functions::{follow_type, get_type::type_variant_of},
  records::{
    arena_handle::{alias, alias_ref},
    free_type::FreeType,
    pending_slot::PendingSlot,
    pending_type::PendingType,
    pending_type_pack::PendingTypePack,
    txn_log::TxnLog,
    r#type::Type,
    type_arena::TypeArena,
    type_pack_var::TypePackVar,
    union_type::UnionType,
  },
  type_aliases::{
    bound_type::BoundType,
    type_id::TypeId,
    type_variant::{TypeVariant, TypeVariantMember},
  },
};
impl TxnLog {
  pub fn concat_as_union(&mut self, mut rhs: TxnLog, arena: &mut TypeArena) {
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
    // loop, so the PendingSlot-backed pending pointers remain stable.
    let right_keys: vec::Vec<TypeId> = rhs.type_var_changes.iter().map(|(ty, _)| *ty).collect();

    for right_ty in right_keys {
      // `rightRep` lives in rhs's map.
      let right_rep: *mut PendingType = match rhs.type_var_changes.find_mut(&right_ty) {
        Some(rep) => rep.get_mut() as *mut PendingType,
        None => continue,
      };

      if alias_ref(right_rep).dead {
        continue;
      }

      // We explicitly use get_if here because we do not wish to do anything
      // if the uncommitted type is already bound to something else.
      let rf = FreeType::get_if(type_variant_of(right_ty));
      if rf.is_none() {
        continue;
      }
      let rf = rf.expect("上一 is_none() 分支已 continue");

      let rb = BoundType::get_if(&alias_ref(right_rep).pending.ty);
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
        Some(rep) => rep.get_mut() as *mut PendingType,
        None => continue,
      };

      if alias_ref(left_rep).dead {
        continue;
      }

      let lb = BoundType::get_if(&alias_ref(left_rep).pending.ty);
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
          alias(left_rep).dead = true;
        } else {
          alias(right_rep).dead = true;
        }
      }
    }

    // Snapshot rhs's keys again; loop 2 inserts into self's map but never into rhs.
    let right_keys: vec::Vec<TypeId> = rhs.type_var_changes.iter().map(|(ty, _)| *ty).collect();

    for ty in right_keys {
      // Move the rhs slot out so we can `std::move` it into self when needed.
      let dead = match rhs.type_var_changes.find(&ty) {
        Some(rep) => rep.get().dead,
        None => continue,
      };
      if dead {
        continue;
      }

      // Determine whether self already has a live entry for `ty`.
      let left_live = matches!(self.type_var_changes.find(&ty), Some(rep) if !rep.get().dead);

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
          (
            left_rep.get().pending.clone(),
            right_rep.get().pending.clone(),
          )
        };

        let left_ty: TypeId = arena.add_type::<Type>(left_clone);
        let right_ty: TypeId = arena.add_type::<Type>(right_clone);

        if follow_type::follow(left_ty) == follow_type::follow(right_ty) {
          // typeVarChanges[ty] = std::move(rightRep);
          let right_box = take_pending(&mut rhs, ty);
          *self.type_var_changes.get_or_insert(ty) = right_box;
        } else {
          // typeVarChanges[ty]->pending.ty = UnionType{{leftTy, rightTy}};
          let slot = self.type_var_changes.get_or_insert(ty);
          slot.get_mut().pending.ty = TypeVariant::Union(UnionType {
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

/// Helper mirroring C++ `std::move(rhs.typeVarChanges[ty])`: removes the slot from
/// rhs's map by replacing it with a fresh tombstone and returning the original.
fn take_pending(rhs: &mut TxnLog, ty: TypeId) -> PendingSlot<PendingType> {
  let slot = rhs.type_var_changes.get_or_insert(ty);
  let placeholder = PendingSlot::new(PendingType {
    // 此占位条目随即被标记 dead，与 C++ move 后留下的默认值语义一致。
    pending: alias_ref(ty).clone(),
    dead: true,
  });
  replace(slot, placeholder)
}

fn take_pending_pack(rhs: &mut TxnLog, tp: *const TypePackVar) -> PendingSlot<PendingTypePack> {
  let slot = rhs.type_pack_changes.get_or_insert(tp);
  let placeholder = PendingSlot::new(PendingTypePack {
    // 占位条目仅用于替换被移走的槽，堆地址稳定不受影响。
    pending: alias_ref(tp).clone(),
  });
  replace(slot, placeholder)
}
