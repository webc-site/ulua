use alloc::{collections::btree_map::Entry, string::String, vec::Vec};
use core::ffi::CStr;

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::ast_stat_declare_extern_type::AstStatDeclareExternType,
};

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, is_metamethod_type_infer::is_metamethod,
  },
  records::{
    extern_type::ExternType, function_argument::FunctionArgument,
    function_definition::FunctionDefinition, function_type::FunctionType,
    generic_error::GenericError, intersection_type::IntersectionType, property_type::Property,
    table_indexer::TableIndexer, table_type::TableType, type_checker::TypeChecker,
    type_fun::TypeFun, type_pack::TypePack,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_declare_extern_type(
    &mut self,
    scope: &ScopePtr,
    declared_extern_type: &AstStatDeclareExternType,
  ) -> ControlFlow {
    // SAFETY: name.value 为 NUL 结尾 C 字符串（AST arena 持有）。
    let class_name: Name = unsafe {
      CStr::from_ptr(declared_extern_type.name.value)
        .to_string_lossy()
        .into_owned()
    };

    if self
      .incorrect_extern_type_definitions
      .contains(&(declared_extern_type as *const AstStatDeclareExternType))
    {
      return ControlFlow::None;
    }

    let binding: Option<TypeFun> = scope.exported_type_bindings.get(&class_name).cloned();
    let Some(binding) = binding else {
      self.ice_string("Extern type not predeclared");
      return ControlFlow::None;
    };

    let extern_ty = binding.r#type;
    let Some(etv) = get_mutable_type_id::<ExternType>(extern_ty) else {
      self.ice_string("Extern type binding was not an extern type");
      return ControlFlow::None;
    };

    let Some(metatable_ty) = etv.metatable else {
      self.ice_string("No metatable for declared extern type");
      return ControlFlow::None;
    };

    if !declared_extern_type.indexer.is_null() {
      // SAFETY: indexer 非 null，指向 AST arena 节点。
      let indexer = unsafe { &*declared_extern_type.indexer };
      // SAFETY: 注解节点指向 AST arena 节点。
      let index_type = self.resolve_type(scope.clone(), unsafe { &*indexer.index_type });
      let result_type = self.resolve_type(scope.clone(), unsafe { &*indexer.result_type });
      etv.indexer = Some(TableIndexer {
        index_type,
        index_result_type: result_type,
        is_read_only: indexer.access == AstTableAccess::Read,
      });
    }

    let Some(metatable) = get_mutable_type_id::<TableType>(metatable_ty) else {
      self.ice_string("Declared extern type metatable was not a table");
      return ControlFlow::None;
    };

    for prop in declared_extern_type.props.iter() {
      // SAFETY: prop.name.value 为 NUL 结尾 C 字符串。
      let prop_name: Name = unsafe { CStr::from_ptr(prop.name.value) }
        .to_string_lossy()
        .into_owned();
      // SAFETY: prop.ty 指向 AST arena 节点。
      let prop_ty = self.resolve_type(scope.clone(), unsafe { &*prop.ty });
      let assign_to_metatable = is_metamethod(&prop_name);

      if prop.is_method
        && let Some(ftv) = get_mutable_type_id::<FunctionType>(prop_ty)
      {
        ftv.arg_names.insert(
          0,
          Some(FunctionArgument {
            name: "self".to_string(),
            location: prop.location,
          }),
        );
        let old_arg_types = ftv.arg_types;
        ftv.arg_types = self.add_type_pack_type_pack(TypePack {
          head: Vec::from([extern_ty]),
          tail: Some(old_arg_types),
        });
        ftv.has_self = true;
        ftv.definition = Some(FunctionDefinition {
          definition_module_name: Some(self.current_module.as_ref().unwrap().name.clone()),
          definition_location: prop.location,
          vararg_location: None,
          original_name_location: prop.name_location,
        });
      }

      let assign_to = if assign_to_metatable {
        &mut metatable.props
      } else {
        &mut etv.props
      };

      match assign_to.entry(prop_name.clone()) {
        Entry::Vacant(e) => {
          e.insert(
            Property::property_type_id_bool_string_optional_location_tags_optional_string_optional_location(
              prop_ty,
              false,
              String::new(),
              Some(prop.location),
              Default::default(),
              None,
              None,
            ),
          );
        }
        Entry::Occupied(mut e) => {
          let property = e.get_mut();
          let current_ty = property.type_deprecated();
          let current_ty = follow_type_id(current_ty);

          if let Some(current_intersection) = get_type_id::<IntersectionType>(current_ty) {
            let mut options = current_intersection.parts.clone();
            options.push(prop_ty);
            let new_itv = self.add_type(&IntersectionType { parts: options });
            property.read_ty = Some(new_itv);
            property.write_ty = Some(new_itv);
          } else if get_type_id::<FunctionType>(current_ty).is_some() {
            let intersection = self.add_type(&IntersectionType {
              parts: Vec::from([current_ty, prop_ty]),
            });
            property.read_ty = Some(intersection);
            property.write_ty = Some(intersection);
          } else {
            self.report_error_location_type_error_data(
              &declared_extern_type.base.base.location,
              TypeErrorData::GenericError(GenericError::new(format!(
                "Cannot overload non-function class member '{}'",
                prop_name
              ))),
            );
          }
        }
      }
    }

    ControlFlow::None
  }
}
