use alloc::{ffi::CString, string::String};
use core::ptr::null;

use ulua_common::{
  functions::{escape::escape, format_append::formatAppend},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
  records::{
    any_type::AnyType, blocked_type::BlockedType, boolean_singleton::BooleanSingleton,
    error_type::ErrorType, extern_type::ExternType, free_type::FreeType,
    function_type::FunctionType, generic_type::GenericType, intersection_type::IntersectionType,
    lazy_type::LazyType, metatable_type::MetatableType, negation_type::NegationType,
    never_type::NeverType, no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, state_dot::StateDot, string_singleton::StringSingleton,
    table_type::TableType, type_function_instance_type::TypeFunctionInstanceType,
    union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{
    bound_type::BoundType, singleton_variant::SingletonVariantMember, type_id::TypeId,
  },
};
impl StateDot {
  pub fn visit_children_type_id_i32(&mut self, ty: TypeId, index: i32) {
    if self.seen_ty.contains(&ty) {
      return;
    }
    self.seen_ty.insert(ty);

    self.start_node(index);
    self.start_node_label();

    if let Some(t) = get_type_id::<BoundType>(ty) {
      formatAppend(&mut self.result, format_args!("BoundType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      unsafe { self.visit_child_type_id_i32_c_char(t.bound_to, index, null()) };
      return;
    }

    if get_type_id::<BlockedType>(ty).is_some() {
      formatAppend(&mut self.result, format_args!("BlockedType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<FunctionType>(ty) {
      formatAppend(&mut self.result, format_args!("FunctionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      unsafe { self.visit_child_type_pack_id_i32_c_char(t.arg_types, index, c"arg".as_ptr()) };
      unsafe { self.visit_child_type_pack_id_i32_c_char(t.ret_types, index, c"ret".as_ptr()) };
      return;
    }

    if let Some(t) = get_type_id::<TableType>(ty) {
      if let Some(name) = &t.name {
        formatAppend(&mut self.result, format_args!("TableType {}", name));
      } else if let Some(synthetic_name) = &t.synthetic_name {
        formatAppend(
          &mut self.result,
          format_args!("TableType {}", synthetic_name),
        );
      } else {
        formatAppend(&mut self.result, format_args!("TableType {}", index));
      }
      self.finish_node_label_type_id(ty);
      self.finish_node();

      if let Some(bound_to) = t.bound_to {
        unsafe { self.visit_child_type_id_i32_c_char(bound_to, index, c"bound_to".as_ptr()) };
        return;
      }

      for (name, prop) in t.props.iter() {
        if prop.is_shared() {
          let c_name = CString::new(name.as_bytes()).unwrap();
          unsafe {
            self.visit_child_type_id_i32_c_char(prop.read_ty.unwrap(), index, c_name.as_ptr())
          };
        } else {
          if let Some(read_ty) = prop.read_ty {
            let read_name = alloc::format!("read {}", name);
            let c_name = CString::new(read_name.as_bytes()).unwrap();
            unsafe { self.visit_child_type_id_i32_c_char(read_ty, index, c_name.as_ptr()) };
          }

          if let Some(write_ty) = prop.write_ty {
            let write_name = alloc::format!("write {}", name);
            let c_name = CString::new(write_name.as_bytes()).unwrap();
            unsafe { self.visit_child_type_id_i32_c_char(write_ty, index, c_name.as_ptr()) };
          }
        }
      }
      if let Some(indexer) = &t.indexer {
        unsafe {
          self.visit_child_type_id_i32_c_char(indexer.index_type, index, c"[index]".as_ptr())
        };
        unsafe {
          self.visit_child_type_id_i32_c_char(indexer.index_result_type, index, c"[value]".as_ptr())
        };
      }
      for itp in &t.instantiated_type_params {
        unsafe { self.visit_child_type_id_i32_c_char(*itp, index, c"typeParam".as_ptr()) };
      }

      for itp in &t.instantiated_type_pack_params {
        unsafe { self.visit_child_type_pack_id_i32_c_char(*itp, index, c"typePackParam".as_ptr()) };
      }
      return;
    }

    if let Some(t) = get_type_id::<MetatableType>(ty) {
      formatAppend(&mut self.result, format_args!("MetatableType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      unsafe { self.visit_child_type_id_i32_c_char(t.table, index, c"table".as_ptr()) };
      unsafe { self.visit_child_type_id_i32_c_char(t.metatable, index, c"metatable".as_ptr()) };
      return;
    }

    if let Some(t) = get_type_id::<UnionType>(ty) {
      formatAppend(&mut self.result, format_args!("UnionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      for opt in &t.options {
        unsafe { self.visit_child_type_id_i32_c_char(*opt, index, null()) };
      }
      return;
    }

    if let Some(t) = get_type_id::<IntersectionType>(ty) {
      formatAppend(&mut self.result, format_args!("IntersectionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      for part in &t.parts {
        unsafe { self.visit_child_type_id_i32_c_char(*part, index, null()) };
      }
      return;
    }

    if get_type_id::<LazyType>(ty).is_some() {
      formatAppend(&mut self.result, format_args!("LazyType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<PendingExpansionType>(ty).is_some() {
      formatAppend(
        &mut self.result,
        format_args!("PendingExpansionType {}", index),
      );
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<GenericType>(ty) {
      if t.explicit_name {
        formatAppend(&mut self.result, format_args!("GenericType {}", t.name));
      } else {
        formatAppend(&mut self.result, format_args!("GenericType {}", index));
      }
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<FreeType>(ty) {
      formatAppend(&mut self.result, format_args!("FreeType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      if !t.lower_bound.is_null() && get_type_id::<NeverType>(t.lower_bound).is_none() {
        unsafe {
          self.visit_child_type_id_i32_c_char(t.lower_bound, index, c"[lowerBound]".as_ptr())
        };
      }

      if !t.upper_bound.is_null() && get_type_id::<UnknownType>(t.upper_bound).is_none() {
        unsafe {
          self.visit_child_type_id_i32_c_char(t.upper_bound, index, c"[upperBound]".as_ptr())
        };
      }
      return;
    }

    if get_type_id::<AnyType>(ty).is_some() {
      formatAppend(&mut self.result, format_args!("AnyType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<NoRefineType>(ty).is_some() {
      formatAppend(&mut self.result, format_args!("NoRefineType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<UnknownType>(ty).is_some() {
      formatAppend(&mut self.result, format_args!("UnknownType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<NeverType>(ty).is_some() {
      formatAppend(&mut self.result, format_args!("NeverType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<PrimitiveType>(ty).is_some() {
      let s = to_string_type_id(ty);
      formatAppend(&mut self.result, format_args!("PrimitiveType {}", s));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<ErrorType>(ty).is_some() {
      formatAppend(&mut self.result, format_args!("ErrorType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<ExternType>(ty) {
      formatAppend(&mut self.result, format_args!("ExternType {}", t.name));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      for (name, prop) in t.props.iter() {
        if prop.is_shared() {
          let c_name = CString::new(name.as_bytes()).unwrap();
          unsafe {
            self.visit_child_type_id_i32_c_char(prop.read_ty.unwrap(), index, c_name.as_ptr())
          };
        } else {
          if let Some(read_ty) = prop.read_ty {
            let read_name = alloc::format!("read {}", name);
            let c_name = CString::new(read_name.as_bytes()).unwrap();
            unsafe { self.visit_child_type_id_i32_c_char(read_ty, index, c_name.as_ptr()) };
          }

          if let Some(write_ty) = prop.write_ty {
            let write_name = alloc::format!("write {}", name);
            let c_name = CString::new(write_name.as_bytes()).unwrap();
            unsafe { self.visit_child_type_id_i32_c_char(write_ty, index, c_name.as_ptr()) };
          }
        }
      }

      if let Some(parent) = t.parent {
        unsafe { self.visit_child_type_id_i32_c_char(parent, index, c"[parent]".as_ptr()) };
      }

      if let Some(metatable) = t.metatable {
        unsafe { self.visit_child_type_id_i32_c_char(metatable, index, c"[metatable]".as_ptr()) };
      }

      if let Some(indexer) = &t.indexer {
        unsafe {
          self.visit_child_type_id_i32_c_char(indexer.index_type, index, c"[index]".as_ptr())
        };
        unsafe {
          self.visit_child_type_id_i32_c_char(indexer.index_result_type, index, c"[value]".as_ptr())
        };
      }
      return;
    }

    if let Some(t) = get_type_id::<SingletonType>(ty) {
      let res: String;

      let ss = StringSingleton::get_if(&t.variant);
      if let Some(ss) = ss {
        // Don't put in quotes anywhere. If it's outside of the call to escape,
        // then it's invalid syntax. If it's inside, then escaping is super noisy.
        res = alloc::format!("string: {}", escape(&ss.value, false));
      } else if let Some(bs) = BooleanSingleton::get_if(&t.variant) {
        res = alloc::format!("boolean: {}", if bs.value { "true" } else { "false" });
      } else {
        LUAU_ASSERT!(false);
        res = String::new();
      }

      formatAppend(&mut self.result, format_args!("SingletonType {}", res));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<NegationType>(ty) {
      formatAppend(&mut self.result, format_args!("NegationType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      unsafe { self.visit_child_type_id_i32_c_char(t.ty, index, c"[negated]".as_ptr()) };
      return;
    }

    if let Some(t) = get_type_id::<TypeFunctionInstanceType>(ty) {
      formatAppend(
        &mut self.result,
        format_args!(
          "TypeFunctionInstanceType {} {}",
          // SAFETY: function 为 NonNull<TypeFunction>，指向有效的类型函数描述符
          unsafe { t.function.as_ref() }.name,
          index
        ),
      );
      self.finish_node_label_type_id(ty);
      self.finish_node();

      for ty_param in &t.type_arguments {
        unsafe { self.visit_child_type_id_i32_c_char(*ty_param, index, null()) };
      }

      for tp_param in &t.pack_arguments {
        unsafe { self.visit_child_type_pack_id_i32_c_char(*tp_param, index, null()) };
      }
      return;
    }

    // unknown type kind
    LUAU_ASSERT!(false);
  }
}
