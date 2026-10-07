//! SetMetatableChecked x64 lowering 寄存器分配回归测试（本 fork 扩展刀的守门）。
//!
//! CI x64 runner（cargo bench）炸点：`LUAU_ASSERT failed: user != K_INVALID_INST_IDX`
//! （ir_reg_alloc_x_64.rs take_reg GPR 臂）——free=false 且占用者索引为
//! K_INVALID_INST_IDX 的坏寄存器状态被后续 take_reg 撞上。
//!
//! x64 lowering 是纯写字节缓冲的代码生成（不执行产物），arm64 宿主即可复现：
//! 编译含 `setmetatable(t, m)`（FASTCALL2/LBF_SETMETATABLE → SetMetatableChecked
//! 内联形状）的负载，跑完整 x64 native 函数编译管线。

use core::slice::from_raw_parts;
use std::fs;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_code_gen::{
  enums::abix_64::ABIX64,
  functions::{
    assemble_helpers_x_64::assemble_helpers, create_native_function::create_native_function_x_64,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, compilation_options::CompilationOptions,
    module::ModuleHelpers,
  },
};
use ulua_common::{fflag, records::f_value::set_luau_bool_flags};
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{
  functions::{
    lua_a_toobject::lua_a_toobject, lua_close::lua_close, lua_l_newstate::lua_l_newstate,
    luau_load::luau_load,
  },
  records::proto::Proto,
};

/// 对单个 Lua 负载跑 x64 native 编译全管线（纯代码生成，不执行产物）。
fn compile_x64_abi(name: &str, src: &str, abi: ABIX64) {
  let bytecode = compile(
    src.as_bytes(),
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  unsafe {
    let l = lua_l_newstate();
    assert!(!l.is_null());
    let chunkname = format!("={name}");
    let rc = luau_load(l, &chunkname, &bytecode, 0);
    assert_eq!(rc, 0, "luau_load failed for {name}");

    let cl = (*lua_a_toobject(&*l, -1)).as_closure();
    let root_proto = cl.inner.l.p;

    let mut all_protos = vec![root_proto];
    fn collect_protos(p: *mut Proto, list: &mut Vec<*mut Proto>) {
      unsafe {
        if (*p).sizep > 0 && !(*p).p.is_null() {
          let sub_protos = from_raw_parts((*p).p, (*p).sizep as usize);
          for &sp in sub_protos {
            list.push(sp);
            collect_protos(sp, list);
          }
        }
      }
    }
    collect_protos(root_proto, &mut all_protos);

    let mut build = AssemblyBuilderX64::new_with_abi(true, abi, 0);
    let mut helpers = ModuleHelpers::default();
    let options = CompilationOptions::default();
    let mut total_inst = 0u32;

    for (i, &proto) in all_protos.iter().enumerate() {
      let res = create_native_function_x_64(
        &mut build,
        &mut helpers,
        proto,
        &mut total_inst,
        &options,
        l,
      );
      assert!(
        res.is_ok(),
        "compile proto {} failed for {name}: {:?}",
        i,
        res.err()
      );
    }

    assemble_helpers(&mut build, &mut helpers);

    lua_close(l);
  }
}

/// 双 ABI 各编一遍：Windows x64 的参数寄存器序列（RCX/RDX/R8 + 影子空间）与
/// System-V（RDI/RSI/RDX）不同，CI runner 与本机宿主可能异 ABI。
fn compile_x64(name: &str, src: &str) {
  compile_x64_abi(name, src, ABIX64::SYSTEM_V);
  compile_x64_abi(name, src, ABIX64::Windows);
}

/// 最小 FASTCALL2 形状：`setmetatable(t, m)` 双参直调，编译器折叠为
/// LBF_SETMETATABLE → IrBuilder 内联 SetMetatableChecked。
#[test]
fn test_x64_setmetatable_checked_lowering_regalloc() {
  fflag::LuauJitSetmetatableFastcall.push_test_override(true);

  compile_x64(
    "setmt_min",
    r#"
local function touch(t, m)
  local u = setmetatable(t, m)
  return u
end
return touch
"#,
  );

  fflag::LuauJitSetmetatableFastcall.pop_test_override();
}

/// bench 负载整卷兜底：全部 bench 用例过一遍 x64 管线（对齐 CI cargo bench 面）。
#[test]
fn test_x64_setmetatable_bench_cases_regalloc() {
  // 对齐 CI bench 的旗标面：rt 建州批量点亮全部 Luau* 前缀旗标（call inline 观测
  // 等同族刀会改变 IR 形状与寄存器压力，单点一个旗标复现不了 CI 现场）。
  set_luau_bool_flags(true);

  let cases_dir = fs::read_dir("../../benchmarks/cases")
    .or_else(|_| fs::read_dir("benchmarks/cases"))
    .expect("read benchmarks/cases dir");

  for entry in cases_dir {
    let path = entry.expect("valid entry").path();
    if path.extension().and_then(|s| s.to_str()) == Some("lua") {
      let src = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
      compile_x64(path.file_name().unwrap().to_str().unwrap(), &src);
    }
  }
}
