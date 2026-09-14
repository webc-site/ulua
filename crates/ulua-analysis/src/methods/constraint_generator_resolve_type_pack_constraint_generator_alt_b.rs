use alloc::sync::Arc;
use core::ffi::CStr;

use ulua_ast::records::{
  ast_node::AstNode, ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
  ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
};
use ulua_common::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{
    follow_type_pack::follow_type_pack_id, get_mutable_type_pack::get_mutable_type_pack_id,
  },
  records::{
    constraint_generator::ConstraintGenerator,
    generic_type_pack::GenericTypePack,
    module::Module,
    scope::Scope,
    unknown_symbol::{Context, UnknownSymbol},
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_error_data::IntoTypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
    &mut self,
    scope: *mut Scope,
    tp: *mut AstTypePack,
    in_type_argument: bool,
    replace_error_with_fresh: bool,
  ) -> TypePackId {
    let result: TypePackId;

    let node = tp as *mut AstNode;
    let explicit = unsafe { (*node).as_item_mut::<AstTypePackExplicit>() };
    if !explicit.is_null() {
      result = self.resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
        scope,
        &unsafe { &*explicit }.type_list,
        in_type_argument,
        replace_error_with_fresh,
      );
    } else {
      let variadic = unsafe { (*node).as_item_mut::<AstTypePackVariadic>() };
      if !variadic.is_null() {
        let ty: TypeId = self.resolve_type_constraint_generator_alt_b(
          scope,
          unsafe { (*variadic).variadic_type },
          in_type_argument,
          replace_error_with_fresh,
        );
        result = unsafe { (*self.arena).add_type_pack_t(VariadicTypePack { ty, hidden: false }) };
      } else {
        let generic = unsafe { (*node).as_item_mut::<AstTypePackGeneric>() };
        if !generic.is_null() {
          let generic_name_ptr = unsafe { (*generic).generic_name.value };
          let generic_name_str = unsafe {
            CStr::from_ptr(generic_name_ptr)
              .to_string_lossy()
              .into_owned()
          };

          if let Some(lookup) = unsafe { &*scope }.lookup_pack(&generic_name_str) {
            result = lookup;
          } else {
            let error = UnknownSymbol::new(generic_name_str, Context::Type);
            let location = unsafe { (*tp).base.location };
            self.report_error(location, error.into_type_error_data());
            result = unsafe { (*self.builtin_types).error_type_pack };
          }
        } else {
          LUAU_ASSERT!(false);
          result = unsafe { (*self.builtin_types).error_type_pack };
        }
      }
    }

    // 对照 C++：`result = follow(result); if (auto gtp = getMutable<GenericTypePack>(result))`
    let followed = unsafe { follow_type_pack_id(result) };
    if let Some(gtp) = get_mutable_type_pack_id::<GenericTypePack>(followed) {
      gtp.polarity = (gtp.polarity & Polarity::Mixed) | self.polarity;
    }

    if let Some(module) = &self.module {
      let module_ptr = Arc::as_ptr(module) as *mut Module;
      unsafe {
        *(*module_ptr)
          .ast_resolved_type_packs
          .get_or_insert(tp as *const AstTypePack) = result;
      }
    }

    result
  }
}
