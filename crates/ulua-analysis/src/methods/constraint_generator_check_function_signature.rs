use alloc::{string::String, sync::Arc, vec::Vec};
use core::{ffi::CStr, mem::take};
use std::ptr::null_mut;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_local::AstLocal, location::Location,
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type_pack::as_mutable_type_pack_id, bind_free_type::bind_free_type,
    emplace_type_pack::emplace_type_pack, extend_type_pack::extend_type_pack,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id, is_nil::is_nil,
    is_optional::is_optional,
  },
  records::{
    any_type::AnyType, ast_array::AstArray, ast_node::AstNode, binding::Binding,
    class_decl_record::ClassDeclRecord, constraint_generator::ConstraintGenerator,
    free_type::FreeType, function_argument::FunctionArgument,
    function_definition::FunctionDefinition, function_signature::FunctionSignature, function_type,
    module::Module, scope::Scope, symbol::Symbol, type_pack::TypePack, union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_function_signature(
    &mut self,
    parent: &ScopePtr,
    enclosing_class: *mut ClassDeclRecord,
    fn_node: *mut AstExprFunction,
    expected_type: Option<TypeId>,
    original_name: Option<Location>,
  ) -> FunctionSignature {
    let fn_ref = unsafe { &*fn_node };
    LUAU_ASSERT!(
      FFlag::DebugLuauUserDefinedClasses.get() || enclosing_class.is_null(),
      "check_function_signature: enclosing_class must be null when DebugLuauUserDefinedClasses is off"
    );

    let mut generic_types: Vec<TypeId> = Vec::new();
    let mut generic_type_packs: Vec<TypePackId> = Vec::new();

    let mut expected_type_opt = expected_type;
    if let Some(et) = expected_type_opt {
      expected_type_opt = Some(follow_type_id(et));
    }

    let has_generics = fn_ref.generics.size > 0 || fn_ref.generic_packs.size > 0;

    let signature_scope: ScopePtr = unsafe { self.child_scope(fn_node as *mut AstNode, parent) };
    let signature_scope_mut = signature_scope.as_ref() as *const Scope as *mut Scope;

    // We need to assign returnType before creating bodyScope so that the
    // return type gets propagated to bodyScope.
    let mut return_type: TypePackId = self.fresh_type_pack(&signature_scope, Polarity::Positive);
    unsafe {
      (*signature_scope_mut).return_type = return_type;
    }

    let body_scope: ScopePtr =
      unsafe { self.child_scope(fn_ref.body as *mut AstNode, &signature_scope) };
    let body_scope_mut = body_scope.as_ref() as *const Scope as *mut Scope;

    if has_generics {
      let generic_definitions = self.create_generics(
        &signature_scope,
        AstArray {
          data: fn_ref.generics.data,
          size: fn_ref.generics.size,
        },
        // C++ `createGenerics(signatureScope, fn->generics)` uses the
        // default `useCache = false` (ConstraintGenerator.h:481): each
        // function signature gets fresh generics keyed to its own
        // signature scope. Passing `true` here aliased every same-named
        // generic (e.g. `S`) to the first one created, leaving its
        // `scope` pointing at an unrelated sibling scope so `subsumes`
        // failed and nested generic subtyping spuriously rejected.
        false,
        true,
      );
      let generic_pack_definitions = self.create_generic_packs(
        &signature_scope,
        AstArray {
          data: fn_ref.generic_packs.data,
          size: fn_ref.generic_packs.size,
        },
        // C++ `createGenericPacks(signatureScope, fn->genericPacks)` uses
        // the default `useCache = false` (ConstraintGenerator.h:498),
        // matching the `createGenerics` call above: fresh per-signature
        // generic packs rather than name-aliased cached ones.
        false,
        true,
      );

      for (_name, g) in &generic_definitions {
        generic_types.push(g.ty);
      }

      for (_name, g) in &generic_pack_definitions {
        generic_type_packs.push(g.tp);
      }

      expected_type_opt = None;
    }

    let mut arg_types: Vec<TypeId> = Vec::new();
    let mut arg_names: Vec<Option<FunctionArgument>> = Vec::new();
    let mut expected_arg_pack = TypePack {
      head: Vec::new(),
      tail: None,
    };

    // 对照 C++:4254 `const FunctionType* expectedFunction = expectedType ? get<FunctionType>(*expectedType) : nullptr;`
    let mut expected_function: Option<&function_type::FunctionType> =
      expected_type_opt.and_then(get_type_id::<function_type::FunctionType>);

    // This check ensures that expected_type is precisely optional and not any
    // (since any is also an optional type)
    if let Some(et) = expected_type_opt
      && is_optional(et)
      && get_type_id::<AnyType>(et).is_none()
      && let Some(ut) = get_type_id::<UnionType>(et)
    {
      for u in &ut.options {
        // 对照 C++:4261-4266 `if (!isNil(u) && get<FunctionType>(follow(u)))`
        if let Some(ft) = get_type_id::<function_type::FunctionType>(*u)
          && !is_nil(*u)
        {
          expected_function = Some(ft);
          break;
        }
      }
    }

    if let Some(ef) = expected_function {
      expected_arg_pack = unsafe {
        extend_type_pack(
          &mut *self.arena,
          self.builtin_types,
          ef.arg_types,
          fn_ref.args.size,
          Vec::new(),
        )
      };

      generic_types = ef.generics.clone();
      generic_type_packs = ef.generic_packs.clone();
    }

    let mut has_explicit_self = false;
    let mut has_self = false;

    if FFlag::DebugLuauUserDefinedClasses.get() {
      if !enclosing_class.is_null() && fn_ref.args.size > 0 {
        let first_arg = unsafe { &**fn_ref.args.data };
        let arg_name = unsafe { CStr::from_ptr(first_arg.name.value).to_string_lossy() };
        has_explicit_self = arg_name == "self";
      }
      has_self = has_explicit_self || !fn_ref.self_.is_null();

      if has_self {
        let self_type: TypeId = if !enclosing_class.is_null() {
          unsafe { (*enclosing_class).ty }
        } else {
          self.fresh_type(&signature_scope, Polarity::Negative)
        };

        let mut self_local: *mut AstLocal = null_mut();
        if !fn_ref.self_.is_null() {
          self_local = fn_ref.self_;
        } else if has_explicit_self {
          self_local = unsafe { *fn_ref.args.data };
        }

        LUAU_ASSERT!(!self_local.is_null());

        arg_types.push(self_type);
        let arg_name = unsafe { CStr::from_ptr((*self_local).name.value).to_string_lossy() };
        arg_names.push(Some(FunctionArgument {
          name: arg_name.into_owned(),
          location: unsafe { (*self_local).location },
        }));

        unsafe {
          (*signature_scope_mut).bindings.insert(
            Symbol::from_local(self_local),
            Binding {
              type_id: self_type,
              location: (*self_local).location,
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            },
          );
        }

        let def = unsafe { (*self.dfg).get_def_local(self_local) };
        unsafe {
          *(*signature_scope_mut).lvalue_types.get_or_insert(def) = self_type;
        }
        self.update_r_value_refinements_scope_ptr_def_id_type_id(&signature_scope, def, self_type);
      }
    } else {
      if !fn_ref.self_.is_null() {
        let self_type = self.fresh_type(&signature_scope, Polarity::Negative);
        arg_types.push(self_type);
        let arg_name = unsafe { CStr::from_ptr((*fn_ref.self_).name.value).to_string_lossy() };
        arg_names.push(Some(FunctionArgument {
          name: arg_name.into_owned(),
          location: unsafe { (*fn_ref.self_).location },
        }));

        unsafe {
          (*signature_scope_mut).bindings.insert(
            Symbol::from_local(fn_ref.self_),
            Binding {
              type_id: self_type,
              location: (*fn_ref.self_).location,
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            },
          );
        }

        let def = unsafe { (*self.dfg).get_def_local(fn_ref.self_) };
        unsafe {
          *(*signature_scope_mut).lvalue_types.get_or_insert(def) = self_type;
        }
        self.update_r_value_refinements_scope_ptr_def_id_type_id(&signature_scope, def, self_type);
      }
    }

    for i in 0..fn_ref.args.size {
      if FFlag::DebugLuauUserDefinedClasses.get() && has_explicit_self && i == 0 {
        continue;
      }

      let local_ptr = unsafe { *fn_ref.args.data.add(i) };
      let local = unsafe { &*local_ptr };

      let arg_ty: TypeId = if !local.annotation.is_null() {
        self.resolve_type(
          signature_scope_mut,
          local.annotation,
          false,
          true,
          Polarity::Negative,
        )
      } else if i < expected_arg_pack.head.len() {
        expected_arg_pack.head[i]
      } else {
        self.fresh_type(&signature_scope, Polarity::Negative)
      };

      arg_types.push(arg_ty);
      let arg_name = unsafe { CStr::from_ptr(local.name.value).to_string_lossy() };
      arg_names.push(Some(FunctionArgument {
        name: arg_name.into_owned(),
        location: local.location,
      }));

      let def = unsafe { (*self.dfg).get_def_local(local_ptr) };
      unsafe {
        (*signature_scope_mut).bindings.insert(
          Symbol::from_local(local_ptr),
          Binding {
            type_id: arg_ty,
            location: local.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
        *(*signature_scope_mut).lvalue_types.get_or_insert(def) = arg_ty;
      }
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&signature_scope, def, arg_ty);
    }

    let mut vararg_pack: TypePackId;

    if fn_ref.vararg {
      if !fn_ref.vararg_annotation.is_null() {
        vararg_pack = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
          signature_scope_mut,
          fn_ref.vararg_annotation,
          false,
          true,
          Polarity::Negative,
        );
      } else if expected_arg_pack.tail.is_some()
        && get_type_pack_id::<VariadicTypePack>(expected_arg_pack.tail.unwrap()).is_some()
      {
        vararg_pack = expected_arg_pack.tail.unwrap();
      } else {
        vararg_pack = unsafe { (*self.builtin_types).any_type_pack };
      }

      unsafe {
        (*signature_scope_mut).vararg_pack = Some(vararg_pack);
      }
      unsafe {
        (*body_scope_mut).vararg_pack = Some(vararg_pack);
      }
    } else {
      vararg_pack = unsafe {
        (*self.arena).add_type_pack_t(VariadicTypePack {
          ty: { (*self.builtin_types).any_type },
          hidden: true,
        })
      };

      unsafe {
        (*signature_scope_mut).vararg_pack = None;
      }
      unsafe {
        (*body_scope_mut).vararg_pack = None;
      }
    }

    LUAU_ASSERT!(!vararg_pack.is_null());

    if !fn_ref.self_.is_null() {
      generic_types.push(arg_types[0]);
    }

    let offset: usize = if !fn_ref.self_.is_null() { 1 } else { 0 };
    // arg_types 跳过 self 位置后与 AST 参数一一对应
    for (i, &ast_arg) in fn_ref.args.as_slice().iter().enumerate() {
      let arg_ty = arg_types[i + offset];
      if unsafe { (*ast_arg).annotation.is_null() } {
        generic_types.push(arg_ty);
      }
    }

    vararg_pack = unsafe { follow_type_pack_id(vararg_pack) };
    return_type = unsafe { follow_type_pack_id(return_type) };
    if fn_ref.vararg_annotation.is_null() {
      generic_type_packs.push(vararg_pack);
    }
    if fn_ref.return_annotation.is_null() {
      generic_type_packs.push(return_type);
    }

    if !fn_ref.return_annotation.is_null() {
      let annotated_ret_type = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
        signature_scope_mut,
        fn_ref.return_annotation,
        false,
        true,
        Polarity::Negative,
      );
      {
        unsafe {
          emplace_type_pack(
            as_mutable_type_pack_id(return_type),
            TypePackVariant::Bound(annotated_ret_type),
          )
        };
      }
    } else if let Some(expected_function) = expected_function {
      // 对照 C++:4432-4434 `else if (expectedFunction) emplaceTypePack<BoundTypePack>(...)`
      unsafe {
        emplace_type_pack(
          as_mutable_type_pack_id(return_type),
          TypePackVariant::Bound(expected_function.ret_types),
        )
      };
    }

    let mut actual_function = function_type::FunctionType::function_type_new(
      unsafe {
        (*self.arena).add_type_pack_vector_type_id_optional_type_pack_id(
          take(&mut arg_types),
          Some(vararg_pack),
        )
      },
      return_type,
      None,
      if FFlag::DebugLuauUserDefinedClasses.get() {
        has_self
      } else {
        !fn_ref.self_.is_null()
      },
    );
    actual_function.generics = generic_types;
    actual_function.generic_packs = generic_type_packs;
    actual_function.arg_names = arg_names;

    let defn = FunctionDefinition {
      definition_module_name: Some(self.module.as_ref().unwrap().name.clone()),
      definition_location: fn_ref.base.base.location,
      vararg_location: if fn_ref.vararg {
        Some(fn_ref.vararg_location)
      } else {
        None
      },
      original_name_location: original_name
        .unwrap_or(Location::with_length(fn_ref.base.base.location.begin, 0)),
    };
    actual_function.definition = Some(defn);

    let actual_function_type = unsafe { (*self.arena).add_type(actual_function) };
    LUAU_ASSERT!(!actual_function_type.is_null());
    unsafe {
      let module_ptr = Arc::as_ptr(self.module.as_ref().unwrap()) as *mut Module;
      *(*module_ptr)
        .ast_types
        .get_or_insert(fn_node as *const AstExpr) = actual_function_type;
    }

    if let Some(et) = expected_type_opt
      && get_type_id::<FreeType>(et).is_some()
    {
      bind_free_type(et, actual_function_type);
    }

    *self.scope_to_function.get_or_insert(signature_scope_mut) = actual_function_type;

    FunctionSignature {
      signature: actual_function_type,
      signature_scope,
      body_scope,
    }
  }
}
