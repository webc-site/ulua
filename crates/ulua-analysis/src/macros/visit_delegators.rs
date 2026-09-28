//! 类型遍历 trait 默认委托方法生成宏。
//!
//! C++ `VisitType.h` 中 `GenericTypeVisitor` 与 `IterativeTypeVisitor`
//! 携带完全相同的 28 个按变体重载的默认实现：转发到裸 `visit(TypeId)` /
//! `visit(TypePackId)`。本宏在 trait 体内展开这批默认方法，供
//! `records/generic_type_visitor.rs` 与 `records/iterative_type_visitor.rs`
//! 共用，消除逐文件手写重复。

/// 在 trait 体内生成全部 `visit_type_id_*` / `visit_type_pack_id_*`
/// 默认委托方法（与原手写转发实现逐条等价）。要求调用处已导入各变体
/// 类型以及 `TypeId`/`TypePackId`。
macro_rules! visit_type_delegators {
  () => {
    fn visit_type_id_bound_type(&mut self, ty: TypeId, _btv: &BoundType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_free_type(&mut self, ty: TypeId, _ftv: &FreeType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_generic_type(&mut self, ty: TypeId, _gtv: &GenericType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_error_type(&mut self, ty: TypeId, _etv: &ErrorType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_primitive_type(&mut self, ty: TypeId, _ptv: &PrimitiveType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_function_type(&mut self, ty: TypeId, _ftv: &FunctionType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_table_type(&mut self, ty: TypeId, _ttv: &TableType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_metatable_type(&mut self, ty: TypeId, _mtv: &MetatableType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_extern_type(&mut self, ty: TypeId, _etv: &ExternType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_any_type(&mut self, ty: TypeId, _atv: &AnyType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_no_refine_type(&mut self, ty: TypeId, _nrt: &NoRefineType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_unknown_type(&mut self, ty: TypeId, _utv: &UnknownType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_never_type(&mut self, ty: TypeId, _ntv: &NeverType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_union_type(&mut self, ty: TypeId, _utv: &UnionType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_intersection_type(&mut self, ty: TypeId, _itv: &IntersectionType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_blocked_type(&mut self, ty: TypeId, _btv: &BlockedType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_pending_expansion_type(
      &mut self,
      ty: TypeId,
      _petv: &PendingExpansionType,
    ) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_singleton_type(&mut self, ty: TypeId, _stv: &SingletonType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_negation_type(&mut self, ty: TypeId, _ntv: &NegationType) -> bool {
      self.visit_type_id(ty)
    }
    fn visit_type_id_type_function_instance_type(
      &mut self,
      ty: TypeId,
      _tfit: &TypeFunctionInstanceType,
    ) -> bool {
      self.visit_type_id(ty)
    }

    fn visit_type_pack_id_bound_type_pack(&mut self, tp: TypePackId, _btp: &BoundTypePack) -> bool {
      self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, _ftp: &FreeTypePack) -> bool {
      self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_generic_type_pack(
      &mut self,
      tp: TypePackId,
      _gtp: &GenericTypePack,
    ) -> bool {
      self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_error_type_pack(&mut self, tp: TypePackId, _etp: &ErrorTypePack) -> bool {
      self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_type_pack(&mut self, tp: TypePackId, _pack: &TypePack) -> bool {
      self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_variadic_type_pack(
      &mut self,
      tp: TypePackId,
      _vtp: &VariadicTypePack,
    ) -> bool {
      self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_blocked_type_pack(
      &mut self,
      tp: TypePackId,
      _btp: &BlockedTypePack,
    ) -> bool {
      self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_type_function_instance_type_pack(
      &mut self,
      tp: TypePackId,
      _tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
      self.visit_type_pack_id(tp)
    }
  };
}
pub(crate) use visit_type_delegators;
