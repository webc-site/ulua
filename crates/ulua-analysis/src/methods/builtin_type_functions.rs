//! `builtin_type_functions` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::ToString;
use core::ptr::NonNull;

// The 28 reducer fns. 25 already existed; `unm`/`refine` were ported in this // cluster; `user_defined_type_function` is the VM-bridge reducer owned by another // agent (now ported, signature-compatible with `ReducerFunction`).
use crate::functions::add_type_function::add_type_function;
use crate::{
  enums::polarity::Polarity,
  functions::{
    and_type_function::and_type_function, concat_type_function::concat_type_function,
    div_type_function::div_type_function, getmetatable_type_function::getmetatable_type_function,
    idiv_type_function::idiv_type_function, index_type_function::index_type_function,
    intersect_type_function::intersect_type_function, keyof_type_function::keyof_type_function,
    le_type_function::le_type_function, len_type_function::len_type_function,
    lt_type_function::lt_type_function, mod_type_function::mod_type_function,
    mul_type_function::mul_type_function, not_type_function::not_type_function,
    objectof_type_function::objectof_type_function, or_type_function::or_type_function,
    pow_type_function::pow_type_function, rawget_type_function::rawget_type_function,
    rawkeyof_type_function::rawkeyof_type_function, refine_type_function::refine_type_function,
    setmetatable_type_function::setmetatable_type_function,
    singleton_type_function::singleton_type_function, sub_type_function::sub_type_function,
    union_type_function::union_type_function, unm_type_function::unm_type_function,
    user_defined_type_function::user_defined_type_function,
    weakoptional_type_func::weakoptional_type_func,
  },
  records::{
    arena_handle::Handle, builtin_type_functions::BuiltinTypeFunctions, generic_type::GenericType,
    generic_type_definition::GenericTypeDefinition, scope::Scope, type_arena::TypeArena,
    type_fun::TypeFun, type_function::TypeFunction,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::reducer_function::ReducerFunction,
};

impl BuiltinTypeFunctions {
  /// 直译 cpp `BuiltinTypeFunctions::addToScope(TypeArena&, Scope&)`。
  ///
  /// 前提（原 `# Safety` 契约由签名承担）：`arena` 句柄指向本次注册期间存活、
  /// 被独占可写的 `TypeArena`（下方所有 `add_type` 追加类型均要求该独占窗口）；
  /// `&self` 持有的各 `TypeFunction`（`len_func` 等）必须比登记进 `arena` 的
  /// `TypeFunctionInstanceType` 更长寿——实例类型内嵌了指向其函数体的裸指针句柄。
  pub fn add_to_scope(&self, arena: Handle<TypeArena>, scope: &mut Scope) {
    // Closure for unary type function
    let mk_unary = |tf: &TypeFunction| -> TypeFun {
      let t = arena
        .get_mut()
        .add_type(GenericType::generic_type_name_polarity(
          "T",
          Polarity::Negative,
        ));
      let generic_t = GenericTypeDefinition {
        ty: t,
        default_value: None,
      };
      // `tf` 为 &self.<fn> 的身份裸指针，self 比本 arena 用法长寿（见函数级
      // 前提），NonNull 恒非空；arena 由 Handle 契约保证独占可写，登记实例类型。
      let result_type = arena.get_mut().add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
          NonNull::from(tf),
          vec![t],
          vec![],
        ));
      TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
        vec![generic_t],
        result_type,
        None,
      )
    };

    // Closure for binary type function with default (second type defaults to first)
    let mk_binary_with_default = |tf: &TypeFunction| -> TypeFun {
      let t = arena
        .get_mut()
        .add_type(GenericType::generic_type_name_polarity(
          "T",
          Polarity::Negative,
        ));
      let u = arena
        .get_mut()
        .add_type(GenericType::generic_type_name_polarity(
          "U",
          Polarity::Negative,
        ));
      let generic_t = GenericTypeDefinition {
        ty: t,
        default_value: None,
      };
      let generic_u = GenericTypeDefinition {
        ty: u,
        default_value: Some(t),
      };
      let result_type = arena.get_mut().add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
          NonNull::from(tf),
          vec![t, u],
          vec![],
        ));
      TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
        vec![generic_t, generic_u],
        result_type,
        None,
      )
    };

    // Closure for binary type function without default
    let mk_binary = |tf: &TypeFunction| -> TypeFun {
      let t = arena
        .get_mut()
        .add_type(GenericType::generic_type_name_polarity(
          "T",
          Polarity::Negative,
        ));
      let u = arena
        .get_mut()
        .add_type(GenericType::generic_type_name_polarity(
          "U",
          Polarity::Negative,
        ));
      let generic_t = GenericTypeDefinition {
        ty: t,
        default_value: None,
      };
      let generic_u = GenericTypeDefinition {
        ty: u,
        default_value: None,
      };
      let result_type = arena.get_mut().add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
          NonNull::from(tf),
          vec![t, u],
          vec![],
        ));
      TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
        vec![generic_t, generic_u],
        result_type,
        None,
      )
    };

    let exported = &mut scope.exported_type_bindings;
    exported.insert(self.len_func.name.clone(), mk_unary(&self.len_func));
    exported.insert(self.unm_func.name.clone(), mk_unary(&self.unm_func));

    exported.insert(
      self.add_func.name.clone(),
      mk_binary_with_default(&self.add_func),
    );
    exported.insert(
      self.sub_func.name.clone(),
      mk_binary_with_default(&self.sub_func),
    );
    exported.insert(
      self.mul_func.name.clone(),
      mk_binary_with_default(&self.mul_func),
    );
    exported.insert(
      self.div_func.name.clone(),
      mk_binary_with_default(&self.div_func),
    );
    exported.insert(
      self.idiv_func.name.clone(),
      mk_binary_with_default(&self.idiv_func),
    );
    exported.insert(
      self.pow_func.name.clone(),
      mk_binary_with_default(&self.pow_func),
    );
    exported.insert(
      self.mod_func.name.clone(),
      mk_binary_with_default(&self.mod_func),
    );
    exported.insert(
      self.concat_func.name.clone(),
      mk_binary_with_default(&self.concat_func),
    );

    exported.insert(
      self.lt_func.name.clone(),
      mk_binary_with_default(&self.lt_func),
    );
    exported.insert(
      self.le_func.name.clone(),
      mk_binary_with_default(&self.le_func),
    );
    exported.insert(self.keyof_func.name.clone(), mk_unary(&self.keyof_func));
    exported.insert(
      self.rawkeyof_func.name.clone(),
      mk_unary(&self.rawkeyof_func),
    );

    exported.insert(self.index_func.name.clone(), mk_binary(&self.index_func));
    exported.insert(self.rawget_func.name.clone(), mk_binary(&self.rawget_func));

    exported.insert(
      self.setmetatable_func.name.clone(),
      mk_binary(&self.setmetatable_func),
    );
    exported.insert(
      self.getmetatable_func.name.clone(),
      mk_unary(&self.getmetatable_func),
    );
  }
}

// C++ `BuiltinTypeFunctions::BuiltinTypeFunctions()` — the real default ctor
// (BuiltinTypeFunctions.cpp:2555-2585). Brace-initializes all 28 `TypeFunction`
// members `{name, reducerFn[, canReduceGenerics]}`. Each reducer is stored as a
// plain `fn` item in the `reducer` field (the project's MagicFunction-style
// fn-pointer wiring; no transmute / no erased cast).

impl BuiltinTypeFunctions {
  /// C++ `BuiltinTypeFunctions::BuiltinTypeFunctions()`.
  pub fn new() -> Self {
    // Helper to build a `TypeFunction { name, reducer, can_reduce_generics }`.
    fn tf(name: &str, reducer: ReducerFunction) -> TypeFunction {
      TypeFunction {
        name: name.to_string(),
        reducer,
        can_reduce_generics: false,
      }
    }
    fn tf_gen(name: &str, reducer: ReducerFunction) -> TypeFunction {
      TypeFunction {
        name: name.to_string(),
        reducer,
        can_reduce_generics: true,
      }
    }

    BuiltinTypeFunctions {
      user_func: tf("user", user_defined_type_function),

      not_func: tf("not", not_type_function),
      len_func: tf("len", len_type_function),
      unm_func: tf("unm", unm_type_function),

      add_func: tf("add", add_type_function),
      sub_func: tf("sub", sub_type_function),
      mul_func: tf("mul", mul_type_function),
      div_func: tf("div", div_type_function),
      idiv_func: tf("idiv", idiv_type_function),
      pow_func: tf("pow", pow_type_function),
      mod_func: tf("mod", mod_type_function),

      concat_func: tf("concat", concat_type_function),

      and_func: tf_gen("and", and_type_function),
      or_func: tf_gen("or", or_type_function),

      lt_func: tf("lt", lt_type_function),
      le_func: tf("le", le_type_function),

      refine_func: tf_gen("refine", refine_type_function),
      singleton_func: tf("singleton", singleton_type_function),
      union_func: tf("union", union_type_function),
      intersect_func: tf("intersect", intersect_type_function),

      keyof_func: tf("keyof", keyof_type_function),
      rawkeyof_func: tf("rawkeyof", rawkeyof_type_function),
      index_func: tf("index", index_type_function),
      rawget_func: tf("rawget", rawget_type_function),

      setmetatable_func: tf("setmetatable", setmetatable_type_function),
      getmetatable_func: tf("getmetatable", getmetatable_type_function),

      objectof_func: tf("objectof", objectof_type_function),

      weakoptional_func: tf("weakoptional", weakoptional_type_func),
    }
  }
}
impl Default for BuiltinTypeFunctions {
  fn default() -> Self {
    Self::new()
  }
}
