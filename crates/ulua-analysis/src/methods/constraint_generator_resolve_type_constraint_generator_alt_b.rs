use alloc::{string::String, sync::Arc, vec::Vec};
use core::{mem::ManuallyDrop, str::from_utf8};

use ulua_ast::records::{
  ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
  ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
  ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
  ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
  ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
  ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, intersection_type::IntersectionType, module::Module,
    scope::Scope, singleton_type::SingletonType, string_singleton::StringSingleton,
    union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `scope、`ty` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  // ConstraintGenerator::resolveType_(const ScopePtr&, AstType*, bool, bool)
  // (ConstraintGenerator.cpp:4578).
  pub(crate) fn resolve_type_constraint_generator_alt_b(
    &mut self,
    scope: *mut Scope,
    ty: *mut AstType,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
  ) -> TypeId {
    // The resolve helpers and `check`/`freshType` want a `const ScopePtr&`; the
    // C++ overload also takes a `const ScopePtr&`. Reconstruct one without
    // taking ownership of the refcount.
    let sp = ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });

    let node = ty as *mut AstNode;
    let mut result: TypeId;

    let ref_ = unsafe { (*node).as_item_mut::<AstTypeReference>() };
    let tab = unsafe { (*node).as_item_mut::<AstTypeTable>() };
    let fn_ = unsafe { (*node).as_item_mut::<AstTypeFunction>() };
    let tof = unsafe { (*node).as_item_mut::<AstTypeTypeof>() };
    if !ref_.is_null() {
      result = unsafe {
        self.resolve_reference_type(&sp, ty, ref_, in_type_arguments, replace_error_with_fresh)
      };
    } else {
      if !tab.is_null() {
        result = unsafe {
          self.resolve_table_type(scope, ty, tab, in_type_arguments, replace_error_with_fresh)
        };
      } else {
        if !fn_.is_null() {
          result = self.resolve_function_type(
            &sp,
            ty,
            unsafe { &*fn_ },
            in_type_arguments,
            replace_error_with_fresh,
          );
        } else {
          if !tof.is_null() {
            let expr_type = self
              .check_scope_ptr_ast_expr(&sp, unsafe { (*tof).expr })
              .ty;
            result = expr_type;
          } else if !unsafe { (*node).as_item_mut::<AstTypeOptional>() }.is_null() {
            result = unsafe { (*self.builtin_types).nil_type };
          } else if !unsafe { (*node).as_item_mut::<AstTypeUnion>() }.is_null() {
            let union_annotation = unsafe { (*node).as_item_mut::<AstTypeUnion>() };
            if unsafe { (*union_annotation).types.size } == 1 {
              result = self.resolve_type_constraint_generator_alt_b(
                scope,
                unsafe { *(*union_annotation).types.data },
                in_type_arguments,
                false,
              );
            } else {
              let mut parts: Vec<TypeId> = Vec::new();
              let types = unsafe { (*union_annotation).types };
              for &part in types.as_slice() {
                parts.push(self.resolve_type_constraint_generator_alt_b(
                  scope,
                  part,
                  in_type_arguments,
                  false,
                ));
              }
              result = unsafe { (*self.arena).add_type(UnionType { options: parts }) };
            }
          } else if !unsafe { (*node).as_item_mut::<AstTypeIntersection>() }.is_null() {
            let intersection_annotation = unsafe { (*node).as_item_mut::<AstTypeIntersection>() };
            if unsafe { (*intersection_annotation).types.size } == 1 {
              result = self.resolve_type_constraint_generator_alt_b(
                scope,
                unsafe { *(*intersection_annotation).types.data },
                in_type_arguments,
                false,
              );
            } else {
              let mut parts: Vec<TypeId> = Vec::new();
              let types = unsafe { (*intersection_annotation).types };
              for &part in types.as_slice() {
                parts.push(self.resolve_type_constraint_generator_alt_b(
                  scope,
                  part,
                  in_type_arguments,
                  false,
                ));
              }
              result = unsafe { (*self.arena).add_type(IntersectionType { parts }) };
            }
          } else if !unsafe { (*node).as_item_mut::<AstTypeGroup>() }.is_null() {
            let type_group_annotation = unsafe { (*node).as_item_mut::<AstTypeGroup>() };
            result = self.resolve_type_constraint_generator_alt_b(
              scope,
              unsafe { (*type_group_annotation).type_ },
              in_type_arguments,
              false,
            );
          } else if !unsafe { (*node).as_item_mut::<AstTypeSingletonBool>() }.is_null() {
            let bool_annotation = unsafe { (*node).as_item_mut::<AstTypeSingletonBool>() };
            if unsafe { (*bool_annotation).value } {
              result = unsafe { (*self.builtin_types).true_type };
            } else {
              result = unsafe { (*self.builtin_types).false_type };
            }
          } else if !unsafe { (*node).as_item_mut::<AstTypeSingletonString>() }.is_null() {
            let string_annotation = unsafe { (*node).as_item_mut::<AstTypeSingletonString>() };
            let s: String = unsafe {
              String::from(from_utf8((*string_annotation).value.as_bytes()).unwrap_or(""))
            };
            result = unsafe {
              (*self.arena).add_type(SingletonType::new(SingletonVariant::V1(
                StringSingleton::new(s),
              )))
            };
          } else if !unsafe { (*node).as_item_mut::<AstTypeError>() }.is_null() {
            result = unsafe { (*self.builtin_types).error_type };
            if replace_error_with_fresh {
              result = self.fresh_type(&sp, self.polarity);
            }
          } else {
            LUAU_ASSERT!(false);
            result = unsafe { (*self.builtin_types).error_type };
          }
        }
      }
    }

    if let Some(module) = &self.module {
      let module_ptr = Arc::as_ptr(module) as *mut Module;
      unsafe {
        *(*module_ptr)
          .ast_resolved_types
          .get_or_insert(ty as *const AstType) = result;
      }
    }

    result
  }
}
