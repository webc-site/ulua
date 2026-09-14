use core::{ffi::c_void, ptr::NonNull};
use std::collections::{HashMap, HashSet};

use ulua_common::{FInt, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack_id,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_mutable_type::get_mutable_type_id, get_tail::get_tail, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, invert_polarity::invert,
  },
  records::{
    builtin_types::BuiltinTypes, counter_state::CounterState, extern_type::ExternType,
    free_type::FreeType, function_type::FunctionType, generic_type::GenericType,
    generic_type_pack::GenericTypePack, intersection_type::IntersectionType,
    metatable_type::MetatableType, negation_type::NegationType, scope::Scope,
    table_type::TableType, type_arena::TypeArena,
    type_function_instance_type::TypeFunctionInstanceType, type_pack::TypePack,
    union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
    type_variant::TypeVariant,
  },
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn prune_unnecessary_generics(
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  _scope: *mut Scope,
  cached_types: *mut DenseHashSet<TypeId>,
  ty: TypeId,
) {
  // SAFETY: builtin_types 指向全局内建类型表（C++ NotNull<BuiltinTypes> 契约）；
  // NonNull 封装解引用。
  let builtin_types = unsafe { NonNull::new_unchecked(builtin_types).as_ref() };

  let ty = follow_type_id(ty);

  // SAFETY: ty 指向 TypeArena 内 Type；读取归属 arena 与 persistent 标志。
  if unsafe { (*ty).owning_arena != arena || (*ty).persistent } {
    return;
  }

  let function_ty = get_mutable_type_id::<FunctionType>(ty);
  let Some(function_ty) = function_ty else {
    return;
  };

  let mut counter = GenericCounterLocal::new(cached_types);

  for generic in function_ty.generics.iter().copied() {
    let generic = follow_type_id(generic);
    if let Some(g) = get_type_id::<GenericType>(generic)
      && !g.explicit_name
    {
      counter.generics.entry(generic).or_default();
    }
  }

  let mut i = 0;
  while i < function_ty.generic_packs.len() {
    let generic_pack = unsafe { follow_type_pack_id(function_ty.generic_packs[i]) };
    let tail = get_tail(generic_pack);

    if tail != generic_pack {
      function_ty.generic_packs.push(tail);
    }

    if let Some(g) = get_type_pack_id::<GenericTypePack>(tail)
      && !g.explicit_name
    {
      counter.generic_packs.entry(generic_pack).or_default();
    }

    i += 1;
  }

  counter.traverse_type_id(ty);

  if !counter.hit_limits {
    for (&generic, state) in counter.generics.iter() {
      if state.count == 1 && state.polarity != Polarity::Mixed {
        // SAFETY: generic 为 TypeId 句柄；读 owning_arena 做归属检查。
        if unsafe { (*generic).owning_arena } != arena {
          continue;
        }

        // SAFETY: 该 Type 由本 arena 分配且引用计数为 1，改写为 Bound 安全。
        unsafe {
          (*as_mutable_type_id(generic)).ty = TypeVariant::Bound(builtin_types.unknown_type)
        };
      }
    }
  }

  let mut seen = HashSet::new();
  function_ty.generics.retain(|generic| {
    let generic = follow_type_id(*generic);
    if !seen.insert(generic) {
      return false;
    }

    if !counter.hit_limits
      && let Some(state) = counter.generics.get(&generic)
      && state.count == 0
    {
      return false;
    }

    get_type_id::<GenericType>(generic).is_some()
  });

  if !counter.hit_limits {
    for (&generic_pack, state) in counter.generic_packs.iter() {
      if state.count == 1 {
        // SAFETY: 同上，generic_pack 引用计数为 1，改写 Bound 安全。
        unsafe {
          (*as_mutable_type_pack_id(generic_pack)).ty =
            TypePackVariant::Bound(builtin_types.unknown_type_pack)
        };
      }
    }
  }

  let mut seen = HashSet::new();
  function_ty.generic_packs.retain(|generic_pack| {
    let generic_pack = unsafe { follow_type_pack_id(*generic_pack) };
    if !seen.insert(generic_pack) {
      return false;
    }

    if !counter.hit_limits
      && let Some(state) = counter.generic_packs.get(&generic_pack)
      && state.count == 0
    {
      return false;
    }

    get_type_pack_id::<GenericTypePack>(generic_pack).is_some()
  });
}

struct GenericCounterLocal {
  /// C++ `GenericTypeVisitor::seen` (VisitType.h:76) — the recursion-stack
  /// guard inherited by `GenericCounter : TypeVisitor`. Distinct from
  /// `seen_counts`: this set is cleared on the way back up (`unsee`) so the
  /// same type can be counted via multiple sibling paths, but a type that is
  /// currently *on the stack* (a cycle, e.g. `t1 = Instance & { IsA: (t1,
  /// ...) }`) is not re-entered — without it the recursive type is traversed
  /// repeatedly at flipped polarity, double-counting generics and forcing
  /// their polarity to `Mixed`.
  seen: HashSet<*const c_void>,
  seen_counts: HashMap<TypeId, usize>,
  seen_pack_counts: HashMap<TypePackId, usize>,
  generics: HashMap<TypeId, CounterState>,
  generic_packs: HashMap<TypePackId, CounterState>,
  polarity: Polarity,
  steps: i32,
  hit_limits: bool,
}

impl GenericCounterLocal {
  fn new(cached_types: *mut DenseHashSet<TypeId>) -> Self {
    let _ = cached_types;
    Self {
      seen: HashSet::new(),
      seen_counts: HashMap::new(),
      seen_pack_counts: HashMap::new(),
      generics: HashMap::new(),
      generic_packs: HashMap::new(),
      polarity: Polarity::Positive,
      steps: 0,
      hit_limits: false,
    }
  }

  fn check_limits(&mut self) -> bool {
    self.steps += 1;
    self.hit_limits |= self.steps > FInt::LuauGenericCounterMaxSteps.get();
    !self.hit_limits
  }

  fn traverse_type_id(&mut self, ty: TypeId) {
    if ty.is_null() || !self.check_limits() {
      return;
    }

    let ty = follow_type_id(ty);

    // C++ `GenericTypeVisitor::traverse` recursion-stack `seen` guard
    // (VisitType.h:235): skip a type already on the stack, then `unsee` it
    // afterwards so sibling paths still count it.
    let key = ty as *const c_void;
    if !self.seen.insert(key) {
      return;
    }
    self.dispatch_type_id(ty);
    self.seen.remove(&key);
  }

  fn dispatch_type_id(&mut self, ty: TypeId) {
    if get_type_id::<GenericType>(ty).is_some() {
      if let Some(state) = self.generics.get_mut(&ty) {
        state.count += 1;
        state.polarity |= self.polarity;
      }
      return;
    }

    if get_type_id::<ExternType>(ty).is_some() {
      return;
    }

    if let Some(ft) = get_type_id::<FunctionType>(ty) {
      // SAFETY: ty 指向 TypeArena 内 Type；仅读 persistent 标志。
      if unsafe { (*ty).persistent } {
        return;
      }

      let seen_count = self.seen_counts.entry(ty).or_default();
      if *seen_count > 1 {
        return;
      }
      *seen_count += 1;

      self.polarity = invert(self.polarity);
      self.traverse_type_pack_id(ft.arg_types);
      self.polarity = invert(self.polarity);
      self.traverse_type_pack_id(ft.ret_types);
      return;
    }

    if let Some(tt) = get_type_id::<TableType>(ty) {
      // SAFETY: ty 指向 TypeArena 内 Type；仅读 persistent 标志。
      if unsafe { (*ty).persistent } {
        return;
      }

      let seen_count = self.seen_counts.entry(ty).or_default();
      if *seen_count > 1 {
        return;
      }
      *seen_count += 1;

      let previous = self.polarity;
      for prop in tt.props.values() {
        if prop.is_read_only() {
          if let Some(read_ty) = prop.read_ty {
            self.traverse_type_id(read_ty);
          }
        } else if prop.is_write_only() {
          let p = self.polarity;
          self.polarity = Polarity::Negative;
          if let Some(write_ty) = prop.write_ty {
            self.traverse_type_id(write_ty);
          }
          self.polarity = p;
        } else if prop.is_shared() {
          let p = self.polarity;
          self.polarity = Polarity::Mixed;
          if let Some(read_ty) = prop.read_ty {
            self.traverse_type_id(read_ty);
          }
          self.polarity = p;
        } else {
          if let Some(read_ty) = prop.read_ty {
            self.traverse_type_id(read_ty);
          }

          let p = self.polarity;
          self.polarity = Polarity::Negative;
          if let Some(write_ty) = prop.write_ty {
            self.traverse_type_id(write_ty);
          }
          self.polarity = p;
        }
      }

      if let Some(indexer) = &tt.indexer {
        self.polarity = Polarity::Mixed;
        self.traverse_type_id(indexer.index_type);
        self.traverse_type_id(indexer.index_result_type);
        self.polarity = previous;
      }
      return;
    }

    if let Some(ft) = get_type_id::<FreeType>(ty) {
      self.traverse_type_id(ft.lower_bound);
      self.traverse_type_id(ft.upper_bound);
    } else if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty) {
      let seen_count = self.seen_counts.entry(ty).or_default();
      if *seen_count > 1 {
        return;
      }
      *seen_count += 1;

      for &arg in &tfit.type_arguments {
        self.traverse_type_id(arg);
      }

      for &arg_pack in &tfit.pack_arguments {
        self.traverse_type_pack_id(arg_pack);
      }
    } else if let Some(ut) = get_type_id::<UnionType>(ty) {
      for &option in &ut.options {
        self.traverse_type_id(option);
      }
    } else if let Some(it) = get_type_id::<IntersectionType>(ty) {
      for &part in &it.parts {
        self.traverse_type_id(part);
      }
    } else if let Some(mt) = get_type_id::<MetatableType>(ty) {
      self.traverse_type_id(mt.table);
      self.traverse_type_id(mt.metatable);
    } else if let Some(nt) = get_type_id::<NegationType>(ty) {
      self.traverse_type_id(nt.ty);
    }
  }

  fn traverse_type_pack_id(&mut self, tp: TypePackId) {
    if tp.is_null() || !self.check_limits() {
      return;
    }

    let tp = unsafe { follow_type_pack_id(tp) };

    let key = tp as *const c_void;
    if !self.seen.insert(key) {
      return;
    }
    self.dispatch_type_pack_id(tp);
    self.seen.remove(&key);
  }

  fn dispatch_type_pack_id(&mut self, tp: TypePackId) {
    if get_type_pack_id::<GenericTypePack>(tp).is_some() {
      if let Some(state) = self.generic_packs.get_mut(&tp) {
        state.count += 1;
        state.polarity |= self.polarity;
      }
      return;
    }

    let seen_count = self.seen_pack_counts.entry(tp).or_default();
    if *seen_count > 1 {
      return;
    }
    *seen_count += 1;

    if let Some(pack) = get_type_pack_id::<TypePack>(tp) {
      for &head in &pack.head {
        self.traverse_type_id(head);
      }

      if let Some(tail) = pack.tail {
        self.traverse_type_pack_id(tail);
      }
    } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp) {
      self.traverse_type_id(vtp.ty);
    }
  }
}
