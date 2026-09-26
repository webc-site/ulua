// C API 直调用例（上游 TEST_CASE("Api*")）
// 移植自 `cpp/tests/Conformance.test.cpp`。
//
// 已知缺件（对照 cpp Conformance.test.cpp）：`ApiEncode`（:3183，依赖
// `lua_setpointerencodekey`，Rust VM 未实现）与 `BuffersWithCage`（:1295，依赖
// `LuauBufferCage`，本端口无 cage 形态）未移植。
//
// `lua_*` C ABI 调用统一经 [`safe_api`] 门面收口（每次调用内部一次性 `unsafe`，
// 契约见门面模块文档），用例侧只做 safe 调用；仅剩的裸 `unsafe` 是对 VM 自有
// 缓冲的直接写（buffer 填充），其 `# Safety` 契约就地标注。

use std::ptr::fn_addr_eq;

use crate::common::functions::safe_api::*;

#[test]
fn conformance_api_alloc() {
  use core::ffi::c_void;

  use crate::common::{functions::limited_realloc::limited_realloc, records::state_ref::StateRef};

  let mut ud = 0;
  let global_state = StateRef::new(newstate(
    Some(limited_realloc),
    (&mut ud as *mut i32).cast(),
  ))
  .expect("lua state allocation failed");
  let l = global_state.as_ptr();

  let (allocf, ud_check) = getallocf(l);
  let expected =
    limited_realloc as unsafe extern "C-unwind" fn(*mut c_void, *mut u8, usize, usize) -> *mut u8;

  assert!(matches!(
      allocf,
      Some(f) if fn_addr_eq(f, expected)
  ));
  assert_eq!(ud_check, (&mut ud as *mut i32).cast());
}

#[test]
fn conformance_api_atoms() {
  use crate::common::functions::{
    conformance_api_atoms_useratom::conformance_api_atoms_useratom, new_state::new_state,
  };

  let global_state = new_state();
  let l = global_state.as_ptr();

  set_useratom(l, Some(conformance_api_atoms_useratom));

  // 按 cpp 顺序压三个串、把相邻两个 concat 成 1，再压第四个，
  // 末态栈顶三槽即 -3/-2/-1 的被读对象。
  pushstr(l, b"string\0");
  pushstr(l, b"import\0");
  pushstr(l, b"ant\0");
  concat(l, 2);
  pushstr(l, b"unimportant\0");

  let (s1, a1) = tostratom(l, -3);
  let (s2, a2) = tostratom(l, -2);
  let (s3, a3) = tostratom(l, -1);

  assert_eq!(s1, b"string");
  assert_eq!(a1, 0);

  assert_eq!(s2, b"important");
  assert_eq!(a2, 1);

  assert_eq!(s3, b"unimportant");
  assert_eq!(a3, -1);
}

#[test]
fn conformance_api_buffer() {
  use core::ptr::write_bytes;

  use ulua_vm::enums::lua_type::LuaType;

  use crate::common::functions::new_state::new_state;

  let global_state = new_state();
  let l = global_state.as_ptr();

  // 建 1000 字节缓冲后就地读回类型与长度（下列读取只读栈，不改动它）。
  newbuffer(l, 1000);

  assert_eq!(type_(l, -1), LuaType::Buffer as i32);

  assert!(isbuffer(l, -1));
  assert_eq!(objlen(l, -1), 1000);

  assert_eq!(typename_bytes(l, LuaType::Buffer as i32), b"buffer");
  assert_eq!(l_typename_bytes(l, -1), b"buffer");

  // 同一缓冲的重复取址必须返回同一指针，长度出参写进本帧局部。
  let p1 = tobuffer_ptr(l, -1);

  let mut len = 0usize;
  let p2 = tobuffer_len(l, -1, &mut len);
  assert_eq!(len, 1000);
  assert_eq!(p1, p2);

  let p3 = l_checkbuffer_ptr(l, -1);
  assert_eq!(p1, Some(p3));

  // 长度出参写进本帧局部（新建并置 0，与原 `len = 0` 复用等值）。
  let mut len = 0usize;
  let p4 = l_checkbuffer_len(l, -1, &mut len);
  assert_eq!(len, 1000);
  assert_eq!(p1, Some(p4));

  // Safety: `p1` 指向上面确认过的 1000 字节缓冲自有内存，写入长度不越界。
  unsafe { write_bytes(p1.expect("buffer data pointer").cast::<u8>(), 0xab, 1000) };

  assert!(!topointer(l, -1).is_null());

  // 再建一个 0 字节缓冲，并把其下那个（-2）复制压栈用于比较。
  newbuffer(l, 0);
  pushvalue(l, -2);

  // 栈顶三个槽位为缓冲对象；equal 只读比较，末了弹掉复制槽自平衡。
  assert_ne!(equal(l, -3, -1), 0);
  assert_eq!(equal(l, -2, -1), 0);

  pop(l, 1);
}

/// apicalls.luau 中人为截断的 `pi = 3.1415926`（非 math.pi），按位精确比较。
const APICALLS_LUAU_PI: f64 = f64::from_bits(0x4009_21fb_4d12_d84a);

#[test]
fn conformance_api_calls() {
  use core::ptr::null_mut;

  use ulua_vm::{
    enums::{lua_gc_op::LuaGcOp, lua_status::LuaStatus},
    macros::{
      lua_globalsindex::LUA_GLOBALSINDEX, lua_multret::LUA_MULTRET, luai_maxcstack::LUAI_MAXCSTACK,
    },
  };

  use crate::common::functions::{
    conformance_api_calls_check_not_yieldable::conformance_api_calls_check_not_yieldable,
    limited_realloc::limited_realloc, run_conformance::run_conformance,
  };

  let global_state = run_conformance(
    "apicalls.luau",
    None,
    None,
    // FFI: c-API 要求 NULL
    Some(newstate(Some(limited_realloc), null_mut())),
    None,
    false,
    None,
  );
  let l = global_state.as_ptr();

  // 调全局 add 读回 42，push→call→读→pop 自平衡栈。
  getfield(l, LUA_GLOBALSINDEX, b"add\0");
  pushnumber(l, 40.0);
  pushnumber(l, 2.0);
  call(l, 2, 1);
  assert_ne!(isnumber(l, -1), 0);
  assert_eq!(tonumber(l, -1), 42.0);
  pop(l, 1);

  // lua_call 以 LUA_MULTRET 返回 200 个结果，gettop 校验后全数弹回。
  getfield(l, LUA_GLOBALSINDEX, b"getnresults\0");
  pushinteger(l, 200);
  call(l, 1, LUA_MULTRET);
  assert_eq!(gettop(l), 200);
  pop(l, 200);

  // 以 pcall 走与上文 call 同形的多变故返回值路径，各步严格
  // push→pcall→读→pop 自平衡栈。
  getfield(l, LUA_GLOBALSINDEX, b"add\0");
  pushnumber(l, 40.0);
  pushnumber(l, 2.0);
  let pcall_status = pcall(l, 2, 1, 0);
  assert_eq!(pcall_status, LuaStatus::Ok as i32);
  assert_ne!(isnumber(l, -1), 0);
  assert_eq!(tonumber(l, -1), 42.0);
  pop(l, 1);

  // pcall 变体返回 LUA_MULTRET 的 200 个结果，gettop 校验后全数弹回。
  getfield(l, LUA_GLOBALSINDEX, b"getnresults\0");
  pushinteger(l, 200);
  let pcall_status = pcall(l, 1, LUA_MULTRET, 0);
  assert_eq!(pcall_status, LuaStatus::Ok as i32);
  assert_eq!(gettop(l), 200);
  pop(l, 200);

  // 调 Lua 侧 pcall 包 getnresults，MULTRET 结果多留一个布尔返回值，
  // 弹 200 后 -1 即该布尔。
  getfield(l, LUA_GLOBALSINDEX, b"pcall\0");
  getfield(l, LUA_GLOBALSINDEX, b"getnresults\0");
  pushinteger(l, 200);
  call(l, 2, LUA_MULTRET);
  assert_eq!(gettop(l), 201);
  pop(l, 200);
  assert_eq!(toboolean(l, -1), 1);
  pop(l, 1);

  // cpcall 以 `&mut should_fail` 裸指针为载荷，仅在本语句内
  // 存活并透传给被调 C 闭包 cpcall_test，不应失败。
  let mut should_fail = false;
  assert_eq!(cpcall_bool(l, &mut should_fail), LuaStatus::Ok as i32);

  // 成功路径不改写 status，仍为 Ok。
  assert_eq!(status(l), LuaStatus::Ok as i32);

  // cpcallvalue 由成功路径压入全局，读回后弹栈平衡。
  getfield(l, LUA_GLOBALSINDEX, b"cpcallvalue\0");
  assert_eq!(l_checkinteger(l, -1), 123);
  pop(l, 1);

  // 载荷改 true 后 cpcall_test 抛错，指针仅在本语句内存活。
  let mut should_fail = true;
  assert_eq!(cpcall_bool(l, &mut should_fail), LuaStatus::ErrRun as i32);

  // 变故后 -1 为错误串 "Failed"，读回即弹栈，status 复位 Ok。
  assert_ne!(isstring(l, -1), 0);
  assert_eq!(to_bytes(l, -1), b"Failed");
  pop(l, 1);

  assert_eq!(status(l), LuaStatus::Ok as i32);

  // 载荷 should_fail 为本帧局部，裸指针仅在下句调用语句内存活。
  let mut should_fail = false;
  // checkstack 预先撑足容量后，向栈压入 LUAI_MAXCSTACK-1 个数字逼出栈极限。
  assert_eq!(gettop(l), 0);
  l_checkstack(l, LUAI_MAXCSTACK - 1, "must succeed");

  for _ in 0..LUAI_MAXCSTACK - 1 {
    pushnumber(l, 1.0);
  }

  // 栈已逼满，cpcall 必以 ErrRun 失败，指针载荷仅在本语句内存活。
  assert_eq!(cpcall_bool(l, &mut should_fail), LuaStatus::ErrRun as i32);

  // -1 为栈极限错误串，读回即弹栈。
  assert_ne!(isstring(l, -1), 0);
  assert_eq!(to_bytes(l, -1), b"stack limit");
  pop(l, 1);

  // status 复位后把压入的 LUAI_MAXCSTACK-1 个数字全部弹回。
  assert_eq!(status(l), LuaStatus::Ok as i32);
  pop(l, LUAI_MAXCSTACK - 1);

  // newthread 新建主线程协程栈，由 `l` 持有且在本用例期间存活；
  // 闭包 check_not_yieldable 直接调用不报错（主状态不可 yield）。
  let l2 = newthread(l);
  pushcfunction(l2, Some(conformance_api_calls_check_not_yieldable));
  call(l2, 0, 0);

  // resume 单参调用 getnresults 后读回 1 个结果并弹栈自平衡；线程留在 `l` 上。
  getfield(l2, LUA_GLOBALSINDEX, b"getnresults\0");
  pushinteger(l2, 1);
  let resume_status = resume(l2, None, 1);
  assert_eq!(resume_status, LuaStatus::Ok as i32);
  assert_eq!(gettop(l2), 1);
  pop(l2, 1);

  // 再装一次不可 yield 闭包直接调用不报错，末句 pop(l, 1) 弹回线程。
  pushcfunction(l2, Some(conformance_api_calls_check_not_yieldable));
  call(l2, 0, 0);
  pop(l, 1);

  // 再新建一个线程栈；两度 pcall 调 create_with_tm 各造一张 42 内容的表留在栈顶。
  let l2 = newthread(l);
  getfield(l2, LUA_GLOBALSINDEX, b"create_with_tm\0");
  pushnumber(l2, 42.0);
  pcall(l2, 1, 1, 0);

  getfield(l2, LUA_GLOBALSINDEX, b"create_with_tm\0");
  pushnumber(l2, 42.0);
  pcall(l2, 1, 1, 0);

  // gc 步进回收后比较两表引用（同内容模板表相等），
  // 弹掉两表后 pop(l, 1) 弹回线程保持主栈平衡。
  gc(l2, LuaGcOp::Collect as i32, 0);
  gc(l2, LuaGcOp::Step as i32, 8);

  assert_eq!(equal(l2, -1, -2), 1);
  pop(l2, 2);

  pop(l, 1);

  // getpi 直接调用读回截断的 pi 常量后弹栈。
  getfield(l, LUA_GLOBALSINDEX, b"getpi\0");
  call(l, 0, 1);
  assert_eq!(tonumber(l, -1), APICALLS_LUAU_PI);
  pop(l, 1);

  // getpi 的 clonefunction 副本配一张含 pi=42 的新表经 setfenv 装载，
  // 各步与上游 cpp 逐句同形，句间栈态由后续语句继续消费。
  getfield(l, LUA_GLOBALSINDEX, b"getpi\0");

  clonefunction(l, -1);
  newtable(l);
  pushnumber(l, 42.0);
  setfield(l, -2, b"pi\0");
  setfenv(l, -2);

  // 先调装了 fenv 的副本得 42，再调原件仍得 pi，各读后即弹。
  call(l, 0, 1);
  assert_eq!(tonumber(l, -1), 42.0);
  pop(l, 1);

  call(l, 0, 1);
  assert_eq!(tonumber(l, -1), APICALLS_LUAU_PI);
  pop(l, 1);

  // incuv 直接调用，首个返回值读 1 后即弹。
  getfield(l, LUA_GLOBALSINDEX, b"incuv\0");
  call(l, 0, 1);
  assert_eq!(tonumber(l, -1), 1.0);
  pop(l, 1);

  // 再取 incuv 并 clonefunction 两份，三件闭包共享同一上值。
  getfield(l, LUA_GLOBALSINDEX, b"incuv\0");
  clonefunction(l, -1);
  clonefunction(l, -2);

  // 依次调用三个共享上值的闭包读回 2/3/4，各 call 后读顶即弹。
  call(l, 0, 1);
  assert_eq!(tonumber(l, -1), 2.0);
  pop(l, 1);
  call(l, 0, 1);
  assert_eq!(tonumber(l, -1), 3.0);
  pop(l, 1);
  call(l, 0, 1);
  assert_eq!(tonumber(l, -1), 4.0);
  pop(l, 1);

  // largealloc 触发分配失败，无 msgh 的 pcall 直接得 ErrMem 后弹栈。
  getfield(l, LUA_GLOBALSINDEX, b"largealloc\0");
  let res = pcall(l, 0, 0, 0);
  assert_eq!(res, LuaStatus::ErrMem as i32);
  pop(l, 1);

  // oops 与 largealloc 压栈后以 -2 作 msgh，内存耗尽时 msgh 串 "oops"
  // 成为错误值，读回后 pop(l, 2) 平衡。
  getfield(l, LUA_GLOBALSINDEX, b"oops\0");
  getfield(l, LUA_GLOBALSINDEX, b"largealloc\0");
  let res = pcall(l, 0, 1, -2);
  assert_eq!(res, LuaStatus::ErrMem as i32);
  assert_ne!(isstring(l, -1), 0);
  assert_eq!(to_bytes(l, -1), b"oops");
  pop(l, 2);

  // msgh 本身是 error 函数时变故套错，得 ErrErr。
  getfield(l, LUA_GLOBALSINDEX, b"error\0");
  getfield(l, LUA_GLOBALSINDEX, b"largealloc\0");
  let res = pcall(l, 0, 1, -2);
  assert_eq!(res, LuaStatus::ErrErr as i32);

  // 错误值仍是固定串 "error in error handling"，读回后弹栈平衡。
  assert_ne!(isstring(l, -1), 0);
  assert_eq!(to_bytes(l, -1), b"error in error handling");
  pop(l, 2);

  // largealloc 套 largealloc，第二次分配失败时以第一次的错误串
  // 作 msgh，读回 "not enough memory" 后弹栈。
  getfield(l, LUA_GLOBALSINDEX, b"largealloc\0");
  getfield(l, LUA_GLOBALSINDEX, b"largealloc\0");
  let res = pcall(l, 0, 1, -2);
  assert_eq!(res, LuaStatus::ErrMem as i32);
  assert_ne!(isstring(l, -1), 0);
  assert_eq!(to_bytes(l, -1), b"not enough memory");
  pop(l, 2);

  // largealloc 与 error 压栈，pcall 得 ErrErr。
  getfield(l, LUA_GLOBALSINDEX, b"largealloc\0");
  getfield(l, LUA_GLOBALSINDEX, b"error\0");
  let res = pcall(l, 0, 1, -2);
  assert_eq!(res, LuaStatus::ErrErr as i32);
  assert_ne!(isstring(l, -1), 0);

  // 读回固定错误串、弹栈并校验栈已归零。
  assert_eq!(to_bytes(l, -1), b"error in error handling");
  pop(l, 2);

  assert_eq!(gettop(l), 0);
}

#[test]
fn conformance_api_iter() {
  use crate::common::functions::new_state::new_state;

  let global_state = new_state();
  let l = global_state.as_ptr();

  /// 以 `rawiter` 遍历栈顶表的两两键/值并对数字求和（sum2/sum3 两处同形副本收敛）。
  ///
  /// `l` 须为存活 lua_State 且栈顶（索引 -1）为可迭代表（[`safe_api`] 契约）；
  /// 逐对读→pop 自配平，返回时栈深与进入时一致（unsafe 收口进门面后为 safe fn）。
  fn sum_table_via_rawiter(l: L) -> f64 {
    let mut sum = 0.0;
    let mut index = 0;
    loop {
      index = rawiter(l, -1, index);
      if index < 0 {
        break;
      }
      sum += tonumber(l, -2);
      sum += tonumber(l, -1);
      pop(l, 2);
    }
    sum
  }

  // 建表并塞入三个数字键值。
  newtable(l);
  pushnumber(l, 123.0);
  setfield(l, -2, b"key\0");
  pushnumber(l, 456.0);
  rawsetfield(l, -2, b"key2\0");
  pushstr(l, b"test\0");
  rawseti(l, -2, 1);

  let mut sum1 = 0.0;
  // next 游标遍历每对键/值，数字槽由 tonumber 强转
  // （非数字得 0），每对读完弹值保键在顶，遍历结束表回到栈顶。
  pushnil(l);
  while next(l, -2) != 0 {
    sum1 += tonumber(l, -2);
    sum1 += tonumber(l, -1);
    pop(l, 1);
  }
  assert_eq!(sum1, 580.0);

  // 栈顶为上方构造的表；rawiter 求和见 sum_table_via_rawiter 契约。
  let sum2 = sum_table_via_rawiter(l);
  assert_eq!(sum2, 580.0);

  // settop/pushvalue 把栈顶表复制到索引 19 处撑出深栈，校验 gettop 后继续以
  // rawiter 遍历（cpp 仅断言 gettop，深栈下 rawiter 正常迭代即 C 栈极限兼容性）。
  settop(l, 18);
  pushvalue(l, 1);

  assert_eq!(gettop(l), 19);

  // 索引 19 为复制出的同一表；rawiter 求和见 sum_table_via_rawiter 契约。
  let sum3 = sum_table_via_rawiter(l);
  assert_eq!(sum3, 580.0);

  // 段末 pop(l, 19) 复位栈。
  pop(l, 19);
}

#[test]
fn conformance_api_stack() {
  use core::{ptr::null_mut, sync::atomic::Ordering};

  use ulua_vm::{enums::lua_status::LuaStatus, macros::luai_maxcstack::LUAI_MAXCSTACK};

  use crate::common::{
    functions::{
      blockable_realloc::blockable_realloc, blockable_realloc_allowed::BLOCKABLE_REALLOC_ALLOWED,
      slowly_overflow_stack::slowly_overflow_stack,
    },
    records::state_ref::StateRef,
  };

  // FFI: c-API 要求 NULL
  let global_state = StateRef::new(newstate(Some(blockable_realloc), null_mut()))
    .expect("lua state allocation failed");
  let gl = global_state.as_ptr();

  // newthread 在 `gl` 上派生线程，线程栈由 `gl` 持有、在本用例期间存活；
  // slowly_overflow_stack 撑爆栈，pcall 得 ErrRun，错误串读顶。
  let l = newthread(gl);

  pushcclosurek(l, Some(slowly_overflow_stack), Some(b"foo\0"), 0, None);
  let result = pcall(l, 0, 0, 0);
  assert_eq!(result, LuaStatus::ErrRun as i32);
  assert_eq!(l_checkstring_bytes(l, -1), b"stack overflow (test)");

  // 再派生一个线程栈：checkstack 扩容走 blockable_realloc——关闭放行开关后
  // 1000 槽请求必须失败，100 槽在开关前成功。
  let l = newthread(gl);

  assert_eq!(checkstack(l, 100), 1);

  BLOCKABLE_REALLOC_ALLOWED.store(false, Ordering::Relaxed);
  assert_eq!(checkstack(l, 1000), 0);

  BLOCKABLE_REALLOC_ALLOWED.store(true, Ordering::Relaxed);

  // 恢复放行后 1000 槽再次成功，超过 LUAI_MAXCSTACK*2 的请求必被拒绝。
  assert_eq!(checkstack(l, 1000), 1);

  assert_eq!(checkstack(l, LUAI_MAXCSTACK * 2), 0);
}

#[test]
fn conformance_api_tables() {
  use core::ffi::c_void;

  use ulua_vm::enums::lua_type::LuaType;

  use crate::common::functions::new_state::new_state;

  let global_state = new_state();
  let l = global_state.as_ptr();
  let mut lu1 = 1;
  let mut lu2 = 2;
  let lu1p = (&mut lu1 as *mut i32).cast::<c_void>();
  let lu2p = (&mut lu2 as *mut i32).cast::<c_void>();

  // 建表并以两种键型塞入前四个字段；
  // lu1p/lu2p 为本帧局部整数的裸指针，仅作为键存储、不被解除引用。
  newtable(l);
  pushnumber(l, 123.0);
  setfield(l, -2, b"key\0");
  pushnumber(l, 456.0);
  rawsetfield(l, -2, b"key2\0");
  pushstr(l, b"key3\0");
  rawseti(l, -2, 5);

  // 续塞指针键与两个 tagged 指针键字段，表留在栈顶。
  pushstr(l, b"key4\0");
  rawsetp(l, -2, lu1p);
  pushstr(l, b"key5\0");
  rawsetptagged(l, -2, lu2p, 1);
  pushstr(l, b"key6\0");
  rawsetptagged(l, -2, lu2p, 2);

  // 以 gettable/getfield 读取器取数值字段，均 push 键→读→pop 单值自平衡。
  pushstr(l, b"key\0");
  assert_eq!(gettable(l, -2), LuaType::Number as i32);
  assert_eq!(tonumber(l, -1), 123.0);
  pop(l, 1);

  assert_eq!(getfield(l, -1, b"key\0"), LuaType::Number as i32);
  assert_eq!(tonumber(l, -1), 123.0);
  pop(l, 1);

  // rawgetfield 直取字段读后即弹。
  assert_eq!(rawgetfield(l, -1, b"key2\0"), LuaType::Number as i32);
  assert_eq!(tonumber(l, -1), 456.0);
  pop(l, 1);

  // rawget 以栈上键读表，push→读→pop 自平衡。
  pushstr(l, b"key\0");
  assert_eq!(rawget(l, -2), LuaType::Number as i32);
  assert_eq!(tonumber(l, -1), 123.0);
  pop(l, 1);

  // rawgeti/rawgetp 取整数键与指针键字段，读回后即弹栈。
  assert_eq!(rawgeti(l, -1, 5), LuaType::String as i32);
  assert_eq!(to_bytes(l, -1), b"key3");
  pop(l, 1);

  assert_eq!(rawgetp(l, -1, lu1p), LuaType::String as i32);
  assert_eq!(to_bytes(l, -1), b"key4");
  pop(l, 1);

  // tagged 1/2 变体取回 key5/key6，读后即弹。
  assert_eq!(rawgetptagged(l, -1, lu2p, 1), LuaType::String as i32);
  assert_eq!(to_bytes(l, -1), b"key5");
  pop(l, 1);

  assert_eq!(rawgetptagged(l, -1, lu2p, 2), LuaType::String as i32);
  assert_eq!(to_bytes(l, -1), b"key6");
  pop(l, 1);

  // tag 0 与已存 1/2 不合，读回 nil 后仍弹栈保持平衡。
  assert_eq!(rawgetptagged(l, -1, lu2p, 0), LuaType::Nil as i32);
  pop(l, 1);

  // clonetable 复制栈顶表后从克隆件读回 key=123，读后即弹。
  clonetable(l, -1);

  assert_eq!(getfield(l, -1, b"key\0"), LuaType::Number as i32);
  assert_eq!(tonumber(l, -1), 123.0);
  pop(l, 1);

  // rawsetfield 就地改写克隆件的 key，写毕弹掉克隆件回到原表。
  pushnumber(l, 456.0);
  rawsetfield(l, -2, b"key\0");

  pop(l, 1);

  // 原表的 key 不受改写影响，仍读回 123。
  assert_eq!(getfield(l, -1, b"key\0"), LuaType::Number as i32);
  assert_eq!(tonumber(l, -1), 123.0);
  pop(l, 1);

  // cleartable 清空后 pushnil + next 返回 0 证实无残留，末句弹掉 nil 游标保持平衡。
  cleartable(l, -1);
  pushnil(l);
  assert_eq!(next(l, -2), 0);

  pop(l, 1);
}

#[test]
fn conformance_api_type() {
  use ulua_vm::enums::lua_type::LuaType;

  use crate::common::functions::new_state::new_state;

  let global_state = new_state();
  let l = global_state.as_ptr();

  // 压入一个数字后就地用 l_typename/type 读回 -1 与 1 两槽的类型，各调用只读栈。
  pushnumber(l, 2.0);
  assert_eq!(l_typename_bytes(l, -1), b"number");
  assert_eq!(l_typename_bytes(l, 1), b"number");
  assert_eq!(type_(l, -1), LuaType::Number as i32);
  assert_eq!(type_(l, 1), LuaType::Number as i32);

  // 索引 2 超出栈顶，typename/type 均报 no value。
  assert_eq!(l_typename_bytes(l, 2), b"no value");
  assert_eq!(type_(l, 2), LuaType::None as i32);
  assert_eq!(typename_bytes(l, type_(l, 2)), b"no value");

  // 压 0 字节 userdata 后读回其类型名。
  newuserdata(l, 0);
  assert_eq!(l_typename_bytes(l, -1), b"userdata");
  assert_eq!(type_(l, -1), LuaType::UserData as i32);

  // 建含 __type="hello" 的元表并 setmetatable 到 userdata，
  // l_typename 走元方法名而 type 仍是 userdata。
  newtable(l);
  pushstr(l, b"hello\0");
  setfield(l, -2, b"__type\0");
  setmetatable(l, -2);

  assert_eq!(l_typename_bytes(l, -1), b"hello");
  assert_eq!(type_(l, -1), LuaType::UserData as i32);
}
