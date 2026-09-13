extern crate alloc;

mod ir_call_wrapper_x_64_address_in_stack_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:493:ir_call_wrapper_x_64_address_in_stack_arguments`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_address_in_stack_arguments

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_address_in_stack_arguments() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{
        operand_x_64::{ADDR, QWORD},
        register_x_64::RegisterX64,
      },
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_easy_interference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:158:ir_call_wrapper_x_64_easy_interference`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_easy_interference

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_easy_interference() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_extra_coverage {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:474:ir_call_wrapper_x_64_extra_coverage`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::vmovups (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_extra_coverage

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_extra_coverage() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{
        operand_x_64::{ADDR, QWORD, XMMWORD},
        register_x_64::RegisterX64,
      },
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_fake_interference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:179:ir_call_wrapper_x_64_fake_interference`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_fake_interference

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_fake_interference() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_fake_multiuse_interference_mem {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:279:ir_call_wrapper_x_64_fake_multiuse_interference_mem`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_fake_multiuse_interference_mem

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_fake_multiuse_interference_mem() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_fixed_registers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:139:ir_call_wrapper_x_64_fixed_registers`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_fixed_registers

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_fixed_registers() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_hard_interference_both {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:256:ir_call_wrapper_x_64_hard_interference_both`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_interference_both

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_hard_interference_both() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_hard_interference_fp {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:240:ir_call_wrapper_x_64_hard_interference_fp`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_interference_fp

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_hard_interference_fp() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_hard_interference_int {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:194:ir_call_wrapper_x_64_hard_interference_int`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_interference_int

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_hard_interference_int() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_hard_interference_int_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:217:ir_call_wrapper_x_64_hard_interference_int_2`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_interference_int_2

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_hard_interference_int2() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_hard_multiuse_interference_mem_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:296:ir_call_wrapper_x_64_hard_multiuse_interference_mem_1`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_multiuse_interference_mem_1

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_hard_multiuse_interference_mem1() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_hard_multiuse_interference_mem_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:314:ir_call_wrapper_x_64_hard_multiuse_interference_mem_2`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_multiuse_interference_mem_2

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_hard_multiuse_interference_mem2() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_hard_multiuse_interference_mem_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:332:ir_call_wrapper_x_64_hard_multiuse_interference_mem_3`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_multiuse_interference_mem_3

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_hard_multiuse_interference_mem3() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_immediate_conflict_with_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:513:ir_call_wrapper_x_64_immediate_conflict_with_function`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_immediate_conflict_with_function

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_immediate_conflict_with_function() {
    use ulua_code_gen::{enums::size_x_64::SizeX64, records::operand_x_64::QWORD};
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_interference_with_call_arg_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:355:ir_call_wrapper_x_64_interference_with_call_arg_1`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_interference_with_call_arg_1

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_interference_with_call_arg1() {
    use ulua_code_gen::{enums::size_x_64::SizeX64, records::operand_x_64::QWORD};
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_interference_with_call_arg_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:368:ir_call_wrapper_x_64_interference_with_call_arg_2`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_interference_with_call_arg_2

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_interference_with_call_arg2() {
    use ulua_code_gen::{enums::size_x_64::SizeX64, records::operand_x_64::QWORD};
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_interference_with_call_arg_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:382:ir_call_wrapper_x_64_interference_with_call_arg_3`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_interference_with_call_arg_3

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_interference_with_call_arg3() {
    use ulua_code_gen::{enums::size_x_64::SizeX64, records::operand_x_64::QWORD};
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_simple_mem_imm {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:97:ir_call_wrapper_x_64_simple_mem_imm`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_simple_mem_imm

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_simple_mem_imm() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_simple_regs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:56:ir_call_wrapper_x_64_simple_regs`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_simple_regs

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_simple_regs() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{
        ir_data::K_INVALID_INST_IDX, operand_x_64::QWORD, register_x_64::RegisterX64,
        scoped_reg_x_64::ScopedRegX64,
      },
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_simple_stack_args {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:116:ir_call_wrapper_x_64_simple_stack_args`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_simple_stack_args

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_simple_stack_args() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_suggested_conflict_with_reserved {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:531:ir_call_wrapper_x_64_suggested_conflict_with_reserved`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> record IrCallWrapperX64 (CodeGen/include/Luau/IrCallWrapperX64.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - type_ref -> record RegisterX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64::suggestNextArgumentRegister (CodeGen/src/IrCallWrapperX64.cpp)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_suggested_conflict_with_reserved

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_suggested_conflict_with_reserved() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{
        ir_call_wrapper_x_64::IrCallWrapperX64, ir_op::IrOp, operand_x_64::OperandX64,
        register_x_64::RegisterX64,
      },
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture_system_v::IrCallWrapperX64FixtureSystemV;

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
}

mod ir_call_wrapper_x_64_tricky_use_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:70:ir_call_wrapper_x_64_tricky_use_1`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_tricky_use_1

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_tricky_use1() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_tricky_use_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:83:ir_call_wrapper_x_64_tricky_use_2`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_tricky_use_2

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_tricky_use2() {
    use ulua_code_gen::{
      enums::size_x_64::SizeX64,
      records::{operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_with_last_ir_inst_use_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:393:ir_call_wrapper_x_64_with_last_ir_inst_use_1`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrInst (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum IrOpKind (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_with_last_ir_inst_use_1

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_with_last_ir_inst_use1() {
    use ulua_code_gen::{
      enums::{ir_op_kind::IrOpKind, size_x_64::SizeX64},
      records::{ir_inst::IrInst, ir_op::IrOp, operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_with_last_ir_inst_use_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:412:ir_call_wrapper_x_64_with_last_ir_inst_use_2`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrInst (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum IrOpKind (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_with_last_ir_inst_use_2

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_with_last_ir_inst_use2() {
    use ulua_code_gen::{
      enums::{ir_op_kind::IrOpKind, size_x_64::SizeX64},
      records::{ir_inst::IrInst, ir_op::IrOp, operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_with_last_ir_inst_use_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:432:ir_call_wrapper_x_64_with_last_ir_inst_use_3`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrInst (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum IrOpKind (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_with_last_ir_inst_use_3

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_with_last_ir_inst_use3() {
    use ulua_code_gen::{
      enums::{ir_op_kind::IrOpKind, size_x_64::SizeX64},
      records::{ir_inst::IrInst, ir_op::IrOp, operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}

mod ir_call_wrapper_x_64_with_last_ir_inst_use_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:451:ir_call_wrapper_x_64_with_last_ir_inst_use_4`
  //! Source: `tests/IrCallWrapperX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrInst (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum IrOpKind (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
  //!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
  //!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_call_wrapper_x_64_with_last_ir_inst_use_4

  #[cfg(test)]
  #[test]
  fn ir_call_wrapper_x_64_with_last_ir_inst_use4() {
    use ulua_code_gen::{
      enums::{ir_op_kind::IrOpKind, size_x_64::SizeX64},
      records::{ir_inst::IrInst, ir_op::IrOp, operand_x_64::QWORD, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;

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
}
