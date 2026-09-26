use alloc::string::String;

use ulua_common::{
  functions::{escape::escape, format_append::format_append},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  functions::{get_type, get_type_pack, to_string_to_string::to_string_type_id},
  records::{
    any_type::AnyType, blocked_type::BlockedType, boolean_singleton::BooleanSingleton,
    extern_type::ExternType, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, lazy_type::LazyType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, state_dot::StateDot, string_singleton::StringSingleton,
    table_type::TableType, type_function_instance_type::TypeFunctionInstanceType,
    type_pack::TypePack, union_type::UnionType, unknown_type::UnknownType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, error_type::ErrorType,
    error_type_pack::ErrorTypePack, props_type::Props, singleton_variant::SingletonVariantMember,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl StateDot {
  /// 输出一组属性的子边：shared 属性单条边（名为属性名）；
  /// 否则按 read/write 各输出一条 `read <name>` / `write <name>` 边。
  /// 合并 C++ `StateDot::visitChildren` 中 TableType 与 ExternType 两处相同的 props 循环。
  fn visit_prop_edges(&mut self, props: &Props, index: i32) {
    for (name, prop) in props.iter() {
      if prop.is_shared() {
        // 不变式：shared 属性由同一 TypeId 同时填 read_ty/write_ty，
        // `is_shared()` 蕴含 read_ty 为 Some（cpp `*prop.readTy` 同前提）。
        self.visit_child_type_id(
          prop.read_ty.expect("is_shared() 蕴含 read_ty 为 Some"),
          index,
          Some(name),
        );
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

    if let Some(t) = get_type::get::<BoundType>(ty) {
      format_append(&mut self.result, format_args!("BoundType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_child_type_id(t.bound_to, index, None);
      return;
    }

    if get_type::get::<BlockedType>(ty).is_some() {
      format_append(&mut self.result, format_args!("BlockedType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type::get::<FunctionType>(ty) {
      format_append(&mut self.result, format_args!("FunctionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_child_type_pack_id(t.arg_types, index, Some("arg"));
      self.visit_child_type_pack_id(t.ret_types, index, Some("ret"));
      return;
    }

    if let Some(t) = get_type::get::<TableType>(ty) {
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

    if let Some(t) = get_type::get::<MetatableType>(ty) {
      format_append(&mut self.result, format_args!("MetatableType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_child_type_id(t.table, index, Some("table"));
      self.visit_child_type_id(t.metatable, index, Some("metatable"));
      return;
    }

    if let Some(t) = get_type::get::<UnionType>(ty) {
      format_append(&mut self.result, format_args!("UnionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      for opt in &t.options {
        self.visit_child_type_id(*opt, index, None);
      }
      return;
    }

    if let Some(t) = get_type::get::<IntersectionType>(ty) {
      format_append(&mut self.result, format_args!("IntersectionType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      for part in &t.parts {
        self.visit_child_type_id(*part, index, None);
      }
      return;
    }

    if get_type::get::<LazyType>(ty).is_some() {
      format_append(&mut self.result, format_args!("LazyType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type::get::<PendingExpansionType>(ty).is_some() {
      format_append(
        &mut self.result,
        format_args!("PendingExpansionType {}", index),
      );
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type::get::<GenericType>(ty) {
      if t.explicit_name {
        format_append(&mut self.result, format_args!("GenericType {}", t.name));
      } else {
        format_append(&mut self.result, format_args!("GenericType {}", index));
      }
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type::get::<FreeType>(ty) {
      format_append(&mut self.result, format_args!("FreeType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      if !t.lower_bound.is_null() && get_type::get::<NeverType>(t.lower_bound).is_none() {
        self.visit_child_type_id(t.lower_bound, index, Some("[lowerBound]"));
      }

      if !t.upper_bound.is_null() && get_type::get::<UnknownType>(t.upper_bound).is_none() {
        self.visit_child_type_id(t.upper_bound, index, Some("[upperBound]"));
      }
      return;
    }

    if get_type::get::<AnyType>(ty).is_some() {
      format_append(&mut self.result, format_args!("AnyType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type::get::<NoRefineType>(ty).is_some() {
      format_append(&mut self.result, format_args!("NoRefineType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type::get::<UnknownType>(ty).is_some() {
      format_append(&mut self.result, format_args!("UnknownType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type::get::<NeverType>(ty).is_some() {
      format_append(&mut self.result, format_args!("NeverType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type::get::<PrimitiveType>(ty).is_some() {
      let s = to_string_type_id(ty);
      format_append(&mut self.result, format_args!("PrimitiveType {}", s));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if get_type::get::<ErrorType>(ty).is_some() {
      format_append(&mut self.result, format_args!("ErrorType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();
      return;
    }

    if let Some(t) = get_type::get::<ExternType>(ty) {
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

    if let Some(t) = get_type::get::<SingletonType>(ty) {
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

    if let Some(t) = get_type::get::<NegationType>(ty) {
      format_append(&mut self.result, format_args!("NegationType {}", index));
      self.finish_node_label_type_id(ty);
      self.finish_node();

      self.visit_child_type_id(t.ty, index, Some("[negated]"));
      return;
    }

    if let Some(t) = get_type::get::<TypeFunctionInstanceType>(ty) {
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

  pub fn visit_children_type_pack_id_i32(&mut self, tp: TypePackId, index: i32) {
    if self.seen_tp.contains(&tp) {
      return;
    }
    self.seen_tp.insert(tp);

    self.start_node(index);
    self.start_node_label();

    if let Some(btp) = get_type_pack::get::<BoundTypePack>(tp) {
      format_append(&mut self.result, format_args!("BoundTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      self.visit_child_type_pack_id(btp.bound_to, index, None);
    } else if let Some(tpp) = get_type_pack::get::<TypePack>(tp) {
      format_append(&mut self.result, format_args!("TypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      for tv in &tpp.head {
        self.visit_child_type_id(*tv, index, None);
      }
      if let Some(tail) = tpp.tail {
        self.visit_child_type_pack_id(tail, index, Some("tail"));
      }
    } else if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tp) {
      format_append(
        &mut self.result,
        format_args!(
          "VariadicTypePack {}{}",
          if vtp.hidden { "hidden " } else { "" },
          index
        ),
      );
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      self.visit_child_type_id(vtp.ty, index, None);
    } else if get_type_pack::get::<FreeTypePack>(tp).is_some() {
      format_append(&mut self.result, format_args!("FreeTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else if let Some(gtp) = get_type_pack::get::<GenericTypePack>(tp) {
      if gtp.explicit_name {
        format_append(
          &mut self.result,
          format_args!("GenericTypePack {}", gtp.name),
        );
      } else {
        format_append(&mut self.result, format_args!("GenericTypePack {}", index));
      }
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else if get_type_pack::get::<ErrorTypePack>(tp).is_some() {
      format_append(&mut self.result, format_args!("ErrorTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else {
      LUAU_ASSERT!(false);
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    }
  }
}
