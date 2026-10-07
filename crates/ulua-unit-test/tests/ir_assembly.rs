extern crate alloc;

// Source: `tests/IrAssembly.test.cpp`
#[test]
fn ir_assembly_dse_hint_materializes_int_into_dead_vm_reg() {
  use ulua_code_gen::{
    enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
    functions::update_use_counts::update_use_counts,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::ir_assembly_fixture::IrAssemblyFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_codegen_dse_restore_hints =
    ScopedFastFlag::new(&fflag::LuauCodegenDseRestoreHints, true);

  let mut fixture = IrAssemblyFixture::new();
  let entry = fixture.build.block(IrBlockKind::Internal);
  fixture.build.begin_block(entry);

  let r1 = fixture.build.vm_reg(1);
  let d = fixture.build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
  let i = fixture.build.inst_ir_cmd_ir_op(IrCmd::NumToInt, d);

  let doubled = fixture.build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, d, d);
  let r1 = fixture.build.vm_reg(1);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, doubled);
  let r1 = fixture.build.vm_reg(1);
  let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);

  let roundtrip = fixture.build.inst_ir_cmd_ir_op(IrCmd::IntToNum, i);
  let r4 = fixture.build.vm_reg(4);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r4, roundtrip);
  let r4 = fixture.build.vm_reg(4);
  let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r4, tnumber);

  let pcpos = fixture.build.const_uint(0);
  fixture.build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);

  let r2 = fixture.build.vm_reg(2);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, roundtrip);
  let r2 = fixture.build.vm_reg(2);
  let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);

  let r1 = fixture.build.vm_reg(1);
  let count = fixture.build.const_int(2);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, count);
  update_use_counts(&mut fixture.build.function);

  let expected = r#"
; align 32 using ud2
bb_0:
.L11:
  %0 = LOAD_DOUBLE R1
 vmovsd      xmm0,qword ptr [r14+010h]
  %1 = NUM_TO_INT %0
 vcvttsd2si  eax,xmm0
  %2 = ADD_NUM %0, %0
 vaddsd      xmm0,xmm0,xmm0
  STORE_DOUBLE R1, %2
 vmovsd      qword ptr [r14+010h],xmm0
  STORE_TAG R1, tnumber
 mov         dword ptr [r14+01Ch],3
  %5 = INT_TO_NUM %1
 vcvtsi2sd   xmm0,xmm0,eax
  INTERRUPT 0u
 vmovsd      qword ptr [r14+040h],xmm0
 mov         dword ptr [r14+04Ch],0
 mov         rax,qword ptr [r15+<offset>]
 cmp         qword ptr [rax+<offset>],0
 jne         .L12
.L13:
  STORE_DOUBLE R2, %5
 vmovsd      xmm0,qword ptr [r14+040h]
 vmovsd      qword ptr [r14+020h],xmm0
  STORE_TAG R2, tnumber
 mov         dword ptr [r14+02Ch],3
  RETURN R1, 2i
 lea         rdi,[r14-010h]
 vmovups     xmm0,xmmword ptr [r14+010h]
 vmovups     xmmword ptr [rdi],xmm0
 vmovups     xmm0,xmmword ptr [r14+020h]
 vmovups     xmmword ptr [rdi+010h],xmm0
 add         rdi,20h
 mov         ecx,2
 jmp         .L7

"#;

  assert_eq!(expected, format!("\n{}", fixture.lower()));
}

// 缺口（cpp-gaps-r4 探针挂账，对照 `tests/IrAssembly.test.cpp:342-423` /
// `:507-580`）：`DseHintCorruptsTagOnPartialValueKill` 与
// `DseHintUpdateRedirectsLazyRestoreToLaterReg` 两例不可绿，本端口
// remove_dead_store 的 DSE restore-hint 未达上游修复语义：
// 1. corrupts-tag：R1 被部分值杀死（STORE_DOUBLE 后跟将被 DSE 删除的冗余
//    STORE_TAG）后，hint 位置须失效、INTERRUPT 只能溢出到栈。cpp 期望
//    `vmovsd qword ptr [rsp+048h],xmm0`（惰性恢复自 [rsp+048h] 读回），
//    本端口实测仍把 R1 当 hint 目标：`vmovsd qword ptr [r14+010h],xmm0`
//    且随附 `mov dword ptr [r14+01Ch],0`（正是上游钉住的 tag 腐蚀形态）。
// 2. update-redirect：`LuauCodegenDseRestoreHintUpdate` 旗标已在位
//    （ulua-common fflag.rs:123）且 `remove_dead_store_state.rs` 有分支，
//    但后续死 store 的 hint 目标更新（R4→R5 重定向）未落地：cpp 期望
//    INTERRUPT 溢至 R5（[r14+050h]，tag 0），实测停在 R4（[r14+040h]）。
// 两例均属产品侧（ulua-code-gen）待修复缺口，测试面不引入错误期望；
// 修复落地后按 cpp 逐字期望补回并撤本段挂账。

// Source: `tests/IrAssembly.test.cpp:425-506`
#[test]
fn ir_assembly_multi_num_to_x_shared_source_strands_restore() {
  use ulua_code_gen::{
    enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
    functions::update_use_counts::update_use_counts,
  };
  use ulua_unit_test::records::ir_assembly_fixture::IrAssemblyFixture;

  let mut fixture = IrAssemblyFixture::new();
  let entry = fixture.build.block(IrBlockKind::Internal);
  fixture.build.begin_block(entry);

  let r1 = fixture.build.vm_reg(1);
  let d = fixture.build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
  let i = fixture.build.inst_ir_cmd_ir_op(IrCmd::NumToInt, d);
  let u = fixture.build.inst_ir_cmd_ir_op(IrCmd::NumToUint, d);

  let doubled = fixture.build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, d, d);
  let r1 = fixture.build.vm_reg(1);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, doubled);
  let r1 = fixture.build.vm_reg(1);
  let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);

  let pcpos = fixture.build.const_uint(0);
  fixture.build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);

  let int_to_num = fixture.build.inst_ir_cmd_ir_op(IrCmd::IntToNum, i);
  let r2 = fixture.build.vm_reg(2);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, int_to_num);
  let r2 = fixture.build.vm_reg(2);
  let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);

  let uint_to_num = fixture.build.inst_ir_cmd_ir_op(IrCmd::UintToNum, u);
  let r3 = fixture.build.vm_reg(3);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r3, uint_to_num);
  let r3 = fixture.build.vm_reg(3);
  let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r3, tnumber);

  let r1 = fixture.build.vm_reg(1);
  let count = fixture.build.const_int(3);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, count);
  update_use_counts(&mut fixture.build.function);

  let expected = r#"
; align 32 using ud2
bb_0:
.L11:
  %0 = LOAD_DOUBLE R1
 vmovsd      xmm0,qword ptr [r14+010h]
  %1 = NUM_TO_INT %0
 vcvttsd2si  eax,xmm0
  %2 = NUM_TO_UINT %0
 vcvttsd2si  rdx,xmm0
  %3 = ADD_NUM %0, %0
 vaddsd      xmm0,xmm0,xmm0
  STORE_DOUBLE R1, %3
 vmovsd      qword ptr [r14+010h],xmm0
  STORE_TAG R1, tnumber
 mov         dword ptr [r14+01Ch],3
  INTERRUPT 0u
 mov         dword ptr [rsp+048h],eax
 mov         dword ptr [rsp+04Ch],edx
 mov         rax,qword ptr [r15+<offset>]
 cmp         qword ptr [rax+<offset>],0
 jne         .L12
.L13:
  %7 = INT_TO_NUM %1
 mov         eax,dword ptr [rsp+048h]
 vcvtsi2sd   xmm0,xmm0,eax
  STORE_DOUBLE R2, %7
 vmovsd      qword ptr [r14+020h],xmm0
  STORE_TAG R2, tnumber
 mov         dword ptr [r14+02Ch],3
  %10 = UINT_TO_NUM %2
 mov         edx,dword ptr [rsp+04Ch]
 mov         eax,edx
 vcvtsi2sd   xmm0,xmm0,rax
  STORE_DOUBLE R3, %10
 vmovsd      qword ptr [r14+030h],xmm0
  STORE_TAG R3, tnumber
 mov         dword ptr [r14+03Ch],3
  RETURN R1, 3i
 lea         rdi,[r14-010h]
 vmovups     xmm0,xmmword ptr [r14+010h]
 vmovups     xmmword ptr [rdi],xmm0
 vmovups     xmm0,xmmword ptr [r14+020h]
 vmovups     xmmword ptr [rdi+010h],xmm0
 vmovups     xmm0,xmmword ptr [r14+030h]
 vmovups     xmmword ptr [rdi+020h],xmm0
 add         rdi,30h
 mov         ecx,3
 jmp         .L7

"#;

  assert_eq!(expected, format!("\n{}", fixture.lower()));
}

// Source: `tests/IrAssembly.test.cpp`
#[test]
fn ir_assembly_preserve_int_chained_from_double_vm_reg() {
  use ulua_code_gen::{
    enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
    functions::update_use_counts::update_use_counts,
  };
  use ulua_unit_test::records::ir_assembly_fixture::IrAssemblyFixture;

  let mut fixture = IrAssemblyFixture::new();
  let entry = fixture.build.block(IrBlockKind::Internal);

  fixture.build.begin_block(entry);
  let r1 = fixture.build.vm_reg(1);
  let d = fixture.build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
  let i = fixture.build.inst_ir_cmd_ir_op(IrCmd::NumToInt, d);
  let pcpos = fixture.build.const_uint(0);
  fixture.build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);
  let r0 = fixture.build.vm_reg(0);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, i);
  let r0 = fixture.build.vm_reg(0);
  let tboolean = fixture.build.const_tag(IrAssemblyFixture::TBOOLEAN);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tboolean);
  let r0 = fixture.build.vm_reg(0);
  let count = fixture.build.const_int(1);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, count);
  update_use_counts(&mut fixture.build.function);

  let expected = r#"
; align 32 using ud2
bb_0:
.L11:
  %0 = LOAD_DOUBLE R1
 vmovsd      xmm0,qword ptr [r14+010h]
  %1 = NUM_TO_INT %0
 vcvttsd2si  eax,xmm0
  INTERRUPT 0u
 mov         rax,qword ptr [r15+<offset>]
 cmp         qword ptr [rax+<offset>],0
 jne         .L12
.L13:
  STORE_INT R0, %1
 vcvttsd2si  eax,qword ptr [r14+010h]
 mov         dword ptr [r14],eax
  STORE_TAG R0, tboolean
 mov         dword ptr [r14+0Ch],1
  RETURN R0, 1i
 vmovups     xmm0,xmmword ptr [r14]
 vmovups     xmmword ptr [r14-010h],xmm0
 mov         rdi,r14
 mov         ecx,1
 jmp         .L7

"#;

  assert_eq!(expected, format!("\n{}", fixture.lower()));
}

// Source: `tests/IrAssembly.test.cpp`
#[test]
fn ir_assembly_preserve_int_chained_from_double_vm_reg_both() {
  use ulua_code_gen::{
    enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
    functions::update_use_counts::update_use_counts,
  };
  use ulua_unit_test::records::ir_assembly_fixture::IrAssemblyFixture;

  let mut fixture = IrAssemblyFixture::new();
  let entry = fixture.build.block(IrBlockKind::Internal);

  fixture.build.begin_block(entry);
  let r2 = fixture.build.vm_reg(2);
  let d = fixture.build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
  let i = fixture.build.inst_ir_cmd_ir_op(IrCmd::NumToInt, d);
  let pcpos = fixture.build.const_uint(0);
  fixture.build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);
  let r0 = fixture.build.vm_reg(0);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, i);
  let r0 = fixture.build.vm_reg(0);
  let tboolean = fixture.build.const_tag(IrAssemblyFixture::TBOOLEAN);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tboolean);
  let r1 = fixture.build.vm_reg(1);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, d);
  let r1 = fixture.build.vm_reg(1);
  let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
  let r0 = fixture.build.vm_reg(0);
  let count = fixture.build.const_int(2);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, count);
  update_use_counts(&mut fixture.build.function);

  let expected = r#"
; align 32 using ud2
bb_0:
.L11:
  %0 = LOAD_DOUBLE R2
 vmovsd      xmm0,qword ptr [r14+020h]
  %1 = NUM_TO_INT %0
 vcvttsd2si  eax,xmm0
  INTERRUPT 0u
 mov         rax,qword ptr [r15+<offset>]
 cmp         qword ptr [rax+<offset>],0
 jne         .L12
.L13:
  STORE_INT R0, %1
 vcvttsd2si  eax,qword ptr [r14+020h]
 mov         dword ptr [r14],eax
  STORE_TAG R0, tboolean
 mov         dword ptr [r14+0Ch],1
  STORE_DOUBLE R1, %0
 vmovsd      xmm0,qword ptr [r14+020h]
 vmovsd      qword ptr [r14+010h],xmm0
  STORE_TAG R1, tnumber
 mov         dword ptr [r14+01Ch],3
  RETURN R0, 2i
 lea         rdi,[r14-010h]
 vmovups     xmm0,xmmword ptr [r14]
 vmovups     xmmword ptr [rdi],xmm0
 vmovups     xmm0,xmmword ptr [r14+010h]
 vmovups     xmmword ptr [rdi+010h],xmm0
 add         rdi,20h
 mov         ecx,2
 jmp         .L7

"#;

  assert_eq!(expected, format!("\n{}", fixture.lower()));
}

// Source: `tests/IrAssembly.test.cpp`
#[test]
fn ir_assembly_preserve_int_without_chain_spills_to_stack() {
  use ulua_code_gen::{
    enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
    functions::update_use_counts::update_use_counts,
  };
  use ulua_unit_test::records::ir_assembly_fixture::IrAssemblyFixture;

  let mut fixture = IrAssemblyFixture::new();
  let entry = fixture.build.block(IrBlockKind::Internal);

  fixture.build.begin_block(entry);
  let r1 = fixture.build.vm_reg(1);
  let a = fixture.build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
  let r2 = fixture.build.vm_reg(2);
  let b = fixture.build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
  let sum = fixture.build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a, b);
  let i = fixture.build.inst_ir_cmd_ir_op(IrCmd::NumToInt, sum);
  let pcpos = fixture.build.const_uint(0);
  fixture.build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);
  let r3 = fixture.build.vm_reg(3);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r3, i);
  let r0 = fixture.build.vm_reg(0);
  let count = fixture.build.const_int(0);
  fixture
    .build
    .inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, count);
  update_use_counts(&mut fixture.build.function);

  let expected = r#"
; align 32 using ud2
bb_0:
.L11:
  %0 = LOAD_DOUBLE R1
 vmovsd      xmm0,qword ptr [r14+010h]
  %2 = ADD_NUM %0, R2
 vaddsd      xmm0,xmm0,qword ptr [r14+020h]
  %3 = NUM_TO_INT %2
 vcvttsd2si  eax,xmm0
  INTERRUPT 0u
 mov         dword ptr [rsp+048h],eax
 mov         rax,qword ptr [r15+<offset>]
 cmp         qword ptr [rax+<offset>],0
 jne         .L12
.L13:
  STORE_INT R3, %3
 mov         eax,dword ptr [rsp+048h]
 mov         dword ptr [r14+030h],eax
  RETURN R0, 0i
 lea         rdi,[r14-010h]
 xor         ecx,ecx
 jmp         .L7

"#;

  assert_eq!(expected, format!("\n{}", fixture.lower()));
}
