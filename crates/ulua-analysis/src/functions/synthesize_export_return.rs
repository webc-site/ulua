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
  rtti::{AstNodePtr, ast_node_try_as_ptr},
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::table_state::TableState,
  functions::{arc_as_mut::arc_as_mut, follow_type},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, module::Module, property_type::Property,
    symbol::Symbol, table_type::TableType, type_pack::TypePack,
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

/// # Safety
/// 对应 cpp `synthesizeExportReturn(NotNull<BuiltinTypes>, NotNull<Module>)`
/// （`Module.cpp:381`）。逐参数契约：
/// - `builtin_types`：非空且指向存活的 `BuiltinTypes`（cpp `NotNull` 语义），
///   生命周期覆盖整个调用；本函数只读其 `error_type` 字段。
/// - `module`：非空且指向存活的 `Module`，调用期间由本函数独占驱动（以 `&mut`
///   视角贯穿函数体）；要求其 `root` 非空、指向随 Module 存活的 AST arena 根，
///   且 `scopes` 非空（`get_module_scope` 断言 `scopes.front()`），scope 树内
///   `children` 裸指针均指向仍由 `module.scopes` 中 `Arc<Scope>` 保活的节点。
pub unsafe fn synthesize_export_return(builtin_types: Handle<BuiltinTypes>, module: *mut Module) {
  // Safety: `module` 的非空与存活由上方契约保证，且调用期间无其他持有的
  // `&mut Module`，此独占借用可安全覆盖整个函数体。
  let module_ref = unsafe { &mut *module };
  LUAU_ASSERT!(!module_ref.root.is_null());

  let module_scope = module_ref.get_module_scope();
  let module_scope_ptr = arc_as_mut(&module_scope);
  let mut props: Props = Props::new();

  let lookup_exported_binding_type = |local: *mut AstLocal| -> TypeId {
    // Safety: `module_scope_ptr` 指向的模块 Scope 由上方局部强引用
    // `module_scope`（自 `module.scopes.front()` clone 的 Arc）保活至函数结束；
    // `find_narrowest_scope_containing` 的 `&mut` 借用半径止于本次调用，其内部
    // 只沿 `children`（NotNull 裸指针，同样由 module 的 Arc 树保活）下钻。
    // `local` 是下方循环自 AST `vars` 数组取出的存活 AstLocal。
    let scope = unsafe { (*module_scope_ptr).find_narrowest_scope_containing((*local).location) };

    // Safety: `scope` 由上式在同一棵存活的模块 scope 树中选出（NotNull 语义，
    // 不会悬垂）；`lookup_ex_symbol` 为 `&self` 只读，返回借用只在本语句内被
    // `binding.type_id` 消费。
    if let Some((binding, _scope)) = unsafe { &*scope }.lookup_ex_symbol(Symbol::from_local(local))
    {
      return follow_type::follow(binding.type_id);
    }

    // Safety: `builtin_types` 依函数契约指向存活 BuiltinTypes，此处仅读
    // `error_type`（对应 cpp `builtinTypes->errorType`）。
    builtin_types.get().error_type
  };

  let lookup_expr_type = |expr: *mut AstExpr| -> TypeId {
    // Safety: `module` 与 `module_ref` 同源、指向调用内存活的 Module，
    // `ast_types` 容器随其存活；这是语句内的只读 find，返回的 `&TypeId` 立刻被
    // `follow_type::follow(*ty)` 消费，不与后续对 `module_ref` 的可变使用并存。
    if let Some(ty) = unsafe { (*module).ast_types.find(&(expr as *const AstExpr)) } {
      return follow_type::follow(*ty);
    }

    // Safety: 同上——`builtin_types` 由函数契约保证存活且此处只读 `error_type`。
    builtin_types.get().error_type
  };

  let mut exported_locals: DenseHashSet<*mut AstLocal> = DenseHashSet::default();

  // Safety: 入口 `LUAU_ASSERT!(!module_ref.root.is_null())` 已确认 root 非空；
  // 它指向随 Module 存活的 AST arena 根节点。此处共享借用只读，且整个遍历循环
  // 中不出现对 Module/AST 的可变访问，借用覆盖循环是安全的。
  let body = unsafe { &(*module_ref.root).body };
  for &statement in body.as_slice() {
    let node = statement.as_ast_node();

    // Safety: `statement` 取自上面存活的 `module.root->body`，指向 AST arena 内
    // 存活的 repr(C) 节点；`ast_node_try_as_ptr` 先判空再按 RTTI 判型，命中即
    // 可安全下转为 `AstStatLocal`（cpp `statement->as<AstStatLocal>()` 直译）。
    if let Some(local_stat) = unsafe { ast_node_try_as_ptr::<AstStatLocal>(node) } {
      if !local_stat.is_exported {
        continue;
      }

      // vars/values 一一对应；长度不等时全部按 binding 处理
      for (i, &local) in local_stat.vars.as_slice().iter().enumerate() {
        exported_locals.insert(local);

        // Safety: `local` 是 parser 写入 `local_stat.vars` 的 NotNull 元素，
        // 指向 AST arena 内存活的 AstLocal；本处只 Copy 其 name/location 字段。
        let (local_name, local_location) = unsafe { ((*local).name, (*local).location) };
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
    } else if let Some(local_function) =
      // Safety: 同 AstStatLocal 分支——`node` 是 `module.root->body` 遍历中的
      // 存活 repr(C) 节点，判型命中后下转为 `AstStatLocalFunction`。
      unsafe { ast_node_try_as_ptr::<AstStatLocalFunction>(node) }
    {
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
    } else if let Some(assign) =
      // Safety: 同上——`node` 为 body 遍历中的存活节点，判型命中后下转为
      // `AstStatAssign`。
      unsafe { ast_node_try_as_ptr::<AstStatAssign>(node) }
    {
      // vars/values 一一对应；长度不等时全部按 binding 处理
      for (i, &local) in assign.vars.as_slice().iter().enumerate() {
        // Safety: `local` 取自存活 `assign.vars`（AST arena 内 repr(C) 表达式
        // 节点），判空与 RTTI 判型一次完成，命中后可安全按 `AstExprLocal` 读。
        let Some(expr_local) =
          (unsafe { ast_node_try_as_ptr::<AstExprLocal>(local.as_ast_node()) })
        else {
          continue;
        };
        // local 槽已句柄化恒非空；exported_locals 键值为既有裸指针形态，经 as_ptr 桥接。
        let local = expr_local.local.as_ptr();
        if !exported_locals.contains(&local) {
          continue;
        }

        // Safety: `expr_local.local` 是 parser 写入 AstExprLocal 的 NotNull
        // `*mut AstLocal`（且已确认在 exported_locals 集合中，源自存活 vars），
        // 本处只 Copy 其 name/location。
        let (local_name, local_location) = unsafe { ((*local).name, (*local).location) };
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
    } else if let Some(func_stat) =
      // Safety: 同上——`node` 为 body 遍历中的存活节点，判型命中后下转为
      // `AstStatFunction`。
      unsafe { ast_node_try_as_ptr::<AstStatFunction>(node) }
    {
      if let Some(expr_local) =
        // Safety: `func_stat.name` 是 cpp `AstStatFunction::name`（NotNull
        // `AstExpr*`，赋值型函数声明必为 AstExprLocal），指向 AST arena 内存活
        // 表达式节点，判型下转一次完成。
        unsafe { ast_node_try_as_ptr::<AstExprLocal>(func_stat.name.as_ast_node()) }
        && exported_locals.contains(&expr_local.local.as_ptr())
      {
        let local = expr_local.local.as_ptr();
        // Safety: `expr_local.local` 为 parser 写入的 NotNull `*mut AstLocal`
        // 且已确认在 exported_locals（源自存活 vars），此处只 Copy name/location。
        let (local_name, local_location) = unsafe { ((*local).name, (*local).location) };
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
      && let Some(class_stat) =
        // Safety: 同上——`node` 为 body 遍历中的存活节点，判型命中后下转为
        // `AstStatClass`。
        unsafe { ast_node_try_as_ptr::<AstStatClass>(node) }
    {
      if !class_stat.exported {
        continue;
      }

      // Safety: `class_stat.name` 是 cpp `AstStatClass::name` 的 NotNull 直译
      // （`*mut AstLocal`，类名与 location 都在其上），parser 保证指向 AST arena
      // 内存活节点。
      let name_ref = unsafe { &*class_stat.name };
      let key = key_of(name_ref.name);
      // 对齐 cpp Module.cpp:496-505：`export class` 按**名字**在模块作用域直查
      // 类型绑定 `moduleScope->lookup(Symbol{name})`，未命中回退 errorType。
      // 这里不能复用 lookup_exported_binding_type——后者是 cpp 里给
      // `export local` 用的按 AstLocal 定位最窄作用域的 lambda，而类名是类型
      // 绑定（moduleScope->typeBindings/bindings 按 Symbol 名命中），语义不同。
      // Safety: `module_scope_ptr` 指向的模块 Scope 由局部强引用 `module_scope`
      // 保活（同文件开头两闭包的证成），`lookup_symbol` 为 `&self` 只读；
      // `builtin_types` 回退读同样只取 `error_type`。
      let ty = unsafe { &*module_scope_ptr }
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

  // Safety: `module_scope_ptr` 指向由局部强引用 `module_scope` 保活的模块 Scope；
  // 此处只 Copy 其 `level` 字段，借用半径止于该表达式。
  let level = unsafe { (*module_scope_ptr).level };
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
  // Safety: 写穿 `Arc<Scope>` 内容是 arc_as_mut 记录的 cpp 直译惯用法（单线程
  // 独占驱动）：目标 Scope 仍由 `module_scope` 强引用保活，且此前经
  // `module_scope_ptr` 的所有只读借用均止于各自语句，此刻无并存引用冲突；
  // 等价 cpp `moduleScope->returnType = exportsPack`（exports 由 `module_ref`
  // 自身的 arena 创建，不触碰被借用遍历的 AST/body）。
  unsafe { (*module_scope_ptr).return_type = exports_pack };
}
