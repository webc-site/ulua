use core::ffi::CStr;

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_table_indexer::AstTableIndexer,
  },
};
use ulua_common::{DFInt, functions::format::format, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity, table_state::TableState},
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type::follow_type_id,
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    constraint_generator::ConstraintGenerator,
    extern_type::ExternType,
    function_argument::FunctionArgument,
    function_definition::FunctionDefinition,
    function_type::FunctionType,
    generic_error::GenericError,
    intersection_type::IntersectionType,
    property_type::Property,
    scope::Scope,
    table_indexer::TableIndexer,
    table_type::TableType,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{
    name_type::Name, props_type::Props, type_error_data::TypeErrorData, type_id::TypeId,
    type_variant::TypeVariant,
  },
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_declare_extern_type(
    &mut self,
    scope: *mut Scope,
    declared_extern_type: *mut AstStatDeclareExternType,
  ) -> ControlFlow {
    unsafe {
      let scope_ref: &Scope = &*scope;
      let decl_ref: &AstStatDeclareExternType = &*declared_extern_type;

      // If a class with the same name was already defined, we skip over.
      let name_key: Name = CStr::from_ptr(decl_ref.name.value)
        .to_string_lossy()
        .into_owned();
      let binding_it = match scope_ref.exported_type_bindings.get(&name_key) {
        Some(b) => b.clone(),
        None => return ControlFlow::None,
      };

      let mut super_ty: Option<TypeId> = Some((*self.builtin_types).extern_type);

      if let Some(super_name_node) = decl_ref.super_name.as_ref() {
        let super_name: Name = CStr::from_ptr(super_name_node.value)
          .to_string_lossy()
          .into_owned();

        let lookup_type = scope_ref.lookup_type(&super_name);

        if lookup_type.is_none() {
          self.report_error(
            decl_ref.base.base.location,
            TypeErrorData::UnknownSymbol(UnknownSymbol::new(super_name, Context::Type)),
          );
          return ControlFlow::None;
        }

        let lookup_type = lookup_type.unwrap();

        // We don't have generic extern type_arguments, so this assertion
        // _should_ never be hit.
        LUAU_ASSERT!(
          lookup_type.type_params().is_empty() && lookup_type.type_pack_params().is_empty()
        );

        let followed = follow_type_id(lookup_type.r#type());
        super_ty = Some(followed);

        if get_type_id::<ExternType>(follow_type_id(super_ty.unwrap())).is_none() {
          self.report_error(
            decl_ref.base.base.location,
            TypeErrorData::GenericError(GenericError::new(format(format_args!(
              "Cannot use non-class type '{}' as a superclass of class '{}'",
              super_name.as_str(),
              name_key.as_str()
            )))),
          );

          // If we don't emplace an error type here, then later we'll be
          // exposing a blocked type in this file's type interface. This
          // is _normally_ harmless.
          let class_bind_ty = binding_it.r#type();
          (*as_mutable_type_id(class_bind_ty)).ty =
            TypeVariant::Bound((*self.builtin_types).error_type);

          return ControlFlow::None;
        }
      }

      let class_name: Name = name_key.clone();

      let module_name = self.module.as_ref().unwrap().name.clone();

      let extern_ty: TypeId = (*self.arena).add_type(ExternType {
        name: class_name,
        props: Default::default(),
        parent: super_ty,
        metatable: None,
        tags: Default::default(),
        user_data: None,
        definition_module_name: module_name,
        definition_location: Some(decl_ref.base.base.location),
        indexer: None,
        relation: None,
      });
      // extern_ty / meta_ty 刚由 add_type 分配，必命中；对照 C++:2338-2342
      // `ExternType* etv = getMutable<ExternType>(externTy); TableType* metatable = ...`
      let etv = get_mutable_type_id::<ExternType>(extern_ty).unwrap();

      let meta_ty: TypeId =
        (*self.arena).add_type(TableType::table_type_table_state_type_level_scope(
          TableState::Sealed,
          scope_ref.level,
          scope,
        ));
      let metatable = get_mutable_type_id::<TableType>(meta_ty).unwrap();

      etv.metatable = Some(meta_ty);

      let class_bind_ty = binding_it.r#type();
      (*as_mutable_type_id(class_bind_ty)).ty = TypeVariant::Bound(extern_ty);

      if !decl_ref.indexer.is_null() {
        let indexer: &AstTableIndexer = &*decl_ref.indexer;
        if self.recursion_count >= DFInt::LuauConstraintGeneratorRecursionLimit.get() {
          self.report_code_too_complex(indexer.location);
        } else {
          // I don't think extern types can *be* generic, but if they
          // have an indexer over those generics, the polarity is
          // mixed.
          let index_type =
            self.resolve_type(scope, indexer.index_type, false, false, Polarity::Mixed);
          let index_result_type =
            self.resolve_type(scope, indexer.result_type, false, false, Polarity::Mixed);
          etv.indexer = Some(TableIndexer {
            index_type,
            index_result_type,
            is_read_only: false,
          });
        }
      }

      let mut prop_i = 0;
      while prop_i < decl_ref.props.size {
        let extern_prop = *decl_ref.props.data.add(prop_i);
        prop_i += 1;

        let prop_name: Name = CStr::from_ptr(extern_prop.name.value)
          .to_string_lossy()
          .into_owned();
        let prop_ty = self.resolve_type(scope, extern_prop.ty, false, false, Polarity::Mixed);

        let assign_to_metatable = is_metamethod_mut(&prop_name);

        // Function type_arguments always take 'self', but this isn't
        // reflected in the parsed annotation. Add it here.
        if extern_prop.is_method {
          // 对照 C++:2382 `if (FunctionType* ftv = getMutable<FunctionType>(propTy))`
          if let Some(ftv) = get_mutable_type_id::<FunctionType>(prop_ty) {
            ftv.arg_names.insert(
              0,
              Some(FunctionArgument {
                name: "self".into(),
                location: Default::default(),
              }),
            );
            ftv.arg_types = self.add_type_pack(alloc::vec![extern_ty], Some(ftv.arg_types));

            ftv.has_self = true;

            let defn = FunctionDefinition {
              definition_module_name: Some(self.module.as_ref().unwrap().name.clone()),
              definition_location: extern_prop.location,
              // No data is preserved for vararg_location
              vararg_location: None,
              original_name_location: extern_prop.name_location,
            };

            ftv.definition = Some(defn);
          }
        }

        let props: &mut Props = if assign_to_metatable {
          &mut metatable.props
        } else {
          &mut etv.props
        };

        if !props.contains_key(&prop_name) {
          let mut table_prop = if extern_prop.access == AstTableAccess::Read {
            Property::readonly(prop_ty)
          } else if extern_prop.access == AstTableAccess::Write {
            Property::writeonly(prop_ty)
          } else {
            Property::rw_type_id(prop_ty)
          };

          table_prop.location = Some(extern_prop.location);

          props.insert(prop_name.clone(), table_prop);
        } else {
          let prop = props.get_mut(&prop_name).unwrap();
          let mut added_write_type_by_overload = false;

          if let Some(read_ty) = prop.read_ty {
            // We special-case this logic to keep the intersection
            // flat; otherwise we would create a ton of nested
            // intersection type_arguments.
            // 对照 C++：`if (const IntersectionType* itv = get<IntersectionType>(readTy))`
            if let Some(itv) = get_type_id::<IntersectionType>(read_ty) {
              let mut options = itv.parts.clone();
              options.push(prop_ty);
              let new_itv = (*self.arena).add_type(IntersectionType { parts: options });
              prop.read_ty = Some(new_itv);
            } else if !get_type_id::<FunctionType>(read_ty).is_none() {
              let intersection = (*self.arena).add_type(IntersectionType {
                parts: alloc::vec![read_ty, prop_ty],
              });
              prop.read_ty = Some(intersection);
            } else if extern_prop.access == AstTableAccess::Write && prop.write_ty.is_none() {
              prop.write_ty = Some(prop_ty);
              added_write_type_by_overload = true;
            } else {
              self.report_error(
                decl_ref.base.base.location,
                TypeErrorData::GenericError(GenericError::new(format(format_args!(
                  "Cannot overload read type of non-function extern type member '{}'",
                  prop_name.as_str()
                )))),
              );
            }
          }

          if let Some(write_ty) = prop.write_ty
            && !added_write_type_by_overload
          {
            // We special-case this logic to keep the
            // intersection flat; otherwise we would create a ton
            // of nested intersection type_arguments.
            // 对照 C++：`if (const IntersectionType* itv = get<IntersectionType>(writeTy))`
            if let Some(itv) = get_type_id::<IntersectionType>(write_ty) {
              let mut options = itv.parts.clone();
              options.push(prop_ty);
              let new_itv = (*self.arena).add_type(IntersectionType { parts: options });
              prop.write_ty = Some(new_itv);
            } else if !get_type_id::<FunctionType>(write_ty).is_none() {
              let intersection = (*self.arena).add_type(IntersectionType {
                parts: alloc::vec![write_ty, prop_ty],
              });
              prop.write_ty = Some(intersection);
            } else if extern_prop.access == AstTableAccess::Read && prop.read_ty.is_none() {
              prop.read_ty = Some(prop_ty);
            } else {
              self.report_error(
                decl_ref.base.base.location,
                TypeErrorData::GenericError(GenericError::new(format(format_args!(
                  "Cannot overload write type of non-function extern type member '{}'",
                  prop_name.as_str()
                )))),
              );
            }
          }
        }
      }

      ControlFlow::None
    }
  }
}

use crate::functions::is_metamethod_constraint_generator::is_metamethod_mut;
