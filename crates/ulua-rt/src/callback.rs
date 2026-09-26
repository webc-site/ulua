//! The Rust-closure-as-Lua-function trampoline.
//!
//! ## Design (see also the crate-level docs)
//!
//! A user-supplied Rust closure `F: Fn(&Lua, A) -> Result<R>` is **not** type-
//! erased: [`create_callback_function`] is generic over `(F, A, R)` and the
//! closure is boxed as `Box<F>` and stored (inside an `Option`, see below) in
//! a Lua userdata created with [`lua_newuserdatadtor`] — the (per-`F`)
//! destructor reconstitutes and drops the `Box`, so the closure's captured
//! environment is freed exactly when the GC collects the function. The
//! userdata is then captured as **upvalue 1** of a monomorphized C trampoline
//! function ([`trampoline::<F, A, R>`]) pushed via [`lua_pushcclosurek`] with
//! `nup = 1`. No `dyn` fat pointer, no vtable hop: the call is a direct call
//! on the concrete closure type.
//!
//! When Lua calls the function:
//!  1. The trampoline fetches upvalue 1 with [`lua_upvalueindex`] and recovers
//!     `&Option<Box<F>>` from the userdata pointer.
//!  2. It pops all on-stack arguments into a [`MultiValue`] and converts them
//!     to `A` via [`FromLuaMulti`].
//!  3. It runs the closure **inside [`catch_unwind`]** — so a `panic!` in user
//!     code can never become a nested panic while we are about to call
//!     [`lua_error`].
//!  4. On success it converts the `R` return via [`IntoLuaMulti`], pushes the
//!     results and returns the count.
//!  5. On a returned `Err`, or a caught panic, it pushes a message string and
//!     calls [`lua_error`]. `lua_error` raises the VM's normal longjmp-style
//!     error (a `panic_any(lua_exception)`), which unwinds this trampoline
//!     frame up to the VM's protected-call boundary — the VM's own mechanism.
//!
//! Because the user panic is caught *before* `lua_error` is called, there is
//! never a double-unwind, and a genuine Rust panic in user code surfaces as an
//! ordinary catchable Lua error, not a process abort.
//!
//! ## Why the slot is an `Option`
//!
//! [`Lua::scope`](crate::Scope) callbacks borrow non-`'static` data. Since the
//! trampoline is monomorphized over the concrete `F` (which carries the
//! borrow's lifetime), **no lifetime erasure is needed at all** — the soundness
//! of dropping the closure before its borrowed data dies rests purely on
//! [`destruct_callback::<F>`]: the scope's exit runs it, `take()`ing the box
//! out of the slot (ending the borrows). The slot then reads `None`, and any
//! later call raises a structured [`Error::CallbackDestructed`] — the Lua
//! function object itself stays fully valid, only its behavior changes.

use core::{
  any::Any,
  mem::size_of,
  ptr::{NonNull, drop_in_place},
};
use std::{
  any::TypeId,
  borrow::Cow,
  panic::{AssertUnwindSafe, catch_unwind},
  thread::Result as ThreadResult,
};

use crate::{
  error::{Error, Result},
  function::Function,
  multi::MultiValue,
  registry::RegHandle,
  state::Lua,
  sys::*,
  traits::{FromLuaMulti, IntoLuaMulti},
  userdata::alloc_userdata_slot,
};

// ---------------------------------------------------------------------------
// Structured error objects (for errors that must survive the Lua boundary)
// ---------------------------------------------------------------------------
//
// Most callback errors are raised as plain Lua strings via `lua_error`, and
// `Lua::pop_error` rebuilds a flat `Error::RuntimeError`. That keeps the simple,
// message-only error path that the rest of the crate (and its tests) rely on.
//
// A small set of errors, however, carry *structured* meaning that the caller
// must be able to pattern-match after the error has travelled up through a
// `lua_pcall` boundary — specifically `CallbackDestructed` and
// `UserDataDestructed`, raised when a scope-created callback/userdata is invoked
// after its `Lua::scope` has ended. For those we push a **userdata error
// object** holding a boxed `Error` (tagged with a magic `TypeId` header), call
// `lua_error`, and recover the structured error in `pop_error`, wrapping it in
// `Error::CallbackError { cause, .. }` exactly like mlua.

/// 用户回调 panic 转为 Lua 错误时的消息前缀（callback / async / interrupt
/// 三处的 panic guard 共用）。
pub(crate) const PANIC_MSG_PREFIX: &str = "rust panic: ";

/// The wrapped-error userdata storage: a magic `TypeId` followed by the boxed
/// structured `Error`.
#[repr(C)]
struct WrappedError {
  type_id: TypeId,
  error: Box<Error>,
}

/// A private marker type whose `TypeId` tags our wrapped-error userdata.
struct WrappedErrorTag;

/// The magic tag identifying a ulua-rt wrapped-error userdata.
pub(crate) fn wrapped_error_tag() -> TypeId {
  TypeId::of::<WrappedErrorTag>()
}

/// Destructor for the [`WrappedError`] userdata: drops the boxed `Error`.
///
/// # Safety
/// 仅由 VM 作为 `lua_newuserdatadtor` 注册的终结器调用：`ptr` 必须为 null 或
/// 指向本模块 `raise_structured_error` 按 `WrappedError` 布局写入、且尚未被
/// drop 的 userdata 载荷；VM 保证其恰被调用一次。
unsafe extern "C-unwind" fn wrapped_error_dtor(ptr: *mut c_void) {
  if !ptr.is_null() {
    // Safety: `ptr` 由 VM 在终结该 wrapped-error userdata 时传入，只可能来自
    // `raise_structured_error` 的 `lua_newuserdatadtor(size_of::<WrappedError>(),
    // wrapped_error_dtor)` 配对：载荷长度恰覆盖 `#[repr(C)]` 的
    // `WrappedError`（TypeId + 薄盒指针，载荷 `alignas(8)` 足够），且写入已
    // 在 lua_error 发散前完成；终结器由 VM 保证恰调用一次。
    unsafe { drop_in_place(ptr.cast::<WrappedError>()) };
  }
}

/// Whether the given error should be raised as a *structured* userdata error
/// object (so the caller can match on it) rather than a flat string. Only the
/// scope-destruction errors qualify; everything else keeps the string path to
/// preserve backward-compatible `RuntimeError` behavior.
pub(crate) fn is_structured(err: &Error) -> bool {
  match err {
    Error::CallbackDestructed | Error::UserDataDestructed | Error::RecursiveMutCallback => true,
    // A `CallbackError` produced when a *nested* structured error crossed a
    // `pcall` boundary is itself re-raised structured, so each boundary adds
    // one `CallbackError` wrapper layer — mirroring mlua's nested
    // `CallbackError { cause: CallbackError { cause: .. } }`.
    Error::CallbackError { cause, .. } => is_structured(cause),
    _ => false,
  }
}

/// Push a structured [`Error`] as a wrapped-error userdata error object and
/// invoke [`lua_error`]. Diverges (unwinds via the VM's longjmp).
///
/// # Safety
/// `state` must be a valid `LuaState` with at least one free stack slot.
pub(crate) unsafe fn raise_structured_error(state: *mut LuaState, err: Error) -> ! {
  // Safety: 函数头契约给出存活 `state` 与 ≥1 空位；`wrapped_error_dtor` 与载荷
  // `WrappedError` 同一类型单态化。返回 `None` 即分配失败。
  let storage = unsafe { alloc_userdata_slot::<WrappedError>(state, wrapped_error_dtor) };
  let Some(storage) = storage else {
    // Fall back to a string error if we cannot allocate the userdata.
    // Safety: `state` 存活、正于 C 边界内被当前线程驱动、≥1 空位（与函数头同前提）；
    // `err.to_string()` 在本帧存活且被 `lua_pushlstring` 当场拷贝。同为发散。
    unsafe { raise_lua_error(state, &err.to_string()) }
  };
  // Safety: 非空时载荷恰 `size_of::<WrappedError>()`，对齐由 `alloc_userdata_slot` 的
  // `AlignOk` 编译期断言把住（TypeId + 薄盒指针 ≤ 8 字节）；`write` 覆盖整个未初始化
  // 槽位（首次初始化，不泄漏旧值），之后所有权移交 userdata——`lua_error` 发散前无
  // 任何可失败点，之后由 `wrapped_error_dtor` 唯一一次 drop；不存在双重释放或半初始化读。
  unsafe {
    storage.write(WrappedError {
      type_id: wrapped_error_tag(),
      error: Box::new(err),
    })
  };
  // Safety: 函数头契约（正于 C 边界内被当前线程驱动）；`lua_error` 按 VM 的
  // longjmp 式展开逃逸到受保护边界，发散不返回。
  unsafe { lua_error(state) } // diverges (`-> !`)
}

/// If the value at stack index `idx` is a ulua-rt wrapped-error userdata,
/// return a clone of the contained [`Error`]. Does not pop.
///
/// # Safety
/// `state` must be valid and `idx` a valid (absolute) stack index.
pub(crate) unsafe fn recover_wrapped_error(state: *mut LuaState, idx: c_int) -> Option<Error> {
  // Safety: 函数头契约给出存活 `state` 与有效索引 `idx`；`lua_type` 只读该槽类型
  // 标记，不触发 GC、不动栈，读取期间值稳定（不被收集、不移动）。
  if unsafe { lua_type(state, idx) } != LuaType::UserData as c_int {
    return None;
  }
  // Safety: 同上——`lua_touserdata`/`lua_objlen` 也是 `idx` 槽上的只读查询。
  // 缺失/空载荷由 `Option` 归一为 `None`（消灭 null 哨兵分支）。
  let payload = unsafe { lua_touserdata(state, idx) }.map(NonNull::from);
  // A script can raise any userdata (`error(newproxy())` raises a zero-length
  // one), so the stored length must cover the layout before it is read.
  // Safety: 同上，`lua_objlen` 对 userdata 返回载荷字节数，只读；`filter` 只在载荷
  // 非空时才查询长度，与原本「先非空、再长度」的短路顺序一致。
  let payload = payload.filter(|_| {
    // Safety: `lua_objlen` 是 `idx` 槽上的只读查询。
    unsafe { lua_objlen(state, idx) >= size_of::<WrappedError>() as c_int }
  })?;
  let wrapped = payload.cast::<WrappedError>();
  // Safety: 三道闸门（userdata 类型、载荷非空、长度覆盖 `WrappedError`）已过，
  // 故 `as_ref()` 落在已校验的载荷区内；`#[repr(C)]` 布局与 `alignas(8)` 的
  // userdata 载荷相容；值由栈槽引用钉住、GC 不移动，本块内只读无别名。
  // `type_id` 字段比对垃圾位值只是整数比较，命中魔数标记才读 `error` 盒。
  // 返回克隆出的 owned 值，出块后无栈/对象依赖。
  let wrapped = unsafe { wrapped.as_ref() };
  // Only our wrapped errors carry the magic tag.
  if wrapped.type_id != wrapped_error_tag() {
    return None;
  }
  Some((*wrapped.error).clone())
}

/// The closure slot stored in the callback userdata: `Some(box)` while live,
/// `None` after [`destruct_callback`] neutralised a scope-created callback.
/// A thin pointer (`Option<Box<F>>` is niche-optimised to one word) — the
/// closure body itself lives on the heap box, so the userdata payload has no
/// alignment requirement beyond the 8 bytes the VM guarantees.
type CallbackSlot<F> = Option<Box<F>>;

/// The destructor installed on the callback userdata: reconstruct the
/// `CallbackSlot<F>` inside the userdata storage and drop it (running `Drop`
/// on the closure's captures).
///
/// `lua_newuserdatadtor` stores the data inline; `lua_touserdata` returns a
/// pointer to that storage, which is exactly where we wrote the slot. We drop
/// it in place.
///
/// # Safety
/// 仅由 VM 作为终结器调用：`ptr` 必须为 null，或指向
/// `create_callback_function` 以同一 `F` 的 `CallbackSlot<F>` 布局写入、
/// 尚未 drop 的 userdata 载荷；VM 保证其恰被调用一次。
unsafe extern "C-unwind" fn callback_dtor<F>(ptr: *mut c_void) {
  if !ptr.is_null() {
    // Safety: `ptr` 由 VM 终结回调 userdata 时传入，只可能配对
    // `create_callback_function` 的 `lua_newuserdatadtor(
    // size_of::<CallbackSlot<F>>(), callback_dtor::<F>)`——同一 `F` 的
    // 单态化写死布局；`CallbackSlot<F>` 是薄盒指针的 niche 枚举，8 字节
    // 载荷即可安全读写。VM 保证终结器恰一次调用且在 GC 回收路径上，
    // 此时不会再有 trampoline 持 `&slot`（同线程、调用栈内借用已退出）。
    unsafe { drop_in_place(ptr.cast::<CallbackSlot<F>>()) };
  }
}

/// The trampoline for one `(F, A, R)` instantiation of `create_function`-style
/// closures. Monomorphized: the call into user code is a direct static call.
///
/// # Safety
/// 仅由 VM 作为 `LuaCFunction` 调用：`state` 必须是正在受保护 C 边界内驱动的
/// 存活 `LuaState`，且 upvalue 1 是该闭包注册的 `CallbackSlot<F>` userdata
/// （布局由同一 `F` 单态化确定，脚本不可替换）。
unsafe extern "C-unwind" fn trampoline<F, A, R>(state: *mut LuaState) -> c_int
where
  F: Fn(&Lua, A) -> Result<R>,
  A: FromLuaMulti,
  R: IntoLuaMulti,
{
  // 本函数的共用前置（下面每个 `raise_*` 调用点都引它）：VM 按 `LuaCFunction`
  // 约定实时传入 `state`，故其存活且正由当前线程在受保护 C 边界内驱动；CI 帧
  // 建立时预留的 LUA_MINSTACK 头寸满足错误对象的 push。用户闭包连同 `A`/`R`
  // 转换都在 `catch_unwind` 内，panic 不会裸跨 C-unwind 边界；结果路径先
  // `lua_checkstack` 再逐层 `push_value`，失败全部以发散收敛；返回计数与实际
  // 压入层数一致。
  //
  // 1. Recover the closure slot from upvalue 1.
  // Safety: upvalue 1 的 userdata 由同一 `(F, A, R)` 单态化的
  // `create_callback_function` 写入、脚本无法替换，故
  // `cast::<CallbackSlot<F>>().as_ref()` 解读的布局即写入布局（niche 薄盒指针，
  // 8 字节载荷对齐足够）；`lua_touserdata` 的 null 虽不可能仍经 `NonNull::new`
  // 归一为 `None`，在同一块内由 `raise_lua_error`（共用前置）发散收敛。
  let slot: Option<&CallbackSlot<F>> = unsafe {
    match lua_touserdata(state, lua_upvalueindex(1)) {
      Some(ud) => Some(NonNull::from(ud).cast::<CallbackSlot<F>>().as_ref()),
      None => raise_lua_error(state, "ulua-rt: missing callback upvalue"),
    }
  };
  // 2. A destructed scope callback reports the structured
  //    `CallbackDestructed` (identical externally to the previous design's
  //    sentinel box). `slot.as_ref()` 只是把 `&Option<Box<F>>` 收成
  //    `Option<&Box<F>>`，纯 Rust。
  let Some(callback) = slot.and_then(|slot| slot.as_ref()) else {
    // Safety: 共用前置成立；`err` 按值移入，错误 userdata 由被调方自行分配写入。
    unsafe { raise_structured_error(state, Error::CallbackDestructed) }
  };

  // 3. Build a borrowed Lua handle for the calling thread (must NOT close it).
  // `Lua::from_borrowed` 是带契约的 safe fn（只存指针不解引用）：`state` 由 VM
  // 实时传入即满足「存活期覆盖句柄及其克隆」。
  let lua = Lua::from_borrowed(state);

  // 4. Pull the arguments off the stack into a MultiValue. They occupy
  //    stack indices 1..=nargs.
  // Safety: `state` 存活，`lua_gettop` 只读当前栈深——`1..=nargs` 因而是有效槽位，
  // 正是 `collect_stack_args` 的头注释前提；转换失败在同一块内由 `raise_lua_error`
  // （共用前置）发散收敛。
  let args = unsafe {
    let nargs = lua_gettop(state);
    match collect_stack_args(&lua, nargs) {
      Ok(a) => a,
      Err(e) => raise_lua_error(state, &e.to_string()),
    }
  };

  // 5. Run the user closure inside catch_unwind so a user `panic!` never
  //    becomes a nested panic when we then call lua_error. The `A`/`R`
  //    conversion is part of the guarded region (same as before, when it
  //    lived inside the boxed wrapper). 纯 Rust 区：无 C 边界，故无 unsafe。
  let outcome: ThreadResult<Result<MultiValue>> = catch_unwind(AssertUnwindSafe(|| {
    let a = A::from_lua_multi(args, &lua)?;
    callback(&lua, a)?.into_lua_multi(&lua)
  }));

  match outcome {
    Ok(Ok(results)) => {
      // 6a. Push every result and return its count. Reserve stack space
      //     first: an unchecked push of a very large result list would
      //     overflow the Lua stack and trip a fatal VM assertion
      //     (SIGTRAP) instead of erroring. Guard it and raise a
      //     catchable error if the results cannot fit (mirroring mlua).
      let n = results.len() as c_int;
      // Safety: `state` 存活；`lua_checkstack` 只报告头寸（0 表示扩不动）。
      let fits = unsafe { lua_checkstack(state, n.max(1)) != 0 };
      if !fits {
        // Safety: 共用前置成立（消息当场被 `lua_pushlstring` 拷贝，不寄存指针）。
        unsafe { raise_lua_error(state, "too many results to return to Lua") }
      }
      for v in &results {
        // `push_value` 是带契约的 safe 门面；n 层头寸已由上一行 `lua_checkstack`
        // 保证，满栈时它返回 `Err` 而非越栈写。
        if let Err(e) = lua.push_value(v) {
          // Safety: 共用前置成立。
          unsafe { raise_lua_error(state, &e.to_string()) }
        }
      }
      n
    }
    Ok(Err(err)) => {
      // 6b. The closure returned Err -> raise it as a Lua error.
      //     Structured errors (scope destruction) travel as a userdata
      //     error object so the caller can pattern-match on them; all
      //     others keep the flat string path.
      if is_structured(&err) {
        // Safety: 共用前置成立；`err` 按值移入，错误 userdata 由被调方自行分配写入。
        unsafe { raise_structured_error(state, err) }
      }
      // 非结构化错误走扁平字符串路径（两个分支都是发散）
      // Safety: 共用前置成立；`err.to_string()` 在本帧存活且被当场拷贝。
      unsafe { raise_lua_error(state, &err.to_string()) }
    }
    Err(panic_payload) => {
      // 6c. The closure panicked -> turn it into a catchable Lua error.
      // Safety: 共用前置成立；格式化出的消息在本帧存活且被当场拷贝。
      unsafe { raise_lua_error(state, &panic_error_message(&*panic_payload)) }
    }
  }
}

/// Collect stack arguments `1..=nargs` into a [`MultiValue`] (stopping at the
/// first conversion error). Shared by the sync trampoline and the async
/// `get_future` closure.
///
/// `1..=nargs` 应为 `lua` 栈上的有效槽位（调用方传刚取的 `lua_gettop`）；
/// 越界索引由 `value_from_stack` 读取门面归为转换错误，不致 UB，故本函数
/// 无前置条件、是 safe fn。
pub(crate) fn collect_stack_args(lua: &Lua, nargs: c_int) -> Result<MultiValue> {
  (1..=nargs).map(|i| lua.value_from_stack(i)).collect()
}

/// Push `msg` as the error object and invoke [`lua_error`]. Diverges (unwinds
/// via the VM's longjmp-style error). Shared with `async.rs` / `interrupt.rs`.
///
/// # Safety
/// `state` 必须是存活、正由当前线程在 C 边界（trampoline/hook）内驱动的
/// `LuaState`——`lua_error` 沿 VM 的 longjmp 式展开逃逸本函数，只有站在
/// 受保护的 C 调用内部才可吸收；且栈上须有至少 1 个空位供 push 错误对象
/// （与 [`raise_structured_error`] 同一要求）。
pub(crate) unsafe fn raise_lua_error(state: *mut LuaState, msg: &str) -> ! {
  // Safety: 函数头契约（存活 state、正于 C 边界内被当前线程驱动、≥1 空位）
  // 即本块全部前提。`msg.as_ptr()/len()` 描述一段合法可读字节（空串指针也
  // 非 null），`lua_pushlstring` 把内容拷成内部 TString、不持有借用指针；
  // `lua_error` 按契约以 VM 的 longjmp 式展开逃逸到受保护边界，发散不返回。
  unsafe {
    lua_pushlstring(state, msg.as_ptr().cast::<c_char>(), msg.len());
    lua_error(state)
  }
}

/// Best-effort extraction of a panic payload's message (borrowed, no
/// allocation). 无前缀的裸消息——`typecheck.rs` 把 panic 折成合成诊断时用的就是
/// 这一形态；「panic → Lua 错误」的五处收口点请改用 [`panic_error_message`]。
///
/// 保留 `dyn Any`：形参类型即 `std::panic::catch_unwind` 的错误形态
/// （载荷类型集合运行期开放），无擦除替代。
///
/// 调用方务必写 `&*payload`（先解引用 `Box`）而非 `&payload`：`Box<dyn Any + Send>`
/// 自身也是 `'static`，因而实现了 `Any + Send`；若直接传 `&payload`，会触发 unsizing
/// 强转（`&Box<dyn Any+Send>` → `&dyn Any+Send`），得到的 trait object 具体类型是外层
/// `Box<dyn Any+Send>` 而非内部真正载荷（`&str`/`String`），令下面的 `downcast_ref`
/// 全部落空、退化成 `unknown panic`。`&*payload` 才能让 `dyn Any` 指向实际载荷。
pub(crate) fn panic_message(payload: &(dyn Any + Send)) -> Cow<'_, str> {
  if let Some(s) = payload.downcast_ref::<&'static str>() {
    Cow::Borrowed(*s)
  } else if let Some(s) = payload.downcast_ref::<String>() {
    Cow::Borrowed(s)
  } else {
    Cow::Borrowed("unknown panic")
  }
}

/// [`panic_message`] 拼上 [`PANIC_MSG_PREFIX`]：五处「panic 转 Lua 错误」收口点
/// （同步 trampoline、interrupt trampoline、async 的 get_future/poll/unpack）共用，
/// 前缀文案就此单点定义。
pub(crate) fn panic_error_message(payload: &(dyn Any + Send)) -> String {
  format!("{PANIC_MSG_PREFIX}{}", panic_message(payload))
}

/// 把任意 `Fn(&Lua, A) -> Result<R>` 具体闭包包装成 Lua [`Function`]：
/// 闭包装箱后存进 upvalue 1 的 userdata 槽位，trampoline 按 `(F, A, R)`
/// 单态化，参数经 [`FromLuaMulti`] 解包、返回值经 [`IntoLuaMulti`] 打包。
/// [`Lua::create_function`](crate::Lua::create_function) 与 userdata 方法
///  machinery 共用。
///
/// `F` 只需是 `Sized` 闭包——**不要求 `'static`**：`'static` 约束由公开 API
/// （[`Lua::create_function`](crate::Lua::create_function)）自己加，而
/// [`Scope::create_function`](crate::Scope::create_function) 借此把借 `'scope`
/// 数据的闭包直接存进槽位（无任何生命周期 transmute）。
/// `FnMut` 闭包的 `RefCell` 守卫适配：包成可共享的 `Fn`，重入（回调在途时
/// 经 Lua 再次调用自身）报 [`Error::RecursiveMutCallback`] 而非借用 panic。
/// `Lua::create_function_mut` 与 `Scope::create_function_mut` 共用。
///
/// 是宏而非泛型函数：泛型版须经 RPIT 返回闭包，会强加 `A/R: 'opaque`
/// outlive 约束，收紧调用方（尤其 scope 版）的 lifetime 语义；宏在各调用点
/// 内联展开，类型即局部闭包，无额外约束。
macro_rules! wrap_mut_closure {
  ($func:expr) => {{
    let func = ::std::cell::RefCell::new($func);
    move |lua, args| {
      let mut borrow = func
        .try_borrow_mut()
        .map_err(|_| $crate::error::Error::RecursiveMutCallback)?;
      (borrow)(lua, args)
    }
  }};
}
pub(crate) use wrap_mut_closure;

/// `trampoline` 闭包的调试名：静态 NUL 结尾字节串，交给 `lua_pushcclosurek` 的
/// `*const c_char` 收口点（被闭包长期持有，`'static` 永不失效）。
const CALLBACK_NAME: &[u8] = b"ulua-rt-callback\0";

pub(crate) fn create_callback_function<F, A, R>(lua: &Lua, func: F) -> Result<Function>
where
  F: Fn(&Lua, A) -> Result<R>,
  A: FromLuaMulti,
  R: IntoLuaMulti,
{
  let state = lua.state();
  // Allocate userdata sized for the slot, with our dtor.
  // Safety: `state` 存活（lua 的 `XRc<LuaInner>`）且由当前线程驱动；
  // `callback_dtor::<F>` 与载荷 `CallbackSlot<F>` 同一 `F` 单态化。返回 `None`
  // 即分配失败（`lua_newuserdatadtor` 的 null 经 `NonNull::new` 归一），转错误返回。
  let slot = unsafe { alloc_userdata_slot::<CallbackSlot<F>>(state, callback_dtor::<F>) };
  let Some(slot) = slot else {
    return Err(Error::runtime(
      "ulua-rt: failed to allocate callback userdata",
    ));
  };
  // Safety: `slot` 非空由上一行闸门保证；载荷恰 `size_of::<CallbackSlot<F>>()`
  // （niche 薄盒指针，对齐 ≤8 由 `alloc_userdata_slot` 的 `AlignOk` 编译期断言把住）
  // 且是刚分配、未初始化的分配，`write` 即首次初始化（do NOT run its drop here）。
  // 盒的 Drop 责任移交同 `F` 单态化配对的 `callback_dtor::<F>`，不双放不泄漏。
  unsafe { slot.write(Some(Box::new(func))) };
  // The userdata is now on top of the stack; capture it as upvalue 1 of
  // the trampoline closure.
  // Safety: `state` 存活；`lua_pushcclosurek` 的 `nup=1` 消费栈顶刚压的 userdata
  // （其上的确有一层，满足元素数断言）；绑定的 `trampoline::<F, A, R>` 与载荷布局
  // 同一 `(F, A, R)` 单态化；debugname `CALLBACK_NAME` 是被闭包长期持有的
  // `'static` 静态 NUL 结尾字节串；`cont=None` 合法。
  unsafe {
    lua_pushcclosurek(
      state,
      Some(trampoline::<F, A, R>),
      CALLBACK_NAME.as_ptr().cast(),
      1, // nup: consumes the userdata above as upvalue 1
      None,
    );
  }
  // The closure is now on top; take a registry ref. `pop_ref`（safe fn）弹走闭包并
  // 登记注册表引用，净栈变化为零。
  Ok(Function::from_ref(lua.pop_ref()))
}

/// Neutralise a scope-created callback: `take()` the boxed closure out of the
/// function's upvalue-1 userdata slot, dropping it (and thereby ending any
/// borrows it held). The slot is left `None`; the Lua function object itself is
/// left fully valid — only its behavior changes to "destructed" — so a
/// post-scope call from Lua hits [`Error::CallbackDestructed`] instead of
/// touching freed memory.
///
/// This is the **invalidation half** of `Lua::scope`'s soundness guarantee: the
/// original closure (which may borrow non-`'static` data) is dropped here, on
/// scope exit, *before* the borrowed data's lifetime can end.
///
/// `F` must be the exact closure type the function was created with — the
/// scope registers this via the same `(F, A, R)` instantiation it used to
/// build the function, so the slot layout always matches.
///
/// Must be called while the scope (and hence the VM) is still alive.
pub(crate) fn destruct_callback<F>(func: &Function) {
  let lua = func.lua();
  let state = lua.state();
  // `func.push_to_stack()` 是 safe 封装：`reference.push` 落 VM 侧 `lua_rawgeti` 自带
  // 栈预留，注册表 id 在 unref 前指实槽位（`state` 经 func 句柄链的 `XRc<LuaInner>` 保活）。
  func.push_to_stack();
  // Safety: 上一行把本函数压在栈顶，-1 指它；`lua_getupvalue(state, -1, 1)` 只把
  // upvalue **值**再压一层（name 走返回值），返回 null 即无该 upvalue 且不压栈。
  let has_upvalue = unsafe { !lua_getupvalue(state, -1, 1).is_null() };
  if !has_upvalue {
    // No upvalue (should not happen for our callbacks); just pop the fn.
    // Safety: 栈顶即 push 压入的函数值，`state` 存活且 top>base。
    unsafe { lua_pop(state, 1) };
    return;
  }
  // stack: [func, upvalue-userdata]
  // Safety: 栈顶是刚压入的 upvalue 值；`lua_touserdata` 对非 userdata 返回 null，
  // 由 `NonNull::new` 归一为 `None`。非空时它正是 `create_callback_function` 写入
  // 的回调 userdata，`cast::<CallbackSlot<F>>().as_mut()` 与载荷同一 `F` 单态化
  // 布局；无并存可写别名的前提——destruct 只在 scope 退出、本函数回调帧不在途时
  // 于同线程运行，trampoline 的 `&slot` 借用只存在于其调用栈内。
  let slot: Option<&mut CallbackSlot<F>> = unsafe {
    lua_touserdata(state, -1).map(|ud| NonNull::from(ud).cast::<CallbackSlot<F>>().as_mut())
  };
  if let Some(slot) = slot {
    // Empty the slot; the taken box is dropped here, running Drop on the
    // original closure's captures. `Option::take` 与盒的 drop 都是纯 Rust 操作。
    drop(slot.take());
  }
  // Safety: 弹回 push 与 getupvalue 各压入的一层，净栈变化为零；`state` 存活。
  unsafe { lua_pop(state, 2) };
}
