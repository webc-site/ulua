//! 移植自 `cpp/tests/IrCallWrapperX64.test.cpp`（27 例）。
//! Fixture：`ulua_unit_test::records::ir_call_wrapper_x_64_fixture`。
//! 断言文本逐字符取自 C++ `CHECK_EQ` 期望串（以 `\n` 前缀 + 汇编）。
//! 各用例原先在函数内重复同一份 `use` 清单，已收敛为文件根并集导入，
//! `mod` 经 `use super::*` 统一引入（R126 在 ir_lowering.rs 的同型收口）。

extern crate alloc;

use ulua_code_gen::{
  enums::{ir_op_kind::IrOpKind, size_x_64::SizeX64},
  records::{
    ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX,
    ir_inst::IrInst,
    ir_op::IrOp,
    operand_x_64::{ADDR, OperandX64, QWORD, XMMWORD},
    register_x_64::RegisterX64,
    scoped_reg_x_64::ScopedRegX64,
  },
};
use ulua_unit_test::records::{
  ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture,
  ir_call_wrapper_x_64_fixture_system_v::IrCallWrapperX64FixtureSystemV,
};

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_address_in_stack_arguments() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  fixture.add_arg(SizeX64::Dword, 1);
  fixture.add_arg(SizeX64::Dword, 2);
  fixture.add_arg(SizeX64::Dword, 3);
  fixture.add_arg(SizeX64::Dword, 4);
  fixture.add_arg(SizeX64::Qword, ADDR.operator_bracket(RegisterX64::R12 + 16));
  fixture.call(QWORD.operator_bracket(RegisterX64::R14.into()));

  fixture.check_match(String::from(
    r#"
 lea         rax,[r12+010h]
 mov         qword ptr [rsp+020h],rax
 mov         ecx,1
 mov         edx,2
 mov         r8d,3
 mov         r9d,4
 call        qword ptr [r14]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_easy_interference() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(RegisterX64::RDI);
  let mut tmp2 = fixture.take_scoped(RegisterX64::RSI);
  let mut tmp3 = fixture.take_scoped(fixture.r_arg2);
  let mut tmp4 = fixture.take_scoped(fixture.r_arg1);

  fixture.add_scoped(SizeX64::Qword, &mut tmp1);
  fixture.add_scoped(SizeX64::Qword, &mut tmp2);
  fixture.add_scoped(SizeX64::Qword, &mut tmp3);
  fixture.add_scoped(SizeX64::Qword, &mut tmp4);
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         r8,rdx
 mov         rdx,rsi
 mov         r9,rcx
 mov         rcx,rdi
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_extra_coverage() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg2);

  fixture.add_arg(SizeX64::Qword, ADDR.operator_bracket(RegisterX64::R12 + 8));
  fixture.add_arg(SizeX64::Qword, ADDR.operator_bracket(RegisterX64::R12 + 16));
  fixture.add_arg(
    SizeX64::Xmmword,
    XMMWORD.operator_bracket(RegisterX64::R13.into()),
  );
  fixture.call(QWORD.operator_bracket(tmp1.release() + tmp2.release()));

  fixture.check_match(String::from(
    r#"
 vmovups     xmm2,xmmword ptr [r13]
 mov         rax,rcx
 lea         rcx,[r12+8]
 mov         rbx,rdx
 lea         rdx,[r12+010h]
 call        qword ptr [rax+rbx]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_fake_interference() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg2);

  fixture.add_arg(SizeX64::Qword, QWORD.operator_bracket(tmp1.release() + 8));
  fixture.add_arg(SizeX64::Qword, QWORD.operator_bracket(tmp2.release() + 8));
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rcx,qword ptr [rcx+8]
 mov         rdx,qword ptr [rdx+8]
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_fake_multiuse_interference_mem() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg2);

  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(tmp1.reg + tmp2.reg + 8),
  );
  fixture.add_arg(SizeX64::Qword, QWORD.operator_bracket(tmp2.reg + 16));
  tmp1.release();
  tmp2.release();
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rcx,qword ptr [rcx+rdx+8]
 mov         rdx,qword ptr [rdx+010h]
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_fixed_registers() {
  let mut fixture = IrCallWrapperX64Fixture::windows();

  fixture.add_arg(SizeX64::Dword, 1);
  fixture.add_arg(SizeX64::Qword, 2);
  fixture.add_arg(SizeX64::Qword, 3);
  fixture.add_arg(SizeX64::Qword, 4);
  fixture.add_arg(SizeX64::Qword, RegisterX64::R14);
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         qword ptr [rsp+020h],r14
 mov         ecx,1
 mov         rdx,2
 mov         r8,3
 mov         r9,4
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_hard_interference_both() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut int1 = fixture.take_scoped(fixture.r_arg2);
  let mut int2 = fixture.take_scoped(fixture.r_arg1);
  let mut fp1 = fixture.take_scoped(RegisterX64::XMM3);
  let mut fp2 = fixture.take_scoped(RegisterX64::XMM2);

  fixture.add_scoped(SizeX64::Qword, &mut int1);
  fixture.add_scoped(SizeX64::Qword, &mut int2);
  fixture.add_scoped(SizeX64::Xmmword, &mut fp1);
  fixture.add_scoped(SizeX64::Xmmword, &mut fp2);
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rax,rdx
 mov         rdx,rcx
 mov         rcx,rax
 vmovsd      xmm0,xmm3,xmm3
 vmovsd      xmm3,xmm2,xmm2
 vmovsd      xmm2,xmm0,xmm0
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_hard_interference_fp() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(RegisterX64::XMM1);
  let mut tmp2 = fixture.take_scoped(RegisterX64::XMM0);

  fixture.add_scoped(SizeX64::Xmmword, &mut tmp1);
  fixture.add_scoped(SizeX64::Xmmword, &mut tmp2);
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 vmovsd      xmm2,xmm1,xmm1
 vmovsd      xmm1,xmm0,xmm0
 vmovsd      xmm0,xmm2,xmm2
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_hard_interference_int() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg4);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg3);
  let mut tmp3 = fixture.take_scoped(fixture.r_arg2);
  let mut tmp4 = fixture.take_scoped(fixture.r_arg1);

  fixture.add_scoped(SizeX64::Qword, &mut tmp1);
  fixture.add_scoped(SizeX64::Qword, &mut tmp2);
  fixture.add_scoped(SizeX64::Qword, &mut tmp3);
  fixture.add_scoped(SizeX64::Qword, &mut tmp4);
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rax,r9
 mov         r9,rcx
 mov         rcx,rax
 mov         rax,r8
 mov         r8,rdx
 mov         rdx,rax
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_hard_interference_int2() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg4d);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg3d);
  let mut tmp3 = fixture.take_scoped(fixture.r_arg2d);
  let mut tmp4 = fixture.take_scoped(fixture.r_arg1d);

  fixture.add_scoped(SizeX64::Dword, &mut tmp1);
  fixture.add_scoped(SizeX64::Dword, &mut tmp2);
  fixture.add_scoped(SizeX64::Dword, &mut tmp3);
  fixture.add_scoped(SizeX64::Dword, &mut tmp4);
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         eax,r9d
 mov         r9d,ecx
 mov         ecx,eax
 mov         eax,r8d
 mov         r8d,edx
 mov         edx,eax
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_hard_multiuse_interference_mem1() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg2);

  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(tmp1.reg + tmp2.reg + 8),
  );
  fixture.add_arg(SizeX64::Qword, QWORD.operator_bracket(tmp1.reg + 16));
  tmp1.release();
  tmp2.release();
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rax,rcx
 mov         rcx,qword ptr [rax+rdx+8]
 mov         rdx,qword ptr [rax+010h]
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_hard_multiuse_interference_mem2() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg2);

  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(tmp1.reg + tmp2.reg + 8),
  );
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(tmp1.reg + tmp2.reg + 16),
  );
  tmp1.release();
  tmp2.release();
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rax,rcx
 mov         rcx,qword ptr [rax+rdx+8]
 mov         rdx,qword ptr [rax+rdx+010h]
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_hard_multiuse_interference_mem3() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg3);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg2);
  let mut tmp3 = fixture.take_scoped(fixture.r_arg1);

  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(tmp1.reg + tmp2.reg + 8),
  );
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(tmp2.reg + tmp3.reg + 16),
  );
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(tmp3.reg + tmp1.reg + 16),
  );
  tmp1.release();
  tmp2.release();
  tmp3.release();
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rax,r8
 mov         r8,qword ptr [rcx+rax+010h]
 mov         rbx,rdx
 mov         rdx,qword ptr [rbx+rcx+010h]
 mov         rcx,qword ptr [rax+rbx+8]
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_immediate_conflict_with_function() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg2);

  fixture.add_arg(SizeX64::Dword, 1);
  fixture.add_arg(SizeX64::Dword, 2);
  fixture.call(QWORD.operator_bracket(tmp1.release() + tmp2.release()));

  fixture.check_match(String::from(
    r#"
 mov         rax,rcx
 mov         ecx,1
 mov         rbx,rdx
 mov         edx,2
 call        qword ptr [rax+rbx]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_interference_with_call_arg1() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);

  fixture.add_arg(SizeX64::Qword, QWORD.operator_bracket(tmp1.reg + 8));
  fixture.call(QWORD.operator_bracket(tmp1.release() + 16));

  fixture.check_match(String::from(
    r#"
 mov         rax,rcx
 mov         rcx,qword ptr [rax+8]
 call        qword ptr [rax+010h]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_interference_with_call_arg2() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);
  let mut tmp2 = fixture.take_scoped(fixture.r_arg2);

  fixture.add_scoped(SizeX64::Qword, &mut tmp2);
  fixture.call(QWORD.operator_bracket(tmp1.release() + 16));

  fixture.check_match(String::from(
    r#"
 mov         rax,rcx
 mov         rcx,rdx
 call        qword ptr [rax+010h]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_interference_with_call_arg3() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);

  fixture.add_arg(SizeX64::Qword, tmp1.reg);
  fixture.call(QWORD.operator_bracket(tmp1.release() + 16));

  fixture.check_match(String::from(
    r#"
 call        qword ptr [rcx+010h]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_simple_mem_imm() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(RegisterX64::RAX);
  let mut tmp2 = fixture.take_scoped(RegisterX64::RSI);

  fixture.add_arg(SizeX64::Dword, 32);
  fixture.add_arg(SizeX64::Dword, -1);
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(RegisterX64::R14 + 32),
  );
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(tmp1.release() + tmp2.release()),
  );
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         r8,qword ptr [r14+020h]
 mov         r9,qword ptr [rax+rsi]
 mov         ecx,20h
 mov         edx,FFFFFFFFh
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_simple_regs() {
  let mut fixture = IrCallWrapperX64Fixture::windows();

  let tmp1_reg = fixture.regs.take_reg(RegisterX64::RAX, K_INVALID_INST_IDX);
  let mut tmp1 = ScopedRegX64 {
    owner: &mut *fixture.regs,
    reg: tmp1_reg,
  };
  let tmp2_reg = fixture.regs.take_reg(fixture.r_arg2, K_INVALID_INST_IDX);
  let mut tmp2 = ScopedRegX64 {
    owner: &mut *fixture.regs,
    reg: tmp2_reg,
  };

  fixture
    .call_wrap
    .add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp1);
  fixture
    .call_wrap
    .add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp2);
  fixture
    .call_wrap
    .call(&QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rcx,rax
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_simple_stack_args() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp = fixture.take_scoped(RegisterX64::RAX);

  fixture.add_scoped(SizeX64::Qword, &mut tmp);
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(RegisterX64::R14 + 16),
  );
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(RegisterX64::R14 + 32),
  );
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(RegisterX64::R14 + 48),
  );
  fixture.add_arg(SizeX64::Dword, 1);
  fixture.add_arg(
    SizeX64::Qword,
    QWORD.operator_bracket(RegisterX64::R13.into()),
  );
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rdx,qword ptr [r13]
 mov         qword ptr [rsp+028h],rdx
 mov         rcx,rax
 mov         rdx,qword ptr [r14+010h]
 mov         r8,qword ptr [r14+020h]
 mov         r9,qword ptr [r14+030h]
 mov         dword ptr [rsp+020h],1
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_suggested_conflict_with_reserved() {
  let mut fixture = IrCallWrapperX64FixtureSystemV::new();
  let mut tmp = fixture.base.take_scoped(RegisterX64::R9);
  let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
    &mut fixture.base.regs,
    &mut fixture.base.build,
    !0u32,
  );

  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    RegisterX64::R12.into(),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    RegisterX64::R13.into(),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    RegisterX64::R14.into(),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Dword,
    OperandX64::from(2),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::from(1),
    IrOp::new(),
  );

  let reg = call_wrap.suggest_next_argument_register(SizeX64::Dword);
  fixture
    .base
    .build
    .mov(OperandX64::from(reg), OperandX64::from(10));
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Dword,
    OperandX64::from(reg),
    IrOp::new(),
  );

  let func = OperandX64::from(tmp.release());
  call_wrap.call(&func);

  fixture.base.check_match(String::from(
    r#"
 mov         eax,Ah
 mov         rdi,r12
 mov         rsi,r13
 mov         rdx,r14
 mov         rcx,r9
 mov         r9d,eax
 mov         rax,rcx
 mov         ecx,2
 mov         r8,1
 call        rax
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_tricky_use1() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);

  fixture.add_arg(SizeX64::Qword, tmp1.reg);
  fixture.add_arg(SizeX64::Qword, tmp1.release());
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rdx,rcx
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_tricky_use2() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut tmp1 = fixture.take_scoped(fixture.r_arg1);

  fixture.add_arg(SizeX64::Qword, QWORD.operator_bracket(tmp1.reg.into()));
  fixture.add_arg(SizeX64::Qword, tmp1.release());
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rdx,rcx
 mov         rcx,qword ptr [rcx]
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_with_last_ir_inst_use1() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut ir_inst1 = IrInst::default();
  let ir_op1 = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, 0);
  ir_inst1.reg_x64 = fixture.regs.take_reg(RegisterX64::XMM0, ir_op1.index());
  ir_inst1.last_use = 1;
  fixture.function.instructions.push(ir_inst1.clone());
  fixture.call_wrap.inst_idx = ir_inst1.last_use;

  fixture.call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Xmmword,
    ir_inst1.reg_x64.into(),
    ir_op1,
  );
  fixture.add_arg(
    SizeX64::Xmmword,
    QWORD.operator_bracket(RegisterX64::R12 + 8),
  );
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 vmovsd      xmm1,qword ptr [r12+8]
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_with_last_ir_inst_use2() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut ir_inst1 = IrInst::default();
  let ir_op1 = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, 0);
  ir_inst1.reg_x64 = fixture.regs.take_reg(RegisterX64::XMM0, ir_op1.index());
  ir_inst1.last_use = 1;
  fixture.function.instructions.push(ir_inst1.clone());
  fixture.call_wrap.inst_idx = ir_inst1.last_use;

  fixture.add_arg(
    SizeX64::Xmmword,
    QWORD.operator_bracket(RegisterX64::R12 + 8),
  );
  fixture.call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Xmmword,
    ir_inst1.reg_x64.into(),
    ir_op1,
  );
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 vmovsd      xmm1,xmm0,xmm0
 vmovsd      xmm0,qword ptr [r12+8]
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_with_last_ir_inst_use3() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut ir_inst1 = IrInst::default();
  let ir_op1 = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, 0);
  ir_inst1.reg_x64 = fixture.regs.take_reg(RegisterX64::XMM0, ir_op1.index());
  ir_inst1.last_use = 1;
  fixture.function.instructions.push(ir_inst1.clone());
  fixture.call_wrap.inst_idx = ir_inst1.last_use;

  fixture.call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Xmmword,
    ir_inst1.reg_x64.into(),
    ir_op1,
  );
  fixture.call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Xmmword,
    ir_inst1.reg_x64.into(),
    ir_op1,
  );
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 vmovsd      xmm1,xmm0,xmm0
 call        qword ptr [r12]
"#,
  ));
}

// Source: `tests/IrCallWrapperX64.test.cpp`
#[test]
fn ir_call_wrapper_x_64_with_last_ir_inst_use4() {
  let mut fixture = IrCallWrapperX64Fixture::windows();
  let mut ir_inst1 = IrInst::default();
  let ir_op1 = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, 0);
  ir_inst1.reg_x64 = fixture.regs.take_reg(RegisterX64::RAX, ir_op1.index());
  ir_inst1.last_use = 1;
  fixture.function.instructions.push(ir_inst1.clone());
  fixture.call_wrap.inst_idx = ir_inst1.last_use;

  let mut tmp = fixture.take_scoped(RegisterX64::RDX);
  fixture.add_arg(SizeX64::Qword, RegisterX64::R15);
  fixture.call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    ir_inst1.reg_x64.into(),
    ir_op1,
  );
  fixture.add_scoped(SizeX64::Qword, &mut tmp);
  fixture.call(QWORD.operator_bracket(RegisterX64::R12.into()));

  fixture.check_match(String::from(
    r#"
 mov         rcx,r15
 mov         r8,rdx
 mov         rdx,rax
 call        qword ptr [r12]
"#,
  ));
}
