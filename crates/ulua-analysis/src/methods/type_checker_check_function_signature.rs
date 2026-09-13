/// 经 Arc<Scope> 取可变借用：C++ shared_ptr<Scope> 直接可变访问的对应物。
/// 每次调用产生独立短命 &mut，与原代码逐块 cast 的别名模式一致。
///
/// # Safety（调用方契约）
/// Scope 在类型检查阶段单线程独占，调用期间不得经由其他路径写同一 Scope。
use alloc::string::String;
use alloc::{sync::Arc, vec::Vec};
use core::ffi::CStr;

use ulua_ast::records::{ast_expr_function::AstExprFunction, location::Location};

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, is_nil::is_nil,
    try_get_type_pack_type_at::try_get_type_pack_type_at,
  },
  records::{
    binding::Binding, function_argument::FunctionArgument, function_definition::FunctionDefinition,
    function_type::FunctionType, scope::Scope, symbol::Symbol, type_checker::TypeChecker,
    union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    error_type::ErrorType, scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
  },
};
/// # Safety
/// 调用方须保证：`scope` 指向的 Scope 在类型检查阶段单线程独占，
/// 调用期间不得经由其他路径写同一 Scope。
pub(crate) unsafe fn scope_mut(scope: *const ScopePtr) -> *mut Scope {
  // SAFETY: Arc::as_ptr 去 shared 语义；单线程独占期内无并发别名，见上方契约。
  unsafe { Arc::as_ptr(&*scope) as *mut Scope }
}

impl TypeChecker {
  pub fn check_function_signature(
    &mut self,
    scope: &ScopePtr,
    sub_level: i32,
    expr: &AstExprFunction,
    original_name: Option<Location>,
    _self_type: Option<TypeId>,
    expected_type: Option<TypeId>,
  ) -> (TypeId, ScopePtr) {
    let fun_scope = self.child_function_scope(scope, &expr.base.base.location, sub_level);

    let mut expected_function_type: Option<&FunctionType> = None;
    if let Some(et) = expected_type {
      let et = follow_type_id(et);
      if let Some(ftv) = get_type_id::<FunctionType>(et) {
        expected_function_type = Some(ftv);
      } else if let Some(utv) = get_type_id::<UnionType>(et) {
        for option in &utv.options {
          if let Some(ftv) = get_type_id::<FunctionType>(follow_type_id(*option)) {
            if expected_function_type.is_none() {
              expected_function_type = Some(ftv);
            } else {
              expected_function_type = None;
              break;
            }
          }
        }
      }
    }

    let generic_defs = self.create_generic_types(
      &fun_scope,
      None,
      &expr.base.base,
      &expr.generics,
      &expr.generic_packs,
      false,
    );

    let ret_pack = if let Some(ann) = unsafe { expr.return_annotation.as_ref() } {
      // if (expr.returnAnnotation) retPack = resolveTypePack(funScope, *expr.returnAnnotation);
      self.resolve_type_pack_scope_ptr_ast_type_pack(fun_scope.clone(), ann)
    } else if self.is_nonstrict_mode() {
      // else if (isNonstrictMode()) retPack = any_type_pack;
      self.any_type_pack
    } else if expected_function_type
      .is_some_and(|ftv| ftv.generics.is_empty() && ftv.generic_packs.is_empty())
    {
      // else if (expectedFunctionType && expectedFunctionType->generics.empty() && expectedFunctionType->genericPacks.empty())
      // auto [head, tail] = flatten(expectedFunctionType->retTypes);
      let (head, tail) = flatten_type_pack_id(expected_function_type.unwrap().ret_types);

      // Do not infer 'nil' as function return type
      // if (!tail && head.size() == 1 && isNil(head[0])) retPack = freshTypePack(funScope);
      // else retPack = addTypePack(head, tail);
      if tail.is_none() && head.len() == 1 && is_nil(head[0]) {
        self.fresh_type_pack_scope_ptr(fun_scope.clone())
      } else {
        self.add_type_pack_vector_type_id_optional_type_pack_id(&head, tail)
      }
    } else {
      // else retPack = freshTypePack(funScope);
      self.fresh_type_pack_scope_ptr(fun_scope.clone())
    };

    // SAFETY: 见 scope_mut 契约。
    unsafe {
      (*scope_mut(&fun_scope)).return_type = ret_pack;
    }

    let mut vararg_pack: Option<TypePackId> = None;
    if expr.vararg {
      if !expr.vararg_annotation.is_null() {
        // funScope->varargPack = resolveTypePack(funScope, *expr.varargAnnotation);
        vararg_pack = Some(
          // SAFETY: vararg_annotation 非 null，指向 AST arena 节点。
          self.resolve_type_pack_scope_ptr_ast_type_pack(fun_scope.clone(), unsafe {
            &*expr.vararg_annotation
          }),
        );
      } else {
        // if (expectedFunctionType && !isNonstrictMode())
        if let Some(expected) = expected_function_type
          && !self.is_nonstrict_mode()
        {
          // auto [head, tail] = flatten(expectedFunctionType->argTypes);
          let (mut head, tail) = flatten_type_pack_id(expected.arg_types);

          if expr.args.size <= head.len() {
            // head.erase(head.begin(), head.begin() + expr.args.size);
            head.drain(0..expr.args.size);
            // funScope->varargPack = addTypePack(head, tail);
            vararg_pack =
              Some(self.add_type_pack_vector_type_id_optional_type_pack_id(&head, tail));
          } else if let Some(tail) = tail {
            // if (get<VariadicTypePack>(follow(*tail)))
            //     funScope->varargPack = addTypePack({}, tail);
            // SAFETY: follow_type_pack_id 为 unsafe 函数，tail 有效。
            let followed = unsafe { follow_type_pack_id(tail) };
            if get_type_pack_id::<VariadicTypePack>(followed).is_some() {
              vararg_pack = Some(
                self.add_type_pack_vector_type_id_optional_type_pack_id(&Vec::new(), Some(tail)),
              );
            }
          } else {
            // funScope->varargPack = addTypePack({});
            vararg_pack =
              Some(self.add_type_pack_vector_type_id_optional_type_pack_id(&Vec::new(), None));
          }
        }

        // TODO: should this be a free type pack? CLI-39910
        // if (!funScope->varargPack)
        //     funScope->varargPack = any_type_pack;
        if vararg_pack.is_none() {
          vararg_pack = Some(self.any_type_pack);
        }
      }

      // SAFETY: 见 scope_mut 契约。
      unsafe {
        (*scope_mut(&fun_scope)).vararg_pack = vararg_pack;
      }
    }

    let mut arg_types = Vec::new();
    if !expr.self_.is_null() {
      let fresh = self.fresh_type_scope_ptr(fun_scope.clone());
      let self_type = self.any_if_nonstrict(fresh);
      // SAFETY: expr.self_ 非 null；见 scope_mut 契约。
      unsafe {
        (*scope_mut(&fun_scope)).bindings.insert(
          Symbol::from_local(expr.self_),
          Binding {
            type_id: self_type,
            location: (*expr.self_).location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }
      arg_types.push(self_type);
    }

    for (i, local) in expr.args.iter().enumerate() {
      let expected_arg_type =
        expected_function_type.and_then(|ftv| try_get_type_pack_type_at(ftv.arg_types, i));

      // SAFETY: local 指向 AST arena 节点（AstArray 迭代产出裸句柄）。
      let arg_type = if let Some(ann) = unsafe { (**local).annotation.as_ref() } {
        let resolved = self.resolve_type(fun_scope.clone(), ann);
        if get_type_id::<ErrorType>(follow_type_id(resolved)).is_some() {
          if let Some(expected_arg_type) = expected_arg_type {
            expected_arg_type
          } else {
            let fresh = self.fresh_type_scope_ptr(fun_scope.clone());
            self.any_if_nonstrict(fresh)
          }
        } else {
          resolved
        }
      } else if let Some(expected_arg_type) = expected_arg_type {
        expected_arg_type
      } else {
        let fresh = self.fresh_type_scope_ptr(fun_scope.clone());
        self.any_if_nonstrict(fresh)
      };
      // SAFETY: local 指向 AST arena 节点；见 scope_mut 契约。
      unsafe {
        (*scope_mut(&fun_scope)).bindings.insert(
          Symbol::from_local(*local),
          Binding {
            type_id: arg_type,
            location: (**local).location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }
      arg_types.push(arg_type);
    }

    let arg_pack = self.add_type_pack_vector_type_id_optional_type_pack_id(&arg_types, vararg_pack);
    let defn = FunctionDefinition {
      definition_module_name: None,
      definition_location: expr.base.base.location,
      vararg_location: if expr.vararg {
        Some(expr.vararg_location)
      } else {
        None
      },
      original_name_location: original_name.unwrap_or(Location::new(
        expr.base.base.location.begin,
        expr.base.base.location.begin,
      )),
    };

    let mut function_type =
      FunctionType::function_type_new(arg_pack, ret_pack, Some(defn), !expr.self_.is_null());
    // C++ TypeInfer.cpp:4047-4058: if we have a generic expected function
    // type and no generics of our own, we should use the expected ones
    // (this is how an anonymous function passed where a generic function is
    // expected acquires the expected type's — possibly vestigial — generics,
    // e.g. `function(x: string) ... end` checked against `<a>(a) -> a`
    // becomes `<a>(string) -> string`).
    function_type.generics = if generic_defs.generic_types.is_empty()
      && let Some(eft) = expected_function_type
    {
      eft.generics.clone()
    } else {
      generic_defs
        .generic_types
        .iter()
        .map(|def| def.ty)
        .collect()
    };
    // C++ TypeInfer.cpp:4060-4071: likewise for generic type packs.
    function_type.generic_packs = if generic_defs.generic_packs.is_empty()
      && let Some(eft) = expected_function_type
    {
      eft.generic_packs.clone()
    } else {
      generic_defs
        .generic_packs
        .iter()
        .map(|def| def.tp)
        .collect()
    };
    function_type
      .arg_names
      .reserve(expr.args.len() + usize::from(!expr.self_.is_null()));
    if !expr.self_.is_null() {
      function_type.arg_names.push(Some(FunctionArgument {
        name: String::from("self"),
        location: Location::new(expr.base.base.location.begin, expr.base.base.location.begin),
      }));
    }
    for local in expr.args.iter() {
      function_type.arg_names.push(Some(FunctionArgument {
        // SAFETY: name.value 为 NUL 结尾 C 字符串（AST arena 持有）。
        name: unsafe { CStr::from_ptr((**local).name.value).to_string_lossy() }.into_owned(),
        // SAFETY: local 指向 AST arena 节点。
        location: unsafe { (**local).location },
      }));
    }

    let fun_ty = self.add_type(&function_type);
    (fun_ty, fun_scope)
  }
}
