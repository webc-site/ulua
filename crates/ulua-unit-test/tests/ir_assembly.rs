extern crate alloc;

mod ir_assembly_dse_hint_materializes_int_into_dead_vm_reg {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrAssembly.test.cpp:272:ir_assembly_dse_hint_materializes_int_into_dead_vm_reg`
  //! Source: `tests/IrAssembly.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrAssembly.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrAssembly.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::align (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::ud2 (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - calls -> method AssemblyBuilderX64::vcvttsd2si (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::vaddsd (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::vcvtsi2sd (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::vmovups (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ir_assembly_dse_hint_materializes_int_into_dead_vm_reg

  #[cfg(test)]
  #[test]
  fn ir_assembly_dse_hint_materializes_int_into_dead_vm_reg() {
    use ulua_code_gen::{
      enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::update_use_counts::update_use_counts,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_assembly_fixture::IrAssemblyFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _luau_codegen_dse_restore_hints =
      ScopedFastFlag::new(&FFlag::LuauCodegenDseRestoreHints, true);
    let _luau_codegen_forward_rematerialize =
      ScopedFastFlag::new(&FFlag::LuauCodegenForwardRematerialize, true);

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
}

mod ir_assembly_multi_num_to_x_shared_source_strands_restore {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrAssembly.test.cpp:346:ir_assembly_multi_num_to_x_shared_source_strands_restore`
  //! Source: `tests/IrAssembly.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrAssembly.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrAssembly.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::align (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::ud2 (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - calls -> method AssemblyBuilderX64::vcvttsd2si (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::vaddsd (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::vcvtsi2sd (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::vmovups (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ir_assembly_multi_num_to_x_shared_source_strands_restore

  #[cfg(test)]
  #[test]
  fn ir_assembly_multi_num_to_x_shared_source_strands_restore() {
    use ulua_code_gen::{
      enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::update_use_counts::update_use_counts,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_assembly_fixture::IrAssemblyFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _luau_codegen_forward_rematerialize =
      ScopedFastFlag::new(&FFlag::LuauCodegenForwardRematerialize, true);

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
}

mod ir_assembly_preserve_int_chained_from_double_vm_reg {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrAssembly.test.cpp:123:ir_assembly_preserve_int_chained_from_double_vm_reg`
  //! Source: `tests/IrAssembly.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrAssembly.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrAssembly.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method AssemblyBuilderX64::vcvttsd2si (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::align (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::ud2 (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - calls -> method AssemblyBuilderX64::vmovups (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ir_assembly_preserve_int_chained_from_double_vm_reg

  #[cfg(test)]
  #[test]
  fn ir_assembly_preserve_int_chained_from_double_vm_reg() {
    use ulua_code_gen::{
      enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::update_use_counts::update_use_counts,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_assembly_fixture::IrAssemblyFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _luau_codegen_forward_rematerialize =
      ScopedFastFlag::new(&FFlag::LuauCodegenForwardRematerialize, true);

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
}

mod ir_assembly_preserve_int_chained_from_double_vm_reg_both {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrAssembly.test.cpp:170:ir_assembly_preserve_int_chained_from_double_vm_reg_both`
  //! Source: `tests/IrAssembly.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrAssembly.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrAssembly.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method AssemblyBuilderX64::vcvttsd2si (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::align (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::ud2 (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - calls -> method AssemblyBuilderX64::vmovups (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ir_assembly_preserve_int_chained_from_double_vm_reg_both

  #[cfg(test)]
  #[test]
  fn ir_assembly_preserve_int_chained_from_double_vm_reg_both() {
    use ulua_code_gen::{
      enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::update_use_counts::update_use_counts,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_assembly_fixture::IrAssemblyFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _luau_codegen_forward_rematerialize =
      ScopedFastFlag::new(&FFlag::LuauCodegenForwardRematerialize, true);

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
}

mod ir_assembly_preserve_int_without_chain_spills_to_stack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrAssembly.test.cpp:227:ir_assembly_preserve_int_without_chain_spills_to_stack`
  //! Source: `tests/IrAssembly.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrAssembly.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrAssembly.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::align (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::ud2 (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - calls -> method AssemblyBuilderX64::vaddsd (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::vcvttsd2si (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ir_assembly_preserve_int_without_chain_spills_to_stack

  #[cfg(test)]
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
}
