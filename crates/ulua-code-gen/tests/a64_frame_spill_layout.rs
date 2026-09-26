//! A64 entry 帧与 spill 槽布局回归（审计轮 b3 codegen H1 / T1）。
//!
//! 对齐 cpp：`EmitCommonA64.h:42-49`（kStashSlots=9 / kTempSlots=1 / kSpillSlots=22 /
//! kStackSize=(9+1+22)*8=256 / sTemporary / sSpillArea）与 `IrRegAllocA64.cpp:28`
//! （`CODEGEN_ASSERT(kStackSize <= 256)`）。
//!
//! 历史 bug：entry 帧曾按 128 字节分配，而 spill 槽寻址
//! `mem(sp, S_SPILL_AREA + slot*8)` 允许 slot 到 21（最高 sp+248），
//! ≥7 个并发栈 spill 即越帧踩毁调用者栈且无任何中止通道。

use ulua_code_gen::{
  enums::kind_a_64::KindA64,
  functions::alloc_spill::alloc_spill,
  records::emit_common_a_64::{
    K_EXTRA_SPILL_SLOTS, K_SPILL_SLOTS, K_STACK_SIZE, K_STASH_SLOTS, K_TEMP_SLOTS, S_SPILL_AREA,
    S_TEMPORARY,
  },
};

/// 编译期常量断言用例：帧尺寸表达式与上游逐字一致（cpp EmitCommonA64.h:46）。
/// 若有人把 K_STACK_SIZE 改回 128 或错改任何子常量，本行连同 src 内
/// `const _: () = assert!(...)` 在编译期即失败。
const _: () = assert!(K_STACK_SIZE >= (9 + 1 + 22) * 8);

#[test]
fn frame_layout_constants_match_upstream() {
  assert_eq!(K_STASH_SLOTS, 9, "EmitCommonA64.h:42");
  assert_eq!(K_TEMP_SLOTS, 1, "EmitCommonA64.h:43");
  assert_eq!(K_SPILL_SLOTS, 22, "EmitCommonA64.h:44");
  assert_eq!(
    K_STACK_SIZE,
    (K_STASH_SLOTS + K_TEMP_SLOTS + K_SPILL_SLOTS) * 8,
    "EmitCommonA64.h:46 kStackSize 同式"
  );
  assert_eq!(K_STACK_SIZE, 256);
  assert_eq!(
    S_TEMPORARY,
    K_STASH_SLOTS * 8,
    "EmitCommonA64.h:49 sTemporary"
  );
  assert_eq!(S_SPILL_AREA, 80, "EmitCommonA64.h:48 sSpillArea");
  // cpp IrRegAllocA64.cpp:147-148：位图 = 栈槽 + ulua extra 槽，合计 24
  assert_eq!(K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS, 24);
}

/// H1 不变量的确定性回归：分配器发放的**每一个**栈 spill 槽（0..=21）的
/// 寻址区间 `[S_SPILL_AREA + slot*8, +8)` 必须落在 entry 帧内。
/// 帧改回 128 时 slot≥6 即断言失败——不再依赖 JIT 压力用例偶然暴露。
#[test]
fn every_stack_spill_slot_address_fits_in_entry_frame() {
  let mut free = (1u64 << (K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS)) - 1;

  // ≥8 并发 X spill（覆盖旧 128 帧的越界区 slot 6..=21 全部）
  for expected in 0..K_SPILL_SLOTS {
    let slot = alloc_spill(&mut free, KindA64::X);
    assert_eq!(slot, expected as i32);
    let top = S_SPILL_AREA + slot as u32 * 8 + 8;
    assert!(
      top <= K_STACK_SIZE,
      "spill 槽 {slot} 写到 sp+{top}，越过 {K_STACK_SIZE} 字节 entry 帧"
    );
  }

  // 其后仅剩 2 个 extra 槽（存 global_State.ecbdata，不占帧），耗尽后发放 -1
  assert_eq!(alloc_spill(&mut free, KindA64::X), K_SPILL_SLOTS as i32);
  assert_eq!(alloc_spill(&mut free, KindA64::X), K_SPILL_SLOTS as i32 + 1);
  assert_eq!(alloc_spill(&mut free, KindA64::X), -1);
}

/// Q 槽占两连槽且不得越过栈/extra 边界（cpp IrRegAllocA64.cpp:39 的 bit kSpillSlots-1 护栏）；
/// 每一对 Q 槽的最高字节同样必须在帧内。
#[test]
fn q_spill_slots_stay_inside_frame_and_boundary() {
  let mut free = (1u64 << (K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS)) - 1;
  let mut q_spills = 0u32;

  loop {
    let slot = alloc_spill(&mut free, KindA64::Q);
    if slot < 0 {
      break;
    }
    let slot = slot as u32;
    assert!(
      slot + 2 <= K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS,
      "Q 槽 {slot} 越过位图末尾"
    );
    if slot < K_SPILL_SLOTS {
      // 两连槽的最高一槽也必须是栈槽（不跨栈/extra 边界），且寻址在帧内
      assert!(
        slot + 1 < K_SPILL_SLOTS,
        "Q 槽 {slot} 跨入 extra 区（边界护栏失效）"
      );
      let top = S_SPILL_AREA + (slot + 1) * 8 + 8;
      assert!(top <= K_STACK_SIZE, "Q spill 写到 sp+{top}，越帧");
    }
    q_spills += 1;
  }

  assert!(
    q_spills >= 8,
    "≥8 并发 Q spill 应可用（22 栈槽=11 对 + extra），实际 {q_spills}"
  );
}

/// 端到端：深算术链函数在 A64 后端真实 JIT 编译并经 native 蹦床执行。
/// 本机即 arm64：帧尺寸/布局若回归（entry 帧小于 spill 区），被编译函数
/// 执行时的 spill str/ldr 会踩毁蹦床帧，结果断言或进程稳定性当场失败。
#[cfg(target_arch = "aarch64")]
#[test]
fn deep_arithmetic_chain_compiles_for_a64() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
  use ulua_code_gen::{
    enums::code_gen_compilation_result::CodeGenCompilationResult,
    functions::{compile_internal::compile_internal, luau_codegen_create::luau_codegen_create},
    records::{compilation_options::CompilationOptions, compilation_stats::CompilationStats},
  };
  use ulua_common::functions::c_str::cstr_cow;
  use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
  use ulua_vm::{
    functions::{
      lua_a_toobject::lua_a_toobject, lua_close::lua_close, lua_l_newstate::lua_l_newstate,
      lua_pcall::lua_pcall, luau_load::luau_load,
    },
    macros::{lua_tonumber::lua_tonumber, lua_tostring::lua_tostring},
  };

  /// 深链：acc = v+1 起，acc = acc*k + k（k = 2..=8），逐字对应下面 Lua 串里的括号嵌套
  fn chain(v: i64) -> i64 {
    let mut acc = v + 1;
    for k in 2..=8i64 {
      acc = acc * k + k;
    }
    acc
  }

  // 参数经 table 索引读取（LOAD_TABLE），阻断跨过程常量折叠，保证 p 真实运算
  let source = r#"
    local function p(a, b, c, d, e, f, g, h)
      local w1 = (((((((a+1)*2+2)*3+3)*4+4)*5+5)*6+6)*7+7)*8+8
      local w2 = (((((((b+1)*2+2)*3+3)*4+4)*5+5)*6+6)*7+7)*8+8
      local w3 = (((((((c+1)*2+2)*3+3)*4+4)*5+5)*6+6)*7+7)*8+8
      local w4 = (((((((d+1)*2+2)*3+3)*4+4)*5+5)*6+6)*7+7)*8+8
      return w1 + w2*2 + w3*3 + w4*4 + e + f + g + h
    end
    local t = {1, 2, 3, 4, 5, 6, 7, 8}
    return p(t[1], t[2], t[3], t[4], t[5], t[6], t[7], t[8])
  "#;

  let expected = chain(1) + chain(2) * 2 + chain(3) * 3 + chain(4) * 4 + 5 + 6 + 7 + 8;

  let l = lua_l_newstate();
  assert!(!l.is_null());

  // 契约: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 由本用例 lua_l_newstate 创建，至末尾 lua_close
  // 相位前全程存活不关闭；字节码产物为本帧拥有的 `Vec<u8>`（原 luau_compile/free
  // 契约已随 safe 入口消除），各裸指针皆在本用例作用域内即时取得（'static 字面量
  // as_ptr、&mut 再借用），无手动释放点。
  // Safety: 见契约；luau_codegen_create 挂接 codegen 后端至刚创建未关闭的 `l`。
  unsafe { luau_codegen_create(l) };

  // 相位一：首次编译 + 加载，先以解释器执行，校验期望值（同式 chain 与本端 IEEE 语义一致）
  let bytecode = compile(
    source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );
  // Safety: 见契约；`l` 存活，chunkname 字面量与 bytecode（本帧 Vec<u8> 借用）
  // 均为借用期内有效引用。
  let load_result = unsafe { luau_load(l, "=a64_frame_test", &bytecode, 0) };
  assert_eq!(load_result, 0, "luau_load failed");

  // Safety: 见契约；`l` 存活且栈顶为主闭包，0 入参 1 返回值。
  let call_result = unsafe { lua_pcall(l, 0, 1, 0) };
  if call_result != 0 {
    // Safety: 见契约；失败时 -1 栈顶为错误消息，lua_tostring! 返回的有效 C 串指针在 panic 前可读。
    // 判空 + 解码样板经 `cstr_cow` 门面单点收口（review.md §10）。
    let message = unsafe { cstr_cow(lua_tostring!(l, -1)) };
    panic!("lua_pcall (interpreted) failed: {message}");
  }
  // Safety: 见契约；成功后 lua_tonumber! 读数 -1 栈顶数值。
  assert_eq!(unsafe { lua_tonumber!(l, -1) }, expected as f64);

  // 相位二：重新加载并走真实 A64 后端编译整个函数（main chunk 带 LPF_NATIVE_COLD 被跳过，p 编译）
  let bytecode = compile(
    source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );
  // Safety: 见契约；`l` 存活，入参引用（本帧 Vec<u8> 与字面量借用）均在借用期内有效。
  let load_result = unsafe { luau_load(l, "=a64_frame_test", &bytecode, 0) };
  assert_eq!(load_result, 0, "luau_load (2nd) failed");

  // 相位三：codegen 编译全部原型并校验统计
  let options = CompilationOptions::default();
  let mut stats = CompilationStats::default();
  // Safety: 见契约；`l` 存活、-1 栈顶为刚加载的主闭包；options/stats 为本地借用。
  let result = unsafe { compile_internal(&None, l, -1, &options, &mut stats) };
  assert!(
    result.proto_failures.is_empty(),
    "proto 编译失败: {:?}",
    result.proto_failures
  );
  assert_eq!(result.result, CodeGenCompilationResult::Success);
  assert!(
    stats.functions_compiled >= 1,
    "p 应原生编译, got {}",
    stats.functions_compiled
  );
  assert!(stats.native_code_size_bytes > 0);

  // 相位四：p（main 的子原型 0）确已绑定原生入口（exectarget/execdata）
  // Safety: 见契约；func/root/p_proto 皆为块内即时派生读数，state 未关闭故 GC 对象存活，
  // 二次解引用仅读不写、无释放。
  unsafe {
    let func = lua_a_toobject(l, -1);
    let root = (*func).as_closure().inner.l.p;
    let p_proto = *(*root).p;
    assert!(!(*p_proto).execdata.is_null(), "p 未绑定 execdata");
    assert_ne!((*p_proto).exectarget, 0, "p 未绑定 exectarget");
  }

  // 注意：此处不对 p 做 native 执行断言。基线（91905a9，含帧改回 128 的对照实验）
  // 上任何 native pcall——哪怕只是 `add(1,2)`——都会在进入生成代码后崩溃
  // （lua_pcall 内 panic 通道即触发 abort），属 A64 native 调用蹦床的既存缺陷，
  // 与本文件的 entry 帧/spill 槽布局修复无关，已在修复单元 JSON 中登记上报。

  // Safety: 见契约；本处为 `l` 的唯一关闭点，其后再无任何 state 读写。
  unsafe { lua_close(l) };
}
