//! Source: `tests/ToDot.test.cpp`

use ulua_analysis::{
  functions::{freeze::freeze, persist_type::persist, unfreeze::unfreeze},
  records::{
    extern_type::ExternType, property_type::Property, table_type::TableType, type_fun::TypeFun,
  },
  type_aliases::{module_name_type::ModuleName, type_id::TypeId},
};

use crate::{functions::raw_handle::raw_handle, records::fixture::Fixture};

/// BaseClass/ChildClass 同构构造（cpp ToDot.test.cpp）：仅 name/parent/metatable/单 prop 四点差异。
fn class(
  name: &str,
  parent: Option<TypeId>,
  metatable: Option<TypeId>,
  prop_name: &str,
  prop_type: TypeId,
) -> ExternType {
  ExternType {
    name: String::from(name),
    props: [(String::from(prop_name), Property::rw_type_id(prop_type))]
      .into_iter()
      .collect(),
    parent,
    metatable,
    tags: Default::default(),
    user_data: None,
    definition_module_name: ModuleName::from("Test"),
    definition_location: None,
    indexer: None,
    relation: None,
  }
}

#[derive(Debug)]
pub struct ToDotClassFixture {
  pub base: Fixture,
}

impl Default for ToDotClassFixture {
  fn default() -> Self {
    let mut fixture = Self {
      base: Fixture::default(),
    };
    let frontend = fixture.base.get_frontend();
    let builtins = frontend.builtin_types_ref();
    let number_type = builtins.number_type;
    let string_type = builtins.string_type;

    let globals = &mut frontend.globals;
    unfreeze(globals.global_types_mut());

    let base_class_meta_type = globals.global_types_mut().add_type(TableType::new());

    let base_class_instance_type = globals.global_types_mut().add_type(class(
      "BaseClass",
      None,
      Some(base_class_meta_type),
      "BaseField",
      number_type,
    ));

    let child_class_instance_type = globals.global_types_mut().add_type(class(
      "ChildClass",
      Some(base_class_instance_type),
      None,
      "ChildField",
      string_type,
    ));

    // Safety: cpp `globalScope->exportedTypeBindings[...] = ...`——`global_scope()` 是
    // `GlobalTypes::globalScope`（`ScopePtr`）的克隆，写入落在同一个 `Scope` 对象上；
    // 本 fixture 为单用例独占，此刻除 `global_scope`/`globals` 的共享读之外无其他借用活跃。
    // 句柄只用作写入点，写入后读走 `Arc` 共享引用，不伪造长期 `&mut`。
    let global_scope = globals.global_scope();
    let global_scope_raw = raw_handle(&global_scope);
    let exported_bindings = [
      ("BaseClass", base_class_instance_type),
      ("ChildClass", child_class_instance_type),
    ];
    // Safety: global_scope_raw 是 Arc<Scope> 克隆指向的堆块地址（底层块由
    // globals.globalScope 强引用保活，非空存活）；cpp globalScope->exportedTypeBindings
    // 同款写入，insert 借用随块尾结束，本帧唯一其他借用仅 globalTypes 读写。
    unsafe {
      let exported = &mut (*global_scope_raw).exported_type_bindings;
      for (name, ty) in exported_bindings {
        exported.insert(String::from(name), TypeFun::type_fun_type_id(ty));
      }
    }

    // 读走 `Arc<Scope>` 共享引用（与被测侧遍历 exportedTypeBindings 同款读法）。
    for tf in global_scope.exported_type_bindings.values() {
      persist(tf.r#type());
    }

    freeze(globals.global_types_mut());
    fixture
  }
}
