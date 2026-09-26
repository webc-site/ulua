use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType, type_function_type_pack::TypeFunctionTypePack,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
  },
};

/// 生成逐变体 visit 钩子的默认实现（cpp `GenericTypeFunctionTypeVisitor` 的
/// 虚函数默认值）：忽略变体负载、统一委托到裸 `visit(TypeFunctionTypeId)` /
/// `visit(TypeFunctionTypePackId)`，返回其布尔结果（true = 继续遍历子节点）。
macro_rules! default_visit_hooks {
  ($($name:ident($handle:ident : $handle_ty:ty, $payload:ident : $payload_ty:ty) -> $delegate:ident;)*) => {
    impl IterativeTypeFunctionTypeVisitor {
      $(
        pub fn $name(&mut self, $handle: $handle_ty, $payload: &$payload_ty) -> bool {
          let _ = $payload;
          self.$delegate($handle)
        }
      )*
    }
  };
}

default_visit_hooks! {
  visit_type_function_type_id_type_function_primitive_type(
    ty: TypeFunctionTypeId, tfpt: TypeFunctionPrimitiveType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_any_type(
    ty: TypeFunctionTypeId, tfat: TypeFunctionAnyType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_unknown_type(
    ty: TypeFunctionTypeId, tfut: TypeFunctionUnknownType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_never_type(
    ty: TypeFunctionTypeId, tfnt: TypeFunctionNeverType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_singleton_type(
    ty: TypeFunctionTypeId, tfst: TypeFunctionSingletonType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_union_type(
    ty: TypeFunctionTypeId, tfut: TypeFunctionUnionType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_intersection_type(
    ty: TypeFunctionTypeId, tfit: TypeFunctionIntersectionType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_negation_type(
    ty: TypeFunctionTypeId, tfnt: TypeFunctionNegationType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_function_type(
    ty: TypeFunctionTypeId, tfft: TypeFunctionFunctionType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_table_type(
    ty: TypeFunctionTypeId, tftt: TypeFunctionTableType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_extern_type(
    ty: TypeFunctionTypeId, tfet: TypeFunctionExternType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_generic_type(
    ty: TypeFunctionTypeId, tfgt: TypeFunctionGenericType
  ) -> visit_type_function_type_id;
  visit_type_function_type_pack_id_type_function_type_pack(
    tp: TypeFunctionTypePackId, tftp: TypeFunctionTypePack
  ) -> visit_type_function_type_pack_id;
  visit_type_function_type_pack_id_type_function_variadic_type_pack(
    tp: TypeFunctionTypePackId, tfvtp: TypeFunctionVariadicTypePack
  ) -> visit_type_function_type_pack_id;
  visit_type_function_type_pack_id_type_function_generic_type_pack(
    tp: TypeFunctionTypePackId, tfgtp: TypeFunctionGenericTypePack
  ) -> visit_type_function_type_pack_id;
}

impl IterativeTypeFunctionTypeVisitor {
  pub fn visit_type_function_type_pack_id(&mut self, _tp: TypeFunctionTypePackId) -> bool {
    true
  }
}
