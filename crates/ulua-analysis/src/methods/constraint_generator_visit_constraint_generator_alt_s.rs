use alloc::{string::String, vec::Vec};
use core::{ffi::CStr, ptr::null_mut};

use ulua_ast::records::{ast_name::AstName, ast_stat_class::AstStatClass};
use ulua_common::{FFlag, LUAU_ASSERT};

use crate::{
  enums::{control_flow::ControlFlow, type_variant::TypeVariant},
  functions::{
    add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns,
    as_mutable_type::as_mutable_type_id, checkpoint::checkpoint, follow_type::follow_type_id,
    for_each_constraint::for_each_constraint, get_mutable_type::get_mutable,
    get_type_alt_j::get_type_id,
    propagate_deprecated_attribute_to_constraint::propagate_deprecated_attribute_to_constraint,
    slice_type_pack::slice_type_pack,
  },
  records::{
    blocked_type::BlockedType, class_decl_record::ClassDeclRecord, constraint::Constraint,
    constraint_generator::ConstraintGenerator, extern_type::ExternType, function_type,
    generalization_constraint::GeneralizationConstraint, property_type::Property,
    table_type::TableType,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr},
};
fn ast_name_to_string(name: AstName) -> String {
  if name.value.is_null() {
    String::new()
  } else {
    unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() }
  }
}

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_class(
    &mut self,
    scope: &ScopePtr,
    stat_class: *mut AstStatClass,
  ) -> ControlFlow {
    let stat_class_ref = unsafe { &*stat_class };
    LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());

    let class_decl_record = self.class_decl_records.find(&stat_class_ref.name).copied();
    let Some(mut class_decl_record) = class_decl_record else {
      // C++: this is unpopulated in fragment autocomplete.
      return ControlFlow::None;
    };

    let mut method_names: Vec<AstName> = Vec::new();

    for member in stat_class_ref.members.as_slice() {
      let Some(method) = member.get_if_1() else {
        continue;
      };

      if method_names.contains(&method.function_name) {
        continue;
      }
      method_names.push(method.function_name);

      let method_name = ast_name_to_string(method.function_name);
      let function_type = {
        let class_ty = follow_type_id(class_decl_record.ty);
        // ClassDeclRecord::ty 记录的是 class 的 ExternType，必命中
        let class_ = get_type_id::<ExternType>(class_ty);
        LUAU_ASSERT!(class_.is_some());
        let class_ = class_.unwrap();
        LUAU_ASSERT!(class_.metatable.is_some());

        let metatable_ty = follow_type_id(class_.metatable.unwrap());
        let metatable = get_type_id::<TableType>(metatable_ty);
        LUAU_ASSERT!(metatable.is_some());
        let metatable = metatable.unwrap();

        let maybe_function_prop: Property =
          if let Some(instance_prop) = class_.props.get(&method_name) {
            instance_prop.clone()
          } else if let Some(meta_instance_prop) = metatable.props.get(&method_name) {
            meta_instance_prop.clone()
          } else {
            LUAU_ASSERT!(false);
            Property::default()
          };

        LUAU_ASSERT!(maybe_function_prop.read_ty.is_some());
        LUAU_ASSERT!(maybe_function_prop.write_ty.is_none());
        maybe_function_prop.read_ty.unwrap()
      };

      let sig = unsafe {
        self.check_function_signature(
          scope,
          &mut class_decl_record as *mut ClassDeclRecord,
          method.function,
          None,
          Some((*method.function).base.base.location),
        )
      };

      let start = unsafe { checkpoint(self as *const _) };
      self.check_function_body(&sig.body_scope, unsafe { &*method.function });
      let end = unsafe { checkpoint(self as *const _) };

      let constraint_scope: &ScopePtr = &sig.signature_scope;
      let c: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
        constraint_scope,
        unsafe { (*method.function).base.base.location },
        ConstraintV::Generalization(GeneralizationConstraint {
          generalized_type: function_type,
          source_type: sig.signature,
          interior_types: Vec::new(),
          has_deprecated_attribute: false,
          deprecated_info: Default::default(),
          no_generics: false,
        }),
      );

      unsafe { propagate_deprecated_attribute_to_constraint(&mut (*c).c, method.function) };

      if FFlag::LuauConstraintGraph.get() {
        unsafe { add_all_as_dependencies_and_chain_returns(start, end, self, c) };
      } else {
        let mut previous: *mut Constraint = null_mut();
        for_each_constraint(start, end, self, |constraint: *mut Constraint| {
          unsafe { (*c).deprecated_dependencies.push(constraint) };
          if let ConstraintV::PackSubtype(psc) = unsafe { &(*constraint).c }
            && psc.returns
          {
            if !previous.is_null() {
              unsafe { (*constraint).deprecated_dependencies.push(previous) };
            }
            previous = constraint;
          }
        });
      }

      // function_type 来自 BlockedType 记录，必命中；对照 C++:2618
      // `getMutable<BlockedType>(functionType)->setOwner(genConstraint)`
      let blocked = get_mutable::<BlockedType>(function_type);
      LUAU_ASSERT!(blocked.is_some());
      blocked.unwrap().set_owner(c as *const _);

      if method_name == "__init"
        && let Some(new_blocked_ty) = class_decl_record.new_blocked_ty
      {
        // 对照 C++:2624 `const FunctionType* initFn = get<FunctionType>(sig.signature); LUAU_ASSERT(initFn);`
        if let Some(init_fn) = get_type_id::<function_type::FunctionType>(sig.signature) {
          let new_args = slice_type_pack(
            1,
            init_fn.arg_types,
            &[],
            None,
            unsafe { &*self.builtin_types },
            unsafe { &mut *self.arena },
          );
          let mut new_fn_type = init_fn.clone();
          new_fn_type.arg_types = new_args;
          new_fn_type.ret_types = unsafe {
            (*self.arena).add_type_pack_initializer_list_type_id(&[class_decl_record.ty])
          };
          new_fn_type.has_self = false;

          let new_fn = unsafe { (*self.arena).add_type(new_fn_type) };
          unsafe {
            (*as_mutable_type_id(new_blocked_ty)).ty = TypeVariant::Bound(new_fn);
          }
        }
      }
    }

    ControlFlow::None
  }
}
