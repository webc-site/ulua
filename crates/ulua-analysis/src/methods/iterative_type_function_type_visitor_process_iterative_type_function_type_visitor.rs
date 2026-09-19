use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_function_runtime_alt_o::get_type_function_type_id,
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};
impl IterativeTypeFunctionTypeVisitor {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn process_type_function_type_id(&mut self, ty: TypeFunctionTypeId) {
    if self.has_seen(ty as *const c_void) {
      return;
    }

    let tfpt = unsafe { get_type_function_type_id::<TypeFunctionPrimitiveType>(ty) };
    let tfat = unsafe { get_type_function_type_id::<TypeFunctionAnyType>(ty) };
    let tfut_unknown = unsafe { get_type_function_type_id::<TypeFunctionUnknownType>(ty) };
    let tfnt_never = unsafe { get_type_function_type_id::<TypeFunctionNeverType>(ty) };
    let tfst = unsafe { get_type_function_type_id::<TypeFunctionSingletonType>(ty) };
    let tfut_union = unsafe { get_type_function_type_id::<TypeFunctionUnionType>(ty) };
    let tfit = unsafe { get_type_function_type_id::<TypeFunctionIntersectionType>(ty) };
    let tfnt_negation = unsafe { get_type_function_type_id::<TypeFunctionNegationType>(ty) };
    let tfft = unsafe { get_type_function_type_id::<TypeFunctionFunctionType>(ty) };
    let tftt = unsafe { get_type_function_type_id::<TypeFunctionTableType>(ty) };
    let tfet = unsafe { get_type_function_type_id::<TypeFunctionExternType>(ty) };
    let tfgt = unsafe { get_type_function_type_id::<TypeFunctionGenericType>(ty) };

    if !tfpt.is_null() {
      self.visit_type_function_type_id_type_function_primitive_type(ty, unsafe { &*tfpt });
    } else if !tfat.is_null() {
      self.visit_type_function_type_id_type_function_any_type(ty, unsafe { &*tfat });
    } else if !tfut_unknown.is_null() {
      self.visit_type_function_type_id_type_function_unknown_type(ty, unsafe { &*tfut_unknown });
    } else if !tfnt_never.is_null() {
      self.visit_type_function_type_id_type_function_never_type(ty, unsafe { &*tfnt_never });
    } else if !tfst.is_null() {
      self.visit_type_function_type_id_type_function_singleton_type(ty, unsafe { &*tfst });
    } else if !tfut_union.is_null() {
      if self.visit_type_function_type_id_type_function_union_type(ty, unsafe { &*tfut_union }) {
        let components = unsafe { &(*tfut_union).components };
        for &component in components {
          self.traverse_type_function_type_id(component);
        }
      }
    } else if !tfit.is_null() {
      if self.visit_type_function_type_id_type_function_intersection_type(ty, unsafe { &*tfit }) {
        let components = unsafe { &(*tfit).components };
        for &component in components {
          self.traverse_type_function_type_id(component);
        }
      }
    } else if !tfnt_negation.is_null() {
      if self
        .visit_type_function_type_id_type_function_negation_type(ty, unsafe { &*tfnt_negation })
      {
        let inner = unsafe { (*tfnt_negation).type_id };
        self.traverse_type_function_type_id(inner);
      }
    } else if !tfft.is_null() {
      if self.visit_type_function_type_id_type_function_function_type(ty, unsafe { &*tfft }) {
        let generics = unsafe { (*tfft).generics.clone() };
        for generic in generics {
          self.traverse_type_function_type_id(generic);
        }

        let generic_packs = unsafe { (*tfft).generic_packs.clone() };
        for generic in generic_packs {
          self.traverse_type_function_type_pack_id(generic);
        }

        let arg_types = unsafe { (*tfft).arg_types };
        self.traverse_type_function_type_pack_id(arg_types);
        let ret_types = unsafe { (*tfft).ret_types };
        self.traverse_type_function_type_pack_id(ret_types);
      }
    } else if !tftt.is_null() {
      if self.visit_type_function_type_id_type_function_table_type(ty, unsafe { &*tftt }) {
        let tftt_ref = unsafe { &*tftt };
        for prop in tftt_ref.props.values() {
          if let Some(read_ty) = prop.read_ty {
            self.traverse_type_function_type_id(read_ty);
          }

          // In the case that the readType and the writeType are the same pointer, just traverse once.
          // Traversing each property twice has pretty significant performance consequences.
          if let Some(write_ty) = prop.write_ty
            && !prop.is_shared()
          {
            self.traverse_type_function_type_id(write_ty);
          }
        }

        if let Some(metatable) = tftt_ref.metatable {
          self.traverse_type_function_type_id(metatable);
        }

        if let Some(indexer) = &tftt_ref.indexer {
          let key_type = indexer.key_type;
          let value_type = indexer.value_type;
          self.traverse_type_function_type_id(key_type);
          self.traverse_type_function_type_id(value_type);
        }
      }
    } else if !tfet.is_null() {
      if self.visit_type_function_type_id_type_function_extern_type(ty, unsafe { &*tfet }) {
        let tfet_ref = unsafe { &*tfet };
        for prop in tfet_ref.props.values() {
          if let Some(read_ty) = prop.read_ty {
            self.traverse_type_function_type_id(read_ty);
          }

          // In the case that the readType and the writeType are the same pointer, just traverse once.
          // Traversing each property twice has pretty significant performance consequences.
          if let Some(write_ty) = prop.write_ty
            && !prop.is_shared()
          {
            self.traverse_type_function_type_id(write_ty);
          }
        }

        if let Some(metatable) = tfet_ref.metatable {
          self.traverse_type_function_type_id(metatable);
        }

        if let Some(read_parent) = tfet_ref.read_parent {
          self.traverse_type_function_type_id(read_parent);
        }
        if let Some(write_parent) = tfet_ref.write_parent {
          self.traverse_type_function_type_id(write_parent);
        }

        if let Some(indexer) = &tfet_ref.indexer {
          let key_type = indexer.key_type;
          let value_type = indexer.value_type;
          self.traverse_type_function_type_id(key_type);
          self.traverse_type_function_type_id(value_type);
        }
      }
    } else if !tfgt.is_null() {
      self.visit_type_function_type_id_type_function_generic_type(ty, unsafe { &*tfgt });
    } else {
      LUAU_ASSERT!(
        false /* "GenericTypeFunctionTypeVisitor::traverse(TypeFunctionTypeId) is not exhaustive!" */
      );
    }

    self.unsee(ty as *const c_void);
  }
}
