use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{get_mutable_type, get_mutable_type_pack},
  records::{
    arena_id::ArenaId, extern_type::ExternType, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, pending_expansion_type::PendingExpansionType,
    substitution::Substitution, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Substitution {
  /// 对应 C++ `void Substitution::replaceChildren(TypeId ty)`
  /// （`cpp/Analysis/src/Substitution.cpp:769`）。降 safe：`ty` 为 arena `TypeId`
  /// 句柄（同 `get_mutable_type_id` 门面纪律，调用点传的是 Tarjan 产出的新类型），
  /// `self.base.log` 构造注入并 LUAU_ASSERT 非空（C++ 成员 `const TxnLog* log`）；
  /// 对二者的解引用收进下方窄 `unsafe` 块，本体改写下钻全部走安全 RTTI 门面。
  pub fn replace_children_type_id(&mut self, ty: TypeId) {
    // Safety: self.base.log 由 Substitution::substitution_new → substitution_txn_log_type_arena
    // 构造注入并 LUAU_ASSERT 非空，指向会话期内存活的 TxnLog；follow_type_id 取
    // &self 只读解链，仅在断言内使用，借用不跨越后续可变操作。
    unsafe {
      LUAU_ASSERT!(ty == (*self.base.log).follow_type_id(ty));
    }

    if self.base.ignore_children_type_id(ty) {
      return;
    }

    // 对应 C++ `ty->owningArena != arena`；探针窄块内证成句柄前提（上一行已断言
    // ty 为已展开的规范形态、非空存活）。
    if type_owning_arena(ty) != self.wired_arena_id() {
      return;
    }

    if let Some(ftv) = get_mutable_type::get_mutable::<FunctionType>(ty) {
      for generic in &mut ftv.generics {
        *generic = self.replace_type_id(*generic);
      }
      for generic_pack in &mut ftv.generic_packs {
        *generic_pack = self.replace_type_pack_id(*generic_pack);
      }
      ftv.arg_types = self.replace_type_pack_id(ftv.arg_types);
      ftv.ret_types = self.replace_type_pack_id(ftv.ret_types);
    } else if let Some(ttv) = get_mutable_type::get_mutable::<TableType>(ty) {
      LUAU_ASSERT!(ttv.bound_to.is_none());
      for prop in ttv.props.values_mut() {
        if let Some(read_ty) = prop.read_ty {
          prop.read_ty = Some(self.replace_type_id(read_ty));
        }
        if let Some(write_ty) = prop.write_ty {
          prop.write_ty = Some(self.replace_type_id(write_ty));
        }
      }
      if let Some(ref mut indexer) = ttv.indexer {
        indexer.index_type = self.replace_type_id(indexer.index_type);
        indexer.index_result_type = self.replace_type_id(indexer.index_result_type);
      }
      for itp in &mut ttv.instantiated_type_params {
        *itp = self.replace_type_id(*itp);
      }
      for itp in &mut ttv.instantiated_type_pack_params {
        *itp = self.replace_type_pack_id(*itp);
      }
    } else if let Some(mtv) = get_mutable_type::get_mutable::<MetatableType>(ty) {
      mtv.table = self.replace_type_id(mtv.table);
      mtv.metatable = self.replace_type_id(mtv.metatable);
    } else if let Some(utv) = get_mutable_type::get_mutable::<UnionType>(ty) {
      for opt in &mut utv.options {
        *opt = self.replace_type_id(*opt);
      }
    } else if let Some(itv) = get_mutable_type::get_mutable::<IntersectionType>(ty) {
      for part in &mut itv.parts {
        *part = self.replace_type_id(*part);
      }
    } else if let Some(petv) = get_mutable_type::get_mutable::<PendingExpansionType>(ty) {
      for a in &mut petv.type_arguments {
        *a = self.replace_type_id(*a);
      }
      for a in &mut petv.pack_arguments {
        *a = self.replace_type_pack_id(*a);
      }
    } else if let Some(tfit) = get_mutable_type::get_mutable::<TypeFunctionInstanceType>(ty) {
      for a in &mut tfit.type_arguments {
        *a = self.replace_type_id(*a);
      }
      for a in &mut tfit.pack_arguments {
        *a = self.replace_type_pack_id(*a);
      }
    } else if let Some(etv) = get_mutable_type::get_mutable::<ExternType>(ty) {
      for prop in etv.props.values_mut() {
        if let Some(read_ty) = prop.read_ty {
          prop.read_ty = Some(self.replace_type_id(read_ty));
        }
        if let Some(write_ty) = prop.write_ty {
          prop.write_ty = Some(self.replace_type_id(write_ty));
        }
      }
      if let Some(ref mut parent) = etv.parent {
        *parent = self.replace_type_id(*parent);
      }
      if let Some(ref mut metatable) = etv.metatable {
        *metatable = self.replace_type_id(*metatable);
      }
      if let Some(ref mut indexer) = etv.indexer {
        indexer.index_type = self.replace_type_id(indexer.index_type);
        indexer.index_result_type = self.replace_type_id(indexer.index_result_type);
      }
    } else if let Some(ntv) = get_mutable_type::get_mutable::<NegationType>(ty) {
      ntv.ty = self.replace_type_id(ntv.ty);
    }
  }

  /// 类型包孪生版。对应 C++ `void Substitution::replaceChildren(TypePackId tp)`
  /// （`cpp/Analysis/src/Substitution.cpp:871`）。降 safe 理由同上：log/句柄解引用
  /// 均在窄 `unsafe` 块内证成。
  pub fn replace_children_type_pack_id(&mut self, tp: TypePackId) {
    // Safety: self.base.log 由 Substitution::substitution_new → substitution_txn_log_type_arena
    // 构造注入并 LUAU_ASSERT 非空，指向会话期内存活的 TxnLog（对应 C++ 成员 `const TxnLog* log`）；
    // follow_type_pack_id 取 &self 只读解链，仅在断言内使用，借用不跨越后续可变操作。
    unsafe {
      LUAU_ASSERT!(tp == (*self.base.log).follow_type_pack_id(tp));
    }

    if self.base.ignore_children_type_pack_id(tp) {
      return;
    }

    // 同上：对应 C++ `tp->owningArena != arena`。
    if pack_owning_arena(tp) != self.wired_arena_id() {
      return;
    }

    if let Some(tpp) = get_mutable_type_pack::get_mutable::<TypePack>(tp) {
      for tv in tpp.head.iter_mut() {
        *tv = self.replace_type_id(*tv);
      }
      if let Some(tail) = tpp.tail {
        tpp.tail = Some(self.replace_type_pack_id(tail));
      }
    } else if let Some(vtp) = get_mutable_type_pack::get_mutable::<VariadicTypePack>(tp) {
      vtp.ty = self.replace_type_id(vtp.ty);
    } else if let Some(tfitp) =
      get_mutable_type_pack::get_mutable::<TypeFunctionInstanceTypePack>(tp)
    {
      for t in tfitp.type_arguments.iter_mut() {
        *t = self.replace_type_id(*t);
      }
      for t in tfitp.pack_arguments.iter_mut() {
        *t = self.replace_type_pack_id(*t);
      }
    }
  }
}

/// arena 句柄 `owning_arena` 只读探针，解引用收口在私有 helper（同
/// `clone_clone::type_is_persistent` 写法）；ty/tp 为 substitute 遍历传入的
/// arena 存活节点（bump 块地址不移动），仅拷贝值。
fn type_owning_arena(ty: TypeId) -> ArenaId {
  // SAFETY: 见函数 doc——存活对齐 arena 句柄，只读拷贝。
  unsafe { (*ty).owning_arena }
}

fn pack_owning_arena(tp: TypePackId) -> ArenaId {
  // SAFETY: 同上。
  unsafe { (*tp).owning_arena }
}
