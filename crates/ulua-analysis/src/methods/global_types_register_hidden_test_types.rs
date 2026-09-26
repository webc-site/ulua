use alloc::string::String;

use crate::{
  enums::polarity::Polarity,
  functions::{arc_as_mut::arc_as_mut, freeze::freeze, unfreeze::unfreeze},
  records::{
    generic_type::GenericType, generic_type_definition::GenericTypeDefinition,
    global_types::GlobalTypes, metatable_type::MetatableType, negation_type::NegationType,
    type_fun::TypeFun,
  },
};

impl GlobalTypes {
  pub fn register_hidden_test_types(&mut self) {
    unfreeze(&mut self.global_types);

    let t = self
      .global_types
      .add_type(GenericType::generic_type_name_polarity(
        &String::from("T"),
        Polarity::Mixed,
      ));
    let generic_t = GenericTypeDefinition {
      ty: t,
      default_value: None,
    };

    let u = self
      .global_types
      .add_type(GenericType::generic_type_name_polarity(
        &String::from("U"),
        Polarity::Mixed,
      ));
    let generic_u = GenericTypeDefinition {
      ty: u,
      default_value: None,
    };

    let not_type = self.global_types.add_type(NegationType::new(t));
    let mt_type = self.global_types.add_type(MetatableType {
      table: t,
      metatable: u,
      synthetic_name: None,
    });

    // 解引用集中于 `builtin_types_ref` chokepoint：`self.builtin_types` 是
    // `GlobalTypes::new` 以 NonNull 契约接线（并由 `Frontend::wire_self_pointers`
    // 落位重布线）的会话级 `BuiltinTypes` 表，只读四个 Copy TypeId 字段。
    let builtins = self.builtin_types_ref();
    let (function_type, extern_type, error_type, table_type) = (
      builtins.function_type,
      builtins.extern_type,
      builtins.error_type,
      builtins.table_type,
    );

    let scope = arc_as_mut(&self.global_scope);
    // Safety: `scope` 经 `arc_as_mut` 取自 `self.global_scope`（本方法 `&mut self`
    // 独占的 Arc<Scope>，函数体内全程存活），裸指针仅作写穿句柄；单线程序列化下
    // 此刻无人持有对同一 Scope 的并存借用，多条 insert 均独占改写
    // `exported_type_bindings`，与 C++ `globalTypes` 注册路径同形。
    unsafe {
      (*scope).exported_type_bindings.insert(
        String::from("Not"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          vec![generic_t],
          not_type,
          None,
        ),
      );
      (*scope).exported_type_bindings.insert(
        String::from("Mt"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          vec![generic_t, generic_u],
          mt_type,
          None,
        ),
      );
      (*scope).exported_type_bindings.insert(
        String::from("fun"),
        TypeFun::type_fun_type_id(function_type),
      );
      (*scope)
        .exported_type_bindings
        .insert(String::from("cls"), TypeFun::type_fun_type_id(extern_type));
      (*scope)
        .exported_type_bindings
        .insert(String::from("err"), TypeFun::type_fun_type_id(error_type));
      (*scope)
        .exported_type_bindings
        .insert(String::from("tbl"), TypeFun::type_fun_type_id(table_type));
    }

    freeze(&mut self.global_types);
  }
}
