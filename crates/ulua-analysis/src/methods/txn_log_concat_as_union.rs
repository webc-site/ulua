use alloc::{boxed::Box, vec};
use core::mem::replace;

use crate::{
  functions::follow_type::follow_type_id,
  records::{
    free_type::FreeType, pending_type::PendingType, pending_type_pack::PendingTypePack,
    txn_log::TxnLog, r#type::Type, type_arena::TypeArena, type_pack_var::TypePackVar,
    union_type::UnionType,
  },
  type_aliases::{
    bound_type::BoundType,
    type_id::TypeId,
    type_variant::{TypeVariant, TypeVariantMember},
  },
};
impl TxnLog {
  /// # Safety
  /// 调用方须保证 `arena` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn concat_as_union(&mut self, mut rhs: TxnLog, arena: *mut TypeArena) {
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
        if (*right_rep).dead {
          continue;
        }

        // We explicitly use get_if here because we do not wish to do anything
        // if the uncommitted type is already bound to something else.
        let rf = FreeType::get_if(&(*right_ty).ty);
        if rf.is_none() {
          continue;
        }
        let rf = rf.unwrap();

        let rb = BoundType::get_if(&(*right_rep).pending.ty);
        if rb.is_none() {
          continue;
        }
        let rb = rb.unwrap();

        let left_ty: TypeId = rb.bound_to;
        let lf = FreeType::get_if(&(*left_ty).ty);
        if lf.is_none() {
          continue;
        }
        let lf = lf.unwrap();

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
        let lb = lb.unwrap();

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
          let left_rep = self.type_var_changes.find(&ty).unwrap();
          let right_rep = rhs.type_var_changes.find(&ty).unwrap();
          (left_rep.pending.clone(), right_rep.pending.clone())
        };

        let left_ty: TypeId = unsafe { (*arena).add_type::<Type>(left_clone) };
        let right_ty: TypeId = unsafe { (*arena).add_type::<Type>(right_clone) };

        if follow_type_id(left_ty) == follow_type_id(right_ty) {
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
    pending: unsafe { (*ty).clone() },
    dead: true,
  });
  replace(slot, placeholder)
}

fn take_pending_pack(rhs: &mut TxnLog, tp: *const TypePackVar) -> Box<PendingTypePack> {
  let slot = rhs.type_pack_changes.get_or_insert(tp);
  let placeholder = Box::new(PendingTypePack {
    pending: unsafe { (*tp).clone() },
  });
  replace(slot, placeholder)
}
