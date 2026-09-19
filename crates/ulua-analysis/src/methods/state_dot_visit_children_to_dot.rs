use alloc::string::String;

use ulua_common::{
  functions::{escape::escape, format_append::format_append},
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
    bound_type::BoundType, props_type::Props, singleton_variant::SingletonVariantMember,
    type_id::TypeId,
  },
};
impl StateDot {
  /// 输出一组属性的子边：shared 属性单条边（名为属性名）；
  /// 否则按 read/write 各输出一条 `read <name>` / `write <name>` 边。
  /// 合并 C++ `StateDot::visitChildren` 中 TableType 与 ExternType 两处相同的 props 循环。
  fn visit_prop_edges(&mut self, props: &Props, index: i32) {
    for (name, prop) in props.iter() {
      if prop.is_shared() {
        self.visit_child_type_id(prop.read_ty.unwrap(), index, Some(name));
      } else {
        if let Some(read_ty) = prop.read_ty {
          self.visit_child_type_id(read_ty, index, Some(&alloc::format!("read {}", name)));
        }

        if let Some(write_ty) = prop.write_ty {
          self.visit_child_type_id(write_ty, index, Some(&alloc::format!("write {}", name)));
        }
      }
    }
  }

  pub fn visit_children_type_id_i32(&mut self, ty: TypeId, index: i32) {
    if self.seen_ty.contains(&ty) {
      return;
    }
    self.seen_ty.insert(ty);

    self.start_node(index);
    self.start_node_label();

    if let Some(t) = get_type_id::<BoundType>(ty) {
      format_append(&mut self.result, format_args!("BoundType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_child_type_id(t.bound_to, index, None);
      return;
    }

    if get_type_id::<BlockedType>(ty).is_some() {
      format_append(&mut self.result, format_args!("BlockedType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<FunctionType>(ty) {
      format_append(&mut self.result, format_args!("FunctionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_child_type_pack_id(t.arg_types, index, Some("arg"));
      self.visit_child_type_pack_id(t.ret_types, index, Some("ret"));
      return;
    }

    if let Some(t) = get_type_id::<TableType>(ty) {
      if let Some(name) = &t.name {
        format_append(&mut self.result, format_args!("TableType {}", name));
      } else if let Some(synthetic_name) = &t.synthetic_name {
        format_append(
          &mut self.result,
          format_args!("TableType {}", synthetic_name),
        );
      } else {
        format_append(&mut self.result, format_args!("TableType {}", index));
      }
      self.finish_node_label_type_id(ty);
      self.finish_node();

      if let Some(bound_to) = t.bound_to {
        self.visit_child_type_id(bound_to, index, Some("bound_to"));
        return;
      }

      self.visit_prop_edges(&t.props, index);
      if let Some(indexer) = &t.indexer {
        self.visit_child_type_id(indexer.index_type, index, Some("[index]"));
        self.visit_child_type_id(indexer.index_result_type, index, Some("[value]"));
      }
      for itp in &t.instantiated_type_params {
        self.visit_child_type_id(*itp, index, Some("typeParam"));
      }

      for itp in &t.instantiated_type_pack_params {
        self.visit_child_type_pack_id(*itp, index, Some("typePackParam"));
      }
      return;
    }

    if let Some(t) = get_type_id::<MetatableType>(ty) {
      format_append(&mut self.result, format_args!("MetatableType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_child_type_id(t.table, index, Some("table"));
      self.visit_child_type_id(t.metatable, index, Some("metatable"));
      return;
    }

    if let Some(t) = get_type_id::<UnionType>(ty) {
      format_append(&mut self.result, format_args!("UnionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      for opt in &t.options {
        self.visit_child_type_id(*opt, index, None);
      }
      return;
    }

    if let Some(t) = get_type_id::<IntersectionType>(ty) {
      format_append(&mut self.result, format_args!("IntersectionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      for part in &t.parts {
        self.visit_child_type_id(*part, index, None);
      }
      return;
    }

    if get_type_id::<LazyType>(ty).is_some() {
      format_append(&mut self.result, format_args!("LazyType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<PendingExpansionType>(ty).is_some() {
      format_append(
        &mut self.result,
        format_args!("PendingExpansionType {}", index),
      );
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<GenericType>(ty) {
      if t.explicit_name {
        format_append(&mut self.result, format_args!("GenericType {}", t.name));
      } else {
        format_append(&mut self.result, format_args!("GenericType {}", index));
      }
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<FreeType>(ty) {
      format_append(&mut self.result, format_args!("FreeType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      if !t.lower_bound.is_null() && get_type_id::<NeverType>(t.lower_bound).is_none() {
        self.visit_child_type_id(t.lower_bound, index, Some("[lowerBound]"));
      }

      if !t.upper_bound.is_null() && get_type_id::<UnknownType>(t.upper_bound).is_none() {
        self.visit_child_type_id(t.upper_bound, index, Some("[upperBound]"));
      }
      return;
    }

    if get_type_id::<AnyType>(ty).is_some() {
      format_append(&mut self.result, format_args!("AnyType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<NoRefineType>(ty).is_some() {
      format_append(&mut self.result, format_args!("NoRefineType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<UnknownType>(ty).is_some() {
      format_append(&mut self.result, format_args!("UnknownType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<NeverType>(ty).is_some() {
      format_append(&mut self.result, format_args!("NeverType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<PrimitiveType>(ty).is_some() {
      let s = to_string_type_id(ty);
      format_append(&mut self.result, format_args!("PrimitiveType {}", s));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type_id::<ErrorType>(ty).is_some() {
      format_append(&mut self.result, format_args!("ErrorType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<ExternType>(ty) {
      format_append(&mut self.result, format_args!("ExternType {}", t.name));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_prop_edges(&t.props, index);

      if let Some(parent) = t.parent {
        self.visit_child_type_id(parent, index, Some("[parent]"));
      }

      if let Some(metatable) = t.metatable {
        self.visit_child_type_id(metatable, index, Some("[metatable]"));
      }

      if let Some(indexer) = &t.indexer {
        self.visit_child_type_id(indexer.index_type, index, Some("[index]"));
        self.visit_child_type_id(indexer.index_result_type, index, Some("[value]"));
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

      format_append(&mut self.result, format_args!("SingletonType {}", res));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type_id::<NegationType>(ty) {
      format_append(&mut self.result, format_args!("NegationType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_child_type_id(t.ty, index, Some("[negated]"));
      return;
    }

    if let Some(t) = get_type_id::<TypeFunctionInstanceType>(ty) {
      format_append(
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
        self.visit_child_type_id(*ty_param, index, None);
      }

      for tp_param in &t.pack_arguments {
        self.visit_child_type_pack_id(*tp_param, index, None);
      }
      return;
    }

    // unknown type kind
    LUAU_ASSERT!(false);
  }
}
