use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  records::{
    blocked_type::BlockedType, blocked_type_pack::BlockedTypePack, extern_type::ExternType,
    free_type::FreeType, free_type_pack::FreeTypePack,
    pending_expansion_type::PendingExpansionType,
    reference_count_initializer::ReferenceCountInitializer, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ReferenceCountInitializer {
  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const FreeType&)`
  /// (Constraint.cpp:26-30).
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, _free_type: &FreeType) -> bool {
    // Safety: mutated_types 由所有构造方以 `&mut local TypeIds` 接线，非空、对齐且
    // 比本 visitor 长寿；单线程串行下经解引用得到的临时可变借用于语句末释放，无别名。
    unsafe {
      (*self.mutated_types).insert_type_id(ty);
    }
    false
  }

  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const BlockedType&)`
  /// (Constraint.cpp:32-36).
  pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _blocked_type: &BlockedType) -> bool {
    // Safety: 同 FreeType 分支——mutated_types 由构造方以 `&mut local TypeIds` 接线，
    // 非空对齐且长寿；单线程串行、临时可变借用语句末释放，无并发别名。
    unsafe {
      (*self.mutated_types).insert_type_id(ty);
    }
    false
  }

  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const PendingExpansionType&)`
  /// (Constraint.cpp:38-42).
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _pending_expansion_type: &PendingExpansionType,
  ) -> bool {
    // Safety: 同上——mutated_types 由构造方以 `&mut local TypeIds` 接线，非空对齐
    // 且比 visitor 长寿；单线程串行下临时可变借用无并发别名。
    unsafe {
      (*self.mutated_types).insert_type_id(ty);
    }
    false
  }

  pub fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
    if matches!(tt.state, TableState::Unsealed | TableState::Free) {
      // Safety: mutated_types 由构造方以 `&mut local TypeIds` 接线，非空对齐且长寿；
      // 向 order 追加为单线程串行下的临时可变借用，语句末即释放，无并发别名。
      unsafe {
        (*self.mutated_types).order.push(ty);
      }
    }

    true
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern_type: &ExternType) -> bool {
    false
  }

  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    _ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    // Safety: tfit.function 是 NonNull<TypeFunction>，其不变量保证非空且对齐；
    // 该 TypeFunction 在实例构造时以 `NonNull::from(&function)` 借用，比实例长寿，
    // 此处仅只读 can_reduce_generics 布尔字段。
    unsafe { (*tfit.function.as_ptr()).can_reduce_generics }
  }

  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    tp: TypePackId,
    _blocked_type_pack: &BlockedTypePack,
  ) -> bool {
    if fflag::LuauConstraintGraph.get() {
      LUAU_ASSERT!(!self.mutated_type_packs.is_null());
      // Safety: 上方 LUAU_ASSERT 已断言 mutated_type_packs 非空；该字段由构造方以
      // `&mut local TypePackIds` 接线，对齐且比 visitor 长寿，单线程串行下临时可变
      // 借用无并发别名。
      unsafe {
        (*self.mutated_type_packs).insert(tp);
      }
    }
    true
  }

  /// C++ `bool ReferenceCountInitializer::visit(TypePackId tp, const FreeTypePack&)`
  /// (Constraint.cpp:74-82).
  pub fn visit_type_pack_id_free_type_pack(
    &mut self,
    tp: TypePackId,
    _free_type_pack: &FreeTypePack,
  ) -> bool {
    if fflag::LuauConstraintGraph.get() {
      LUAU_ASSERT!(!self.mutated_type_packs.is_null());
      // Safety: 同 blocked 分支——LUAU_ASSERT 断言非空，mutated_type_packs 由构造方
      // 以 `&mut local TypePackIds` 接线、对齐且长寿；单线程串行下临时可变借用无别名。
      unsafe {
        (*self.mutated_type_packs).insert(tp);
      }
    }
    true
  }
}
