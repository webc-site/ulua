use core::ptr::NonNull;

use crate::{
  enums::polarity::Polarity,
  records::{
    arena_handle::Handle, builtin_type_functions::BuiltinTypeFunctions, generic_type::GenericType,
    generic_type_definition::GenericTypeDefinition, scope::Scope, type_arena::TypeArena,
    type_fun::TypeFun, type_function::TypeFunction,
    type_function_instance_type::TypeFunctionInstanceType,
  },
};

impl BuiltinTypeFunctions {
  /// # Safety
  /// 直译 cpp `BuiltinTypeFunctions::addToScope(TypeArena&, Scope&)`：
  /// - `arena` 句柄按 `Handle` 契约指向本次注册期间存活、被独占可写的
  ///   `TypeArena`（下方所有 `add_type` 追加类型均要求该独占窗口）；
  /// - `scope` 必须非空并指向存活、被独占可写的 `Scope`（写入其 `exported_type_bindings`）；
  /// - `&self` 持有的各 `TypeFunction`（`len_func` 等）必须比登记进 `arena` 的
  ///   `TypeFunctionInstanceType` 更长寿——实例类型内嵌了指向其函数体的裸指针句柄。
  pub unsafe fn add_to_scope(&self, arena: Handle<TypeArena>, scope: *mut Scope) {
    // Closure for unary type function
    let mk_unary = |tf: &TypeFunction| -> TypeFun {
      // arena 由 Handle 契约保证独占可写存活，add_type 仅追加一个 GenericType
      // 并返回其 arena 内 TypeId。
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
      // `tf`→const 身份裸指针存活（self 长寿于 arena），NonNull 由
      // .expect() 证成非空（`&TypeFunction` 恒非空）；arena 由 Handle 契约保证
      // 独占可写，登记实例类型。
      let result_type = arena.get_mut().add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
          NonNull::new(tf as *const TypeFunction as *mut TypeFunction).expect("&TypeFunction 借用转裸指针恒非空"),
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
      // arena 由 Handle 契约保证独占可写存活：依次追加首型形参 T 与次型形参 U。
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
      // `tf`→const 身份裸指针存活（self 长寿于 arena），NonNull
      // 由 .expect() 证成非空（`&TypeFunction` 恒非空），实例类型登记两个 arena TypeId；
      // arena 由 Handle 契约保证独占可写。
      let result_type = arena.get_mut().add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
          NonNull::new(tf as *const TypeFunction as *mut TypeFunction).expect("&TypeFunction 借用转裸指针恒非空"),
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
      // arena 由 Handle 契约保证独占可写：依次追加 T、U 形参（同前两个闭包）。
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
      // `tf` 为 &self.<fn> 转 const 身份裸指针，self 比本 arena
      // 用法长寿，NonNull 非空成立，登记 [T,U] 两个 TypeId；arena 由 Handle 契约
      // 保证独占可写。
      let result_type = arena.get_mut().add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
          NonNull::new(tf as *const TypeFunction as *mut TypeFunction).expect("&TypeFunction 借用转裸指针恒非空"),
          vec![t, u],
          vec![],
        ));
      TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
        vec![generic_t, generic_u],
        result_type,
        None,
      )
    };

    // Safety: `scope` 为函数级契约保证的非空、独占可写存活 Scope；这里只向其
    // exported_type_bindings 插入绑定，闭包内对 `arena` 的可写前提与其一致。
    unsafe {
      (*scope)
        .exported_type_bindings
        .insert(self.len_func.name.clone(), mk_unary(&self.len_func));
      (*scope)
        .exported_type_bindings
        .insert(self.unm_func.name.clone(), mk_unary(&self.unm_func));

      (*scope).exported_type_bindings.insert(
        self.add_func.name.clone(),
        mk_binary_with_default(&self.add_func),
      );
      (*scope).exported_type_bindings.insert(
        self.sub_func.name.clone(),
        mk_binary_with_default(&self.sub_func),
      );
      (*scope).exported_type_bindings.insert(
        self.mul_func.name.clone(),
        mk_binary_with_default(&self.mul_func),
      );
      (*scope).exported_type_bindings.insert(
        self.div_func.name.clone(),
        mk_binary_with_default(&self.div_func),
      );
      (*scope).exported_type_bindings.insert(
        self.idiv_func.name.clone(),
        mk_binary_with_default(&self.idiv_func),
      );
      (*scope).exported_type_bindings.insert(
        self.pow_func.name.clone(),
        mk_binary_with_default(&self.pow_func),
      );
      (*scope).exported_type_bindings.insert(
        self.mod_func.name.clone(),
        mk_binary_with_default(&self.mod_func),
      );
      (*scope).exported_type_bindings.insert(
        self.concat_func.name.clone(),
        mk_binary_with_default(&self.concat_func),
      );

      (*scope).exported_type_bindings.insert(
        self.lt_func.name.clone(),
        mk_binary_with_default(&self.lt_func),
      );
      (*scope).exported_type_bindings.insert(
        self.le_func.name.clone(),
        mk_binary_with_default(&self.le_func),
      );
      (*scope)
        .exported_type_bindings
        .insert(self.keyof_func.name.clone(), mk_unary(&self.keyof_func));
      (*scope).exported_type_bindings.insert(
        self.rawkeyof_func.name.clone(),
        mk_unary(&self.rawkeyof_func),
      );

      (*scope)
        .exported_type_bindings
        .insert(self.index_func.name.clone(), mk_binary(&self.index_func));
      (*scope)
        .exported_type_bindings
        .insert(self.rawget_func.name.clone(), mk_binary(&self.rawget_func));

      (*scope).exported_type_bindings.insert(
        self.setmetatable_func.name.clone(),
        mk_binary(&self.setmetatable_func),
      );
      (*scope).exported_type_bindings.insert(
        self.getmetatable_func.name.clone(),
        mk_unary(&self.getmetatable_func),
      );
    }
  }
}
