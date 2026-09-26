use alloc::{string::String, vec::Vec};
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::solver_mode::SolverMode,
  functions::arc_as_mut::arc_as_mut,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes,
    clone_public_interface::ClonePublicInterface, clone_state::CloneState,
    internal_error::InternalError, internal_error_reporter::InternalErrorReporter, module::Module,
    txn_log::TxnLog, type_error::TypeError,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl Module {
  /// # Safety
  /// - `builtin_types` 为 Handle（NonNull 编码非空），指向分析期全局唯一且
  ///   在本次调用内存活不变的 `BuiltinTypes`（对应 C++ `NotNull<BuiltinTypes>`）。
  /// - `_ice` 指向存活的 `InternalErrorReporter`，覆盖本次调用（C++ 引用形参；
  ///   当前移植体暂未触碰）。
  /// - `self` 必须是完成内部检查、`get_module_scope()` 可返回有效 ScopePtr 的
  ///   Module（本函数把 `self as *mut Module` 交给 `ClonePublicInterface`，
  ///   其经该指针写 `interface_types` arena）。
  /// - 调用须在单线程下进行：本函数经裸指针写穿 `Arc<Scope>` 共享对象与
  ///   `*mut Module` 句柄，无任何并发保护。
  ///
  /// `void Module::clonePublicInterface(NotNull<BuiltinTypes> builtinTypes, InternalErrorReporter& ice, SolverMode mode)`.
  /// Reference: `Module.cpp:299-348`.
  pub unsafe fn clone_public_interface(
    &mut self,
    builtin_types: Handle<BuiltinTypes>,
    _ice: &mut InternalErrorReporter,
    mode: SolverMode,
  ) {
    // C++ `CloneState cloneState{builtinTypes};` — declared (parity) but the
    // interface clone is driven by `ClonePublicInterface`'s own substitution.
    let _clone_state = CloneState {
      builtin_types,
      seen_types: DenseHashMap::default(),
      seen_type_packs: DenseHashMap::default(),
    };

    let module_scope = self.get_module_scope();
    // The C++ mutates the Scope behind the shared_ptr; mirror that by taking a
    // raw pointer to the aliased Scope object.
    let module_scope_ptr = arc_as_mut(&module_scope);

    // Safety: `module_scope_ptr` 由 `arc_as_mut` 从局部 Arc<Scope> 导出
    // （Arc::as_ptr 恒非空），该 Arc 在本函数内存活；直译 C++ 对
    // moduleScope->returnType 的裸解引用读取，单线程且此刻无其他借用活动。
    let return_type: TypePackId = unsafe { (*module_scope_ptr).return_type };
    let varargpack: Option<TypePackId> = if mode == SolverMode::New {
      None
    } else {
      // Safety: 同源句柄、同一存活论证；仅读 Option 值。
      unsafe { (*module_scope_ptr).vararg_pack }
    };

    // C++ `TxnLog log;` — a fresh, empty transaction log.
    let log = TxnLog {
      type_var_changes: DenseHashMap::default(),
      type_pack_changes: DenseHashMap::default(),
      parent: null_mut(),
      owned_seen: Vec::new(),
      // Empty; lazily owns a box on first `push_seen` (freed on drop).
      shared_seen: null_mut(),
      owned_seen_box: None,
      radioactive: false,
    };
    let mut clone_public_interface =
      // Safety: `self as *mut Module` 源自本方法的 &mut self，非空且在
      // `clone_public_interface` 存活期内保持有效；`builtin_types` 满足函数头
      // 契约；`&log` 按引用→裸指针降级为只读句柄，log 由本函数持有且存活于
      // CPI 使用期——逐项对应 `ClonePublicInterface::new` 的非空存活契约。
      unsafe { ClonePublicInterface::new(&log, builtin_types, self as *mut Module, mode) };

    let return_type = clone_public_interface.clone_type_pack(return_type);

    // Safety: `module_scope_ptr` 有效性同前（局部 Arc 保活）；对 Scope 字段的
    // 写入与 CPI 经 *mut Module 句柄写 `interface_types` arena 是互不重叠的
    // 两个对象，单线程下两处写入交替进行、各自半径止于单条语句。
    unsafe { (*module_scope_ptr).return_type = return_type };
    if let Some(vp) = varargpack {
      let varargpack = clone_public_interface.clone_type_pack(vp);
      // Safety: 同 return_type 写入——同一存活句柄上的另一独立字段赋值。
      unsafe { (*module_scope_ptr).vararg_pack = Some(varargpack) };
    }

    // Safety: 循环写的是 Scope 的 exported_type_bindings；CPI 只写 Module 的
    // interface_types arena，两个对象不重叠。`values_mut()` 迭代期间每次
    // clone_type_fun 只读源 TypeFun 值（&*tf），克隆结果写回同一槽位后才
    // 进入下一元素，读旧值与写新值顺序进行、无并发别名。
    unsafe {
      for tf in (*module_scope_ptr).exported_type_bindings.values_mut() {
        let cloned = clone_public_interface.clone_type_fun(&*tf);
        *tf = cloned;
      }
    }

    for ty in self.declared_globals.values_mut() {
      *ty = clone_public_interface.clone_type(*ty);
    }

    for tf in self.type_function_aliases.iter_mut() {
      let cloned = clone_public_interface.clone_type_fun(tf);
      **tf = cloned;
    }

    if clone_public_interface.internal_type_escaped {
      self
        .errors
        .push(TypeError::type_error_location_module_name_type_error_data(
          // Not amazing but the best we can do.
          Location::default(),
          self.name.clone(),
          InternalError::new(String::from(
            "An internal type is escaping this module; please report this bug at \
                     https://github.com/luau-lang/luau/issues",
          ))
          .into(),
        ));
    }

    // Copy external stuff over to Module itself
    // Safety: module_scope_ptr 仍有效（局部 Arc 存活至函数尾）；读 Scope 的
    // 克隆结果字段写入 self 的对应字段，源（Arc<Scope> 堆对象）与目标
    // （Module 本体）是两个不相邻对象。
    self.return_type = unsafe { (*module_scope_ptr).return_type };
    // Safety: 同上；clone() 对 exported_type_bindings 做一次性只读快照，
    // 随后不再经该句柄访问。
    self.exported_type_bindings = unsafe { (*module_scope_ptr).exported_type_bindings.clone() };
  }
}
