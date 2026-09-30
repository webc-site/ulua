//! C++ `void cloneTypesFromFragment(...)` (FragmentAutocomplete.cpp:671-795).
//!
//! Runs the `UsageFinder` traversal on the fragment and grabs all of the types
//! that are referenced in the fragment. We clone these and place them in the
//! appropriate spots in the scope so that they are available during
//! typechecking.
// C++ `Binding{type}` aggregate initialization with the remaining fields defaulted.
use alloc::string::String;

use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_stat_block::AstStatBlock, location::Location},
};
use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  functions::clone_incremental_clone::{
    clone_incremental as clone_incremental_type_pack, clone_incremental_binding,
    clone_incremental_type_fun, clone_incremental_type_id as clone_incremental_type,
  },
  records::{
    arena_handle::Handle, binding::Binding, blocked_type::BlockedType, builtin_types::BuiltinTypes,
    clone_state::CloneState, data_flow_graph::DataFlowGraph, scope::Scope,
    scope_registry::resolve_scope, symbol::Symbol, type_arena::TypeArena,
    usage_finder::UsageFinder,
  },
  type_aliases::{
    def_id_def::DefId, l_value::LValue, module_ptr_module::ModulePtr, type_id::TypeId,
  },
};
fn binding_from_type(type_id: TypeId) -> Binding {
  Binding {
    type_id,
    location: Location::default(),
    deprecated: false,
    deprecated_suggestion: String::new(),
    documentation_symbol: None,
  }
}

/// 对应 C++ `cloneTypesFromFragment`（FragmentAutocomplete.cpp:671-795）：把 fragment
/// 里用到的 stale 类型/绑定增量克隆进 fresh scope。
///
/// 引用化说明（原 `# Safety` 裸指针前提改由类型承担）：`stale` 全程只读查找，
/// `dest_scope` 独占字段写入，两者须指向互不重叠的 `Scope`；`program` 是本次遍历独占的
/// fragment AST 根；`dfg` 的 def arena 须比 `UsageFinder` 记录的 `DefId` 句柄长寿。
/// arena 句柄（`TypeId`/`TypePackId`）仍按本 crate 既有纪律由调用方保证存活。
pub fn clone_types_from_fragment(
  clone_state: &mut CloneState,
  stale: &Scope,
  stale_module: &ModulePtr,
  dest_arena: Handle<TypeArena>,
  dfg: &mut DataFlowGraph,
  _builtins: Handle<BuiltinTypes>,
  program: &mut AstStatBlock,
  dest_scope: &mut Scope,
) {
  LUAU_TIMETRACE_SCOPE!("Luau::cloneTypesFromFragment", "FragmentAutocomplete");

  // UsageFinder 仍以裸 dfg 句柄驱动（其字段是 cpp `DataFlowGraph*`）。
  let mut f = UsageFinder::new(dfg as *mut DataFlowGraph);
  ast_stat_block_visit(program, &mut f);

  // `dest_arena` 由 Handle 类型编码保证非空；绑定为本函数唯一的可变 arena 引用，
  // 此后新类型写入均经此引用，无并存别名。
  let dest = dest_arena.get_mut();

  // These are defs that have been mentioned. find the appropriate lvalue type and rvalue types and place them in the scope
  // First - any locals that have been mentioned in the fragment need to be placed in the bindings and lvalueTypes sections.
  for d in f.mentioned_defs.iter() {
    let d: DefId = *d;
    if let Some(r_value_refinement) = stale.lookup_r_value_refinement_type(d) {
      let cloned = clone_incremental_type(r_value_refinement, dest, clone_state, dest_scope);
      (*dest_scope.rvalue_refinements.get_or_insert(d)) = cloned;
    }

    if let Some(l_value) = stale.lookup_unrefined_type(d) {
      let cloned = clone_incremental_type(l_value, dest, clone_state, dest_scope);
      (*dest_scope.lvalue_types.get_or_insert(d)) = cloned;
    }
  }

  for (d, loc) in f.local_bindings_referenced.iter() {
    let d: DefId = *d;
    let loc = *loc;
    // Safety: `loc` 是 `UsageFinder` 记录的 fragment `AstLocal*`，由 ast allocator 保活、
    // 遍历期地址稳定；此处仅瞬态只读取 `.name`，不构造长期引用、与 dest 写入无重叠。
    let name = unsafe { (*loc).name.as_str_or_empty().to_string() };
    if let Some((sym, binding)) = stale.linear_search_for_binding_pair(&name, true) {
      let cloned_ty = clone_incremental_type(binding.type_id, dest, clone_state, dest_scope);
      let cloned_binding = clone_incremental_binding(&binding, dest, clone_state, dest_scope);
      (*dest_scope.lvalue_types.get_or_insert(d)) = cloned_ty;
      dest_scope.bindings.insert(sym, cloned_binding);
    }
  }

  for (d, syms) in f.symbols_to_refine.iter() {
    let d: DefId = *d;
    let syms = LValue::Symbol(syms.clone());
    let mut current: Option<&Scope> = Some(stale);
    while let Some(scope) = current {
      if let Some(res) = scope.refinements.get(&syms) {
        let cloned = clone_incremental_type(*res, dest, clone_state, dest_scope);
        (*dest_scope.rvalue_refinements.get_or_insert(d)) = cloned;
        // If we've found a refinement, just break, otherwise we might end up doing the wrong thing.
        // We want the most "narrow" refinement here.
        break;
      }
      current = scope.parent.and_then(resolve_scope);
    }
  }

  // Second - any referenced type alias bindings need to be placed in scope so type annotation can be resolved.
  // If the actual type alias appears in the fragment on the lhs as a definition (in declaredAliases), it will be processed during typechecking anyway
  for x in f.referenced_bindings.iter() {
    if f.declared_aliases.contains(x) {
      continue;
    }
    if let Some(tf) = stale.lookup_type(x) {
      let cloned = clone_incremental_type_fun(&tf, dest, clone_state, dest_scope);
      dest_scope.private_type_bindings.insert(x.clone(), cloned);
    }
  }

  // Third - any referenced imported type bindings need to be imported in
  for (md, name) in f.referenced_imported_bindings.iter() {
    if let Some(tf) = stale.lookup_imported_type(md, name) {
      let cloned = clone_incremental_type_fun(&tf, dest, clone_state, dest_scope);
      dest_scope
        .imported_type_bindings
        .entry(md.clone())
        .or_default()
        .insert(name.clone(), cloned);
    }
  }

  let module_scope = stale_module.get_module_scope();

  // Fourth - prepopulate the global function types
  for name in f.global_functions_referenced.iter() {
    let name = *name;
    if let Some(ty) = module_scope.lookup_symbol(Symbol::from_global(name)) {
      let cloned = clone_incremental_type(ty, dest, clone_state, dest_scope);
      dest_scope
        .bindings
        .insert(Symbol::from_global(name), binding_from_type(cloned));
    } else {
      let bt = dest.add_type(BlockedType::default());
      dest_scope
        .bindings
        .insert(Symbol::from_global(name), binding_from_type(bt));
    }
  }

  // Fifth - prepopulate the globals here
  for (name, def) in f.global_defs_to_pre_populate.iter() {
    let name = *name;
    let def: DefId = *def;
    if let Some(ty) = module_scope.lookup_symbol(Symbol::from_global(name)) {
      let cloned = clone_incremental_type(ty, dest, clone_state, dest_scope);
      (*dest_scope.lvalue_types.get_or_insert(def)) = cloned;
    } else if let Some(ty) = dest_scope.lookup_symbol(Symbol::from_global(name)) {
      // This branch is a little strange - we are looking up a symbol in the destScope.
      // This scope has no parent pointer, and only cloned types are written to it, so this is a
      // safe operation to do without cloning.
      (*dest_scope.lvalue_types.get_or_insert(def)) = ty;
    }
  }

  // Finally, clone the returnType on the staleScope. This helps avoid potential leaks of free types.
  if !stale.return_type.is_null() {
    let cloned = clone_incremental_type_pack(stale.return_type, dest, clone_state, dest_scope);
    dest_scope.return_type = cloned;
  }
}
