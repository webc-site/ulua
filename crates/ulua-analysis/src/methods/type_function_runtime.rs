//! `type_function_runtime` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{
  string::{String, ToString},
  vec::Vec,
};
use core::{
  ptr::{from_mut, null_mut},
  slice::from_ref,
};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_ast::{
  records::{
    allocator::Allocator,
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_name::AstName,
    ast_name_table::AstNameTable,
    ast_stat::AstStat,
    ast_stat_block::AstStatBlock,
    ast_stat_return::AstStatReturn,
    ast_stat_type_function::AstStatTypeFunction,
    location::Location,
    node_handle::{Node, Nodes},
    parse_result::ParseResult,
  },
  type_aliases::cst_node_map::CstNodeMap,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{functions::format::format, records::dense_hash_set::DenseHashSet};
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::{compile_error::CompileError, compile_options::CompileOptions},
};
use ulua_vm::{
  functions::{
    lua_close::lua_close, lua_gettable::lua_gettable, lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread, lua_newstate::lua_newstate,
    lua_newthread::lua_newthread, lua_settable::lua_settable, lua_setthreaddata::lua_setthreaddata,
    lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state,
};

use crate::{
  functions::{
    check_result_for_error::check_result_for_error,
    check_result_for_error_deprecated::check_result_for_error_deprecated,
    register_type_user_data::register_type_user_data,
    register_types_library::register_types_library,
    set_type_function_environment::set_type_function_environment,
    type_function_alloc::type_function_alloc,
  },
  records::{
    failed_to_compile::FailedToCompile, internal_error_reporter::InternalErrorReporter,
    luau_temp_thread_popper::LuauTempThreadPopper, type_check_limits::TypeCheckLimits,
    type_function_error::TypeFunctionError, type_function_missing::TypeFunctionMissing,
    type_function_runtime::TypeFunctionRuntime, typed_allocator::TypedAllocator,
  },
  type_aliases::{
    lua_state::LuaState, scope_ptr_type::ScopePtr, type_function_error_data::TypeFunctionErrorData,
  },
};

impl TypeFunctionRuntime {
  pub fn new(ice: &InternalErrorReporter, limits: &TypeCheckLimits, root_scope: ScopePtr) -> Self {
    Self {
      ice: ice.clone(),
      limits: limits.clone(),
      type_arena: TypedAllocator::default(),
      type_pack_arena: TypedAllocator::default(),
      state: (null_mut(), None),
      initialized: DenseHashSet::default(),
      allow_evaluation: true,
      root_scope,
      messages: Vec::new(),
      runtime_builder: null_mut(),
    }
  }
}

// Faithful port of `void TypeFunctionRuntime::prepareState()`
// (Analysis/src/TypeFunctionRuntime.cpp:230-248).

unsafe extern "C-unwind" fn lua_close_thunk(l: *mut LuaState) {
  unsafe {
    lua_close(l as *mut lua_state::LuaState);
  }
}
impl TypeFunctionRuntime {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn prepare_state(&mut self) {
    unsafe {
      // if (state) return;
      if !self.state.0.is_null() {
        return;
      }

      // state = StateRef(lua_newstate(typeFunctionAlloc, nullptr), lua_close);
      let new_state = lua_newstate(Some(type_function_alloc), null_mut()) as *mut LuaState;
      self.state = (new_state, Some(lua_close_thunk));

      // lua_State* l = state.get();
      let l = self.state.0;
      let vm_l = l as *mut lua_state::LuaState;

      // lua_setthreaddata(l, this);
      // Safety: `vm_l` 是本帧新建、尚无其它引用的存活 `LuaState`，`&mut *vm_l` 排他
      // 引用重建前提成立；`self` 地址只转手存入线程数据槽，不解引用。
      lua_setthreaddata(&mut *vm_l, (self as *mut TypeFunctionRuntime).cast());

      // setTypeFunctionEnvironment(l);
      set_type_function_environment(l);

      // registerTypeUserData(l);
      register_type_user_data(l);

      // registerTypesLibrary(l);
      register_types_library(l);

      // luaL_sandbox(l);
      lua_l_sandbox(vm_l);
      // luaL_sandboxthread(l);
      lua_l_sandboxthread(vm_l);
    }
  }
}

// Faithful port of
// `std::optional<TypeFunctionError> TypeFunctionRuntime::registerFunction(AstStatTypeFunction* function)`
// (Analysis/src/TypeFunctionRuntime.cpp:144-228)。
// 核心逻辑与 `_deprecated` 版共享，见
// [`super::type_function_runtime`]。

/// 新版错误表示：结构化 `TypeFunctionError`（V3 编译失败 / V4 缺失）。
impl RegisterErr for TypeFunctionError {
  fn compile_failed(name: String, what: String) -> Self {
    TypeFunctionError::type_function_error_location_type_function_error_data(
      Location::default(),
      TypeFunctionErrorData::V3(FailedToCompile::new(name, what)),
    )
  }
  fn missing(name: String) -> Self {
    TypeFunctionError::type_function_error_location_type_function_error_data(
      Location::default(),
      TypeFunctionErrorData::V4(TypeFunctionMissing::new(name)),
    )
  }
  fn check_result(l: *mut LuaState, name: &str, lua_result: i32) -> Option<Self> {
    check_result_for_error(l, name, lua_result)
  }
}
impl TypeFunctionRuntime {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn register_function(
    &mut self,
    function: *mut AstStatTypeFunction,
  ) -> Option<TypeFunctionError> {
    unsafe { self.register_function_impl::<TypeFunctionError>(function) }
  }
}

// Faithful port of
// `std::optional<std::string> TypeFunctionRuntime::registerFunction_DEPRECATED(AstStatTypeFunction* function)`
// (Analysis/src/TypeFunctionRuntime.cpp:58-142)。
// 核心逻辑与新版共享，见
// [`super::type_function_runtime`]。

/// 旧版错误表示：格式化字符串（与 cpp 各 format 调用逐字对应）。
impl RegisterErr for String {
  fn compile_failed(name: String, what: String) -> Self {
    format(format_args!(
      "'{}' type function failed to compile with error message: {}",
      name, what
    ))
  }
  fn missing(name: String) -> Self {
    format(format_args!(
      "Could not find '{}' type function in the global scope",
      name
    ))
  }
  fn check_result(l: *mut LuaState, name: &str, lua_result: i32) -> Option<Self> {
    check_result_for_error_deprecated(l, name, lua_result)
  }
}
impl TypeFunctionRuntime {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn register_function_deprecated(
    &mut self,
    function: *mut AstStatTypeFunction,
  ) -> Option<String> {
    unsafe { self.register_function_impl::<String>(function) }
  }
}

// `TypeFunctionRuntime::registerFunction` / `registerFunction_DEPRECATED`
// 的共享核心（Analysis/src/TypeFunctionRuntime.cpp:58-228）。
// 两个移植逐行等价，仅在错误表示上分叉，由 [`RegisterErr`] 参数化。

/// register_function（新版 `TypeFunctionError`）与
/// register_function_deprecated（旧版格式化 `String`）的错误表示分叉点。
pub(super) trait RegisterErr: Sized {
  /// `FailedToCompile{name, what}` / 对应的格式化字符串。
  fn compile_failed(name: String, what: String) -> Self;
  /// `TypeFunctionMissing{name}` / 对应的格式化字符串。
  fn missing(name: String) -> Self;
  /// checkResultForError / checkResultForError_DEPRECATED 的统一入口。
  fn check_result(l: *mut LuaState, name: &str, lua_result: i32) -> Option<Self>;
}
impl TypeFunctionRuntime {
  /// 共享核心：编译、沙箱执行并把用户类型函数登记进注册表。
  ///
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub(super) unsafe fn register_function_impl<E: RegisterErr>(
    &mut self,
    function: *mut AstStatTypeFunction,
  ) -> Option<E> {
    unsafe {
      // If evaluation is disabled, we do not generate additional error messages
      if !self.allow_evaluation {
        return None;
      }

      // Do not evaluate type functions with parse errors inside
      if (*function).has_errors {
        return None;
      }

      self.prepare_state();

      // lua_State* global = state.get();
      let global = self.state.0;
      let global_vm = global as *mut lua_state::LuaState;

      // Fetch to check if function is already registered
      // LUA_PUSHLIGHTUSERDATA(global, function); lua_gettable(global, LUA_REGISTRYINDEX);
      (*global_vm).push_lightuserdata((function as *mut ()).cast());
      lua_gettable(global_vm, LUA_REGISTRYINDEX);

      // if (!lua_isnil(global, -1)) { lua_pop(global, 1); return std::nullopt; }
      if !(*global_vm).is_nil(-1) {
        (*global_vm).pop(1);
        return None;
      }

      (*global_vm).pop(1);

      // AstName name = function->name;
      let name = (*function).name;
      let name_str = ast_name_to_string(name);

      // Construct ParseResult containing the type function
      // Allocator allocator; AstNameTable names(allocator);
      // Box 钉堆：AstNameTable/Parser 捕获宿主地址，宿主移动即悬垂。
      let mut allocator = Box::new(Allocator::new());
      let mut names = AstNameTable::new(&mut allocator);

      // AstExpr* exprFunction = function->body;
      let expr_function: *mut AstExpr = (*function).body.cast::<AstExpr>();
      // AstArray<AstExpr*> exprReturns{&exprFunction, 1};
      // 借用视图构造：`from_ref` 切片的元素地址即 `&exprFunction` 本地槽位
      // （编译器读 `data[0]` 取到的指针值与手写裸指针逐位相同），指针运算
      // 收口在 `AstArray::from_slice` 门面。
      let expr_returns: AstArray<*mut AstExpr> = AstArray::from_slice(from_ref(&expr_function));
      // AstStatReturn stmtReturn{Location{}, exprReturns};
      let mut stmt_return = AstStatReturn::new(Location::default(), expr_returns);
      let stat_node = Node::from_mut(&mut stmt_return).cast::<AstStat>();
      let stmts = Nodes::from_vec(alloc::vec![stat_node]);
      // AstStatBlock exec{Location{}, stmts};
      let mut exec = AstStatBlock::new(Location::default(), stmts, true);
      // ParseResult parseResult{&exec, 1, {}, {}, {}, CstNodeMap{nullptr}};
      let parse_result = ParseResult {
        root: from_mut(&mut exec),
        lines: 1,
        hotcomments: Vec::new(),
        errors: Vec::new(),
        comment_locations: Vec::new(),
        cst_node_map: CstNodeMap::new(null_mut()),
      };

      // BytecodeBuilder builder;
      let mut builder = BytecodeBuilder::default();
      let options = CompileOptions::default();

      // try { compileOrThrow(builder, parseResult, names); }
      // catch (CompileError& e) { /* 版本分叉：FailedToCompile / 格式化字符串 */ }
      let compile_outcome = catch_unwind(AssertUnwindSafe(|| {
        compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
          &mut builder,
          &parse_result,
          &mut names,
          &options,
        );
      }));

      if let Err(payload) = compile_outcome {
        if let Some(e) = payload.downcast_ref::<CompileError>() {
          // C++ `e.what()` -> the error message; CompileError's Display
          // yields exactly `self.message`.
          let what = ToString::to_string(e);
          return Some(E::compile_failed(name_str, what));
        }
        // Non-CompileError panic: re-raise to preserve faithful unwinding.
        resume_unwind(payload);
      }

      // std::string bytecode = builder.getBytecode();
      let bytecode = builder.get_bytecode().to_vec();

      // Separate sandboxed thread for individual execution and private globals
      // lua_State* l = lua_newthread(global);
      let l_vm = lua_newthread(global_vm);
      let l = l_vm as *mut LuaState;
      // luau_temp_thread_popper popper(global);
      let mut popper = LuauTempThreadPopper::new(global);

      // Create individual environment for the type function
      // luaL_sandboxthread(l);
      lua_l_sandboxthread(l_vm);

      // Do not allow global writes to that environment
      // lua_pushvalue(l, LUA_GLOBALSINDEX); lua_setreadonly(l, -1, true); lua_pop(l, 1);
      (*l_vm).push_value(LUA_GLOBALSINDEX);
      (*l_vm).set_readonly(-1, true);
      (*l_vm).pop(1);

      // Load bytecode into Luau state
      // if (auto error = checkResultForError(l, name.value, luau_load(l, name.value, bytecode.data(), bytecode.size(), 0)))
      //     return error;
      // cpp 的 `name.value` 是 NUL 结尾 C 串（`luau_load` 按 strlen 取长度），
      // Rust 侧直传 `&str`
      let load_result = luau_load(l_vm, &name_str, &bytecode, 0);
      if let Some(error) = E::check_result(l, &name_str, load_result) {
        popper.luau_temp_thread_popper();
        return Some(error);
      }

      // Execute the global function which should return our user-defined type function
      // if (auto error = checkResultForError(l, name.value, lua_resume(l, nullptr, 0))) return error;
      let resume_result = (*l_vm).resume(null_mut(), 0);
      if let Some(error) = E::check_result(l, &name_str, resume_result) {
        popper.luau_temp_thread_popper();
        return Some(error);
      }

      // if (!lua_isfunction(l, -1)) { lua_pop(l, 1); /* 版本分叉：TypeFunctionMissing / 格式化字符串 */ }
      if !(*l_vm).is_function(-1) {
        (*l_vm).pop(1);
        popper.luau_temp_thread_popper();
        return Some(E::missing(name_str));
      }

      // Store resulting function in the registry
      // LUA_PUSHLIGHTUSERDATA(global, function); lua_xmove(l, global, 1); lua_settable(global, LUA_REGISTRYINDEX);
      (*global_vm).push_lightuserdata((function as *mut ()).cast());
      lua_xmove(l_vm, global_vm, 1);
      lua_settable(global_vm, LUA_REGISTRYINDEX);

      popper.luau_temp_thread_popper();
      None
    }
  }
}
/// Helper: read an `AstName` into an owned `String`（空名 → ""）。
fn ast_name_to_string(name: AstName) -> String {
  name.as_str_or_empty().to_string()
}
