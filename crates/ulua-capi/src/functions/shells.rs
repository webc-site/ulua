//! capi 导出壳模板宏单源文件：所有同形透传壳宏（`capi_shell_*!`）的定义集中于此，
//! 由 `functions/` 下各壳文件以一次宏调用实例化；`functions/mod.rs` 仅保留模块声明注册表。
//! 每族宏的 `# Safety` 契约与 `// Safety:` 理由只在本文件书写一次，成员文件不得复述契约文本。

/// 单参 `(l) -> c_int` 通用 C ABI 导出壳模板：现 72 枚同形壳单源生成（46 枚裸透传 +
/// 23 枚 `@ref` 独占引用重建变体 + 3 枚 `@refshared` 只读引用重建变体），与
/// `lua_v_doarithimpl.rs` 的 `arith_tm_exports!` 先例同构
/// （退役为显式壳的本族同形成员现仅余 `lua_status.rs` 一枚——它同时是全仓先例本体，
/// 刻意不翻臂以免十余处「见 `lua_status.rs` 先例」引注失效）；lua_b_* 族 15 壳、int64 库族
/// 8 壳与 lua_gettop/lua_isthreadreset/lua_isyieldable 已分别由 `@ref`（r16-v32 一枚、
/// r16-v34 十四枚、r16-v35 八枚）与 `@refshared`（r16-v36 三枚）臂复归宏模板；
/// lua_singlestep/lua_pushinteger_64/lua_setthreaddata 等曾点名者本就非本族形（返回型或
/// 参数目不同）：其中 17 枚已随 r16-v37（4 枚）/r16-v39（13 枚）由 `capi_shell!` 的
/// `refstate`（独占重建）与 `refshared`（只读重建）参数类型臂复归单源，另有 4 枚随
/// r16-v37 由 `capi_shell_l_int!` 的 `@refshared` 臂复归（见该族头注）；capi 侧显式壳现存
/// 实测 13 枚，双引用重建形（`lua_l_buffinit` 的 `&mut *l` + `&mut *b`、`lua_a_pushvalue`
/// 的 `&mut *l` + 只读 `&*o`）尚无对应参数臂，余者见 `functions/lua_status.rs` 先例。
/// 与手写逐壳的差异仅在文本层：
/// 透传目标在 doc 契约中以 `ulua_vm::functions::` 全路径书写；体内
/// `// Safety:` 理由注释转通用表述。导出符号名、签名与 rustdoc 逐参数契约语义与
/// 逐字节手写版一致。
///
/// r16-v32 引用重建变体：被调 vm 核心已收形为 `&mut LuaState` 时，条目尾置 `@ref`
/// （`capi_shell_l_cint!(m, n @ref)`）即落下方第二臂——**导出签名逐字不变**（参数仍为
/// `*mut LuaState` 裸形，C-ABI 镜像红线），仅在既有 `unsafe { … }` 体内把该实参重建为
/// `&mut *l` 独占引用，借用窗严格止于当次调用。该形与 `capi_shell!` 的 `refstate`
/// 参数类型臂（r16-v24）、`lua_lib_fn!` 的 `@ref` 标记位同判例。本族恰一枚参数、
/// 恒名 `l`，故无 `capi_shell!` 那三条引用重建使用约束之必要（详见该族头注）。
///
/// r16-v36 只读引用重建变体：被调 vm 核心收形为**共享** `&LuaState` 时，条目尾置
/// `@refshared`（`capi_shell_l_cint!(m, n @refshared)`）落下方第三臂——体内发 `&*l`
/// 而**非** `&mut *l`。二臂不可互换：给只读核心 mint 独占引用，等于把契约里不存在的写权限
/// 交给被调面（在锚定形门面族已把窗口收成 `&mut` 借用的当下尤其为非），而 `&*l` 与核心
/// `&LuaState` 形参逐字同形、借用窗同样止于当次调用。选臂判据只看被调核心签名的接收者形，
/// 不看本壳导出形。
///
/// 相邻的 `capi_libfn_shell_l_cint!` 族不备本臂，判据见该族头注。
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
  ($m:ident, $n:ident @ref) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），除把 `l` 在本帧重建为独占引用（`&mut *l`）外，仅逐参数透传至 `ulua_vm::functions::", stringify!($m), "::", stringify!($n), "(&mut *l)`，零逻辑，本帧不解引用其余任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
    ) -> ::core::ffi::c_int {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 LuaState*。被调 vm 核心已前移为 `&mut LuaState` 引用形接收者，本帧把 `l` 重建为独占引用（`&mut *l`，借用窗止于当次调用），除此之外不解引用其余任何指针、不跨调用持有该引用，故不存在越窗别名/悬挂；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(&mut *l) }
    }
  };
  ($m:ident, $n:ident @refshared) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），除把 `l` 在本帧重建为只读共享引用（`&*l`）外，仅逐参数透传至 `ulua_vm::functions::", stringify!($m), "::", stringify!($n), "(&*l)`，零逻辑，本帧不解引用其余任何指针、不 mint 独占引用。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
    ) -> ::core::ffi::c_int {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 LuaState*。被调 vm 核心已前移为 `&LuaState` 只读引用形接收者，本帧把 `l` 重建为共享引用（`&*l`，借用窗止于当次调用）——刻意不 mint `&mut`，被调契约仅要求只读访问，扩成独占引用等于交出契约里不存在的写权限；除此之外不解引用其余任何指针、不跨调用持有该引用，故不存在越窗别名/悬挂；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(&*l) }
    }
  };
}

/// 同上 `(l) -> c_int` 导出壳模板之库函数变体（现 55 枚：42 枚裸透传 + 13 枚 `@ref`
/// 引用重建变体；vector_angle/vector_clamp 两壳曾随 vm 侧核心收形退役为显式壳，
/// 已由 `@ref` 臂于 r16-v32 复归宏模板，str_byte/str_char/str_len/str_split/str_sub
/// 五枚随 r16-v38 str_* 族首参收形同臂复归，math_clamp/frexp/ldexp/lerp/map/noise
/// 六枚随 r16-v40 math_* 族首参收形同臂复归）：唯一差异是体内
/// `// Safety:` 理由注释按 b26 校准保留「l 由 Lua VM 按库函数/闭包约定传入」的调用来源表述。
/// r16-v32 同备 `@ref` 引用重建变体臂，形制与措辞随上条所述。
/// 本族刻意不备 `@refshared` 臂：唯一只读形成员即 `lua_status.rs`（本仓先例本体），翻臂会使其余
/// 十余处「见 `lua_status.rs` 先例」引注失效；无消费者的臂既未经展开检验（未命中的宏臂不参与类型
/// 检查），又属死文本，故不开。
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
  ($m:ident, $n:ident @ref) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `ulua_", stringify!($n), "`），除把 `l` 在本帧重建为独占引用（`&mut *l`）外，仅逐参数透传至 `ulua_vm::functions::", stringify!($m), "::", stringify!($n), "(&mut *l)`，零逻辑，本帧不解引用其余任何指针。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = concat!("ulua_", stringify!($n)))]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
    ) -> ::core::ffi::c_int {
      // Safety: C ABI 导出壳：唯一参数 l 是 Lua VM 按库函数/闭包约定传入的当前运行 LuaState*，其栈顶与可接受索引由调用方按 Lua/C API 约定布置。被调 vm 核心已前移为 `&mut LuaState` 引用形接收者，本帧把 `l` 重建为独占引用（`&mut *l`，借用窗止于当次调用），除此之外不解引用其余任何指针、不跨调用持有该引用，故不存在越窗别名/悬挂；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(&mut *l) }
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
/// lua_userdatadirectfield_set* 系 5 文件 6 枚透传导出壳（nil / boolean / number /
/// integer_64 / vector4 与 vector3）共用，导出符号名固定为 `ulua_` + 函数名。
/// 与手写逐壳的差异仅在文本层：体内 `// Safety:` 理由注释逐壳列举的值型参数
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

/// `(l [, 值型参数...]) -> ()` 压栈导出壳模板：lua_pushvector_lapi 的 4/3 分量两枚
/// 透传导出壳共用，导出符号名固定为 `ulua_` + 函数名（lua_pushnil/lua_pushboolean/
/// lua_pushnumber 三枚已随 vm 侧接收者前移退役为显式壳，见 `functions/lua_status.rs`
/// 先例；lua_pushinteger_64 曾同因退役，但已随 r16-v39 由 `capi_shell!` 的 `refstate`
/// 参数类型臂复归该通用族单源，不在本族）。与手写逐壳的差异仅在文本层：体内
/// `// Safety:` 理由注释逐壳列举的值型参数名（如「b 均为值型参数」）统一为不点名
/// 表述；`/// # Safety` 契约逐字不变。
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
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 LuaState*；narg 及其余值型参数（如有）均为栈索引/标量，合法性由调用方按 API 约定给出，无指针前提。被调实现已前移为 `&mut LuaState` 引用形接收者，本帧把 `l` 重建为可变引用（`&mut *l`，借用窗口即时结束）后转调；其余调用序前提与被调方文档所列契约一致。
      unsafe { ::ulua_vm::functions::$m::$n(&mut *l, narg $(, $v)*) }
    }
  };
}

/// `(l, <c_int 值型参数>) -> c_int`（以及无返回值 `unit` 尾缀形态）导出壳模板：现 3 枚
/// 同形透传壳共用（coresumefinish / str_find_aux 返回 c_int，lua_setuserdatametatable 为
/// `unit` 形；第二参数名 r/find/tag 与导出符号名以入参给出——`lua_g_isnative` 的符号是
/// `ulua_luaG_isnative`，与函数名不同形，故符号一律走字面量，同
/// capi_shell_tkeyval! 先例）。
/// r16-v37 只读引用重建变体：本族 4 枚只读形成员（lua_g_hasnative / lua_g_isnative /
/// lua_isstring / lua_type）曾随 vm 核心收形退役为显式壳，现由下方 `@refshared` 臂复归
/// 宏模板单源。本族**不开** `@ref`（独占形）臂，且 r16-v39 后此判据由「留待专票」转为
/// 「永不开臂」：v37 曾指 2 枚显式壳 `lua_singlestep` / `lua_settop` 为该臂候选（核心皆
/// `&mut LuaState` 且返回 unit，须 `@ref` × `-> c_int` 与 `@ref` × `unit` 两臂一并补齐才
/// 不剩半态），但 `capi_shell!` 的 `refstate` 参数类型臂本就承载同形，v39 已实测把这两枚
/// 连同其余 11 枚批量复归该通用族，本族 `@ref` 消费者实测归零；无消费者的
/// 宏臂既未经展开检验（未命中的臂不参与类型检查）又属死文本，与 `capi_libfn_shell_l_cint!`
/// 族同判例（见该族头注）。`str_find_aux` 核心仍是裸 `*mut LuaState`，属本族裸透传成员，
/// 不在 `@ref` 候选之内。
/// 与手写逐壳的差异仅在
/// 文本层：体内 `// Safety:` 理由注释逐壳点名的参数（如「find 均为值型参数」）统一为
/// 不点名表述；`/// # Safety` 契约逐字不变。
macro_rules! capi_shell_l_int {
  // (l, v: c_int) -> c_int
  ($m:ident, $n:ident, $sym:literal, $v:ident) => {
    capi_shell_l_int!(@impl $m, $n, $sym, $v, ::core::ffi::c_int);
  };
  // (l, v: c_int) -> ()
  ($m:ident, $n:ident, $sym:literal, $v:ident, unit) => {
    capi_shell_l_int!(@impl $m, $n, $sym, $v);
  };
  // (l, v: c_int) -> c_int 之只读引用重建变体（r16-v37）：被调 vm 核心首参已收成共享
  // `&LuaState` 时条目尾置 `@refshared`（`capi_shell_l_int!(m, n, "sym", v @refshared)`），
  // 导出签名逐字不变（首参仍裸 `*mut LuaState`），仅体内发 `&*l` 而非 `&mut *l`。选臂判据
  // 只看被调核心接收者形；`@ref`（`&mut` 形）臂本票不开，实测候选与本票判据见本族头注。
  ($m:ident, $n:ident, $sym:literal, $v:ident @refshared) => {
    #[doc = concat!(
      "# Safety\n",
      "C ABI 导出壳（符号 `", $sym, "`），除把 `l` 在本帧重建为只读共享引用（`&*l`）外，仅逐参数透传至 `", stringify!($m), "::", stringify!($n), "(&*l, ", stringify!($v), ")`，零逻辑，本帧不解引用其余任何指针、不 mint 独占引用。调用方须保证：\n",
      "- `l`：指向由本 VM 创建的合法 `lua_State`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）——只读引用重建前提；\n",
      "- 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；\n",
      "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"
    )]
    #[unsafe(export_name = $sym)]
    pub unsafe extern "C-unwind" fn $n(
      l: *mut ::ulua_vm::records::lua_state::LuaState,
      $v: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 lua_State*；其余为栈索引/值型参数。被调 vm 核心已前移为 `&LuaState` 只读引用形接收者，本帧把 `l` 重建为共享引用（`&*l`，借用窗止于当次调用）——刻意不 mint `&mut`，被调契约仅要求只读访问，扩成独占引用等于交出契约里不存在的写权限；除此之外不解引用其余任何指针、不跨调用持有该引用，故不存在越窗别名/悬挂；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n(&*l, $v) }
    }
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
/// - `<名> refstate`：`LuaState` 指针参数之引用重建变体——被调 vm 核心已收形为
///   `&mut LuaState`：导出壳签名零变化（参数仍为 `*mut LuaState` 裸形，C-ABI 镜像红线），
///   仅在既有 `unsafe { … }` 体内把该枚实参重建为 `&mut *<名>` 独占引用、透传调用，
///   借用窗严格止于当次调用；契约行与 `// Safety:` 理由仍只在本模板单源书写；
/// - `<名> refshared`：同上之**只读**重建变体（r16-v37）——被调核心收形为共享 `&LuaState`
///   时体发 `&*<名>` 而非 `&mut *<名>`。二旗标不可互换：给只读核心 mint 独占引用等于把
///   契约里不存在的写权限交给被调面；选旗标只看被调核心签名的接收者形，不看本壳导出形。
///   与 `refstate` 同守三条使用约束，约束皆为臂形事实而非机械检验：条目须居参数表首位
///   （仅「首参前瞻入口」臂为二旗标改写导言措辞，非首位时通用入口仍宣称「本帧不解引用任何
///   指针」而 `@go` 参数臂照旧重建，契约即失真）；参数名须为 `l`（引用重建终结臂的体内
///   `// Safety:` 理由散文硬编码 `l`）；一壳至多一枚（第二枚无人拦，而同帧两枚 `&mut *`
///   本身即别名违例）。
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
  // r16-v24 refstate 首参前瞻入口：参数表以 `<名> refstate` 领头时仅导言契约行换
  // 「引用重建」措辞（同 v23 显式壳先例），其余 munching 与通用入口全同；置于通用
  // 入口臂之前抢先匹配，导出符号名/函数名/参数顺序仍逐字不变。
  ($m:ident, $sym:literal, $n:ident, [$p:ident refstate, $($rest:tt)*]) => {
    capi_shell!(@go $m $n $sym, [
      #[doc = concat!(
        "# Safety\n",
        "C ABI 导出壳（符号 `", $sym, "`），除把 `", stringify!($p), "` 在本帧重建为独占引用",
        "（`&mut *", stringify!($p), "`）外，仅按声明顺序逐参数透传至 `::ulua_vm::functions::",
        stringify!($m), "::", stringify!($n),
        "`，零逻辑，本帧不解引用其余任何指针。调用方须保证："
      )]
    ], [], [], [], [], $p refstate, $($rest)*);
  };
  // r16-v37 refshared 首参前瞻入口：与上方 refstate 入口同构，仅重建形为只读 `&*l`、
  // 契约措辞随形（「不 mint 独占引用」）；同样置于通用入口臂之前抢先匹配。
  ($m:ident, $sym:literal, $n:ident, [$p:ident refshared, $($rest:tt)*]) => {
    capi_shell!(@go $m $n $sym, [
      #[doc = concat!(
        "# Safety\n",
        "C ABI 导出壳（符号 `", $sym, "`），除把 `", stringify!($p), "` 在本帧重建为只读共享引用",
        "（`&*", stringify!($p), "`）外，仅按声明顺序逐参数透传至 `::ulua_vm::functions::",
        stringify!($m), "::", stringify!($n),
        "`，零逻辑，本帧不解引用其余任何指针、不 mint 独占引用。调用方须保证："
      )]
    ], [], [], [], [], $p refshared, $($rest)*);
  };
  ($m:ident, $sym:literal, $n:ident, [ $($params:tt)* ]) => {
    capi_shell!(@go $m $n $sym, [
      #[doc = concat!(
        "# Safety\n",
        "C ABI 导出壳（符号 `", $sym, "`），仅按声明顺序逐参数透传至 `::ulua_vm::functions::",
        stringify!($m), "::", stringify!($n),
        "`，零逻辑，本帧不解引用任何指针。调用方须保证："
      )]
    ], [], [], [], [], $($params)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident state, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；")]],
      [$($s)* $p: *mut ::ulua_vm::records::lua_state::LuaState,],
      [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  // r16-v24 refstate：签名仍收裸形（C-ABI 镜像红线），仅调用点实参换为 `&mut *$p`
  // 一次就地重建（落在终止臂既有 `unsafe { … }` 体内、借用窗止于当次调用），并以
  // `@refstate` 旗标路由到引用重建终止臂（体内 `// Safety:` 理由单源随形）。
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident refstate, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；")]],
      [$($s)* $p: *mut ::ulua_vm::records::lua_state::LuaState,],
      [$($c)* &mut *$p,], [$($r)*], [$($b)* @refstate], $($rest)*);
  };
  // r16-v37 refshared：签名同 refstate 仍收裸形（C-ABI 镜像红线），调用点实参换为
  // `&*$p` 只读重建，并以 `@refshared` 旗标路由到只读引用重建终止臂。
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident refshared, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）——只读引用重建前提；")]],
      [$($s)* $p: *mut ::ulua_vm::records::lua_state::LuaState,],
      [$($c)* &*$p,], [$($r)*], [$($b)* @refshared], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident voidptr, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`*mut c_void`）：C 侧不透明数据指针（userdata/缓冲/ud），可为 null；非 null 时对齐且调用期间存活；")]],
      [$($s)* $p: *mut ::core::ffi::c_void,],
      [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident tvc, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`*const TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue`，调用期间只读存活；")]],
      [$($s)* $p: *const ::ulua_vm::type_aliases::t_value::TValue,],
      [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident tvm, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`*mut TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue`，调用期间可写存活；")]],
      [$($s)* $p: *mut ::ulua_vm::type_aliases::t_value::TValue,],
      [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident stkid, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`StkId`）：合法栈槽指针（`*mut TValue` 别名），指向调用对应栈帧范围内的槽位，调用期间不迁移；")]],
      [$($s)* $p: ::ulua_vm::type_aliases::stk_id::StkId,],
      [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident cstr, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`（`*const c_char`）：指向 NUL 结尾的只读串缓冲（或按被调契约允许 null），对齐且在调用期间存活；")]],
      [$($s)* $p: *const ::core::ffi::c_char,],
      [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  // c_int 与 TMS 形参以裸名传入（调用点不写绝对路径，全路径由宏内给出），置于通用臂之前
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident val c_int, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：值型参数（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，无指针前提；")]],
      [$($s)* $p: ::core::ffi::c_int,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident val TMS, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：值型参数（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，无指针前提；")]],
      [$($s)* $p: ::ulua_vm::enums::tms::TMS,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident val $t:ty, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p),
        "`：值型参数（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，无指针前提；")]],
      [$($s)* $p: $t,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  // 特殊指针参数以裸名传入（全路径由宏内给出），置于通用 ptr 臂之前
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [*mut c_char] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::core::ffi::c_char,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [*const *const c_char] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *const *const ::core::ffi::c_char,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [*mut *mut c_void] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut *mut ::core::ffi::c_void,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [*mut LuauClass] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::ulua_vm::records::luau_class::LuauClass,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [*mut Closure] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::ulua_vm::records::closure::Closure,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [*mut Proto] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::ulua_vm::records::proto::Proto,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [*mut LuaLStrbuf] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: *mut ::ulua_vm::records::lua_l_strbuf::LuaLStrbuf,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [Pfunc] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: ::ulua_vm::type_aliases::pfunc::Pfunc,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      $p:ident ptr [$t:ty] $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym,
      [$($d)* #[doc = concat!("- `", stringify!($p), "`", $doc)]],
      [$($s)* $p: $t,], [$($c)* $p,], [$($r)*], [$($b)*], $($rest)*);
  };
  // 返回值以裸名传入（全路径由宏内给出），置于通用 => $rt:ty 臂之前
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => c_int, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> ::core::ffi::c_int], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => *const c_char, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *const ::core::ffi::c_char], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => *mut c_char, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::core::ffi::c_char], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => *mut c_void, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::core::ffi::c_void], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => *const c_void, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *const ::core::ffi::c_void], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => *const TValue, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *const ::ulua_vm::type_aliases::t_value::TValue], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => *mut TValue, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::ulua_vm::type_aliases::t_value::TValue], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => *mut Udata, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::ulua_vm::records::udata::Udata], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => *mut CallInfo, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> *mut ::ulua_vm::records::call_info::CallInfo], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [], [$($b:tt)*],
      => $rt:ty, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)*], [$($s)*], [$($c)*], [-> $rt], [$($b)*], $($rest)*);
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],
      @ret $doc:literal, $($rest:tt)*) => {
    capi_shell!(@go $m $n $sym, [$($d)* #[doc = $doc]], [$($s)*], [$($c)*], [$($r)*], [$($b)*], $($rest)*);
  };
  // r16-v24 refstate 专属终止臂：`@refstate` 旗标（由 refstate 参数臂置入）命中时，
  // 体内 `// Safety:` 理由行按引用重建语义单源给出；展开的签名/调用形态与其余透传
  // 臂一致，仅注释措辞随形。既有壳（旗标恒空）不匹配本臂，逐字节落回下方通用终止臂。
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [@refstate],) => {
    $($d)*
    #[doc = "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"]
    #[unsafe(export_name = $sym)]
    pub unsafe extern "C-unwind" fn $n($($s)*) $($r)* {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：各参数前提见上方契约，均由调用方保证。被调 vm 核心已前移为 `&mut LuaState` 引用形接收者，本帧把 `l` 重建为独占引用（`&mut *l`，借用窗止于当次调用），除此之外仅按声明顺序透传其余参数、不解引用其余任何指针，不跨调用持有该引用；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n($($c)*) }
    }
  };
  // r16-v37 refshared 专属终止臂：与上方 refstate 终止臂同构，唯重建形为 `&*l`——被调核心
  // 只收共享引用，本帧刻意不 mint `&mut`（否则等于交出契约里不存在的写权限）。
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [@refshared],) => {
    $($d)*
    #[doc = "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"]
    #[unsafe(export_name = $sym)]
    pub unsafe extern "C-unwind" fn $n($($s)*) $($r)* {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：各参数前提见上方契约，均由调用方保证。被调 vm 核心已前移为 `&LuaState` 只读引用形接收者，本帧把 `l` 重建为共享引用（`&*l`，借用窗止于当次调用）——刻意不 mint `&mut`，被调契约仅要求只读访问，扩成独占引用等于交出契约里不存在的写权限；除此之外仅按声明顺序透传其余参数、不解引用其余任何指针，不跨调用持有该引用；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n($($c)*) }
    }
  };
  (@go $m:ident $n:ident $sym:literal, [$($d:tt)*], [$($s:tt)*], [$($c:tt)*], [$($r:tt)*], [$($b:tt)*],) => {
    $($d)*
    #[doc = "- 其余安全前置条件与被调函数的 `# Safety` 契约一致。"]
    #[unsafe(export_name = $sym)]
    pub unsafe extern "C-unwind" fn $n($($s)*) $($r)* {
      // Safety: C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：各参数前提见上方契约，均由调用方保证。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
      unsafe { ::ulua_vm::functions::$m::$n($($c)*) }
    }
  };
}
