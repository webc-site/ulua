//! `void synthesizeExportReturn(NotNull<BuiltinTypes> builtinTypes, NotNull<Module> module)`.
//! Reference: `Module.cpp:361-467`.

use alloc::string::String;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_local::AstLocal, ast_name::AstName,
    ast_stat_assign::AstStatAssign, ast_stat_class::AstStatClass,
    ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
  },
  rtti::ast_node_try_as,
};
use ulua_common::{fflag, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::table_state::TableState,
  functions::{follow_type, shared_mut::shared_mut},
  records::{
    arena_handle::{Handle, alias_ref},
    builtin_types::BuiltinTypes,
    module::Module,
    property_type::Property,
    symbol::Symbol,
    table_type::TableType,
    type_pack::TypePack,
  },
  type_aliases::{props_type::Props, type_id::TypeId},
};
fn key_of(name: AstName) -> String {
  name.as_str_or_empty().to_string()
}

/// C++ `Property(TypeId readTy)` — the single-argument constructor sets
/// `readTy == writeTy` (a read-write property). Reference: `Type.h` Property ctor.
fn prop_from_ty(ty: TypeId) -> Property {
  Property {
    read_ty: Some(ty),
    write_ty: Some(ty),
    ..Property::default()
  }
}

/// 对应 cpp `synthesizeExportReturn(NotNull<BuiltinTypes>, NotNull<Module>)`
/// （`Module.cpp:381`）。参数前提：
/// - `builtin_types`：非空且指向存活的 `BuiltinTypes`（cpp `NotNull` 语义），
///   生命周期覆盖整个调用；本函数只读其 `error_type` 字段。
/// - `module`：借用即「存活且本调用独占」的类型系统证明（cpp `NotNull<Module>`
///   的 Rust 对应）；另要求其 `root` 非空、指向随 Module 存活的 AST arena 根，
///   且 `scopes` 非空（`get_module_scope` 断言 `scopes.front()`），scope 树内
///   `children` 裸指针均指向仍由 `module.scopes` 中 `Arc<Scope>` 保活的节点。
pub fn synthesize_export_return(builtin_types: Handle<BuiltinTypes>, module_ref: &mut Module) {
  // cpp `LUAU_ASSERT(module->root)` 后直取 `module->root->body`：缺席即契约违例，
  // `root` 句柄化后于此单点收口为确定性 panic，非空由 `Handle` 类型承载。
  let root = module_ref
    .root
    .expect("synthesizeExportReturn: 根块应在场（cpp LUAU_ASSERT(module->root)）");

  let module_scope = module_ref.get_module_scope();
  let mut props: Props = Props::new();

  let lookup_exported_binding_type = |local: *mut AstLocal| -> TypeId {
    // `module_scope_ptr` 指向的模块 Scope 由上方局部强引用
    // `module_scope`（自 `module.scopes.front()` clone 的 Arc）保活至函数结束；
    // `find_narrowest_scope_containing` 的 `&mut` 借用半径止于本次调用，其内部
    // 只沿 `children`（NotNull 裸指针，同样由 module 的 Arc 树保活）下钻。
    let scope =
      shared_mut(&module_scope).find_narrowest_scope_containing(alias_ref(local).location);

    if let Some((binding, _scope)) = alias_ref(scope).lookup_ex_symbol(Symbol::from_local(local)) {
      return follow_type::follow(binding.type_id);
    }

    // Safety: `builtin_types` 依函数契约指向存活 BuiltinTypes，此处仅读
    // `error_type`（对应 cpp `builtinTypes->errorType`）。
    builtin_types.get().error_type
  };

  // cpp `module->astTypes.find(expr)`：只读遍历，故取字段的共享借用而非再次
  // 物化写句柄（借用止于本函数最后一次查表，之后的 arena 写入不与之交叠）。
  let ast_types = &module_ref.ast_types;
  let lookup_expr_type = |expr: *mut AstExpr| -> TypeId {
    if let Some(ty) = ast_types.find(&(expr as *const AstExpr)) {
      return follow_type::follow(*ty);
    }
    builtin_types.get().error_type
  };

  let mut exported_locals: DenseHashSet<*mut AstLocal> = DenseHashSet::default();

  let body = &root.get().body;
  for statement in body.iter() {
    // `statement` 出自 `module.root->body`（parser 分配、lint 全程存活的 arena
    // 语句节点，Node 句柄只读视图），下转走生命周期正确的 `ast_node_try_as`，
    // 借用半径即本次迭代的语句引用（cpp `statement->as<AstStatLocal>()` 直译）。
    if let Some(local_stat) = ast_node_try_as::<AstStatLocal>(statement) {
      if !local_stat.is_exported {
        continue;
      }

      // vars/values 一一对应；长度不等时全部按 binding 处理
      for (i, &local) in local_stat.vars.as_slice().iter().enumerate() {
        exported_locals.insert(local);

        let (local_name, local_location) = (alias_ref(local).name, alias_ref(local).location);
        let key = key_of(local_name);

        if local_stat.vars.size != local_stat.values.size || i >= local_stat.values.size {
          props.insert(
            key.clone(),
            prop_from_ty(lookup_exported_binding_type(local)),
          );
        } else {
          let value = local_stat.values.as_slice()[i];
          props.insert(key.clone(), Property::readonly(lookup_expr_type(value)));
        }

        props
          .get_mut(&key)
          .expect("上方刚以同键 insert，get_mut 必命中")
          .location = Some(local_location);
      }
    } else if let Some(local_function) = ast_node_try_as::<AstStatLocalFunction>(statement) {
      // `local_function.name` 已句柄化为 Node<AstLocal>（cpp `AstStatLocalFunction::name`
      // 的 NonNull 直译，导出名与 location 都在 AstLocal 上），`.get()` 即安全只读视图。
      let name_ref = local_function.name.get();
      if !name_ref.is_exported {
        continue;
      }

      let key = key_of(name_ref.name);
      props.insert(
        key.clone(),
        Property::readonly(lookup_exported_binding_type(local_function.name.as_ptr())),
      );
      props
        .get_mut(&key)
        .expect("上方刚以同键 insert，get_mut 必命中")
        .location = Some(name_ref.location);
    } else if let Some(assign) = ast_node_try_as::<AstStatAssign>(statement) {
      // vars/values 一一对应；长度不等时全部按 binding 处理
      for (i, local) in assign.vars.iter_nodes().enumerate() {
        // `local` 为存活 `assign.vars` 槽位的只读共享视图（iter_nodes 契约），
        // 判型下转走生命周期正确的 `ast_node_try_as`。
        let Some(expr_local) = ast_node_try_as::<AstExprLocal>(local) else {
          continue;
        };
        // local 槽已句柄化恒非空；exported_locals 键值为既有裸指针形态，经 as_ptr 桥接。
        let local = expr_local.local.as_ptr();
        if !exported_locals.contains(&local) {
          continue;
        }

        let (local_name, local_location) = (alias_ref(local).name, alias_ref(local).location);
        let key = key_of(local_name);

        if assign.vars.size != assign.values.size || i >= assign.values.size {
          props.insert(
            key.clone(),
            prop_from_ty(lookup_exported_binding_type(local)),
          );
        } else {
          let value = assign.values.as_slice()[i];
          props.insert(key.clone(), Property::readonly(lookup_expr_type(value)));
        }

        props
          .get_mut(&key)
          .expect("上方刚以同键 insert，get_mut 必命中")
          .location = Some(local_location);
      }
    } else if let Some(func_stat) = ast_node_try_as::<AstStatFunction>(statement) {
      // `func_stat.name` 是句柄化的 NotNull `AstExpr*` 槽位，`try_as` 经
      // `Node::get` 借出存活视图后判型下转（赋值型函数声明必为 AstExprLocal）。
      if let Some(expr_local) = func_stat.name.try_as::<AstExprLocal>()
        && exported_locals.contains(&expr_local.local.as_ptr())
      {
        let local = expr_local.local.as_ptr();
        let (local_name, local_location) = (alias_ref(local).name, alias_ref(local).location);
        let key = key_of(local_name);
        props.insert(
          key.clone(),
          Property::readonly(lookup_expr_type(func_stat.func.cast::<AstExpr>().as_ptr())),
        );
        props
          .get_mut(&key)
          .expect("上方刚以同键 insert，get_mut 必命中")
          .location = Some(local_location);
      }
    } else if fflag::DebugLuauUserDefinedClasses.get()
      && let Some(class_stat) = ast_node_try_as::<AstStatClass>(statement)
    {
      if !class_stat.exported {
        continue;
      }

      let name_ref = alias_ref(class_stat.name);
      let key = key_of(name_ref.name);
      // 对齐 cpp Module.cpp:496-505：`export class` 按**名字**在模块作用域直查
      // 类型绑定 `moduleScope->lookup(Symbol{name})`，未命中回退 errorType。
      // 这里不能复用 lookup_exported_binding_type——后者是 cpp 里给
      // `export local` 用的按 AstLocal 定位最窄作用域的 lambda，而类名是类型
      // 绑定（moduleScope->typeBindings/bindings 按 Symbol 名命中），语义不同。
      // Safety: `module_scope_ptr` 指向的模块 Scope 由局部强引用 `module_scope`
      // 保活（同文件开头两闭包的证成），`lookup_symbol` 为 `&self` 只读；
      // `builtin_types` 回退读同样只取 `error_type`。
      let ty = module_scope
        .lookup_symbol(Symbol::from_global(name_ref.name))
        .map(follow_type::follow)
        .unwrap_or(builtin_types.get().error_type);
      props.insert(key.clone(), Property::readonly(ty));
      props
        .get_mut(&key)
        .expect("上方刚以同键 insert，get_mut 必命中")
        .location = Some(name_ref.location);
    }
  }

  // 对齐 C++ `if (props.empty()) return;`（Module.cpp:500-502）：
  // 无 export 语句的模块必须保留类型检查器推断出的 return_type，
  // 否则会被空表覆盖，required 侧全部键变 UnknownProperty。
  if props.is_empty() {
    return;
  }

  let level = module_scope.level;
  let mut exports_tbl = TableType::table_type_props_optional_table_indexer_type_level_table_state(
    &props,
    None,
    level,
    TableState::Sealed,
  );
  // cpp Module.cpp:509 `tbl.definitionModuleName = module->name`（原端口漏赋值）
  exports_tbl.definition_module_name = module_ref.name.clone();
  let exports = module_ref.internal_types.add_type(exports_tbl);
  let exports_pack = module_ref
    .internal_types
    .add_type_pack_t(TypePack::single(exports));
  // cpp `moduleScope->returnType = exportsPack`：写句柄按需物化、止于本语句。
  shared_mut(&module_scope).return_type = exports_pack;
}
