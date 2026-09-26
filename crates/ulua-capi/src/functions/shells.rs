//! capi 导出壳模板宏单源文件：所有同形透传壳宏（`capi_shell_*!`）的定义集中于此，
//! 由 `functions/` 下各壳文件以一次宏调用实例化；`functions/mod.rs` 仅保留模块声明注册表。
//! 每族宏的 `# Safety` 契约与 `// Safety:` 理由只在本文件书写一次，成员文件不得复述契约文本。

/// 单参 `(l) -> c_int` 通用 C ABI 导出壳模板：72 个同形透传壳（`LuaState`/`LuaState`
/// 为同一类型别名，仅源拼写差异）单源生成，与 `lua_v_doarithimpl.rs` 的
/// `arith_tm_exports!` 先例同构。与手写逐壳的差异仅在文本层：透传目标在 doc 契约中
/// 以 `ulua_vm::functions::` 全路径书写；体内 `// Safety:` 理由注释转通用表述。
/// 导出符号名、签名与 rustdoc 逐参数契约语义与逐字节手写版一致。
macro_rules! capi_shell_l_cint {
  ($m:ident, $n:ident) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），仅逐参数透传至 `ulua_vm::functions::", stringify!($m), "::", stringify!($n), "(l)`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
    ) -> ::core::ffi::c_int {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 LuaState*。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l) }
    }
  };
}

/// 同上 `(l) -> c_int` 导出壳模板之库函数变体：唯一差异是体内 `// Safety:` 理由
/// 注释按 b26 校准保留「l 由 Lua VM 按库函数/闭包约定传入」的调用来源表述。
macro_rules! capi_libfn_shell_l_cint {
  ($m:ident, $n:ident) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），仅逐参数透传至 `ulua_vm::functions::", stringify!($m), "::", stringify!($n), "(l)`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
    ) -> ::core::ffi::c_int {
      // Safety: C ABI 导出壳：唯一参数 l 是 Lua VM 按库函数/闭包约定传入的当前运行 LuaState*，其栈顶与可接受索引由调用方按 Lua/C API 约定布置。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l) }
    }
  };
}

/// 同上模板之双栈索引变体 `(l, index1, index2) -> c_int`：lua_equal/lua_lessthan/
/// lua_rawequal 三壳共用。
macro_rules! capi_shell_l_i_i_cint {
  ($m:ident, $n:ident) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），仅逐参数透传至 `ulua_vm::functions::", stringify!($m), "::", stringify!($n), "(l, index1, index2)`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
      index1: ::core::ffi::c_int,
      index2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为调用方提供的有效 LuaState*；index1、index2 均为值型参数（栈索引/标量），合法性由调用方按 API 约定给出，无指针前提。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l, index1, index2) }
    }
  };
}

/// `(l, obj, event: *const c_char) -> c_int` 元方法名壳模板：
/// lua_l_callmeta/lua_l_getmetafield 两壳共用（两份 21 行同形同契约）。
macro_rules! capi_shell_l_obj_event {
  ($m:ident, $n:ident) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），仅逐参数透传至 `ulua_vm::functions::", stringify!($m), "::", stringify!($n), "(l, obj, event)`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      "- `event`（`*const c_char`）：指向 NUL 结尾的只读串缓冲（或按被调契约允许 null），对齐且在调用期间存活；\n",
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
      obj: ::core::ffi::c_int,
      event: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 LuaState*；event 指向调用方在本次调用期间持有的 NUL 结尾缓冲；其余为栈索引/值型参数。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l, obj, event) }
    }
  };
}

/// `(l, t, key, val) -> ()` 表操作壳模板：lua_v_gettable/lua_v_settable 两壳共用
/// （导出符号名与函数名不同形，故符号以字面量入参）。
macro_rules! capi_shell_tkeyval {
  ($m:ident, $n:ident, $sym:literal) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `", $sym, "`），仅逐参数透传至 `ulua_vm::functions::", stringify!($m), "::", stringify!($n), "(l, t, key, val)`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      "- `t`（`*const TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue`，调用期间只读存活；\n",
      "- `key`（`*mut TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue`，调用期间可写存活；\n",
      "- `val`（`StkId`）：合法栈槽指针（`*mut TValue` 别名），指向调用对应栈帧范围内的槽位，调用期间不迁移；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = $sym)]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
      t: *const ::ulua_vm::type_aliases::t_value::TValue,
      key: *mut ::ulua_vm::type_aliases::t_value::TValue,
      val: ::ulua_vm::type_aliases::stk_id::StkId,
    ) {
      // Safety: C ABI 导出壳，仅由 VM 内部/宿主按 cpp LuaV_* 约定调用：l 为当前执行 LuaState*，其余指针参数（key、t、val）按该约定指向调用期间保持存活的 TValue/栈槽内存（栈内槽位或由栈持有的对象，调用方在调用前保留），可写结果槽与只读 TValue 以 const 区分；本帧不跨调用持有。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l, t, key, val) }
    }
  };
}

/// `(result: *mut c_void [, 值型参数...]) -> ()` userdata 直接字段写入壳模板：
/// lua_userdatadirectfield_set* 系 5 文件 7 枚透传导出壳（nil / boolean / number /
/// integer64 与 integer_64 / vector4 与 vector3）共用，导出符号名固定为 `ulua_` +
/// 函数名。与手写逐壳的差异仅在文本层：体内 `// Safety:` 理由注释逐壳列举的值型参数
/// 名（如「其余参数（b）为值型」）统一为不点名表述；`/// # Safety` 契约逐字不变。
macro_rules! capi_shell_udfield_set {
  ($m:ident, $n:ident) => {
    capi_shell_udfield_set!(@impl $m, $n, "");
  };
  // c_int 形参以裸名传入（调用点不写绝对路径，全路径由宏内给出），置于通用臂之前
  ($m:ident, $n:ident, $v:ident : c_int) => {
    capi_shell_udfield_set!(
      @impl $m, $n,
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      $v: ::core::ffi::c_int
    );
  };
  ($m:ident, $n:ident, $($v:ident : $vt:ty),+) => {
    capi_shell_udfield_set!(
      @impl $m, $n,
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      $($v : $vt),+
    );
  };
  (@impl $m:ident, $n:ident, $valbullet:literal $(, $v:ident : $vt:ty)*) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），仅逐参数透传至 `", stringify!($m), "::", stringify!($n), "(result", $(", ", stringify!($v),)* ")`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `result`（`*mut c_void`）：C 侧不透明数据指针（userdata/缓冲/ud），可为 null；非 null 时对齐且调用期间存活；\n",
      $valbullet,
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      result: *mut ::core::ffi::c_void $(, $v: $vt)*
    ) {
      // Safety: C ABI 导出壳，由 C 宿主在 LuauDirectFieldGet 直取路径调用：result 是该路径 lua_newuserdatadirect* 交还宿主、指向存活 userdata 内 TValue 字段槽的可写地址（对象本次调用期被栈钉住不被 GC 回收）；其余值型参数（如有）无指针前提。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(result $(, $v)*) }
    }
  };
}

/// `(l [, 值型参数...]) -> ()` 压栈导出壳模板：lua_push{nil,boolean,number,integer_64,
/// vector_lapi} 系 5 文件 7 枚透传导出壳（含 vector 的 4/3 分量两枚）共用，导出符号名
/// 固定为 `ulua_` + 函数名。与手写逐壳的差异仅在文本层：体内 `// Safety:` 理由注释逐壳
/// 列举的值型参数名（如「b 均为值型参数」）统一为不点名表述；`/// # Safety` 契约逐字不变。
macro_rules! capi_shell_push {
  ($m:ident, $n:ident) => {
    capi_shell_push!(@impl $m, $n, "");
  };
  // c_int 形参以裸名传入（调用点不写绝对路径，全路径由宏内给出），置于通用臂之前
  ($m:ident, $n:ident, $v:ident : c_int) => {
    capi_shell_push!(
      @impl $m, $n,
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      $v: ::core::ffi::c_int
    );
  };
  ($m:ident, $n:ident, $($v:ident : $vt:ty),+) => {
    capi_shell_push!(
      @impl $m, $n,
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      $($v : $vt),+
    );
  };
  (@impl $m:ident, $n:ident, $valbullet:literal $(, $v:ident : $vt:ty)*) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），仅逐参数透传至 `", stringify!($m), "::", stringify!($n), "(l", $(", ", stringify!($v),)* ")`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      $valbullet,
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState $(, $v: $vt)*
    ) {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 LuaState*；其余值型参数（如有）为栈索引/标量，合法性由调用方按 API 约定给出，无指针前提。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l $(, $v)*) }
    }
  };
}

/// `(l, narg [, 附加值型参数...]) -> 标量` check/opt 导出壳模板：lua_l_checkboolean/
/// lua_l_checkinteger_64/lua_l_optinteger_64 三枚同形透传壳共用（仅返回类型与可选 def
/// 参数差异），导出符号名固定为 `ulua_` + 函数名。与手写逐壳的差异仅在文本层：体内
/// `// Safety:` 理由注释逐壳列举的参数名统一为不点名表述；`/// # Safety` 契约逐字不变。
macro_rules! capi_shell_check_opt {
  // c_int（返回类型或 narg 形参）以裸名传入，调用点不写绝对路径，全路径由宏内给出
  ($m:ident, $n:ident, c_int, narg: c_int $(, $v:ident : $vt:ty)*) => {
    capi_shell_check_opt!(@impl $m, $n, ::core::ffi::c_int, narg: ::core::ffi::c_int $(, $v: $vt)*);
  };
  ($m:ident, $n:ident, $ret:ty, narg: c_int $(, $v:ident : $vt:ty)*) => {
    capi_shell_check_opt!(@impl $m, $n, $ret, narg: ::core::ffi::c_int $(, $v: $vt)*);
  };
  (@impl $m:ident, $n:ident, $ret:ty, narg: ::core::ffi::c_int $(, $v:ident : $vt:ty)*) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），仅逐参数透传至 `", stringify!($m), "::", stringify!($n), "(l, narg", $(", ", stringify!($v),)* ")`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
      narg: ::core::ffi::c_int $(, $v: $vt)*,
    ) -> $ret {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 LuaState*；narg 及其余值型参数（如有）均为栈索引/标量，合法性由调用方按 API 约定给出，无指针前提。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l, narg $(, $v)*) }
    }
  };
}

/// `(l, <c_int 值型参数>) -> c_int`（以及无返回值 `unit` 尾缀形态）导出壳模板：
/// coresumefinish / lua_g_hasnative / lua_g_isnative / lua_isstring / lua_type /
/// str_find_aux（返回 c_int）与 lua_settop / lua_setuserdatametatable /
/// lua_singlestep（无返回值）共 9 枚同形透传壳共用（第二参数名 r/level/idx/find/
/// tag/enabled 与导出符号名以入参给出——`lua_g_isnative` 的符号是
/// `ulua_luaG_isnative`，与函数名不同形，故符号一律走字面量，同
/// capi_shell_tkeyval! 先例）。与手写逐壳的差异仅在文本层：体内 `// Safety:` 理由
/// 注释逐壳点名的参数（如「find 均为值型参数」）统一为不点名表述；`/// # Safety`
/// 契约逐字不变。
macro_rules! capi_shell_l_int {
  // (l, v: c_int) -> c_int
  ($m:ident, $n:ident, $sym:literal, $v:ident) => {
    capi_shell_l_int!(@impl $m, $n, $sym, $v, ::core::ffi::c_int);
  };
  // (l, v: c_int) -> ()
  ($m:ident, $n:ident, $sym:literal, $v:ident, unit) => {
    capi_shell_l_int!(@impl $m, $n, $sym, $v);
  };
  (@impl $m:ident, $n:ident, $sym:literal, $v:ident $(, $ret:ty)?) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `", $sym, "`），仅逐参数透传至 `", stringify!($m), "::", stringify!($n), "(l, ", stringify!($v), ")`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `lua_State`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = $sym)]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
      $v: ::core::ffi::c_int,
    ) $(-> $ret)? {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 lua_State*；其余为栈索引/值型参数。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l, $v) }
    }
  };
}

/// `(l, <不透明指针>, v: *mut c_void) -> ()` GC 屏障导出壳模板：lua_c_barrierf/
/// lua_c_barriertable 两枚同形透传壳共用（首个指针参数名 o/t 以入参给出；导出符号名与
/// 函数名不同形，故符号以字面量入参，同 capi_shell_tkeyval! 先例）。与手写逐壳的差异仅在
/// 文本层：体内 `// Safety:` 理由注释逐壳点名的参数统一为不点名表述；`/// # Safety` 契约逐字不变。
macro_rules! capi_shell_barrier_voidptr {
  ($m:ident, $n:ident, $sym:literal, $a:ident) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `", $sym, "`），仅逐参数透传至 `", stringify!($m), "::", stringify!($n), "(l, ", stringify!($a), ", v)`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
      "- `", stringify!($a), "`（`*mut c_void`）：C 侧不透明数据指针（userdata/缓冲/ud），可为 null；非 null 时对齐且调用期间存活；\n",
      "- `v`（`*mut c_void`）：C 侧不透明数据指针（userdata/缓冲/ud），可为 null；非 null 时对齐且调用期间存活；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = $sym)]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
      $a: *mut ::core::ffi::c_void,
      v: *mut ::core::ffi::c_void,
    ) {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 LuaState*；两枚不透明指针参数均为 C 侧在本次调用期间持有的地址（或 null）。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(l, $a, v) }
    }
  };
}

/// 通用逐参数透传导出壳宏（opt-r18 收口）：任意「仅把 C 参数转手给 ulua-vm 实现、
/// 返回值原样转手」的同形壳按参数类型族单源生成，`/// # Safety` 逐参数契约与体内
/// `// Safety:` 理由只在本模板书写一次。条目语法（每项以逗号结尾）：
/// - `<名> state`：合法 `LuaState` 指针参数；
/// - `<名> voidptr`：`*mut c_void` C 侧不透明数据指针；
/// - `<名> tvc` / `<名> tvm`：只读 / 可写 `TValue` 指针；
/// - `<名> stkid`：`StkId` 栈槽指针；
/// - `<名> cstr`：NUL 结尾 `*const c_char` 串缓冲；
/// - `<名> val <类型>`：值型标量参数（栈索引/数值/布尔/枚举）；
/// - `<名> ptr [<类型>] "<契约行>"`：其余指针/特殊参数，逐字保留其独立前提契约；
/// - `=> <返回类型>`：返回值；`@ret "<返回值契约行>"`：返回值生命周期契约（随返回类型给出）。
///
/// 导出符号名、壳函数名与参数顺序逐字不变；仅契约文本归一为模板单源。
macro_rules! capi_shell {
  ($m:ident, $sym:literal, $n:ident, [ $($params:tt)* ]) => {
    capi_shell!(@go $m $n $sym, [
      #[doc = concat!(
        "# Safety\n",
        "C ABI 导出壳（符号 `", $sym, "`），仅按声明顺序逐参数透传至 `::ulua_vm::functions::",
        stringify!($m), "::", stringify!($n),
        "`，零逻辑，本帧不解引用任何指针。调用方须保证："
      )]
    ], [], [], [], $($params)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident state, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；")]],
      [$($s)* $p: *mut ::ulua_vm::records::lua_state::LuaState,],
      [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident voidptr, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`*mut c_void`）：C 侧不透明数据指针（userdata/缓冲/ud），可为 null；非 null 时对齐且调用期间存活；")]],
      [$($s)* $p: *mut ::core::ffi::c_void,],
      [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident tvc, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`*const TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue`，调用期间只读存活；")]],
      [$($s)* $p: *const ::ulua_vm::type_aliases::t_value::TValue,],
      [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident tvm, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`*mut TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue`，调用期间可写存活；")]],
      [$($s)* $p: *mut ::ulua_vm::type_aliases::t_value::TValue,],
      [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident stkid, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`StkId`）：合法栈槽指针（`*mut TValue` 别名），指向调用对应栈帧范围内的槽位，调用期间不迁移；")]],
      [$($s)* $p: ::ulua_vm::type_aliases::stk_id::StkId,],
      [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident cstr, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`*const c_char`）：指向 NUL 结尾的只读串缓冲（或按被调契约允许 null），对齐且在调用期间存活；")]],
      [$($s)* $p: *const ::core::ffi::c_char,],
      [$($c)* $p,], [$($r)*], $($rest)*);
  };
  // c_int 与 TMS 形参以裸名传入（调用点不写绝对路径，全路径由宏内给出），置于通用臂之前
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident val c_int, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：值型参数（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，无指针前提；")]],
      [$($s)* $p: ::core::ffi::c_int,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident val TMS, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：值型参数（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，无指针前提；")]],
      [$($s)* $p: ::ulua_vm::enums::tms::TMS,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident val $t:ty, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：值型参数（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，无指针前提；")]],
      [$($s)* $p: $t,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  // 特殊指针参数以裸名传入（全路径由宏内给出），置于通用 ptr 臂之前
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [*mut c_char] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::core::ffi::c_char,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [*const *const c_char] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *const *const ::core::ffi::c_char,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [*mut *mut c_void] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut *mut ::core::ffi::c_void,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [*mut LuauClass] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::ulua_vm::records::luau_class::LuauClass,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [*mut Closure] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::ulua_vm::records::closure::Closure,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [*mut Proto] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::ulua_vm::records::proto::Proto,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [*mut LuaLStrbuf] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::ulua_vm::records::lua_l_strbuf::LuaLStrbuf,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [Pfunc] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: ::ulua_vm::type_aliases::pfunc::Pfunc,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      $p:ident ptr [$t:ty] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: $t,], [$($c)* $p,], [$($r)*], $($rest)*);
  };
  // 返回值以裸名传入（全路径由宏内给出），置于通用 => $rt:ty 臂之前
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => c_int, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> ::core::ffi::c_int], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => *const c_char, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *const ::core::ffi::c_char], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => *mut c_char, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::core::ffi::c_char], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => *mut c_void, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::core::ffi::c_void], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => *const c_void, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *const ::core::ffi::c_void], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => *const TValue, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *const ::ulua_vm::type_aliases::t_value::TValue], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => *mut TValue, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::ulua_vm::type_aliases::t_value::TValue], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => *mut Udata, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::ulua_vm::records::udata::Udata], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => *mut CallInfo, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::ulua_vm::records::call_info::CallInfo], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [],
      => $rt:ty, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> $rt], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],
      @ret $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)* #[doc = $doc]], [$($s)*], [$($c)*], [$($r)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*],) => {
    $($d)*
    #[doc = "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"]
    #[unsafe(export_name = $sym)]
    pub unsafe extern "C-unwind" fn $n($($s)*) $($r)* {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：各参数前提见上方契约，均由调用方保证。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n($($c)*) }
    }
  };
}
