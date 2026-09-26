//! Source: `Analysis/include/Luau/Scope.h` (Scope.h:33-118, hand-ported; fields only,
//! methods are separate schedule items)

use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    binding::Binding, scope_registry::ScopeId, symbol::Symbol, type_fun::TypeFun,
    type_level::TypeLevel,
  },
  type_aliases::{
    collections::HashMap, def_id_def::DefId, module_name_type::ModuleName, name_type::Name,
    refinement_map::RefinementMap, type_id::TypeId, type_pack_id::TypePackId,
  },
};

#[derive(Debug)]
pub struct Scope {
  pub parent: Option<ScopeId>, // null for the root

  pub children: Vec<ScopeId>, // 原 NotNull<Scope> 子指针的句柄化（见 scope_registry）
  pub bindings: HashMap<Symbol, Binding>,
  pub return_type: TypePackId,
  pub vararg_pack: Option<TypePackId>,

  pub level: TypeLevel,

  /// the spanning location associated with this scope
  pub location: Location,

  pub exported_type_bindings: HashMap<Name, TypeFun>,
  pub private_type_bindings: HashMap<Name, TypeFun>,
  pub type_alias_locations: HashMap<Name, Location>,
  pub type_alias_name_locations: HashMap<Name, Location>,
  /// Mapping from the name in the require statement to the internal module_name.
  pub imported_modules: HashMap<Name, ModuleName>,
  pub imported_type_bindings: HashMap<Name, HashMap<Name, TypeFun>>,
  pub builtin_type_names: DenseHashSet<Name>,

  pub private_type_pack_bindings: HashMap<Name, TypePackId>,

  pub refinements: RefinementMap,
  pub lvalue_types: DenseHashMap<DefId, TypeId>,
  pub rvalue_refinements: DenseHashMap<DefId, TypeId>,

  pub globals_to_warn: DenseHashSet<String>,

  pub type_alias_type_parameters: HashMap<Name, TypeId>,
  pub type_alias_type_pack_parameters: HashMap<Name, TypePackId>,

  pub interior_free_types: Option<Vec<TypeId>>,
  pub interior_free_type_packs: Option<Vec<TypePackId>>,

  pub invalid_type_aliases: DenseHashMap<String, Location>,
}

/// # Safety
///
/// 树导航（`parent: Option<ScopeId>`/`children: Vec<ScopeId>`）已句柄化，本
/// 组 impl 不再为其存在。保留的原因是 `TypeId`/`TypePackId` 字段（`*const Type`/
/// `*const TypePackVar` 类型 arena 裸指针，句柄化属后续任务）令自动
/// Send/Sync 失效，而 `Arc<Scope>` 遍布全 crate 的 scope 创建点（去 Send 即
/// 触发 `arc_with_non_send_sync` 告警群）。
// Safety: Scope 无自定义 Drop、从不 dealloc 这些 arena 借用；转移/共享 Scope
// 只是转移/共享这些只读句柄，单线程分析驱动且借用方 arena 恒比 Scope 长寿，
// 不破坏任何不变量，Send/Sync 成立。
unsafe impl Send for Scope {}
/// # Safety
///
/// 同上：共享 `&Scope` 经 arena 指针只可达 arena 内只读内容，节点追加/freeze
/// 需 `&mut`（编译期排斥并发写），并发持有 `&Scope` 为纯读，无数据竞争。
// Safety: 同上，Sync 成立。
unsafe impl Sync for Scope {}
