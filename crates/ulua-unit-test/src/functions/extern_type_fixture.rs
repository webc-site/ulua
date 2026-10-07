//! 三处 extern type 注册器（`register_extern_type_fixture_types` /
//! `register_ac_extern_type_fixture_types` / `register_refinement_extern_type_fixture_types`）
//! 共用的构造样板：cpp 侧各自复制的 `ExternType{name, {}, parent, nullopt, {}, nullptr, "Test", {}}`
//! 与全局绑定 `Binding` 字面量，收口到一处。

use alloc::string::String;

use ulua_analysis::{
  functions::{get_mutable_type, make_function_builtin_definitions::make_function},
  records::{
    binding::Binding, extern_type::ExternType, global_types::GlobalTypes, property_type::Property,
    table_indexer::TableIndexer, table_type::TableType, type_arena::TypeArena, type_fun::TypeFun,
  },
  type_aliases::{module_name_type::ModuleName, type_id::TypeId},
};
use ulua_ast::records::location::Location;

use crate::functions::raw_handle::raw_handle;

/// 四参完整形态：metatable / indexer 显式给定（对应 cpp 少数带元表/索引器的注册点）。
pub fn extern_type_full(
  name: &str,
  parent: Option<TypeId>,
  metatable: Option<TypeId>,
  indexer: Option<TableIndexer>,
) -> ExternType {
  ExternType {
    name: String::from(name),
    props: Default::default(),
    parent,
    metatable,
    tags: Default::default(),
    user_data: None,
    definition_module_name: ModuleName::from("Test"),
    definition_location: None,
    indexer,
    relation: None,
  }
}

/// 两参默认形态：无 metatable / indexer（cpp `ExternType{name, {}, parent, ...}` 字面量）。
pub fn extern_type(name: &str, parent: Option<TypeId>) -> ExternType {
  extern_type_full(name, parent, None, None)
}

/// cpp 测试侧统一的全局绑定样板（`Binding{ty, {}, false, "", "@test"}`）。
pub fn binding(ty: TypeId) -> Binding {
  Binding {
    type_id: ty,
    location: Location::default(),
    deprecated: false,
    deprecated_suggestion: String::new(),
    documentation_symbol: Some(String::from("@test")),
  }
}

/// 向 extern type 追加一条属性：`get_mutable::<ExternType>` + `props.insert` 样板收口。
pub fn extern_prop(ty: TypeId, name: &str, prop: Property) {
  get_mutable_type::get_mutable::<ExternType>(ty)
    .expect("expected extern type")
    .props
    .insert(String::from(name), prop);
}

/// 向 TableType（元表宿主）追加一条属性：同上，宿主变体为 `TableType`。
pub fn table_prop(ty: TypeId, name: &str, prop: Property) {
  get_mutable_type::get_mutable::<TableType>(ty)
    .expect("expected table type")
    .props
    .insert(String::from(name), prop);
}

/// 实例方法样板：`self_type` 绑定宿主 extern type，`checked` 恒为 false。
pub fn method(
  arena: &mut TypeArena,
  self_type: TypeId,
  params: &[TypeId],
  rets: &[TypeId],
) -> TypeId {
  make_function(
    arena,
    Some(self_type),
    params.to_vec(),
    rets.to_vec(),
    false,
  )
}

/// 自由函数样板：无 self_type（同上，`checked` 恒为 false）。
pub fn free_function(arena: &mut TypeArena, params: &[TypeId], rets: &[TypeId]) -> TypeId {
  make_function(arena, None, params.to_vec(), rets.to_vec(), false)
}

/// 写入全局 scope 的导出类型绑定（cpp `exportedTypeBindings[name] = ty`，共用
/// 同一份 `raw_handle` 写入路径，替代各注册器复制的 unsafe 块）。
pub fn export_type_binding(globals: &mut GlobalTypes, name: &str, ty: TypeId) {
  let module_scope = globals.global_scope();
  let module_scope_ptr = raw_handle(&module_scope);

  // Safety: module_scope_ptr 为 raw_handle(&module_scope)——Arc<Scope> 堆块地址，
  // 本帧强引用存活、底层块由 globals.globalScope 持续保有（非空）；insert 即完即还。
  unsafe {
    (*module_scope_ptr)
      .exported_type_bindings
      .insert(String::from(name), TypeFun::type_fun_type_id(ty));
  }
}
