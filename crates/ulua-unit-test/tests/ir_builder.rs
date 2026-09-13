extern crate alloc;

mod ir_builder_array_elem_checks_negative_index {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4500:ir_builder_array_elem_checks_negative_index`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_array_elem_checks_negative_index

  #[cfg(test)]
  #[test]
  fn ir_builder_array_elem_checks_negative_index() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let table1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, zero, fallback);
      let zero = b.const_int(0);
      let elem1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, zero);
      let zero = b.const_int(0);
      let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem1, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1);

      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, minus_one, fallback);
      let minus_one = b.const_int(-1);
      let elem2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, minus_one);
      let zero = b.const_int(0);
      let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem2, zero);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r4, value1b);

      let r3 = b.vm_reg(3);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let r4 = b.vm_reg(4);
      let b_value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r4);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a, b_value);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, sum);

      let r2 = b.vm_reg(2);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   CHECK_ARRAY_SIZE %0, 0i, bb_fallback_1\n   %2 = GET_ARR_ADDR %0, 0i\n   %3 = LOAD_TVALUE %2, 0i\n   STORE_TVALUE R3, %3\n   JUMP bb_fallback_1\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_bit_32 {
  #[cfg(test)]
  #[test]
  fn ir_builder_bit_32() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let unk = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r0);

      // Binary const-const / identity folds: STORE_INT R{reg}, CMD(a, b)
      macro_rules! bin {
            ($reg:expr, $cmd:expr, ($($a:tt)+), ($($bb:tt)+)) => {{
                let ca = bin!(@operand $($a)+);
                let cb = bin!(@operand $($bb)+);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
            }};
            (@operand unk) => { unk };
            (@operand $v:expr) => { b.const_int($v) };
        }
      macro_rules! un {
        ($reg:expr, $cmd:expr, $a:expr) => {{
          let ca = b.const_int($a);
          let op = b.inst_ir_cmd_ir_op($cmd, ca);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
        }};
      }

      let all_ones = !0u32 as i32;

      bin!(0, IrCmd::BitandUint, (0xfe), (0xe));
      bin!(1, IrCmd::BitandUint, (unk), (0));
      bin!(2, IrCmd::BitandUint, (0), (unk));
      bin!(3, IrCmd::BitandUint, (unk), (all_ones));
      bin!(4, IrCmd::BitandUint, (all_ones), (unk));
      bin!(5, IrCmd::BitxorUint, (0xfe), (0xe));
      bin!(6, IrCmd::BitxorUint, (unk), (0));
      bin!(7, IrCmd::BitxorUint, (0), (unk));
      bin!(8, IrCmd::BitxorUint, (unk), (all_ones));
      bin!(9, IrCmd::BitxorUint, (all_ones), (unk));
      bin!(10, IrCmd::BitorUint, (0xf0), (0xe));
      bin!(11, IrCmd::BitorUint, (unk), (0));
      bin!(12, IrCmd::BitorUint, (0), (unk));
      bin!(13, IrCmd::BitorUint, (unk), (all_ones));
      bin!(14, IrCmd::BitorUint, (all_ones), (unk));
      un!(15, IrCmd::BitnotUint, 0xe);
      bin!(16, IrCmd::BitlshiftUint, (0xf0), (4));
      bin!(17, IrCmd::BitlshiftUint, (unk), (0));
      bin!(18, IrCmd::BitrshiftUint, (0xdeee0000u32 as i32), (8));
      bin!(19, IrCmd::BitrshiftUint, (unk), (0));
      bin!(20, IrCmd::BitarshiftUint, (0xdeee0000u32 as i32), (8));
      bin!(21, IrCmd::BitarshiftUint, (unk), (0));
      bin!(22, IrCmd::BitlrotateUint, (0xdeee0000u32 as i32), (8));
      bin!(23, IrCmd::BitlrotateUint, (unk), (0));
      bin!(24, IrCmd::BitrrotateUint, (0xdeee0000u32 as i32), (8));
      bin!(25, IrCmd::BitrrotateUint, (unk), (0));
      un!(26, IrCmd::BitcountlzUint, 0xff00);
      un!(27, IrCmd::BitcountlzUint, 0);
      un!(28, IrCmd::BitcountrzUint, 0xff00);
      un!(29, IrCmd::BitcountrzUint, 0);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT R0\n   STORE_INT R0, 14i\n   STORE_INT R1, 0i\n   STORE_INT R2, 0i\n   STORE_INT R3, %0\n   STORE_INT R4, %0\n   STORE_INT R5, 240i\n   STORE_INT R6, %0\n   STORE_INT R7, %0\n   %17 = BITNOT_UINT %0\n   STORE_INT R8, %17\n   %19 = BITNOT_UINT %0\n   STORE_INT R9, %19\n   STORE_INT R10, 254i\n   STORE_INT R11, %0\n   STORE_INT R12, %0\n   STORE_INT R13, -1i\n   STORE_INT R14, -1i\n   STORE_INT R15, -15i\n   STORE_INT R16, 3840i\n   STORE_INT R17, %0\n   STORE_INT R18, 14609920i\n   STORE_INT R19, %0\n   STORE_INT R20, -2167296i\n   STORE_INT R21, %0\n   STORE_INT R22, -301989666i\n   STORE_INT R23, %0\n   STORE_INT R24, 14609920i\n   STORE_INT R25, %0\n   STORE_INT R26, 16i\n   STORE_INT R27, 32i\n   STORE_INT R28, 8i\n   STORE_INT R29, 32i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_bit_32_range_reduction {
  #[cfg(test)]
  #[test]
  fn ir_builder_bit_32_range_reduction() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int($a);
          let cb = b.const_int($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg(10);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
        }};
      }

      bin!(IrCmd::BitlshiftUint, 0xf, -10);
      bin!(IrCmd::BitlshiftUint, 0xf, 140);
      bin!(IrCmd::BitrshiftUint, 0xffffff, -10);
      bin!(IrCmd::BitrshiftUint, 0xffffff, 140);
      bin!(IrCmd::BitarshiftUint, 0xffffff, -10);
      bin!(IrCmd::BitarshiftUint, 0xffffff, 140);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R10, 62914560i\n   STORE_INT R10, 61440i\n   STORE_INT R10, 3i\n   STORE_INT R10, 4095i\n   STORE_INT R10, 3i\n   STORE_INT R10, 4095i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_buffer_length_checks_integer_match {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4649:ir_builder_buffer_length_checks_integer_match`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - translates_to -> rust_item ir_builder_buffer_length_checks_integer_match

  #[cfg(test)]
  #[test]
  fn ir_builder_buffer_length_checks_integer_match() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };
    use ulua_vm::enums::lua_type::LuaType;

    let _load_propagate_origin = ScopedFastFlag::new(&FFlag::LuauCodegenLoadPropagateOrigin, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let source_buf = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);

      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, source_buf);
      let r2 = b.vm_reg(2);
      let buffer1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r2);
      let index = b.const_int(0);
      let start = b.const_int(0);
      let size = b.const_int(4);
      let source = b.const_double(0.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer1,
        index,
        start,
        size,
        source,
        fallback,
      );
      let index = b.const_int(0);
      let start = b.const_int(0);
      let size = b.const_int(4);
      let source = b.const_double(0.2);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer1,
        index,
        start,
        size,
        source,
        fallback,
      );
      let index = b.const_int(0);
      let value = b.const_int(32);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei32, buffer1, index, value, tag);
      let r1 = b.vm_reg(1);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TVALUE R0\n   STORE_TVALUE R2, %0\n   %2 = LOAD_POINTER R0\n   CHECK_BUFFER_LEN %2, 0i, 0i, 4i, undef, bb_fallback_1\n   JUMP bb_fallback_1\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_buffer_length_checks_integer_match_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4687:ir_builder_buffer_length_checks_integer_match_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_buffer_length_checks_integer_match_2

  #[cfg(test)]
  #[test]
  fn ir_builder_buffer_length_checks_integer_match_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;
    use ulua_vm::enums::lua_type::LuaType;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let buffer = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);

      let r1 = b.vm_reg(1);
      let value = b.const_double(1000000.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, value);
      let r1 = b.vm_reg(1);
      let tag = b.const_tag(LuaType::Number as u8);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tag);

      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
      let squared = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, value, value);
      let base = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, squared);
      let start = b.const_int(0);
      let size = b.const_int(4);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer,
        base,
        start,
        size,
        squared,
        fallback,
      );
      let index = b.const_int(0);
      let value = b.const_int(32);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei32, buffer, index, value, tag);
      let r1 = b.vm_reg(1);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R1, 1000000\n   STORE_TAG R1, tnumber\n   JUMP bb_fallback_1\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_buffer_length_checks_negative_index {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4616:ir_builder_buffer_length_checks_negative_index`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_buffer_length_checks_negative_index

  #[cfg(test)]
  #[test]
  fn ir_builder_buffer_length_checks_negative_index() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;
    use ulua_vm::enums::lua_type::LuaType;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let source_buf = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);

      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, source_buf);
      let r2 = b.vm_reg(2);
      let buffer1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r2);
      let index = b.const_int(-4);
      let start = b.const_int(0);
      let size = b.const_int(4);
      let source = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer1,
        index,
        start,
        size,
        source,
        fallback,
      );
      let index = b.const_int(-4);
      let value = b.const_int(32);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei32, buffer1, index, value, tag);
      let r1 = b.vm_reg(1);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TVALUE R0\n   STORE_TVALUE R2, %0\n   JUMP bb_fallback_1\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_builtin_fastcalls_may_invalidate_memory {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2857:ir_builder_builtin_fastcalls_may_invalidate_memory`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_builtin_fastcalls_may_invalidate_memory

  #[cfg(test)]
  #[test]
  fn ir_builder_builtin_fastcalls_may_invalidate_memory() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let half = b.const_double(0.5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, half);

      let r0 = b.vm_reg(0);
      let table = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);

      let bfid = b.const_uint(LuauBuiltinFunction::LBF_SETMETATABLE as u32);
      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      let r3 = b.vm_reg(3);
      let undef = b.undef();
      let three = b.const_int(3);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::InvokeFastcall,
        bfid,
        r1,
        r2,
        r3,
        undef,
        three,
        one,
      );

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);

      let r0 = b.vm_reg(0);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, value);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

      b.begin_block(fallback);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 0.5\n   %1 = LOAD_POINTER R0\n   CHECK_NO_METATABLE %1, bb_fallback_1\n   CHECK_READONLY %1, bb_fallback_1\n   %4 = INVOKE_FASTCALL 61u, R1, R2, R3, undef, 3i, 1i\n   CHECK_NO_METATABLE %1, bb_fallback_1\n   CHECK_READONLY %1, bb_fallback_1\n   STORE_DOUBLE R1, 0.5\n   RETURN 0u\n\nbb_fallback_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_builtin_variadic_start {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5243:ir_builder_builtin_variadic_start`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_builtin_variadic_start

  #[cfg(test)]
  #[test]
  fn ir_builder_builtin_variadic_start() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r2 = b.vm_reg(2);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, two);
      let r2 = b.vm_reg(2);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, r2, one);
      let r1 = b.vm_reg(1);
      let minus_one = b.const_int(-1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, r1, minus_one, one);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1\n; in regs: R0\n; out regs: R0, R1\n   STORE_DOUBLE R1, 1\n   STORE_DOUBLE R2, 2\n   ADJUST_STACK_TO_REG R2, 1i\n   CALL R1, -1i, 1i\n   JUMP bb_1\n\nbb_1:\n; predecessors: bb_0\n; in regs: R0, R1\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_call_is_a_blocker {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5537:ir_builder_call_is_a_blocker`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_call_is_a_blocker

  #[cfg(test)]
  #[test]
  fn ir_builder_call_is_a_blocker() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let r1 = b.vm_reg(1);
      let one_params = b.const_int(1);
      let r2 = b.vm_reg(2);
      let one_results = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CALL, r1, one_params, r2, one_results);
      let r0 = b.vm_reg(0);
      let op2 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, op1, op2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, sum);
      let r1 = b.vm_reg(1);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   CALL R1, 1i, R2, 1i\n   %2 = LOAD_DOUBLE R0\n   %3 = ADD_NUM %0, %2\n   STORE_DOUBLE R1, %3\n   RETURN R1, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_check_cmp_num_const_fold_fail {
  #[cfg(test)]
  #[test]
  fn ir_builder_check_cmp_num_const_fold_fail() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let c2 = b.const_double(2.0);
      let c1 = b.const_double(1.0);
      let cond = b.cond(IrCondition::Less);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpNum, c2, c1, cond, fallback);
      let cd = b.const_double(42.0);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, cd);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1u = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1u);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_check_cmp_num_const_fold_pass {
  #[cfg(test)]
  #[test]
  fn ir_builder_check_cmp_num_const_fold_pass() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let c1 = b.const_double(1.0);
      let c2 = b.const_double(2.0);
      let cond = b.cond(IrCondition::Less);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpNum, c1, c2, cond, fallback);
      let cd = b.const_double(42.0);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, cd);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1u = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1u);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 42\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_check_cmp_num_na_n {
  #[cfg(test)]
  #[test]
  fn ir_builder_check_cmp_num_na_n() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let z1 = b.const_double(0.0);
      let z2 = b.const_double(0.0);
      let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, z1, z2);
      let c1 = b.const_double(1.0);
      let cond = b.cond(IrCondition::Equal);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpNum, nan, c1, cond, fallback);
      let cd = b.const_double(42.0);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, cd);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1u = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1u);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_cmp_split_tag_value_simplification {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3553:ir_builder_cmp_split_tag_value_simplification`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_cmp_split_tag_value_simplification

  #[cfg(test)]
  #[test]
  fn ir_builder_cmp_split_tag_value_simplification() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    macro_rules! store_cmp_split {
      ($b:ident, $reg:expr, $tag_a:expr, $tag_b:expr, $value_a:expr, $value_b:expr, $cond:expr) => {{
        let cmp = $b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
          IrCmd::CmpSplitTvalue,
          $tag_a,
          $tag_b,
          $value_a,
          $value_b,
          $cond,
        );
        let reg = $b.vm_reg($reg);
        $b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, reg, cmp);
      }};
    }

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let value_boolean = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r1);
      let zero = b.const_double(0.0);
      let zero2 = b.const_double(0.0);
      let value_nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, zero, zero2);

      let r0 = b.vm_reg(0);
      let op_tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let op_nil = b.const_tag(TNIL);
      let op_bool = b.const_tag(TBOOLEAN);
      let op_num = b.const_tag(TNUMBER);

      let eq = b.cond(IrCondition::Equal);
      let neq = b.cond(IrCondition::NotEqual);

      let zero = b.const_int(0);
      store_cmp_split!(b, 2, op_nil, op_bool, zero, value_boolean, eq);
      let zero1 = b.const_int(0);
      let zero2 = b.const_int(0);
      store_cmp_split!(b, 3, op_bool, op_bool, zero1, zero2, eq);
      let zero = b.const_int(0);
      let one = b.const_int(1);
      store_cmp_split!(b, 4, op_bool, op_bool, zero, one, eq);
      let zero1 = b.const_double(0.0);
      let zero2 = b.const_double(0.0);
      store_cmp_split!(b, 5, op_num, op_num, zero1, zero2, eq);
      let zero = b.const_double(0.0);
      let one = b.const_double(1.0);
      store_cmp_split!(b, 6, op_num, op_num, zero, one, eq);

      let zero = b.const_int(0);
      store_cmp_split!(b, 7, op_nil, op_bool, zero, value_boolean, neq);
      let zero1 = b.const_int(0);
      let zero2 = b.const_int(0);
      store_cmp_split!(b, 8, op_bool, op_bool, zero1, zero2, neq);
      let zero = b.const_int(0);
      let one = b.const_int(1);
      store_cmp_split!(b, 9, op_bool, op_bool, zero, one, neq);
      let zero1 = b.const_double(0.0);
      let zero2 = b.const_double(0.0);
      store_cmp_split!(b, 10, op_num, op_num, zero1, zero2, neq);
      let zero = b.const_double(0.0);
      let one = b.const_double(1.0);
      store_cmp_split!(b, 11, op_num, op_num, zero, one, neq);

      let zero1 = b.const_int(0);
      let zero2 = b.const_int(0);
      store_cmp_split!(b, 12, op_tag, op_bool, zero1, zero2, eq);
      let zero = b.const_int(0);
      let one = b.const_int(1);
      store_cmp_split!(b, 13, op_tag, op_bool, zero, one, eq);
      let zero1 = b.const_double(0.0);
      let zero2 = b.const_double(0.0);
      store_cmp_split!(b, 14, op_tag, op_num, zero1, zero2, eq);
      let zero = b.const_double(0.0);
      let one = b.const_double(1.0);
      store_cmp_split!(b, 15, op_tag, op_num, zero, one, eq);

      let zero1 = b.const_int(0);
      let zero2 = b.const_int(0);
      store_cmp_split!(b, 16, op_tag, op_bool, zero1, zero2, neq);
      let zero = b.const_int(0);
      let one = b.const_int(1);
      store_cmp_split!(b, 17, op_tag, op_bool, zero, one, neq);
      let zero1 = b.const_double(0.0);
      let zero2 = b.const_double(0.0);
      store_cmp_split!(b, 18, op_tag, op_num, zero1, zero2, neq);
      let zero = b.const_double(0.0);
      let one = b.const_double(1.0);
      store_cmp_split!(b, 19, op_tag, op_num, zero, one, neq);

      store_cmp_split!(b, 20, op_num, op_num, value_nan, value_nan, eq);
      store_cmp_split!(b, 21, op_num, op_num, value_nan, value_nan, neq);
      store_cmp_split!(b, 22, op_tag, op_num, value_nan, value_nan, eq);
      store_cmp_split!(b, 23, op_tag, op_num, value_nan, value_nan, neq);

      let r2 = b.vm_reg(2);
      let twenty_one = b.const_int(21);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, twenty_one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %2 = LOAD_TAG R0\n   STORE_INT R2, 0i\n   STORE_INT R3, 1i\n   STORE_INT R4, 0i\n   STORE_INT R5, 1i\n   STORE_INT R6, 0i\n   STORE_INT R7, 1i\n   STORE_INT R8, 0i\n   STORE_INT R9, 1i\n   STORE_INT R10, 0i\n   STORE_INT R11, 1i\n   %23 = CMP_TAG %2, tboolean, eq\n   STORE_INT R12, %23\n   STORE_INT R13, 0i\n   %27 = CMP_TAG %2, tnumber, eq\n   STORE_INT R14, %27\n   STORE_INT R15, 0i\n   %31 = CMP_TAG %2, tboolean, not_eq\n   STORE_INT R16, %31\n   STORE_INT R17, 1i\n   %35 = CMP_TAG %2, tnumber, not_eq\n   STORE_INT R18, %35\n   STORE_INT R19, 1i\n   STORE_INT R20, 0i\n   STORE_INT R21, 1i\n   STORE_INT R22, 0i\n   STORE_INT R23, 1i\n   RETURN R2, 21i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_cmp_tag_simplification {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3523:ir_builder_cmp_tag_simplification`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_cmp_tag_simplification

  #[cfg(test)]
  #[test]
  fn ir_builder_cmp_tag_simplification() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let eq = b.cond(IrCondition::Equal);
      let neq = b.cond(IrCondition::NotEqual);

      let tnil = b.const_tag(TNIL);
      let tnumber = b.const_tag(TNUMBER);
      let cmp = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpTag, tnil, tnumber, eq);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, cmp);
      let tnumber1 = b.const_tag(TNUMBER);
      let tnumber2 = b.const_tag(TNUMBER);
      let cmp = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpTag, tnumber1, tnumber2, eq);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, cmp);
      let tnil = b.const_tag(TNIL);
      let tnumber = b.const_tag(TNUMBER);
      let cmp = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpTag, tnil, tnumber, neq);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r2, cmp);
      let tnumber1 = b.const_tag(TNUMBER);
      let tnumber2 = b.const_tag(TNUMBER);
      let cmp = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpTag, tnumber1, tnumber2, neq);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r3, cmp);

      let r0 = b.vm_reg(0);
      let four = b.const_int(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, four);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 0i\n   STORE_INT R1, 1i\n   STORE_INT R2, 1i\n   STORE_INT R3, 0i\n   RETURN R0, 4i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_concat_invalidation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2815:ir_builder_concat_invalidation`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_concat_invalidation

  #[cfg(test)]
  #[test]
  fn ir_builder_concat_invalidation() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r1 = b.vm_reg(1);
      let ten = b.const_int(10);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, ten);
      let r2 = b.vm_reg(2);
      let half = b.const_double(0.5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, half);
      let r3 = b.vm_reg(3);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r3, two);

      let r0 = b.vm_reg(0);
      let count = b.const_uint(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CONCAT, r0, count);

      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r4, tag);
      let r1 = b.vm_reg(1);
      let int = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r1);
      let r5 = b.vm_reg(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r5, int);
      let r2 = b.vm_reg(2);
      let double = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
      let r6 = b.vm_reg(6);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r6, double);
      let r3 = b.vm_reg(3);
      let double = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let r7 = b.vm_reg(7);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r7, double);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   STORE_INT R1, 10i\n   STORE_DOUBLE R2, 0.5\n   STORE_DOUBLE R3, 2\n   CONCAT R0, 3u\n   %5 = LOAD_TAG R0\n   STORE_TAG R4, %5\n   %7 = LOAD_INT R1\n   STORE_INT R5, %7\n   %9 = LOAD_DOUBLE R2\n   STORE_DOUBLE R6, %9\n   STORE_DOUBLE R7, 2\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_control_flow_cmp_float {
  #[cfg(test)]
  #[test]
  fn ir_builder_control_flow_cmp_float() {
    use ulua_code_gen::{
      enums::{ir_cmd::IrCmd, ir_condition::IrCondition, ir_op_kind::IrOpKind},
      functions::update_use_counts::update_use_counts,
      records::{ir_inst::IrInst, ir_op::IrOp},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    fn compare_fold(
      fix: &mut IrBuilderFixture,
      lhs: IrOp,
      rhs: IrOp,
      cond: IrCondition,
      result: bool,
    ) {
      let mut inst_op = IrOp::default();
      let mut expected_target = IrOp::default();

      fix.with_two_blocks(|b, a, bb| {
        let z1 = b.const_double(0.0);
        let z2 = b.const_double(0.0);
        let nan_val = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, z1, z2);
        let l = if lhs.kind() == IrOpKind::None {
          nan_val
        } else {
          lhs
        };
        let r = if rhs.kind() == IrOpKind::None {
          nan_val
        } else {
          rhs
        };
        let c = b.cond(cond);
        inst_op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpCmpFloat, l, r, c, a, bb);
        expected_target = if result { a } else { bb };
      });

      update_use_counts(&mut fix.build.function);
      fix.constant_fold();

      let expected = IrInst::ir_inst_new(IrCmd::JUMP, &[expected_target]);
      fix.check_eq(inst_op, &expected);
    }

    let mut fix = IrBuilderFixture::new();
    let nan = IrOp::default();

    macro_rules! cfd {
      ($a:expr, $bv:expr, $cond:expr, $res:expr) => {{
        let lhs = fix.build.const_double($a);
        let rhs = fix.build.const_double($bv);
        compare_fold(&mut fix, lhs, rhs, $cond, $res);
      }};
    }

    cfd!(1.0, 1.0, IrCondition::Equal, true);
    cfd!(1.0, 2.0, IrCondition::Equal, false);
    compare_fold(&mut fix, nan, nan, IrCondition::Equal, false);

    cfd!(1.0, 1.0, IrCondition::NotEqual, false);
    cfd!(1.0, 2.0, IrCondition::NotEqual, true);
    compare_fold(&mut fix, nan, nan, IrCondition::NotEqual, true);

    cfd!(1.0, 1.0, IrCondition::Less, false);
    cfd!(1.0, 2.0, IrCondition::Less, true);
    cfd!(2.0, 1.0, IrCondition::Less, false);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::Less, false);
    }

    cfd!(1.0, 1.0, IrCondition::LessEqual, true);
    cfd!(1.0, 2.0, IrCondition::LessEqual, true);
    cfd!(2.0, 1.0, IrCondition::LessEqual, false);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::LessEqual, false);
    }

    cfd!(1.0, 1.0, IrCondition::Greater, false);
    cfd!(1.0, 2.0, IrCondition::Greater, false);
    cfd!(2.0, 1.0, IrCondition::Greater, true);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::Greater, false);
    }

    cfd!(1.0, 1.0, IrCondition::GreaterEqual, true);
    cfd!(1.0, 2.0, IrCondition::GreaterEqual, false);
    cfd!(2.0, 1.0, IrCondition::GreaterEqual, true);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::GreaterEqual, false);
    }
  }
}

mod ir_builder_control_flow_cmp_int {
  #[cfg(test)]
  #[test]
  fn ir_builder_control_flow_cmp_int() {
    use ulua_code_gen::{
      enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
      functions::update_use_counts::update_use_counts,
      records::{ir_inst::IrInst, ir_op::IrOp},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    // lhs/rhs are concrete const ops created against `fix.build` before the call.
    fn compare_fold(
      fix: &mut IrBuilderFixture,
      lhs: IrOp,
      rhs: IrOp,
      cond: IrCondition,
      result: bool,
    ) {
      let mut inst_op = IrOp::default();
      let mut expected_target = IrOp::default();

      fix.with_two_blocks(|b, a, bb| {
        let c = b.cond(cond);
        inst_op =
          b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpCmpInt, lhs, rhs, c, a, bb);
        expected_target = if result { a } else { bb };
      });

      update_use_counts(&mut fix.build.function);
      fix.constant_fold();

      let expected = IrInst::ir_inst_new(IrCmd::JUMP, &[expected_target]);
      fix.check_eq(inst_op, &expected);
    }

    let mut fix = IrBuilderFixture::new();

    macro_rules! cf {
      ($a:expr, $bv:expr, $cond:expr, $res:expr) => {{
        let lhs = fix.build.const_int($a);
        let rhs = fix.build.const_int($bv);
        compare_fold(&mut fix, lhs, rhs, $cond, $res);
      }};
    }

    cf!(1, 1, IrCondition::Equal, true);
    cf!(1, 2, IrCondition::Equal, false);

    cf!(1, 1, IrCondition::NotEqual, false);
    cf!(1, 2, IrCondition::NotEqual, true);

    cf!(1, 1, IrCondition::Less, false);
    cf!(1, 2, IrCondition::Less, true);
    cf!(2, 1, IrCondition::Less, false);

    cf!(1, 1, IrCondition::NotLess, true);
    cf!(1, 2, IrCondition::NotLess, false);
    cf!(2, 1, IrCondition::NotLess, true);

    cf!(1, 1, IrCondition::LessEqual, true);
    cf!(1, 2, IrCondition::LessEqual, true);
    cf!(2, 1, IrCondition::LessEqual, false);

    cf!(1, 1, IrCondition::NotLessEqual, false);
    cf!(1, 2, IrCondition::NotLessEqual, false);
    cf!(2, 1, IrCondition::NotLessEqual, true);

    cf!(1, 1, IrCondition::Greater, false);
    cf!(1, 2, IrCondition::Greater, false);
    cf!(2, 1, IrCondition::Greater, true);

    cf!(1, 1, IrCondition::NotGreater, true);
    cf!(1, 2, IrCondition::NotGreater, true);
    cf!(2, 1, IrCondition::NotGreater, false);

    cf!(1, 1, IrCondition::GreaterEqual, true);
    cf!(1, 2, IrCondition::GreaterEqual, false);
    cf!(2, 1, IrCondition::GreaterEqual, true);

    cf!(1, 1, IrCondition::NotGreaterEqual, false);
    cf!(1, 2, IrCondition::NotGreaterEqual, true);
    cf!(2, 1, IrCondition::NotGreaterEqual, false);
  }
}

mod ir_builder_control_flow_cmp_num {
  #[cfg(test)]
  #[test]
  fn ir_builder_control_flow_cmp_num() {
    use ulua_code_gen::{
      enums::{ir_cmd::IrCmd, ir_condition::IrCondition, ir_op_kind::IrOpKind},
      functions::update_use_counts::update_use_counts,
      records::{ir_inst::IrInst, ir_op::IrOp},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    // A `None`-kind op signals a placement of a freshly-built `nan` inside the
    // block (matching the C++ `lhs.kind == IrOpKind::None ? nan : lhs`).
    fn compare_fold(
      fix: &mut IrBuilderFixture,
      lhs: IrOp,
      rhs: IrOp,
      cond: IrCondition,
      result: bool,
    ) {
      let mut inst_op = IrOp::default();
      let mut expected_target = IrOp::default();

      fix.with_two_blocks(|b, a, bb| {
        let z1 = b.const_double(0.0);
        let z2 = b.const_double(0.0);
        let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, z1, z2);
        let l = if lhs.kind() == IrOpKind::None {
          nan
        } else {
          lhs
        };
        let r = if rhs.kind() == IrOpKind::None {
          nan
        } else {
          rhs
        };
        let c = b.cond(cond);
        inst_op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpCmpNum, l, r, c, a, bb);
        expected_target = if result { a } else { bb };
      });

      update_use_counts(&mut fix.build.function);
      fix.constant_fold();

      let expected = IrInst::ir_inst_new(IrCmd::JUMP, &[expected_target]);
      fix.check_eq(inst_op, &expected);
    }

    let mut fix = IrBuilderFixture::new();
    let nan = IrOp::default();

    macro_rules! cfd {
      ($a:expr, $bv:expr, $cond:expr, $res:expr) => {{
        let lhs = fix.build.const_double($a);
        let rhs = fix.build.const_double($bv);
        compare_fold(&mut fix, lhs, rhs, $cond, $res);
      }};
    }

    cfd!(1.0, 1.0, IrCondition::Equal, true);
    cfd!(1.0, 2.0, IrCondition::Equal, false);
    compare_fold(&mut fix, nan, nan, IrCondition::Equal, false);

    cfd!(1.0, 1.0, IrCondition::NotEqual, false);
    cfd!(1.0, 2.0, IrCondition::NotEqual, true);
    compare_fold(&mut fix, nan, nan, IrCondition::NotEqual, true);

    cfd!(1.0, 1.0, IrCondition::Less, false);
    cfd!(1.0, 2.0, IrCondition::Less, true);
    cfd!(2.0, 1.0, IrCondition::Less, false);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::Less, false);
    }

    cfd!(1.0, 1.0, IrCondition::NotLess, true);
    cfd!(1.0, 2.0, IrCondition::NotLess, false);
    cfd!(2.0, 1.0, IrCondition::NotLess, true);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::NotLess, true);
    }

    cfd!(1.0, 1.0, IrCondition::LessEqual, true);
    cfd!(1.0, 2.0, IrCondition::LessEqual, true);
    cfd!(2.0, 1.0, IrCondition::LessEqual, false);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::LessEqual, false);
    }

    cfd!(1.0, 1.0, IrCondition::NotLessEqual, false);
    cfd!(1.0, 2.0, IrCondition::NotLessEqual, false);
    cfd!(2.0, 1.0, IrCondition::NotLessEqual, true);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::NotLessEqual, true);
    }

    cfd!(1.0, 1.0, IrCondition::Greater, false);
    cfd!(1.0, 2.0, IrCondition::Greater, false);
    cfd!(2.0, 1.0, IrCondition::Greater, true);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::Greater, false);
    }

    cfd!(1.0, 1.0, IrCondition::NotGreater, true);
    cfd!(1.0, 2.0, IrCondition::NotGreater, true);
    cfd!(2.0, 1.0, IrCondition::NotGreater, false);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::NotGreater, true);
    }

    cfd!(1.0, 1.0, IrCondition::GreaterEqual, true);
    cfd!(1.0, 2.0, IrCondition::GreaterEqual, false);
    cfd!(2.0, 1.0, IrCondition::GreaterEqual, true);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::GreaterEqual, false);
    }

    cfd!(1.0, 1.0, IrCondition::NotGreaterEqual, false);
    cfd!(1.0, 2.0, IrCondition::NotGreaterEqual, true);
    cfd!(2.0, 1.0, IrCondition::NotGreaterEqual, false);
    {
      let lhs = fix.build.const_double(1.0);
      compare_fold(&mut fix, lhs, nan, IrCondition::NotGreaterEqual, true);
    }
  }
}

mod ir_builder_control_flow_eq {
  #[cfg(test)]
  #[test]
  fn ir_builder_control_flow_eq() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_cmd::IrCmd, ir_condition::IrCondition},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();

    fix.with_two_blocks(|b, a, bb| {
      let t1 = b.const_tag(TNIL);
      let t2 = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, t1, t2, a, bb);
    });

    fix.with_two_blocks(|b, a, bb| {
      let t1 = b.const_tag(TNIL);
      let t2 = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, t1, t2, a, bb);
    });

    fix.with_two_blocks(|b, a, bb| {
      let c1 = b.const_int(0);
      let c2 = b.const_int(0);
      let cond = b.cond(IrCondition::Equal);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpCmpInt, c1, c2, cond, a, bb);
    });

    fix.with_two_blocks(|b, a, bb| {
      let c1 = b.const_int(0);
      let c2 = b.const_int(1);
      let cond = b.cond(IrCondition::Equal);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpCmpInt, c1, c2, cond, a, bb);
    });

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\nbb_3:\n   JUMP bb_5\n\nbb_5:\n   RETURN 2u\n\nbb_6:\n   JUMP bb_7\n\nbb_7:\n   RETURN 1u\n\nbb_9:\n   JUMP bb_11\n\nbb_11:\n   RETURN 2u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_data_does_not_flow_through_direct_jump_to_non_unique_successor {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3209:ir_builder_data_does_not_flow_through_direct_jump_to_non_unique_successor`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_data_does_not_flow_through_direct_jump_to_non_unique_successor

  #[cfg(test)]
  #[test]
  fn ir_builder_data_does_not_flow_through_direct_jump_to_non_unique_successor() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block1 = b.block(IrBlockKind::Internal);
      let block2 = b.block(IrBlockKind::Internal);
      let block3 = b.block(IrBlockKind::Internal);

      b.begin_block(block1);

      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

      b.begin_block(block2);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tag);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);

      b.begin_block(block3);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   JUMP bb_1\n\nbb_1:\n   %2 = LOAD_TAG R0\n   STORE_TAG R1, %2\n   RETURN 1u\n\nbb_2:\n   JUMP bb_1\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_data_flows_through_direct_jump_to_unique_successor {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3179:ir_builder_data_flows_through_direct_jump_to_unique_successor`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_data_flows_through_direct_jump_to_unique_successor

  #[cfg(test)]
  #[test]
  fn ir_builder_data_flows_through_direct_jump_to_unique_successor() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block1 = b.block(IrBlockKind::Internal);
      let block2 = b.block(IrBlockKind::Internal);

      b.begin_block(block1);

      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

      b.begin_block(block2);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tag);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   JUMP bb_1\n; glued to: bb_1\n\nbb_1:\n   STORE_TAG R1, tnumber\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_do_not_produce_invalid_split_store_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4870:ir_builder_do_not_produce_invalid_split_store_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_do_not_produce_invalid_split_store_1

  #[cfg(test)]
  #[test]
  fn ir_builder_do_not_produce_invalid_split_store_1() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;
    use ulua_vm::enums::lua_type::LuaType;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, one);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let ttable = b.const_tag(LuaType::Table as u8);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, ttable, exit);
      let r0 = b.vm_reg(0);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, value);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 1i\n   %1 = LOAD_TAG R0\n   CHECK_TAG %1, ttable, exit(1)\n   %3 = LOAD_TVALUE R0, 0i, ttable\n   STORE_TVALUE R1, %3\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_do_not_produce_invalid_split_store_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4896:ir_builder_do_not_produce_invalid_split_store_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_do_not_produce_invalid_split_store_2

  #[cfg(test)]
  #[test]
  fn ir_builder_do_not_produce_invalid_split_store_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;
    use ulua_vm::enums::lua_type::LuaType;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, one);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(LuaType::Number as u8);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, value);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 1i\n   %1 = LOAD_TAG R0\n   CHECK_TAG %1, tnumber, exit(1)\n   %3 = LOAD_TVALUE R0, 0i, tnumber\n   STORE_TVALUE R1, %3\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_do_not_produce_invalid_split_store_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4922:ir_builder_do_not_produce_invalid_split_store_3`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_do_not_produce_invalid_split_store_3

  #[cfg(test)]
  #[test]
  fn ir_builder_do_not_produce_invalid_split_store_3() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;
    use ulua_vm::enums::lua_type::LuaType;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);

      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, two);

      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(LuaType::Number as u8);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);

      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(LuaType::Number as u8);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let r0 = b.vm_reg(0);
      let double = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, double);

      let r0 = b.vm_reg(0);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, value);
      let r1 = b.vm_reg(1);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, two);
      let r1 = b.vm_reg(1);
      let tboolean = b.const_tag(LuaType::Boolean as u8);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tboolean);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 2i\n   %1 = LOAD_TAG R0\n   CHECK_TAG %1, tnumber, exit(1)\n   STORE_TAG R2, tnumber\n   %4 = LOAD_DOUBLE R0\n   STORE_DOUBLE R2, %4\n   %6 = LOAD_TVALUE R0, 0i, tnumber\n   STORE_TVALUE R1, %6\n   STORE_TAG R1, tboolean\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_do_not_return_with_partial_stores {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6785:ir_builder_do_not_return_with_partial_stores`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_do_not_return_with_partial_stores

  #[cfg(test)]
  #[test]
  fn ir_builder_do_not_return_with_partial_stores() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TBOOLEAN: u8 = 1;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let success = b.block(IrBlockKind::Internal);
      let fail = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let zero_u = b.const_uint(0);
      let zero_u_2 = b.const_uint(0);
      let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, zero_u, zero_u_2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, table);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      let big = b.const_double(1e20);
      let to_uint = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, big);
      let four = b.const_int(4);
      let bit_and = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, to_uint, four);
      let zero = b.const_int(0);
      let equal = b.cond(IrCondition::Equal);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpInt,
        bit_and,
        zero,
        equal,
        success,
        fail,
      );

      b.begin_block(success);
      let r1 = b.vm_reg(1);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, zero);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(fail);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, one);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r1 = b.vm_reg(1);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tboolean);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1, bb_2\n; in regs: R0\n; out regs: R0\n   %3 = NUM_TO_UINT 1e+20\n   %4 = BITAND_UINT %3, 4i\n   JUMP_CMP_INT %4, 0i, eq, bb_1, bb_2\n\nbb_1:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R0\n; out regs: R0\n   STORE_INT R1, 0i\n   JUMP bb_3\n\nbb_2:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R0\n; out regs: R0\n   STORE_INT R1, 1i\n   JUMP bb_3\n\nbb_3:\n; predecessors: bb_1, bb_2\n; in regs: R0\n   STORE_TAG R1, tboolean\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dominance_verification_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5358:ir_builder_dominance_verification_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - calls -> method IrBuilderFixture::defineCfgTree (tests/IrBuilder.test.cpp)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - translates_to -> rust_item ir_builder_dominance_verification_1

  #[cfg(test)]
  #[test]
  fn ir_builder_dominance_verification_1() {
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    fix.define_cfg_tree(&vec![vec![1, 2], vec![3], vec![4], vec![4], vec![3]]);

    assert_eq!(fix.build.function.cfg.idoms, vec![!0u32, 0, 0, 0, 0]);
  }
}

mod ir_builder_dominance_verification_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5366:ir_builder_dominance_verification_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - calls -> method IrBuilderFixture::defineCfgTree (tests/IrBuilder.test.cpp)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - translates_to -> rust_item ir_builder_dominance_verification_2

  #[cfg(test)]
  #[test]
  fn ir_builder_dominance_verification_2() {
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    fix.define_cfg_tree(&vec![
      vec![1, 16],
      vec![2, 3, 4],
      vec![4, 7],
      vec![9],
      vec![5],
      vec![6],
      vec![2, 8],
      vec![8],
      vec![7, 15],
      vec![10, 11],
      vec![12],
      vec![12],
      vec![13],
      vec![3, 14, 15],
      vec![12],
      vec![16],
      vec![],
    ]);

    assert_eq!(
      fix.build.function.cfg.idoms,
      vec![!0u32, 0, 1, 1, 1, 4, 5, 1, 1, 3, 9, 9, 9, 12, 13, 1, 0]
    );
  }
}

mod ir_builder_dominance_verification_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5374:ir_builder_dominance_verification_3`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - calls -> method IrBuilderFixture::defineCfgTree (tests/IrBuilder.test.cpp)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - translates_to -> rust_item ir_builder_dominance_verification_3

  #[cfg(test)]
  #[test]
  fn ir_builder_dominance_verification_3() {
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    fix.define_cfg_tree(&vec![
      vec![1, 2],
      vec![3],
      vec![3, 4],
      vec![5],
      vec![5, 6],
      vec![7],
      vec![7],
      vec![],
    ]);

    assert_eq!(
      fix.build.function.cfg.idoms,
      vec![!0u32, 0, 0, 0, 2, 0, 4, 0]
    );
  }
}

mod ir_builder_dominance_verification_4_ir_builder_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5382:ir_builder_dominance_verification_4`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - calls -> method IrBuilderFixture::defineCfgTree (tests/IrBuilder.test.cpp)
  //!   - type_ref -> record IdfContext (CodeGen/include/Luau/IrAnalysis.h)
  //!   - calls -> function computeIteratedDominanceFrontierForDefs (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - translates_to -> rust_item ir_builder_dominance_verification_4

  #[cfg(test)]
  #[test]
  fn ir_builder_dominance_verification_4() {
    use ulua_code_gen::{
      functions::compute_iterated_dominance_frontier_for_defs::compute_iterated_dominance_frontier_for_defs,
      records::idf_context::IdfContext,
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    fix.define_cfg_tree(&vec![
      vec![1],
      vec![2, 10],
      vec![3, 7],
      vec![4],
      vec![5],
      vec![4, 6],
      vec![1],
      vec![8],
      vec![5, 9],
      vec![7],
      vec![],
    ]);

    let mut ctx = IdfContext::default();
    compute_iterated_dominance_frontier_for_defs(
      &mut ctx,
      &fix.build.function,
      &[0, 2, 3, 6],
      &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    assert_eq!(ctx.idf, vec![1, 4, 5]);
  }
}

mod ir_builder_dominance_verification_4_ir_builder_test_alt_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5393:ir_builder_dominance_verification_4`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - calls -> method IrBuilderFixture::defineCfgTree (tests/IrBuilder.test.cpp)
  //!   - type_ref -> record IdfContext (CodeGen/include/Luau/IrAnalysis.h)
  //!   - calls -> function computeIteratedDominanceFrontierForDefs (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - translates_to -> rust_item ir_builder_dominance_verification_4

  #[cfg(test)]
  #[test]
  fn ir_builder_dominance_verification_4() {
    use ulua_code_gen::{
      functions::compute_iterated_dominance_frontier_for_defs::compute_iterated_dominance_frontier_for_defs,
      records::idf_context::IdfContext,
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    fix.define_cfg_tree(&vec![
      vec![1],
      vec![2],
      vec![3, 7],
      vec![4, 5],
      vec![6],
      vec![6],
      vec![8],
      vec![8],
      vec![9],
      vec![10, 11],
      vec![11],
      vec![9, 12],
      vec![2],
    ]);

    let mut ctx = IdfContext::default();
    compute_iterated_dominance_frontier_for_defs(
      &mut ctx,
      &fix.build.function,
      &[4, 5, 7, 12],
      &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    );
    assert_eq!(ctx.idf, vec![2, 6, 8]);

    compute_iterated_dominance_frontier_for_defs(
      &mut ctx,
      &fix.build.function,
      &[4, 5, 7, 12],
      &[6, 8, 9],
    );
    assert_eq!(ctx.idf, vec![6, 8]);
  }
}

mod ir_builder_dse_int_64_overwrite {
  #[cfg(test)]
  #[test]
  fn ir_builder_dse_int_64_overwrite() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TINTEGER: u8 = 4;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      b.begin_block(entry);

      let r0 = b.vm_reg(0);
      let val = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);

      let r1 = b.vm_reg(1);
      let tnum = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnum);
      let r1b = b.vm_reg(1);
      let d1 = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1b, d1);

      let r1c = b.vm_reg(1);
      let tint = b.const_tag(TINTEGER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1c, tint);
      let r1d = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r1d, val);

      let r1e = b.vm_reg(1);
      let c1 = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1e, c1);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   %0 = LOAD_INT64 R0\n   STORE_SPLIT_TVALUE R1, tinteger, %0\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_partial_store_with_known_tag_from_predecessors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7184:ir_builder_dse_partial_store_with_known_tag_from_predecessors`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_dse_partial_store_with_known_tag_from_predecessors

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_partial_store_with_known_tag_from_predecessors() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TSTRING: u8 = 6;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let other = b.block(IrBlockKind::Internal);
      let target = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, one);
      let r1 = b.vm_reg(1);
      let tag0 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, tag0, tnumber, target, other);

      b.begin_block(other);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r1 = b.vm_reg(1);
      let tag1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tstring = b.const_tag(TSTRING);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, tag1, tstring, target, exit);

      b.begin_block(target);
      let r0 = b.vm_reg(0);
      let load = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let ten = b.const_double(10.0);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, load, ten);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, sum);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, four);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);

      b.begin_block(exit);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_2, bb_1\n; in regs: R1\n; out regs: R0, R1\n   STORE_TAG R0, tnumber\n   STORE_DOUBLE R0, 1\n   %2 = LOAD_TAG R1\n   JUMP_EQ_TAG %2, tnumber, bb_2, bb_1\n\nbb_1:\n; predecessors: bb_0\n; successors: bb_2, bb_3\n; in regs: R1\n; out regs: R0, R1\n   STORE_TAG R0, tnumber\n   STORE_DOUBLE R0, 2\n   %6 = LOAD_TAG R1\n   JUMP_EQ_TAG %6, tstring, bb_2, bb_3\n\nbb_2:\n; predecessors: bb_0, bb_1\n; in regs: R0\n   STORE_DOUBLE R0, 4\n   RETURN R0, 1i\n\nbb_3:\n; predecessors: bb_1\n; in regs: R1\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_basic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7259:ir_builder_dse_vm_exit_sync_basic`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_basic

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_basic() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, two);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   %2 = LOAD_TAG R0\n   CHECK_TAG %2, tnumber, bb_exit_1\n   ; exit sync: R1, {}\n   RETURN R0, 1i\n\nbb_exit_1:\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, 2\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_deep_sink_chain {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7534:ir_builder_dse_vm_exit_sync_deep_sink_chain`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> function main (tests/main.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_deep_sink_chain

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_deep_sink_chain() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let load = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let one = b.const_double(1.0);
      let add1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, load, one);
      let two = b.const_double(2.0);
      let add2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, add1, two);
      let three = b.const_double(3.0);
      let add3 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, add2, three);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, add3);
      let r2 = b.vm_reg(2);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R2\n   %0 = LOAD_DOUBLE R0\n   %6 = LOAD_TAG R2\n   CHECK_TAG %6, tnumber, bb_exit_1\n   ; exit sync: R1, {%0}\n   RETURN R0, 1i\n\nbb_exit_1:\n   %9 = ADD_NUM %0, 1\n   %10 = ADD_NUM %9, 2\n   %11 = ADD_NUM %10, 3\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, %11\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_multiple_exit_registers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7334:ir_builder_dse_vm_exit_sync_multiple_exit_registers`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_multiple_exit_registers

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_multiple_exit_registers() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let load = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let one = b.const_double(1.0);
      let add = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, load, one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, add);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, load);
      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let r3 = b.vm_reg(3);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r3);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R3\n   %0 = LOAD_DOUBLE R0\n   %6 = LOAD_TAG R3\n   CHECK_TAG %6, tnumber, bb_exit_1\n   ; exit sync: R2, R1, {%0}\n   RETURN R0, 1i\n\nbb_exit_1:\n   %9 = ADD_NUM %0, 1\n   STORE_TAG R2, tnumber\n   STORE_DOUBLE R2, %0\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, %9\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_multiple_registers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7460:ir_builder_dse_vm_exit_sync_multiple_registers`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_multiple_registers

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_multiple_registers() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r3 = b.vm_reg(3);
      let tval = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r3);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, tval);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R3\n   %2 = LOAD_TVALUE R3\n   %4 = LOAD_TAG R0\n   CHECK_TAG %4, tnumber, bb_exit_1\n   ; exit sync: R2, R1, {%2}\n   RETURN R0, 1i\n\nbb_exit_1:\n   STORE_TVALUE R2, %2\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, 1\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_no_record_after_guard {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7502:ir_builder_dse_vm_exit_sync_no_record_after_guard`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_no_record_after_guard

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_no_record_after_guard() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, two);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   %0 = LOAD_TAG R0\n   CHECK_TAG %0, tnumber, exit(1)\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, 2\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_sinking {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7293:ir_builder_dse_vm_exit_sync_sinking`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_sinking

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_sinking() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let load = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let one = b.const_double(1.0);
      let add = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, load, one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, add);
      let r2 = b.vm_reg(2);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R2\n   %0 = LOAD_DOUBLE R0\n   %4 = LOAD_TAG R2\n   CHECK_TAG %4, tnumber, bb_exit_1\n   ; exit sync: R1, {%0}\n   RETURN R0, 1i\n\nbb_exit_1:\n   %7 = ADD_NUM %0, 1\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, %7\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_sinking_no_inline_across_block {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7617:ir_builder_dse_vm_exit_sync_sinking_no_inline_across_block`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_sinking_no_inline_across_block

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_sinking_no_inline_across_block() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r3 = b.vm_reg(3);
      let load = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let lhs = b.const_double(11008.0);
      let sub = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SubNum, lhs, load);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, sub);
      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let exit = b.vm_exit(8);
      b.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);
    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R3\n   %0 = LOAD_DOUBLE R3\n   CHECK_SAFE_ENV bb_exit_1\n   ; exit sync: R2, {%0}\n   RETURN R0, 1i\n\nbb_exit_1:\n   %6 = SUB_NUM 11008, %0\n   STORE_TAG R2, tnumber\n   STORE_DOUBLE R2, %6\n   JUMP exit(8)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_store_tvalue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7425:ir_builder_dse_vm_exit_sync_store_tvalue`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_store_tvalue

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_store_tvalue() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let tval = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, tval);
      let r2 = b.vm_reg(2);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R2\n   %0 = LOAD_TVALUE R0\n   %2 = LOAD_TAG R2\n   CHECK_TAG %2, tnumber, bb_exit_1\n   ; exit sync: R1, {%0}\n   RETURN R0, 1i\n\nbb_exit_1:\n   STORE_TVALUE R1, %0\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_store_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7383:ir_builder_dse_vm_exit_sync_store_vector`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_store_vector

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_store_vector() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;
    const TVECTOR: u8 = 5;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      let x = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, r0, zero);
      let r0 = b.vm_reg(0);
      let four = b.const_int(4);
      let y = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, r0, four);
      let r0 = b.vm_reg(0);
      let eight = b.const_int(8);
      let z = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, r0, eight);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r1, x, y, z);
      let r1 = b.vm_reg(1);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tvector);
      let r2 = b.vm_reg(2);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R2\n   %0 = LOAD_FLOAT R0, 0i\n   %1 = LOAD_FLOAT R0, 4i\n   %2 = LOAD_FLOAT R0, 8i\n   %5 = LOAD_TAG R2\n   CHECK_TAG %5, tnumber, bb_exit_1\n   ; exit sync: R1, {%0, %1, %2}\n   RETURN R0, 1i\n\nbb_exit_1:\n   STORE_TAG R1, tvector\n   STORE_VECTOR R1, %0, %1, %2\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_user_call_prevents_sync {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7581:ir_builder_dse_vm_exit_sync_user_call_prevents_sync`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_user_call_prevents_sync

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_user_call_prevents_sync() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let load = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, load);
      let r3 = b.vm_reg(3);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, r3, r2);
      let r4 = b.vm_reg(4);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r4);
      let tnumber = b.const_tag(TNUMBER);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R2, R4\n   %0 = LOAD_DOUBLE R0\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, %0\n   DO_LEN R3, R2\n   %4 = LOAD_TAG R4\n   CHECK_TAG %4, tnumber, exit(1)\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_dse_vm_exit_sync_vector_full_store {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7659:ir_builder_dse_vm_exit_sync_vector_full_store`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_vector_full_store

  #[cfg(test)]
  #[test]
  fn ir_builder_dse_vm_exit_sync_vector_full_store() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TVECTOR: u8 = 5;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tvector);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      let two = b.const_double(2.0);
      let three = b.const_double(3.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r1, one, two, three);
      let r1 = b.vm_reg(1);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tvector);
      let exit = b.vm_exit(20);
      b.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   CHECK_SAFE_ENV bb_exit_1\n   ; exit sync: R1, {}\n   RETURN R0, 1i\n\nbb_exit_1:\n   STORE_VECTOR R1, 1, 2, 3, tvector\n   JUMP exit(20)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_array_elem_checks_invalidations {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4442:ir_builder_duplicate_array_elem_checks_invalidations`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_array_elem_checks_invalidations

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_array_elem_checks_invalidations() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let table1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, zero, fallback);
      let zero = b.const_int(0);
      let elem1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, zero);
      let zero = b.const_int(0);
      let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem1, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1);

      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::TableSetnum, table1, two);

      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, zero, fallback);
      let zero = b.const_int(0);
      let elem2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, zero);
      let zero = b.const_int(0);
      let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem2, zero);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r4, value1b);

      let r3 = b.vm_reg(3);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let r4 = b.vm_reg(4);
      let b_value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r4);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a, b_value);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, sum);

      let r2 = b.vm_reg(2);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   CHECK_ARRAY_SIZE %0, 0i, bb_fallback_1\n   %2 = GET_ARR_ADDR %0, 0i\n   %3 = LOAD_TVALUE %2, 0i\n   STORE_TVALUE R3, %3\n   %5 = TABLE_SETNUM %0, 2i\n   CHECK_ARRAY_SIZE %0, 0i, bb_fallback_1\n   %7 = GET_ARR_ADDR %0, 0i\n   %8 = LOAD_TVALUE %7, 0i\n   STORE_TVALUE R4, %8\n   %10 = LOAD_DOUBLE R3\n   %11 = LOAD_DOUBLE R4\n   %12 = ADD_NUM %10, %11\n   STORE_DOUBLE R2, %12\n   RETURN R2, 1u\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_array_elem_checks_lower_index {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4388:ir_builder_duplicate_array_elem_checks_lower_index`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_array_elem_checks_lower_index

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_array_elem_checks_lower_index() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let table1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let one_i = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, one_i, fallback);
      let one_i = b.const_int(1);
      let elem1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, one_i);
      let zero = b.const_int(0);
      let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem1, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1);

      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, zero, fallback);
      let zero = b.const_int(0);
      let elem2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, zero);
      let zero = b.const_int(0);
      let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem2, zero);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r4, value1b);

      let r3 = b.vm_reg(3);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let r4 = b.vm_reg(4);
      let b_value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r4);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a, b_value);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, sum);

      let r2 = b.vm_reg(2);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   CHECK_ARRAY_SIZE %0, 1i, bb_fallback_1\n   %2 = GET_ARR_ADDR %0, 1i\n   %3 = LOAD_TVALUE %2, 0i\n   STORE_TVALUE R3, %3\n   %6 = GET_ARR_ADDR %0, 0i\n   %7 = LOAD_TVALUE %6, 0i\n   STORE_TVALUE R4, %7\n   %9 = LOAD_DOUBLE R3\n   %10 = LOAD_DOUBLE R4\n   %11 = ADD_NUM %9, %10\n   STORE_DOUBLE R2, %11\n   RETURN R2, 1u\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_array_elem_checks_same_index {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4278:ir_builder_duplicate_array_elem_checks_same_index`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_array_elem_checks_same_index

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_array_elem_checks_same_index() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let table1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, zero, fallback);
      let zero = b.const_int(0);
      let elem1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, zero);
      let zero = b.const_int(0);
      let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem1, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1);

      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, zero, fallback);
      let zero = b.const_int(0);
      let elem2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, zero);
      let zero = b.const_int(0);
      let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem2, zero);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r4, value1b);

      let r3 = b.vm_reg(3);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let r4 = b.vm_reg(4);
      let b_value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r4);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a, b_value);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, sum);

      let r2 = b.vm_reg(2);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   CHECK_ARRAY_SIZE %0, 0i, bb_fallback_1\n   %2 = GET_ARR_ADDR %0, 0i\n   %3 = LOAD_TVALUE %2, 0i\n   STORE_TVALUE R3, %3\n   STORE_TVALUE R4, %3\n   %9 = LOAD_DOUBLE R3\n   %11 = ADD_NUM %9, %9\n   STORE_DOUBLE R2, %11\n   RETURN R2, 1u\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_array_elem_checks_same_value {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4329:ir_builder_duplicate_array_elem_checks_same_value`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_array_elem_checks_same_value

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_array_elem_checks_same_value() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let table1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let r2 = b.vm_reg(2);
      let index = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
      let valid_index = b.inst_ir_cmd_ir_op_ir_op(IrCmd::TryNumToIndex, index, fallback);
      let one = b.const_int(1);
      let valid_offset = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt, valid_index, one);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, valid_offset, fallback);
      let zero = b.const_int(0);
      let elem1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, zero);
      let zero = b.const_int(0);
      let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem1, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1);

      let valid_index2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::TryNumToIndex, index, fallback);
      let one = b.const_int(1);
      let valid_offset2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt, valid_index2, one);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table1, valid_offset2, fallback);
      let zero = b.const_int(0);
      let elem2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table1, zero);
      let zero = b.const_int(0);
      let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, elem2, zero);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r4, value1b);

      let r3 = b.vm_reg(3);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let r4 = b.vm_reg(4);
      let b_value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r4);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a, b_value);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, sum);

      let r2 = b.vm_reg(2);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   %1 = LOAD_DOUBLE R2\n   %2 = TRY_NUM_TO_INDEX %1, bb_fallback_1\n   %3 = SUB_INT %2, 1i\n   CHECK_ARRAY_SIZE %0, %3, bb_fallback_1\n   %5 = GET_ARR_ADDR %0, 0i\n   %6 = LOAD_TVALUE %5, 0i\n   STORE_TVALUE R3, %6\n   STORE_TVALUE R4, %6\n   %14 = LOAD_DOUBLE R3\n   %16 = ADD_NUM %14, %14\n   STORE_DOUBLE R2, %16\n   RETURN R2, 1u\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_buffer_length_checks {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4547:ir_builder_duplicate_buffer_length_checks`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_buffer_length_checks

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_buffer_length_checks() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };
    use ulua_vm::enums::lua_type::LuaType;

    let _load_propagate_origin = ScopedFastFlag::new(&FFlag::LuauCodegenLoadPropagateOrigin, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let source_buf = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);

      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, source_buf);
      let r2 = b.vm_reg(2);
      let buffer1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r2);
      let index = b.const_int(12);
      let start = b.const_int(0);
      let size = b.const_int(4);
      let source = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer1,
        index,
        start,
        size,
        source,
        fallback,
      );
      let index = b.const_int(12);
      let value = b.const_int(32);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei32, buffer1, index, value, tag);

      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, source_buf);
      let r2 = b.vm_reg(2);
      let buffer2 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r2);
      let index = b.const_int(8);
      let start = b.const_int(0);
      let size = b.const_int(4);
      let source = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer2,
        index,
        start,
        size,
        source,
        fallback,
      );
      let index = b.const_int(8);
      let value = b.const_int(30);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei32, buffer2, index, value, tag);

      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, source_buf);
      let r2 = b.vm_reg(2);
      let buffer3 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r2);
      let index = b.const_int(16);
      let start = b.const_int(0);
      let size = b.const_int(4);
      let source = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer3,
        index,
        start,
        size,
        source,
        fallback,
      );
      let index = b.const_int(16);
      let value = b.const_int(60);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei32, buffer3, index, value, tag);

      let index = b.const_int(16);
      let start = b.const_int(0);
      let size = b.const_int(2);
      let source = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer3,
        index,
        start,
        size,
        source,
        fallback,
      );
      let index = b.const_int(16);
      let value = b.const_int(55);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei16, buffer3, index, value, tag);

      let r1 = b.vm_reg(1);
      let index = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r1);
      let start = b.const_int(0);
      let size = b.const_int(2);
      let source = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer3,
        index,
        start,
        size,
        source,
        fallback,
      );
      let value = b.const_int(1);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei16, buffer3, index, value, tag);
      let start = b.const_int(0);
      let size = b.const_int(2);
      let source = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CheckBufferLen,
        buffer3,
        index,
        start,
        size,
        source,
        fallback,
      );
      let value = b.const_int(2);
      let tag = b.const_tag(LuaType::Buffer as u8);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei16, buffer3, index, value, tag);

      let r1 = b.vm_reg(1);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TVALUE R0\n   STORE_TVALUE R2, %0\n   %2 = LOAD_POINTER R0\n   CHECK_BUFFER_LEN %2, 12i, -4i, 8i, undef, bb_fallback_1\n   BUFFER_WRITEI32 %2, 12i, 32i, tbuffer\n   BUFFER_WRITEI32 %2, 8i, 30i, tbuffer\n   BUFFER_WRITEI32 %2, 16i, 60i, tbuffer\n   BUFFER_WRITEI16 %2, 16i, 55i, tbuffer\n   %15 = LOAD_INT R1\n   CHECK_BUFFER_LEN %2, %15, 0i, 2i, undef, bb_fallback_1\n   BUFFER_WRITEI16 %2, %15, 1i, tbuffer\n   BUFFER_WRITEI16 %2, %15, 2i, tbuffer\n   RETURN R1, 1u\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_hash_slot_checks {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4098:ir_builder_duplicate_hash_slot_checks`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_hash_slot_checks

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_hash_slot_checks() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let table1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let three = b.const_uint(3);
      let k1 = b.vm_const(1);
      let slot1 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, table1, three, k1);
      let k1 = b.vm_const(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, slot1, k1, fallback);
      let zero = b.const_int(0);
      let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, slot1, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1);

      let eight = b.const_uint(8);
      let k1 = b.vm_const(1);
      let slot1b = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, table1, eight, k1);
      let k1 = b.vm_const(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, slot1b, k1, fallback);
      let zero = b.const_int(0);
      let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, slot1b, zero);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r4, value1b);

      let r3 = b.vm_reg(3);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let r4 = b.vm_reg(4);
      let b_value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r4);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a, b_value);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, sum);

      let r2 = b.vm_reg(2);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   %1 = GET_SLOT_NODE_ADDR %0, 3u, K1\n   CHECK_SLOT_MATCH %1, K1, bb_fallback_1\n   %3 = LOAD_TVALUE %1, 0i\n   STORE_TVALUE R3, %3\n   STORE_TVALUE R4, %3\n   %9 = LOAD_DOUBLE R3\n   %11 = ADD_NUM %9, %9\n   STORE_DOUBLE R2, %11\n   RETURN R2, 1u\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_hash_slot_checks_avoid_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4149:ir_builder_duplicate_hash_slot_checks_avoid_nil`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_hash_slot_checks_avoid_nil

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_hash_slot_checks_avoid_nil() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let table1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let three = b.const_uint(3);
      let k1 = b.vm_const(1);
      let slot1 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, table1, three, k1);
      let k1 = b.vm_const(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, slot1, k1, fallback);
      let zero = b.const_int(0);
      let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, slot1, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1);

      let r2 = b.vm_reg(2);
      let table2 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r2);
      let six = b.const_uint(6);
      let k1 = b.vm_const(1);
      let slot2 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, table2, six, k1);
      let k1 = b.vm_const(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, slot2, k1, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table2, fallback);

      let r4 = b.vm_reg(4);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r4, tnil);
      let r4 = b.vm_reg(4);
      let value_nil = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r4);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, slot2, value_nil, zero);

      let eight = b.const_uint(8);
      let k1 = b.vm_const(1);
      let slot1b = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, table1, eight, k1);
      let k1 = b.vm_const(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, slot1b, k1, fallback);
      let zero = b.const_int(0);
      let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, slot1b, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1b);

      let eleven = b.const_uint(11);
      let k1 = b.vm_const(1);
      let slot2b = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, table2, eleven, k1);
      let k1 = b.vm_const(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, slot2b, k1, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table2, fallback);

      let tnumber = b.const_tag(TNUMBER);
      let one_double = b.const_double(1.0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::StoreSplitTvalue,
        slot2b,
        tnumber,
        one_double,
        zero,
      );

      let r3 = b.vm_reg(3);
      let two = b.const_uint(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r3, two);

      b.begin_block(fallback);
      let r1 = b.vm_reg(1);
      let two = b.const_uint(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   %1 = GET_SLOT_NODE_ADDR %0, 3u, K1\n   CHECK_SLOT_MATCH %1, K1, bb_fallback_1\n   %3 = LOAD_TVALUE %1, 0i\n   STORE_TVALUE R3, %3\n   %5 = LOAD_POINTER R2\n   %6 = GET_SLOT_NODE_ADDR %5, 6u, K1\n   CHECK_SLOT_MATCH %6, K1, bb_fallback_1\n   CHECK_READONLY %5, bb_fallback_1\n   STORE_TAG R4, tnil\n   %10 = LOAD_TVALUE R4, 0i, tnil\n   STORE_TVALUE %6, %10, 0i\n   CHECK_NODE_VALUE %1, bb_fallback_1\n   %14 = LOAD_TVALUE %1, 0i\n   STORE_TVALUE R3, %14\n   CHECK_NODE_VALUE %6, bb_fallback_1\n   STORE_SPLIT_TVALUE %6, tnumber, 1, 0i\n   RETURN R3, 2u\n\nbb_fallback_1:\n   RETURN R1, 2u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_hash_slot_checks_invalidation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4218:ir_builder_duplicate_hash_slot_checks_invalidation`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> type_alias TValue (VM/src/lobject.h)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_hash_slot_checks_invalidation

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_hash_slot_checks_invalidation() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let table1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let three = b.const_uint(3);
      let k1 = b.vm_const(1);
      let slot1 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, table1, three, k1);
      let k1 = b.vm_const(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, slot1, k1, fallback);
      let zero = b.const_int(0);
      let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, slot1, zero);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value1);

      b.inst_ir_cmd(IrCmd::CheckGc);

      let eight = b.const_uint(8);
      let k1 = b.vm_const(1);
      let slot1b = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, table1, eight, k1);
      let k1 = b.vm_const(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, slot1b, k1, fallback);
      let zero = b.const_int(0);
      let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, slot1b, zero);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r4, value1b);

      let r3 = b.vm_reg(3);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let r4 = b.vm_reg(4);
      let b_value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r4);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a, b_value);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, sum);

      let r2 = b.vm_reg(2);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   %1 = GET_SLOT_NODE_ADDR %0, 3u, K1\n   CHECK_SLOT_MATCH %1, K1, bb_fallback_1\n   %3 = LOAD_TVALUE %1, 0i\n   STORE_TVALUE R3, %3\n   CHECK_GC\n   %6 = GET_SLOT_NODE_ADDR %0, 8u, K1\n   CHECK_SLOT_MATCH %6, K1, bb_fallback_1\n   %8 = LOAD_TVALUE %6, 0i\n   STORE_TVALUE R4, %8\n   %10 = LOAD_DOUBLE R3\n   %11 = LOAD_DOUBLE R4\n   %12 = ADD_NUM %10, %11\n   STORE_DOUBLE R2, %12\n   RETURN R2, 1u\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_duplicate_pointer_store_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5727:ir_builder_duplicate_pointer_store_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_duplicate_pointer_store_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_duplicate_pointer_store_removal() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let ptr = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, ptr);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);

      let r2 = b.vm_reg(2);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, one);
      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);

      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, ptr);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);

      let r0 = b.vm_reg(0);
      let three = b.const_int(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, three);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R0\n   STORE_POINTER R1, %0\n   STORE_TAG R1, ttable\n   STORE_DOUBLE R2, 1\n   STORE_TAG R2, tnumber\n   RETURN R0, 3i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_entry_block_use_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3246:ir_builder_entry_block_use_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_entry_block_use_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_entry_block_use_removal() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);
      let repeat = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, r0, exit, repeat);

      b.begin_block(exit);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(repeat);
      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, zero);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, entry);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   JUMP bb_1\n; glued to: bb_1\n\nbb_1:\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_explicit_use_of_register_in_vararg_sequence {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5058:ir_builder_explicit_use_of_register_in_vararg_sequence`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_explicit_use_of_register_in_vararg_sequence

  #[cfg(test)]
  #[test]
  fn ir_builder_explicit_use_of_register_in_vararg_sequence() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let zero = b.const_uint(0);
      let r1 = b.vm_reg(1);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackGetvarargs, zero, r1, minus_one);
      let zero = b.const_uint(0);
      let r0 = b.vm_reg(0);
      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      let undef = b.undef();
      let minus_one_params = b.const_int(-1);
      let minus_one_results = b.const_int(-1);
      let results = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::InvokeFastcall,
        zero,
        r0,
        r1,
        r2,
        undef,
        minus_one_params,
        minus_one_results,
      );
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, r0, results);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r0 = b.vm_reg(0);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, minus_one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1\n; out regs: R0...\n   FALLBACK_GETVARARGS 0u, R1, -1i\n   %1 = INVOKE_FASTCALL 0u, R0, R1, R2, undef, -1i, -1i\n   ADJUST_STACK_TO_REG R0, %1\n   JUMP bb_1\n\nbb_1:\n; predecessors: bb_0\n; in regs: R0...\n   RETURN R0, -1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_fallback_does_not_flow_up {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5134:ir_builder_fallback_does_not_flow_up`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_fallback_does_not_flow_up

  #[cfg(test)]
  #[test]
  fn ir_builder_fallback_does_not_flow_up() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let zero = b.const_uint(0);
      let r1 = b.vm_reg(1);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackGetvarargs, zero, r1, minus_one);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r0 = b.vm_reg(0);
      let minus_one_params = b.const_int(-1);
      let minus_one_results = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, r0, minus_one_params, minus_one_results);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(fallback);
      let r0 = b.vm_reg(0);
      let minus_one_params = b.const_int(-1);
      let minus_one_results = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, r0, minus_one_params, minus_one_results);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r0 = b.vm_reg(0);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, minus_one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_2\n; in regs: R0\n; out regs: R0...\n   FALLBACK_GETVARARGS 0u, R1, -1i\n   %1 = LOAD_TAG R0\n   CHECK_TAG %1, tnumber, bb_fallback_1\n   CALL R0, -1i, -1i\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0\n; successors: bb_2\n; in regs: R0, R1...\n; out regs: R0...\n   CALL R0, -1i, -1i\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0...\n   RETURN R0, -1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_falsy_test_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3042:ir_builder_falsy_test_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_falsy_test_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_falsy_test_removal() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let unknown = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, unknown, tnumber, fallback);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfFalsy, r1, true_block, false_block);

      b.begin_block(true_block);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);

      b.begin_block(false_block);
      let two = b.const_uint(2);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, two);

      b.begin_block(fallback);
      let three = b.const_uint(3);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, three);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R1\n   CHECK_TAG %0, tnumber, bb_fallback_3\n   JUMP bb_2\n; glued to: bb_2\n\nbb_2:\n   RETURN 2u\n\nbb_fallback_3:\n   RETURN 3u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_fast_call_effects_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4800:ir_builder_fast_call_effects_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_fast_call_effects_1

  #[cfg(test)]
  #[test]
  fn ir_builder_fast_call_effects_1() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;
    use ulua_vm::enums::lua_type::LuaType;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let bfid = b.const_uint(LuauBuiltinFunction::LBF_MATH_FREXP as u32);
      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::FASTCALL, bfid, r1, r2, two);

      let r1 = b.vm_reg(1);
      let tag1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(LuaType::Number as u8);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag1, tnumber, exit);
      let r2 = b.vm_reg(2);
      let tag2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(LuaType::Number as u8);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag2, tnumber, exit);
      let r1 = b.vm_reg(1);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R2\n   FASTCALL 14u, R1, R2, 2i\n   RETURN R1, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_fast_call_effects_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4823:ir_builder_fast_call_effects_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_fast_call_effects_2

  #[cfg(test)]
  #[test]
  fn ir_builder_fast_call_effects_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;
    use ulua_vm::enums::lua_type::LuaType;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let bfid = b.const_uint(LuauBuiltinFunction::LBF_MATH_MODF as u32);
      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::FASTCALL, bfid, r1, r2, one);

      let r1 = b.vm_reg(1);
      let tag1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(LuaType::Number as u8);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag1, tnumber, exit);
      let r2 = b.vm_reg(2);
      let tag2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(LuaType::Number as u8);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag2, tnumber, exit);
      let r1 = b.vm_reg(1);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R2\n   FASTCALL 20u, R1, R2, 1i\n   %3 = LOAD_TAG R2\n   CHECK_TAG %3, tnumber, exit(1)\n   RETURN R1, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_final_x_64_opt_binary_arith {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:164:ir_builder_final_x_64_opt_binary_arith`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_final_x_64_opt_binary_arith

  #[cfg(test)]
  #[test]
  fn ir_builder_final_x_64_opt_binary_arith() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    let b = &mut fix.build;

    let block = b.block(IrBlockKind::Internal);

    b.begin_block(block);
    let r1 = b.vm_reg(1);
    let op_a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
    let r2 = b.vm_reg(2);
    let op_b = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
    b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, op_a, op_b);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R1\n   %2 = ADD_NUM %0, R2\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_final_x_64_opt_check_tag {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:133:ir_builder_final_x_64_opt_check_tag`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_final_x_64_opt_check_tag

  #[cfg(test)]
  #[test]
  fn ir_builder_final_x_64_opt_check_tag() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    let b = &mut fix.build;

    let block = b.block(IrBlockKind::Internal);
    let fallback = b.fallback_block(0);

    b.begin_block(block);
    let reg2 = b.vm_reg(2);
    let tag1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg2);
    let tnil = b.const_tag(0);
    b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag1, tnil, fallback);
    let k5 = b.vm_const(5);
    let tag2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, k5);
    let tnil = b.const_tag(0);
    b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag2, tnil, fallback);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    b.begin_block(fallback);
    let one = b.const_uint(1);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);

    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   CHECK_TAG R2, tnil, bb_fallback_1\n   CHECK_TAG K5, tnil, bb_fallback_1\n   RETURN 0u\n\nbb_fallback_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_final_x_64_opt_eq_tag_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:187:ir_builder_final_x_64_opt_eq_tag_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_final_x_64_opt_eq_tag_1

  #[cfg(test)]
  #[test]
  fn ir_builder_final_x_64_opt_eq_tag_1() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    let b = &mut fix.build;

    let block = b.block(IrBlockKind::Internal);
    let true_block = b.block(IrBlockKind::Internal);
    let false_block = b.block(IrBlockKind::Internal);

    b.begin_block(block);
    let r1 = b.vm_reg(1);
    let op_a = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
    let r2 = b.vm_reg(2);
    let op_b = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
    b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, op_a, op_b, true_block, false_block);

    b.begin_block(true_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    b.begin_block(false_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %1 = LOAD_TAG R2\n   JUMP_EQ_TAG R1, %1, bb_1, bb_2\n\nbb_1:\n   RETURN 0u\n\nbb_2:\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_final_x_64_opt_eq_tag_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:222:ir_builder_final_x_64_opt_eq_tag_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method NativeModuleRef::swap (CodeGen/src/SharedCodeAllocator.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_final_x_64_opt_eq_tag_2

  #[cfg(test)]
  #[test]
  fn ir_builder_final_x_64_opt_eq_tag_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    let b = &mut fix.build;

    let block = b.block(IrBlockKind::Internal);
    let true_block = b.block(IrBlockKind::Internal);
    let false_block = b.block(IrBlockKind::Internal);

    b.begin_block(block);
    let r1 = b.vm_reg(1);
    let op_a = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
    let r2 = b.vm_reg(2);
    let op_b = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
    let r6 = b.vm_reg(6);
    b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r6, op_a);
    b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, op_a, op_b, true_block, false_block);

    b.begin_block(true_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    b.begin_block(false_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R1\n   STORE_TAG R6, %0\n   JUMP_EQ_TAG R2, %0, bb_1, bb_2\n\nbb_1:\n   RETURN 0u\n\nbb_2:\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_final_x_64_opt_eq_tag_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:260:ir_builder_final_x_64_opt_eq_tag_3`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_final_x_64_opt_eq_tag_3

  #[cfg(test)]
  #[test]
  fn ir_builder_final_x_64_opt_eq_tag_3() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    let b = &mut fix.build;

    let block = b.block(IrBlockKind::Internal);
    let true_block = b.block(IrBlockKind::Internal);
    let false_block = b.block(IrBlockKind::Internal);

    b.begin_block(block);
    let r1 = b.vm_reg(1);
    let table = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
    let zero_i = b.const_int(0);
    let arr_elem = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table, zero_i);
    let op_a = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, arr_elem);
    let tnil = b.const_tag(0);
    b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, op_a, tnil, true_block, false_block);

    b.begin_block(true_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    b.begin_block(false_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   %1 = GET_ARR_ADDR %0, 0i\n   %2 = LOAD_TAG %1\n   JUMP_EQ_TAG %2, tnil, bb_1, bb_2\n\nbb_1:\n   RETURN 0u\n\nbb_2:\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_final_x_64_opt_jump_cmp_num {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:298:ir_builder_final_x_64_opt_jump_cmp_num`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_final_x_64_opt_jump_cmp_num

  #[cfg(test)]
  #[test]
  fn ir_builder_final_x_64_opt_jump_cmp_num() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    let b = &mut fix.build;

    let block = b.block(IrBlockKind::Internal);
    let true_block = b.block(IrBlockKind::Internal);
    let false_block = b.block(IrBlockKind::Internal);

    b.begin_block(block);
    let r1 = b.vm_reg(1);
    let op_a = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
    let r2 = b.vm_reg(2);
    let op_b = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
    b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpCmpNum, op_a, op_b, true_block, false_block);

    b.begin_block(true_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    b.begin_block(false_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %1 = LOAD_DOUBLE R2\n   JUMP_CMP_NUM R1, %1, bb_1, bb_2\n\nbb_1:\n   RETURN 0u\n\nbb_2:\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_forgprep_implicit_use {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5280:ir_builder_forgprep_implicit_use`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_forgprep_implicit_use

  #[cfg(test)]
  #[test]
  fn ir_builder_forgprep_implicit_use() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let direct = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r2 = b.vm_reg(2);
      let ten = b.const_double(10.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, ten);
      let r3 = b.vm_reg(3);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r3, one);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, tag, tnumber, direct, fallback);

      b.begin_block(direct);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);

      b.begin_block(fallback);
      let zero = b.const_uint(0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackForgprep, zero, r1, exit);

      b.begin_block(exit);
      let r1 = b.vm_reg(1);
      let three = b.const_int(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, three);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1, bb_2\n; in regs: R0\n; out regs: R0, R1, R2, R3\n   STORE_DOUBLE R1, 1\n   STORE_DOUBLE R2, 10\n   STORE_DOUBLE R3, 1\n   %3 = LOAD_TAG R0\n   JUMP_EQ_TAG %3, tnumber, bb_1, bb_2\n\nbb_1:\n; predecessors: bb_0\n; in regs: R0\n   RETURN R0, 1i\n\nbb_2:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R1, R2, R3\n; out regs: R1, R2, R3\n   FALLBACK_FORGPREP 0u, R1, bb_3\n\nbb_3:\n; predecessors: bb_2\n; in regs: R1, R2, R3\n   RETURN R1, 3i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_forgprep_invalidation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4761:ir_builder_forgprep_invalidation`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_forgprep_invalidation

  #[cfg(test)]
  #[test]
  fn ir_builder_forgprep_invalidation() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let followup = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let tbl = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, tbl, exit);

      let pc = b.const_uint(2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackForgprep, pc, r1, followup);

      b.begin_block(followup);
      let exit = b.vm_exit(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, tbl, exit);

      let r1 = b.vm_reg(1);
      let three = b.const_int(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, three);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1\n; in regs: R0, R1, R2, R3\n; out regs: R1, R2, R3\n   %0 = LOAD_POINTER R0\n   CHECK_READONLY %0, exit(1)\n   FALLBACK_FORGPREP 2u, R1, bb_1\n\nbb_1:\n; predecessors: bb_0\n; in regs: R1, R2, R3\n   CHECK_READONLY %0, exit(2)\n   RETURN R1, 3i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_full_store_has_to_be_observable_from_fallbacks {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6513:ir_builder_full_store_has_to_be_observable_from_fallbacks`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_full_store_has_to_be_observable_from_fallbacks

  #[cfg(test)]
  #[test]
  fn ir_builder_full_store_has_to_be_observable_from_fallbacks() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);
      let last = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let sixteen = b.const_uint(16);
      let thirty_two = b.const_uint(32);
      let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, sixteen, thirty_two);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, table);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      b.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, fallback);
      let sixteen = b.const_uint(16);
      let thirty_two = b.const_uint(32);
      let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, sixteen, thirty_two);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, table);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(fallback);
      b.inst_ir_cmd(IrCmd::CheckGc);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r1, tnumber, one);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(last);
      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_2\n; in regs: R0\n; out regs: R0, R1\n   CHECK_SAFE_ENV bb_fallback_1\n   %4 = NEW_TABLE 16u, 32u\n   STORE_SPLIT_TVALUE R1, ttable, %4\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0\n; successors: bb_2\n; in regs: R0\n; out regs: R0, R1\n   CHECK_GC\n   STORE_SPLIT_TVALUE R1, tnumber, 1\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0, R1\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_full_store_has_to_be_observable_from_fallbacks_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6569:ir_builder_full_store_has_to_be_observable_from_fallbacks_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_full_store_has_to_be_observable_from_fallbacks_2

  #[cfg(test)]
  #[test]
  fn ir_builder_full_store_has_to_be_observable_from_fallbacks_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);
      let last = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      b.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, fallback);
      let r2 = b.vm_reg(2);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, value);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(fallback);
      b.inst_ir_cmd(IrCmd::CheckGc);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r1, tnumber, one);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(last);
      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_2\n; in regs: R0, R2\n; out regs: R0, R1\n   STORE_TAG R1, tnumber\n   CHECK_SAFE_ENV bb_fallback_1\n   %2 = LOAD_TVALUE R2\n   STORE_TVALUE R1, %2\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0\n; successors: bb_2\n; in regs: R0\n; out regs: R0, R1\n   CHECK_GC\n   STORE_SPLIT_TVALUE R1, tnumber, 1\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0, R1\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_full_store_has_to_be_observable_from_fallbacks_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6623:ir_builder_full_store_has_to_be_observable_from_fallbacks_3`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_full_store_has_to_be_observable_from_fallbacks_3

  #[cfg(test)]
  #[test]
  fn ir_builder_full_store_has_to_be_observable_from_fallbacks_3() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TFUNCTION: u8 = 8;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);
      let last = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tfunction = b.const_tag(TFUNCTION);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tfunction, fallback);
      let constant = b.vm_const(10);
      let pointer = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, constant);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, pointer);
      b.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, fallback);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(fallback);
      b.inst_ir_cmd(IrCmd::CheckGc);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r1, tnumber, one);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(last);
      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_fallback_1, bb_2\n; in regs: R0, R1\n; out regs: R0, R1\n   %0 = LOAD_TAG R1\n   CHECK_TAG %0, tfunction, bb_fallback_1\n   CHECK_SAFE_ENV bb_fallback_1\n   STORE_DOUBLE R1, 1\n   STORE_TAG R1, tnumber\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0, bb_0\n; successors: bb_2\n; in regs: R0\n; out regs: R0, R1\n   CHECK_GC\n   STORE_SPLIT_TVALUE R1, tnumber, 1\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0, R1\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_guards {
  #[cfg(test)]
  #[test]
  fn ir_builder_guards() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();

    fix.with_one_block(|b, a| {
      let t1 = b.const_tag(TNUMBER);
      let t2 = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, t1, t2, a);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    fix.with_one_block(|b, a| {
      let t1 = b.const_tag(TNIL);
      let t2 = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, t1, t2, a);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   RETURN 0u\n\nbb_2:\n   JUMP bb_3\n\nbb_3:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_hidden_pointer_use_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6052:ir_builder_hidden_pointer_use_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_hidden_pointer_use_1

  #[cfg(test)]
  #[test]
  fn ir_builder_hidden_pointer_use_1() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let some_ptr = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, some_ptr);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      let r2 = b.vm_reg(2);
      let zero = b.const_int(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, r2, zero, one);
      let r2 = b.vm_reg(2);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R2\n   CALL R2, 0i, 1i\n   RETURN R2, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_hidden_pointer_use_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6077:ir_builder_hidden_pointer_use_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> type_alias TValue (VM/src/lobject.h)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_hidden_pointer_use_2

  #[cfg(test)]
  #[test]
  fn ir_builder_hidden_pointer_use_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let some_ptr_a = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, some_ptr_a);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      let r2 = b.vm_reg(2);
      let zero = b.const_int(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, r2, zero, one);
      let r2 = b.vm_reg(2);
      let some_ptr_b = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, some_ptr_b);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      let r2 = b.vm_reg(2);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R2\n   CALL R2, 0i, 1i\n   RETURN R2, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_hidden_pointer_use_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6107:ir_builder_hidden_pointer_use_3`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_hidden_pointer_use_3

  #[cfg(test)]
  #[test]
  fn ir_builder_hidden_pointer_use_3() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let some_ptr_a = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, some_ptr_a);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      let r2 = b.vm_reg(2);
      let some_tv = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, some_tv);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R2\n   %3 = LOAD_TVALUE R2\n   STORE_TVALUE R1, %3\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_hidden_pointer_use_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6135:ir_builder_hidden_pointer_use_4`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_hidden_pointer_use_4

  #[cfg(test)]
  #[test]
  fn ir_builder_hidden_pointer_use_4() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, one);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      b.inst_ir_cmd(IrCmd::CheckGc);
      let r0 = b.vm_reg(0);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnil);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   CHECK_GC\n   STORE_TAG R0, tnil\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_hidden_pointer_use_5 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6167:ir_builder_hidden_pointer_use_5`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_hidden_pointer_use_5

  #[cfg(test)]
  #[test]
  fn ir_builder_hidden_pointer_use_5() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let sixteen = b.const_uint(16);
      let zero = b.const_uint(0);
      let some_ptr_a = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, sixteen, zero);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, some_ptr_a);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      let r3 = b.vm_reg(3);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, r3, r2);
      let r1 = b.vm_reg(1);
      let some_ptr_b = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r2, some_ptr_b);
      let r2 = b.vm_reg(2);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, ttable);
      let r2 = b.vm_reg(2);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R2\n   %0 = NEW_TABLE 16u, 0u\n   STORE_POINTER R1, %0\n   STORE_TAG R1, ttable\n   DO_LEN R3, R2\n   %4 = LOAD_POINTER R1\n   STORE_POINTER R2, %4\n   STORE_TAG R2, ttable\n   RETURN R2, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_hidden_pointer_use_6 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6203:ir_builder_hidden_pointer_use_6`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_hidden_pointer_use_6

  #[cfg(test)]
  #[test]
  fn ir_builder_hidden_pointer_use_6() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let ptr = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let table = b.inst_ir_cmd_ir_op(IrCmd::DupTable, ptr);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, table);
      let r1 = b.vm_reg(1);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, ttable);
      b.inst_ir_cmd(IrCmd::CheckGc);
      let table_len = b.inst_ir_cmd_ir_op(IrCmd::TableLen, table);
      let len_num = b.inst_ir_cmd_ir_op(IrCmd::IntToNum, table_len);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, len_num);
      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let r1 = b.vm_reg(1);
      let one_double = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one_double);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   %0 = LOAD_POINTER R0\n   %1 = DUP_TABLE %0\n   STORE_POINTER R1, %1\n   STORE_TAG R1, ttable\n   CHECK_GC\n   %5 = TABLE_LEN %1\n   %6 = INT_TO_NUM %5\n   STORE_DOUBLE R2, %6\n   STORE_TAG R2, tnumber\n   STORE_DOUBLE R1, 1\n   STORE_TAG R1, tnumber\n   RETURN R1, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_hidden_pointer_use_7 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6245:ir_builder_hidden_pointer_use_7`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_hidden_pointer_use_7

  #[cfg(test)]
  #[test]
  fn ir_builder_hidden_pointer_use_7() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let zero = b.const_int(0);
      let ttable = b.const_tag(TTABLE);
      let table_value = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, r1, zero, ttable);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, table_value);

      let sixteen = b.const_uint(16);
      let zero = b.const_uint(0);
      let some_ptr_a = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, sixteen, zero);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r0, some_ptr_a);

      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, one);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R1\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_ignore_fastcall_adjustment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6346:ir_builder_ignore_fastcall_adjustment`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_ignore_fastcall_adjustment

  #[cfg(test)]
  #[test]
  fn ir_builder_ignore_fastcall_adjustment() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let minus_one = b.const_double(-1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, minus_one);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, r1, one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   ADJUST_STACK_TO_REG R1, 1i\n   STORE_SPLIT_TVALUE R1, tnumber, 1\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_implicit_fixed_registers_in_vararg_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5025:ir_builder_implicit_fixed_registers_in_vararg_call`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_implicit_fixed_registers_in_vararg_call

  #[cfg(test)]
  #[test]
  fn ir_builder_implicit_fixed_registers_in_vararg_call() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let zero = b.const_uint(0);
      let r3 = b.vm_reg(3);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackGetvarargs, zero, r3, minus_one);
      let r0 = b.vm_reg(0);
      let minus_one = b.const_int(-1);
      let five = b.const_int(5);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, r0, minus_one, five);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r0 = b.vm_reg(0);
      let five = b.const_int(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, five);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1\n; in regs: R0, R1, R2\n; out regs: R0, R1, R2, R3, R4\n   FALLBACK_GETVARARGS 0u, R3, -1i\n   CALL R0, -1i, 5i\n   JUMP bb_1\n\nbb_1:\n; predecessors: bb_0\n; in regs: R0, R1, R2, R3, R4\n   RETURN R0, 5i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_indirect_float_load_extraction_must_respect_version {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5871:ir_builder_indirect_float_load_extraction_must_respect_version`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_indirect_float_load_extraction_must_respect_version

  #[cfg(test)]
  #[test]
  fn ir_builder_indirect_float_load_extraction_must_respect_version() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);

      let r4 = b.vm_reg(4);
      let tag4 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r4);
      let tvector = b.const_tag(TVECTOR);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag4, tvector, exit);
      let r5 = b.vm_reg(5);
      let tag5 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r5);
      let tvector = b.const_tag(TVECTOR);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag5, tvector, exit);

      let r4 = b.vm_reg(4);
      let zero = b.const_int(0);
      let x1_float = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, r4, zero);
      let x1 = b.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x1_float);
      let r5 = b.vm_reg(5);
      let zero = b.const_int(0);
      let x2_float = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, r5, zero);
      let x2 = b.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x2_float);

      let min = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MinNum, x1, x2);
      let x_min = b.inst_ir_cmd_ir_op(IrCmd::NumToFloat, min);
      let r7 = b.vm_reg(7);
      let zero_a = b.const_double(0.0);
      let zero_b = b.const_double(0.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r7, x_min, zero_a, zero_b);
      let r7 = b.vm_reg(7);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r7, tvector);

      let r7 = b.vm_reg(7);
      let zero = b.const_int(0);
      let tvector = b.const_tag(TVECTOR);
      let x_min_vec = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, r7, zero, tvector);
      let r6 = b.vm_reg(6);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r6, x_min_vec);

      let max = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MaxNum, x1, x2);
      let x_max = b.inst_ir_cmd_ir_op(IrCmd::NumToFloat, max);
      let r7 = b.vm_reg(7);
      let zero_a = b.const_double(0.0);
      let zero_b = b.const_double(0.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r7, x_max, zero_a, zero_b);
      let r7 = b.vm_reg(7);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r7, tvector);

      let r7 = b.vm_reg(7);
      let zero = b.const_int(0);
      let tvector = b.const_tag(TVECTOR);
      let x_max_vec = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, r7, zero, tvector);
      let r5 = b.vm_reg(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r5, x_max_vec);

      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r4, x_min_vec);

      let r4 = b.vm_reg(4);
      let zero = b.const_int(0);
      let x_min_copy_float = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, r4, zero);
      let x_min_copy = b.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x_min_copy_float);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, x_min_copy);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R4, R5\n   %0 = LOAD_TAG R4\n   CHECK_TAG %0, tvector, exit(1)\n   %2 = LOAD_TAG R5\n   CHECK_TAG %2, tvector, exit(1)\n   %4 = LOAD_FLOAT R4, 0i\n   %5 = FLOAT_TO_NUM %4\n   %6 = LOAD_FLOAT R5, 0i\n   %7 = FLOAT_TO_NUM %6\n   %8 = MIN_NUM %5, %7\n   %9 = NUM_TO_FLOAT %8\n   STORE_VECTOR R7, %9, 0, 0\n   STORE_TAG R7, tvector\n   %12 = LOAD_TVALUE R7, 0i, tvector\n   STORE_TVALUE R6, %12\n   %14 = MAX_NUM %5, %7\n   %15 = NUM_TO_FLOAT %14\n   STORE_VECTOR R7, %15, 0, 0\n   %18 = LOAD_TVALUE R7, 0i, tvector\n   STORE_TVALUE R5, %18\n   STORE_TVALUE R4, %12\n   %21 = EXTRACT_VEC %12, 0i\n   %22 = FLOAT_TO_NUM %21\n   STORE_DOUBLE R0, %22\n   STORE_TAG R0, tnumber\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_infer_number_tag_from_limited_context {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4848:ir_builder_infer_number_tag_from_limited_context`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_infer_number_tag_from_limited_context

  #[cfg(test)]
  #[test]
  fn ir_builder_infer_number_tag_from_limited_context() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;
    use ulua_vm::enums::lua_type::LuaType;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let ttable = b.const_tag(LuaType::Table as u8);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, ttable, exit);
      let r0 = b.vm_reg(0);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, value);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 2\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_infinite_loop_in_path_analysis {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3985:ir_builder_infinite_loop_in_path_analysis`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function createLinearBlocks (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_infinite_loop_in_path_analysis

  #[cfg(test)]
  #[test]
  fn ir_builder_infinite_loop_in_path_analysis() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains,
        create_linear_blocks::create_linear_blocks, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block1 = b.block(IrBlockKind::Internal);
      let block2 = b.block(IrBlockKind::Internal);

      b.begin_block(block1);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

      b.begin_block(block2);
      let r1 = b.vm_reg(1);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tboolean);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    create_linear_blocks(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   JUMP bb_1\n\nbb_1:\n   STORE_TAG R1, tboolean\n   JUMP bb_1\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_arith_chain_const_fold {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_arith_chain_const_fold() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      // ((10 + 20) * 3) - 5
      let c10 = b.const_int_64(10);
      let c20 = b.const_int_64(20);
      let add = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt64, c10, c20);
      let c3 = b.const_int_64(3);
      let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MulInt64, add, c3);
      let c5 = b.const_int_64(5);
      let sub = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt64, mul, c5);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r0, sub);

      // (0xFF & 0x0F) | 0xF0
      let cff = b.const_int_64(0xFF);
      let c0f = b.const_int_64(0x0F);
      let band = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandInt64, cff, c0f);
      let cf0 = b.const_int_64(0xF0);
      let bor = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BitorInt64, band, cf0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r1, bor);

      // (1 << 10) >> 5
      let c1 = b.const_int_64(1);
      let c10b = b.const_int_64(10);
      let lshift = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BitlshiftInt64, c1, c10b);
      let c5b = b.const_int_64(5);
      let rshift = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftInt64, lshift, c5b);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r2, rshift);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 85i\n   STORE_INT64 R1, 255i\n   STORE_INT64 R2, 32i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_arithmetic {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_arithmetic() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      bin!(0, IrCmd::AddInt64, 10, 20);
      bin!(1, IrCmd::AddInt64, i64::MAX, 1);
      bin!(2, IrCmd::SubInt64, 10, 20);
      bin!(3, IrCmd::SubInt64, i64::MIN, 1);
      bin!(4, IrCmd::MulInt64, 6, 7);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 30i\n   STORE_INT64 R1, -9223372036854775808i\n   STORE_INT64 R2, -10i\n   STORE_INT64 R3, 9223372036854775807i\n   STORE_INT64 R4, 42i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_arithmetic_extended {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_arithmetic_extended() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      bin!(0, IrCmd::MulInt64, i64::MAX, 2);
      bin!(1, IrCmd::MulInt64, i64::MAX, 0);
      bin!(2, IrCmd::MulInt64, 42, -1);
      bin!(3, IrCmd::AddInt64, 100, 0);
      bin!(4, IrCmd::SubInt64, 100, 100);
      bin!(5, IrCmd::AddInt64, -10, -20);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, -2i\n   STORE_INT64 R1, 0i\n   STORE_INT64 R2, -42i\n   STORE_INT64 R3, 100i\n   STORE_INT64 R4, 0i\n   STORE_INT64 R5, -30i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_bitwise {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_bitwise() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let unk = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);

      macro_rules! bin {
            ($reg:expr, $cmd:expr, ($($a:tt)+), ($($bb:tt)+)) => {{
                let ca = bin!(@op $($a)+);
                let cb = bin!(@op $($bb)+);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
            }};
            (@op unk) => { unk };
            (@op $v:expr) => { b.const_int_64($v) };
        }

      bin!(0, IrCmd::BitandInt64, (0xFE), (0x0E));
      bin!(1, IrCmd::BitandInt64, (unk), (0));
      bin!(2, IrCmd::BitandInt64, (0), (unk));
      bin!(3, IrCmd::BitandInt64, (unk), (-1));
      bin!(4, IrCmd::BitandInt64, (-1), (unk));
      bin!(5, IrCmd::BitxorInt64, (0xFE), (0x0E));
      bin!(6, IrCmd::BitxorInt64, (unk), (0));
      bin!(7, IrCmd::BitxorInt64, (0), (unk));
      bin!(8, IrCmd::BitorInt64, (0xF0), (0x0E));
      bin!(9, IrCmd::BitorInt64, (unk), (0));
      bin!(10, IrCmd::BitorInt64, (0), (unk));
      bin!(11, IrCmd::BitorInt64, (unk), (-1));
      bin!(12, IrCmd::BitorInt64, (-1), (unk));

      let c = b.const_int_64(0x0E);
      let op = b.inst_ir_cmd_ir_op(IrCmd::BitnotInt64, c);
      let r = b.vm_reg(13);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   STORE_INT64 R0, 14i\n   STORE_INT64 R1, 0i\n   STORE_INT64 R2, 0i\n   STORE_INT64 R3, %0\n   STORE_INT64 R4, %0\n   STORE_INT64 R5, 240i\n   STORE_INT64 R6, %0\n   STORE_INT64 R7, %0\n   STORE_INT64 R8, 254i\n   STORE_INT64 R9, %0\n   STORE_INT64 R10, %0\n   STORE_INT64 R11, -1i\n   STORE_INT64 R12, -1i\n   STORE_INT64 R13, -15i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_bitwise_extended {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_bitwise_extended() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }
      macro_rules! un {
        ($reg:expr, $cmd:expr, $a:expr) => {{
          let ca = b.const_int_64($a);
          let op = b.inst_ir_cmd_ir_op($cmd, ca);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      un!(0, IrCmd::BitnotInt64, 0);
      un!(1, IrCmd::BitnotInt64, -1);
      bin!(2, IrCmd::BitandInt64, 0xABCD, 0xABCD);
      bin!(3, IrCmd::BitxorInt64, 0xABCD, 0xABCD);
      bin!(4, IrCmd::BitorInt64, 0xABCD, 0xABCD);
      un!(5, IrCmd::BitcountlzInt64, 1);
      un!(6, IrCmd::BitcountlzInt64, -1);
      un!(7, IrCmd::BitcountrzInt64, 1);
      un!(8, IrCmd::BitcountrzInt64, -1);
      un!(9, IrCmd::BitcountrzInt64, 1i64 << 32);
      un!(10, IrCmd::ByteswapInt64, 0);
      un!(11, IrCmd::ByteswapInt64, -1);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, -1i\n   STORE_INT64 R1, 0i\n   STORE_INT64 R2, 43981i\n   STORE_INT64 R3, 0i\n   STORE_INT64 R4, 43981i\n   STORE_INT64 R5, 63i\n   STORE_INT64 R6, 0i\n   STORE_INT64 R7, 0i\n   STORE_INT64 R8, 0i\n   STORE_INT64 R9, 32i\n   STORE_INT64 R10, 0i\n   STORE_INT64 R11, -1i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_bitwise_large_values {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_bitwise_large_values() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let hi_val = 0x8000000000000000u64 as i64; // INT64_MIN
      let hi_mask = 0xFFFFFFFF00000000u64 as i64;
      let lo_mask = 0x00000000FFFFFFFFu64 as i64;

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }
      macro_rules! un {
        ($reg:expr, $cmd:expr, $a:expr) => {{
          let ca = b.const_int_64($a);
          let op = b.inst_ir_cmd_ir_op($cmd, ca);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      bin!(0, IrCmd::BitandInt64, 0x123456789ABCDEF0i64, hi_mask);
      bin!(1, IrCmd::BitandInt64, 0x123456789ABCDEF0i64, lo_mask);
      bin!(2, IrCmd::BitxorInt64, hi_val, hi_val);
      bin!(
        3,
        IrCmd::BitorInt64,
        0xFF00000000000000u64 as i64,
        0x00000000000000FFi64
      );
      un!(4, IrCmd::ByteswapInt64, 0x0123456789ABCDEFi64);
      bin!(5, IrCmd::BitlrotateInt64, 0x0000000100000002i64, 32);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 1311768464867721216i\n   STORE_INT64 R1, 2596069104i\n   STORE_INT64 R2, 0i\n   STORE_INT64 R3, -72057594037927681i\n   STORE_INT64 R4, -1167088121787636991i\n   STORE_INT64 R5, 8589934593i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_check_cmp_fold {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_check_cmp_fold() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      macro_rules! chk {
        ($a:expr, $bb:expr, $cond:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let cc = b.cond($cond);
          b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt64, ca, cb, cc, fallback);
        }};
      }

      chk!(0, 0, IrCondition::NotEqual);
      chk!(10, 20, IrCondition::Less);
      chk!(5, 0, IrCondition::NotEqual);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_check_cmp_fold_pass {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_check_cmp_fold_pass() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let c5 = b.const_int_64(5);
      let c0c = b.const_int_64(0);
      let cc = b.cond(IrCondition::NotEqual);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt64, c5, c0c, cc, fallback);
      let c42 = b.const_int_64(42);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, c42);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 42i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_check_cmp_unsigned_fold {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_check_cmp_unsigned_fold() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let cm1 = b.const_int_64(-1);
      let c0c = b.const_int_64(0);
      let cc = b.cond(IrCondition::UnsignedGreater);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt64, cm1, c0c, cc, fallback);
      let c1c = b.const_int_64(1);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, c1c);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 1i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_check_cmp_unsigned_fold_fail {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_check_cmp_unsigned_fold_fail() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let c0c = b.const_int_64(0);
      let cm1 = b.const_int_64(-1);
      let cc = b.cond(IrCondition::UnsignedGreater);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt64, c0c, cm1, cc, fallback);
      let c1c = b.const_int_64(1);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, c1c);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_check_div_fold_overflow {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_check_div_fold_overflow() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let cmin = b.const_int_64(i64::MIN);
      let cm1 = b.const_int_64(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckDivInt64, cmin, cm1, fallback);
      let c99 = b.const_int_64(99);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, c99);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_check_div_fold_safe {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_check_div_fold_safe() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let c100 = b.const_int_64(100);
      let c7 = b.const_int_64(7);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckDivInt64, c100, c7, fallback);
      let c99 = b.const_int_64(99);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, c99);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 99i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_check_div_fold_zero_divisor {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_check_div_fold_zero_divisor() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let c100 = b.const_int_64(100);
      let c0c = b.const_int_64(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckDivInt64, c100, c0c, fallback);
      let c99 = b.const_int_64(99);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, c99);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_comparison_boundary_values {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_comparison_boundary_values() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! cmp {
        ($reg:expr, $a:expr, $bb:expr, $cond:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let cc = b.cond($cond);
          let op = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpInt64, ca, cb, cc);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
        }};
      }

      cmp!(0, i64::MIN, i64::MAX, IrCondition::Less);
      cmp!(1, i64::MAX, i64::MIN, IrCondition::Greater);
      cmp!(2, 0, 0, IrCondition::UnsignedLess);
      cmp!(3, -1, 0, IrCondition::UnsignedGreaterEqual);
      cmp!(4, 0, -1, IrCondition::UnsignedLessEqual);
      cmp!(5, -1, -1, IrCondition::GreaterEqual);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 1i\n   STORE_INT R1, 1i\n   STORE_INT R2, 0i\n   STORE_INT R3, 1i\n   STORE_INT R4, 1i\n   STORE_INT R5, 1i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_comparisons {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_comparisons() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! cmp {
        ($reg:expr, $a:expr, $bb:expr, $cond:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let cc = b.cond($cond);
          let op = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpInt64, ca, cb, cc);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
        }};
      }

      cmp!(0, 10, 20, IrCondition::Less);
      cmp!(1, 20, 10, IrCondition::Less);
      cmp!(2, 10, 10, IrCondition::Equal);
      cmp!(3, 10, 20, IrCondition::NotEqual);
      cmp!(4, -1, 0, IrCondition::Less);
      cmp!(5, i64::MIN, i64::MAX, IrCondition::Less);
      cmp!(6, -1, 0, IrCondition::UnsignedGreater);
      cmp!(7, -1, 0, IrCondition::UnsignedLess);
      cmp!(8, 10, 10, IrCondition::GreaterEqual);
      cmp!(9, 10, 10, IrCondition::LessEqual);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 1i\n   STORE_INT R1, 0i\n   STORE_INT R2, 1i\n   STORE_INT R3, 1i\n   STORE_INT R4, 1i\n   STORE_INT R5, 1i\n   STORE_INT R6, 1i\n   STORE_INT R7, 0i\n   STORE_INT R8, 1i\n   STORE_INT R9, 1i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_conversion_const_fold {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_conversion_const_fold() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! i2n {
        ($reg:expr, $a:expr) => {{
          let c = b.const_int_64($a);
          let op = b.inst_ir_cmd_ir_op(IrCmd::Int64ToNum, c);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);
        }};
      }
      macro_rules! n2i {
        ($reg:expr, $a:expr) => {{
          let c = b.const_double($a);
          let op = b.inst_ir_cmd_ir_op(IrCmd::NumToInt64, c);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      i2n!(0, 42);
      i2n!(1, 0);
      i2n!(2, -100);
      n2i!(3, 42.0);
      n2i!(4, 0.0);
      n2i!(5, -100.0);
      n2i!(6, 3.7);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 42\n   STORE_DOUBLE R1, 0\n   STORE_DOUBLE R2, -100\n   STORE_INT64 R3, 42i\n   STORE_INT64 R4, 0i\n   STORE_INT64 R5, -100i\n   STORE_INT64 R6, 3i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_conversion_const_prop {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_conversion_const_prop() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let val = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let as_num1 = b.inst_ir_cmd_ir_op(IrCmd::Int64ToNum, val);
      let as_num2 = b.inst_ir_cmd_ir_op(IrCmd::Int64ToNum, val);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, as_num1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, as_num2);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   %1 = INT64_TO_NUM %0\n   STORE_DOUBLE R1, %1\n   STORE_DOUBLE R2, %1\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_conversion_dedup {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_conversion_dedup() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let dbl = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let i1 = b.inst_ir_cmd_ir_op(IrCmd::NumToInt64, dbl);
      let i2 = b.inst_ir_cmd_ir_op(IrCmd::NumToInt64, dbl);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r1, i1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r2, i2);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   %1 = NUM_TO_INT64 %0\n   STORE_INT64 R1, %1\n   STORE_INT64 R2, %1\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_div_guard_fold_known_non_zero {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_div_guard_fold_known_non_zero() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);

      let c7 = b.const_int_64(7);
      let c0c = b.const_int_64(0);
      let cc = b.cond(IrCondition::NotEqual);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt64, c7, c0c, cc, fallback);

      let c7b = b.const_int_64(7);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivInt64, a, c7b);
      let r = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   %2 = DIV_INT64 %0, 7i\n   STORE_INT64 R1, %2\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_div_guard_zero_divisor_jumps {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_div_guard_zero_divisor_jumps() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let c0a = b.const_int_64(0);
      let c0b = b.const_int_64(0);
      let cc = b.cond(IrCondition::NotEqual);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt64, c0a, c0b, cc, fallback);
      let c99 = b.const_int_64(99);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, c99);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

      b.begin_block(fallback);
      let c1 = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_division_const_fold {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_division_const_fold() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      bin!(0, IrCmd::DivInt64, 42, 7);
      bin!(1, IrCmd::DivInt64, -7, 2);
      bin!(2, IrCmd::IdivInt64, -7, 2);
      bin!(3, IrCmd::IdivInt64, 7, 2);
      bin!(4, IrCmd::IdivInt64, -6, 2);
      bin!(5, IrCmd::UdivInt64, -1, 2);
      bin!(6, IrCmd::RemInt64, 7, 3);
      bin!(7, IrCmd::RemInt64, -7, 3);
      bin!(8, IrCmd::UremInt64, -1, 10);
      bin!(9, IrCmd::ModInt64, -7, 3);
      bin!(10, IrCmd::ModInt64, 7, 3);
      bin!(11, IrCmd::ModInt64, 7, -3);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 6i\n   STORE_INT64 R1, -3i\n   STORE_INT64 R2, -4i\n   STORE_INT64 R3, 3i\n   STORE_INT64 R4, -3i\n   STORE_INT64 R5, 9223372036854775807i\n   STORE_INT64 R6, 1i\n   STORE_INT64 R7, -1i\n   STORE_INT64 R8, 5i\n   STORE_INT64 R9, 2i\n   STORE_INT64 R10, 1i\n   STORE_INT64 R11, -2i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_division_ops_preserved {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_division_ops_preserved() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let r1 = b.vm_reg(1);
      let bb = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r1);

      macro_rules! div {
        ($reg:expr, $cmd:expr) => {{
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, a, bb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      div!(2, IrCmd::DivInt64);
      div!(3, IrCmd::IdivInt64);
      div!(4, IrCmd::UdivInt64);
      div!(5, IrCmd::RemInt64);
      div!(6, IrCmd::UremInt64);
      div!(7, IrCmd::ModInt64);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   %1 = LOAD_INT64 R1\n   %2 = DIV_INT64 %0, %1\n   STORE_INT64 R2, %2\n   %4 = IDIV_INT64 %0, %1\n   STORE_INT64 R3, %4\n   %6 = UDIV_INT64 %0, %1\n   STORE_INT64 R4, %6\n   %8 = REM_INT64 %0, %1\n   STORE_INT64 R5, %8\n   %10 = UREM_INT64 %0, %1\n   STORE_INT64 R6, %10\n   %12 = MOD_INT64 %0, %1\n   STORE_INT64 R7, %12\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_division_store_forward {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_division_store_forward() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let r1 = b.vm_reg(1);
      let bb = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r1);
      let div_result = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivInt64, a, bb);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r2, div_result);

      let r2l = b.vm_reg(2);
      let loaded = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r2l);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r3, loaded);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   %1 = LOAD_INT64 R1\n   %2 = DIV_INT64 %0, %1\n   STORE_INT64 R2, %2\n   STORE_INT64 R3, %2\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_division_unsafe_cases_not_folded {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_division_unsafe_cases_not_folded() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      bin!(0, IrCmd::DivInt64, 42, 0);
      bin!(1, IrCmd::DivInt64, i64::MIN, -1);
      bin!(2, IrCmd::IdivInt64, 42, 0);
      bin!(3, IrCmd::RemInt64, 42, 0);
      bin!(4, IrCmd::RemInt64, i64::MIN, -1);
      bin!(5, IrCmd::UdivInt64, 42, 0);
      bin!(6, IrCmd::ModInt64, i64::MIN, -1);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = DIV_INT64 42i, 0i\n   STORE_INT64 R0, %0\n   %2 = DIV_INT64 -9223372036854775808i, -1i\n   STORE_INT64 R1, %2\n   %4 = IDIV_INT64 42i, 0i\n   STORE_INT64 R2, %4\n   %6 = REM_INT64 42i, 0i\n   STORE_INT64 R3, %6\n   %8 = REM_INT64 -9223372036854775808i, -1i\n   STORE_INT64 R4, %8\n   %10 = UDIV_INT64 42i, 0i\n   STORE_INT64 R5, %10\n   %12 = MOD_INT64 -9223372036854775808i, -1i\n   STORE_INT64 R6, %12\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_duplicate_store_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2584:ir_builder_int_64_duplicate_store_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_int_64_duplicate_store_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_duplicate_store_removal() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let val = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r1, val);

      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r1, val);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   STORE_INT64 R1, %0\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_negation_const_fold {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_negation_const_fold() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! neg {
        ($reg:expr, $a:expr) => {{
          let ca = b.const_int_64(0);
          let cb = b.const_int_64($a);
          let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt64, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      neg!(0, 42);
      neg!(1, i64::MIN);
      neg!(2, 0);
      neg!(3, -1);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, -42i\n   STORE_INT64 R1, -9223372036854775808i\n   STORE_INT64 R2, 0i\n   STORE_INT64 R3, 1i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_num_roundtrip_elimination {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2525:ir_builder_int_64_num_roundtrip_elimination`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_int_64_num_roundtrip_elimination

  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_num_roundtrip_elimination() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let int_val = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let dbl_val = b.inst_ir_cmd_ir_op(IrCmd::Int64ToNum, int_val);
      let back_to_int = b.inst_ir_cmd_ir_op(IrCmd::NumToInt64, dbl_val);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r1, back_to_int);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   STORE_INT64 R1, %0\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_num_to_int_64_out_of_range_not_folded {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_num_to_int_64_out_of_range_not_folded() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let c = b.const_double(1e19);
      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToInt64, c);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);

      let z1 = b.const_double(0.0);
      let z2 = b.const_double(0.0);
      let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, z1, z2);
      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToInt64, nan);
      let r = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);

      let o1 = b.const_double(1.0);
      let o2 = b.const_double(0.0);
      let inf = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, o1, o2);
      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToInt64, inf);
      let r = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = NUM_TO_INT64 1e+19\n   STORE_INT64 R0, %0\n   %3 = NUM_TO_INT64 nan\n   STORE_INT64 R1, %3\n   %6 = NUM_TO_INT64 inf\n   STORE_INT64 R2, %6\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_select_preserved_with_different_branches {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_select_preserved_with_different_branches() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let r1 = b.vm_reg(1);
      let bb = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r1);

      let cond1 = b.cond(IrCondition::Less);
      let op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectInt64, a, bb, a, bb, cond1);
      let r = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);

      let cond2 = b.cond(IrCondition::LessEqual);
      let op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectInt64, a, a, a, bb, cond2);
      let r = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   %1 = LOAD_INT64 R1\n   %2 = SELECT_INT64 %0, %1, %0, %1, lt\n   STORE_INT64 R2, %2\n   %4 = SELECT_INT64 %0, %0, %0, %1, le\n   STORE_INT64 R3, %4\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_shift_boundary_63 {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_shift_boundary_63() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      bin!(0, IrCmd::BitlshiftInt64, 1, 63);
      bin!(1, IrCmd::BitrshiftInt64, i64::MIN, 63);
      bin!(2, IrCmd::BitarshiftInt64, -1, 63);
      bin!(3, IrCmd::BitarshiftInt64, i64::MAX, 63);
      bin!(4, IrCmd::BitlshiftInt64, i64::MIN, -63);
      bin!(5, IrCmd::BitrshiftInt64, 1, -63);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, -9223372036854775808i\n   STORE_INT64 R1, 1i\n   STORE_INT64 R2, -1i\n   STORE_INT64 R3, 0i\n   STORE_INT64 R4, 1i\n   STORE_INT64 R5, -9223372036854775808i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_shift_edge_cases {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_shift_edge_cases() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      bin!(0, IrCmd::BitrshiftInt64, 0xF, -4);
      bin!(1, IrCmd::BitrshiftInt64, 0xF, 64);
      bin!(2, IrCmd::BitarshiftInt64, -1, 64);
      bin!(3, IrCmd::BitarshiftInt64, 1, 64);
      bin!(4, IrCmd::BitarshiftInt64, 0xF, -4);
      bin!(5, IrCmd::BitarshiftInt64, 0xF, -64);
      bin!(6, IrCmd::BitlshiftInt64, 0xFF, 0);
      bin!(7, IrCmd::BitlrotateInt64, 0xFF, 64);
      bin!(8, IrCmd::BitrrotateInt64, 0xFF, 0);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 240i\n   STORE_INT64 R1, 0i\n   STORE_INT64 R2, -1i\n   STORE_INT64 R3, 0i\n   STORE_INT64 R4, 240i\n   STORE_INT64 R5, 0i\n   STORE_INT64 R6, 255i\n   STORE_INT64 R7, 255i\n   STORE_INT64 R8, 255i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_shifts_and_rotates {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_shifts_and_rotates() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      macro_rules! bin {
        ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int_64($a);
          let cb = b.const_int_64($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }
      macro_rules! un {
        ($reg:expr, $cmd:expr, $a:expr) => {{
          let ca = b.const_int_64($a);
          let op = b.inst_ir_cmd_ir_op($cmd, ca);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r, op);
        }};
      }

      bin!(0, IrCmd::BitlshiftInt64, 0xF, 4);
      bin!(1, IrCmd::BitlshiftInt64, 0xF0, -4);
      bin!(2, IrCmd::BitlshiftInt64, 0xF, 64);
      bin!(3, IrCmd::BitrshiftInt64, 0xF0, 4);
      bin!(4, IrCmd::BitarshiftInt64, -16, 2);
      bin!(5, IrCmd::BitlrotateInt64, 1, 63);
      bin!(6, IrCmd::BitrrotateInt64, 1, 1);
      un!(7, IrCmd::BitcountlzInt64, 0xFF00);
      un!(8, IrCmd::BitcountlzInt64, 0);
      un!(9, IrCmd::BitcountrzInt64, 0xFF00);
      un!(10, IrCmd::BitcountrzInt64, 0);
      un!(11, IrCmd::ByteswapInt64, 0x0102030405060708i64);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 240i\n   STORE_INT64 R1, 15i\n   STORE_INT64 R2, 0i\n   STORE_INT64 R3, 15i\n   STORE_INT64 R4, -4i\n   STORE_INT64 R5, -9223372036854775808i\n   STORE_INT64 R6, -9223372036854775808i\n   STORE_INT64 R7, 48i\n   STORE_INT64 R8, 64i\n   STORE_INT64 R9, 8i\n   STORE_INT64 R10, 64i\n   STORE_INT64 R11, 578437695752307201i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_split_tvalue_store_const_prop {
  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_split_tvalue_store_const_prop() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TINTEGER: u8 = 4;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      b.begin_block(entry);

      let r0 = b.vm_reg(0);
      let tag = b.const_tag(TINTEGER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tag);
      let r0b = b.vm_reg(0);
      let c42 = b.const_int_64(42);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r0b, c42);

      let r0c = b.vm_reg(0);
      let c1 = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0c, c1);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected =
      "\nbb_0:\n   STORE_TAG R0, tinteger\n   STORE_INT64 R0, 42i\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_64_store_forward_to_load {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2553:ir_builder_int_64_store_forward_to_load`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_int_64_store_forward_to_load

  #[cfg(test)]
  #[test]
  fn ir_builder_int_64_store_forward_to_load() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let val = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r1, val);

      let r1 = b.vm_reg(1);
      let loaded = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r2, loaded);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   STORE_INT64 R1, %0\n   STORE_INT64 R2, %0\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_eq_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3115:ir_builder_int_eq_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_int_eq_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_int_eq_removal() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let five = b.const_int(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, five);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r1);
      let five = b.const_int(5);
      let equal = b.cond(IrCondition::Equal);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpInt,
        value,
        five,
        equal,
        true_block,
        false_block,
      );

      b.begin_block(true_block);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);

      b.begin_block(false_block);
      let two = b.const_uint(2);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected =
      "\nbb_0:\n   STORE_INT R1, 5i\n   JUMP bb_1\n; glued to: bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_num_int_peepholes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3348:ir_builder_int_num_int_peepholes`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_int_num_int_peepholes

  #[cfg(test)]
  #[test]
  fn ir_builder_int_num_int_peepholes() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let i1 = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r0);
      let r1 = b.vm_reg(1);
      let u1 = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r1);
      let ni1 = b.inst_ir_cmd_ir_op(IrCmd::IntToNum, i1);
      let nu1 = b.inst_ir_cmd_ir_op(IrCmd::UintToNum, u1);
      let to_int = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, ni1);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, to_int);
      let to_uint = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, nu1);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, to_uint);
      let to_uint = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, ni1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r2, to_uint);
      let to_int = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, nu1);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r3, to_int);
      let r0 = b.vm_reg(0);
      let four = b.const_uint(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, four);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT R0\n   %1 = LOAD_INT R1\n   STORE_INT R2, %0\n   STORE_INT R3, %1\n   RETURN R0, 4u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_num_int_peepholes_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3378:ir_builder_int_num_int_peepholes_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_int_num_int_peepholes_2

  #[cfg(test)]
  #[test]
  fn ir_builder_int_num_int_peepholes_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let d1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let u = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, d1);
      let d2 = b.inst_ir_cmd_ir_op(IrCmd::UintToNum, u);
      let to_uint = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, d2);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, to_uint);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   %1 = NUM_TO_UINT %0\n   STORE_INT R0, %1\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_int_num_int_peepholes_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3403:ir_builder_int_num_int_peepholes_3`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_int_num_int_peepholes_3

  #[cfg(test)]
  #[test]
  fn ir_builder_int_num_int_peepholes_3() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let table = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let len = b.inst_ir_cmd_ir_op(IrCmd::TableLen, table);
      let d = b.inst_ir_cmd_ir_op(IrCmd::IntToNum, len);
      let u = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, d);
      let u2 = b.inst_ir_cmd_ir_op(IrCmd::TruncateUint, u);
      let one = b.const_int(1);
      let result = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt, u2, one);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, result);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R0\n   %1 = TABLE_LEN %0\n   %5 = ADD_INT %1, 1i\n   STORE_INT R0, %5\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_invalidate_reglink_version {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3432:ir_builder_invalidate_reglink_version`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_invalidate_reglink_version

  #[cfg(test)]
  #[test]
  fn ir_builder_invalidate_reglink_version() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TSTRING: u8 = 6;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r2 = b.vm_reg(2);
      let tstring = b.const_tag(TSTRING);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tstring);
      let r2 = b.vm_reg(2);
      let tv2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, tv2);
      let zero = b.const_uint(0);
      let zero2 = b.const_uint(0);
      let ft = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, zero, zero2);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r2, ft);
      let r2 = b.vm_reg(2);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, ttable);
      let r1 = b.vm_reg(1);
      let tv1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, tv1);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, ttable, fallback);
      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

      b.begin_block(fallback);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R2, tstring\n   %1 = LOAD_TVALUE R2, 0i, tstring\n   STORE_TVALUE R1, %1\n   %3 = NEW_TABLE 0u, 0u\n   STORE_POINTER R2, %3\n   STORE_TAG R2, ttable\n   STORE_TVALUE R0, %1\n   JUMP bb_fallback_1\n\nbb_fallback_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_jump_implicit_live_out {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6372:ir_builder_jump_implicit_live_out`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_jump_implicit_live_out

  #[cfg(test)]
  #[test]
  fn ir_builder_jump_implicit_live_out() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let next = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

      b.begin_block(next);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, 1\n   JUMP bb_1\n; glued to: bb_1\n\nbb_1:\n; predecessors: bb_0\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_keep_captured_register_stores {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6408:ir_builder_keep_captured_register_stores`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_keep_captured_register_stores

  #[cfg(test)]
  #[test]
  fn ir_builder_keep_captured_register_stores() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let one_u = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, r1, one_u);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r0 = b.vm_reg(0);
      let r2 = b.vm_reg(2);
      let r3 = b.vm_reg(3);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::DoArith, r0, r2, r3, zero);
      let r1 = b.vm_reg(1);
      let minus_one = b.const_double(-1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, minus_one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let r4 = b.vm_reg(4);
      let r5 = b.vm_reg(5);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::DoArith, r1, r4, r5, zero);
      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\n; captured regs: R1\n\nbb_0:\n; in regs: R1, R2, R3, R4, R5\n   CAPTURE R1, 1u\n   STORE_DOUBLE R1, 1\n   STORE_TAG R1, tnumber\n   DO_ARITH R0, R2, R3, 0i\n   STORE_DOUBLE R1, -1\n   STORE_TAG R1, tnumber\n   DO_ARITH R1, R4, R5, 0i\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_late_table_state_link {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5438:ir_builder_late_table_state_link`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_late_table_state_link

  #[cfg(test)]
  #[test]
  fn ir_builder_late_table_state_link() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let tmp = b.inst_ir_cmd_ir_op(IrCmd::DupTable, r0);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r0, tmp);
      let r0 = b.vm_reg(0);
      let table = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

      b.begin_block(fallback);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = DUP_TABLE R0\n   STORE_POINTER R0, %0\n   CHECK_NO_METATABLE %0, bb_fallback_1\n   CHECK_READONLY %0, bb_fallback_1\n   RETURN 0u\n\nbb_fallback_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_load_propagates_only_right_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4070:ir_builder_load_propagates_only_right_type`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_load_propagates_only_right_type

  #[cfg(test)]
  #[test]
  fn ir_builder_load_propagates_only_right_type() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, two);
      let r0 = b.vm_reg(0);
      let value1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, value1);
      let r1 = b.vm_reg(1);
      let value2 = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r2, value2);
      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 2i\n   %1 = LOAD_DOUBLE R0\n   STORE_DOUBLE R1, %1\n   %3 = LOAD_INT R1\n   STORE_INT R2, %3\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_nil_store_implicit_value_clear_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7073:ir_builder_nil_store_implicit_value_clear_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_nil_store_implicit_value_clear_1

  #[cfg(test)]
  #[test]
  fn ir_builder_nil_store_implicit_value_clear_1() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TBOOLEAN: u8 = 1;
    const TNIL: u8 = 0;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, one);
      let r0 = b.vm_reg(0);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tboolean);
      let r0 = b.vm_reg(0);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnil);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, one);
      let r0 = b.vm_reg(0);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tboolean);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnil\n   STORE_INT R0, 1i\n   STORE_TAG R0, tboolean\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_nil_store_implicit_value_clear_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7100:ir_builder_nil_store_implicit_value_clear_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_nil_store_implicit_value_clear_2

  #[cfg(test)]
  #[test]
  fn ir_builder_nil_store_implicit_value_clear_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnil);
      let r0 = b.vm_reg(0);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r0 = b.vm_reg(0);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnil);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, value);
      let r1 = b.vm_reg(1);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnil = b.const_tag(TNIL);
      let exit = b.vm_exit(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnil, exit);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnil\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_no_dead_load_reuse {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5596:ir_builder_no_dead_load_reuse`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_no_dead_load_reuse

  #[cfg(test)]
  #[test]
  fn ir_builder_no_dead_load_reuse() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let op1i = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, op1);
      let zero = b.const_int(0);
      let res = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, op1i, zero);
      let resd = b.inst_ir_cmd_ir_op(IrCmd::IntToNum, res);
      let r0 = b.vm_reg(0);
      let op2 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, resd, op2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, sum);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %4 = LOAD_DOUBLE R0\n   %5 = ADD_NUM 0, %4\n   STORE_DOUBLE R1, %5\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_no_dead_value_reuse {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5623:ir_builder_no_dead_value_reuse`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_no_dead_value_reuse

  #[cfg(test)]
  #[test]
  fn ir_builder_no_dead_value_reuse() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let op1i = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, op1);
      let zero = b.const_int(0);
      let res = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, op1i, zero);
      let op2i = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, op1);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt, res, op2i);
      let resd = b.inst_ir_cmd_ir_op(IrCmd::IntToNum, sum);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, resd);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   %3 = NUM_TO_INT %0\n   %4 = ADD_INT 0i, %3\n   %5 = INT_TO_NUM %4\n   STORE_DOUBLE R1, %5\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_no_path_extraction_for_blocks_with_live_out_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3906:ir_builder_no_path_extraction_for_blocks_with_live_out_values`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function createLinearBlocks (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_no_path_extraction_for_blocks_with_live_out_values

  #[cfg(test)]
  #[test]
  fn ir_builder_no_path_extraction_for_blocks_with_live_out_values() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains,
        create_linear_blocks::create_linear_blocks, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block1 = b.block(IrBlockKind::Internal);
      let fallback1 = b.fallback_block(0);
      let block2 = b.block(IrBlockKind::Internal);
      let fallback2 = b.fallback_block(0);
      let block3 = b.block(IrBlockKind::Internal);
      let block4a = b.block(IrBlockKind::Internal);
      let block4b = b.block(IrBlockKind::Internal);

      b.begin_block(block1);
      let r2 = b.vm_reg(2);
      let tag1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag1, tnumber, fallback1);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

      b.begin_block(fallback1);
      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, r1, r2);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

      b.begin_block(block2);
      let r2 = b.vm_reg(2);
      let tag2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag2, tnumber, fallback2);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block3);

      b.begin_block(fallback2);
      let r0 = b.vm_reg(0);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, r0, r2);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block3);

      b.begin_block(block3);
      let r3 = b.vm_reg(3);
      let tag3a = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r3);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, tag3a, tnil, block4a, block4b);

      b.begin_block(block4a);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tag3a);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(block4b);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tag3a);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    create_linear_blocks(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R2\n   CHECK_TAG %0, tnumber, bb_fallback_1\n   JUMP bb_2\n\nbb_fallback_1:\n   DO_LEN R1, R2\n   JUMP bb_2\n\nbb_2:\n   %5 = LOAD_TAG R2\n   CHECK_TAG %5, tnumber, bb_fallback_3\n   JUMP bb_4\n\nbb_fallback_3:\n   DO_LEN R0, R2\n   JUMP bb_4\n\nbb_4:\n   %10 = LOAD_TAG R3\n   JUMP_EQ_TAG %10, tnil, bb_5, bb_6\n\nbb_5:\n   STORE_TAG R0, %10\n   RETURN R0, 0i\n\nbb_6:\n   STORE_TAG R0, %10\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_no_propagation_of_captured_regs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5565:ir_builder_no_propagation_of_captured_regs`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
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
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_no_propagation_of_captured_regs

  #[cfg(test)]
  #[test]
  fn ir_builder_no_propagation_of_captured_regs() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, r0, one);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let r0 = b.vm_reg(0);
      let op2 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, op1, op2);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, sum);
      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\n; captured regs: R0\n\nbb_0:\n; in regs: R0\n   CAPTURE R0, 1u\n   %1 = LOAD_DOUBLE R0\n   %2 = LOAD_DOUBLE R0\n   %3 = ADD_NUM %1, %2\n   STORE_DOUBLE R1, %3\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_num_cmp_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3147:ir_builder_num_cmp_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_num_cmp_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_num_cmp_removal() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, four);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
      let eight = b.const_double(8.0);
      let greater = b.cond(IrCondition::Greater);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpNum,
        value,
        eight,
        greater,
        true_block,
        false_block,
      );

      b.begin_block(true_block);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);

      b.begin_block(false_block);
      let two = b.const_uint(2);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected =
      "\nbb_0:\n   STORE_DOUBLE R1, 4\n   JUMP bb_2\n; glued to: bb_2\n\nbb_2:\n   RETURN 2u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_num_to_index {
  #[cfg(test)]
  #[test]
  fn ir_builder_num_to_index() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();

    fix.with_one_block(|b, a| {
      let c = b.const_double(4.0);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::TryNumToIndex, c, a);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    fix.with_one_block(|b, a| {
      let c = b.const_double(1.2);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::TryNumToIndex, c, a);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    fix.with_one_block(|b, a| {
      let z1 = b.const_double(0.0);
      let z2 = b.const_double(0.0);
      let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, z1, z2);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::TryNumToIndex, nan, a);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 4i\n   RETURN 0u\n\nbb_2:\n   JUMP bb_3\n\nbb_3:\n   RETURN 1u\n\nbb_4:\n   JUMP bb_5\n\nbb_5:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_number_over_combined_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6998:ir_builder_number_over_combined_vector`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_number_over_combined_vector

  #[cfg(test)]
  #[test]
  fn ir_builder_number_over_combined_vector() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      let two = b.const_double(2.0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, one, two, four);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let three = b.const_double(3.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, three);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_SPLIT_TVALUE R0, tnumber, 3\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_number_over_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6954:ir_builder_number_over_nil`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_number_over_nil

  #[cfg(test)]
  #[test]
  fn ir_builder_number_over_nil() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnil);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_SPLIT_TVALUE R0, tnumber, 2\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_number_over_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6931:ir_builder_number_over_vector`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_number_over_vector

  #[cfg(test)]
  #[test]
  fn ir_builder_number_over_vector() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      let two = b.const_double(2.0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, one, two, four);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_SPLIT_TVALUE R0, tnumber, 2\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_numeric {
  #[cfg(test)]
  #[test]
  fn ir_builder_numeric() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;
    const TBOOLEAN: u8 = 1;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      // Binary integer/number folds: STORE R{reg}, CMD(a, b)
      macro_rules! bin_i {
        ($store:expr, $reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_int($a);
          let cb = b.const_int($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op($store, r, op);
        }};
      }
      macro_rules! bin_d {
        ($store:expr, $reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
          let ca = b.const_double($a);
          let cb = b.const_double($bb);
          let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op($store, r, op);
        }};
      }
      macro_rules! un_d {
        ($store:expr, $reg:expr, $cmd:expr, $a:expr) => {{
          let ca = b.const_double($a);
          let op = b.inst_ir_cmd_ir_op($cmd, ca);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op($store, r, op);
        }};
      }
      macro_rules! un_i {
        ($store:expr, $reg:expr, $cmd:expr, $a:expr) => {{
          let ca = b.const_int($a);
          let op = b.inst_ir_cmd_ir_op($cmd, ca);
          let r = b.vm_reg($reg);
          b.inst_ir_cmd_ir_op_ir_op($store, r, op);
        }};
      }

      bin_i!(IrCmd::StoreInt, 0, IrCmd::AddInt, 10, 20);
      bin_i!(IrCmd::StoreInt, 1, IrCmd::AddInt, i32::MAX, 1);
      bin_i!(IrCmd::StoreInt, 2, IrCmd::SubInt, 10, 20);
      bin_i!(IrCmd::StoreInt, 3, IrCmd::SubInt, i32::MIN, 1);

      bin_d!(IrCmd::StoreDouble, 4, IrCmd::AddNum, 2.0, 5.0);
      bin_d!(IrCmd::StoreDouble, 5, IrCmd::SubNum, 2.0, 5.0);
      bin_d!(IrCmd::StoreDouble, 6, IrCmd::MulNum, 2.0, 5.0);
      bin_d!(IrCmd::StoreDouble, 7, IrCmd::DivNum, 2.0, 5.0);
      bin_d!(IrCmd::StoreDouble, 8, IrCmd::ModNum, 5.0, 2.0);
      bin_d!(IrCmd::StoreDouble, 10, IrCmd::MinNum, 5.0, 2.0);
      bin_d!(IrCmd::StoreDouble, 11, IrCmd::MaxNum, 5.0, 2.0);

      un_d!(IrCmd::StoreDouble, 12, IrCmd::UnmNum, 5.0);
      un_d!(IrCmd::StoreDouble, 13, IrCmd::FloorNum, 2.5);
      un_d!(IrCmd::StoreDouble, 14, IrCmd::CeilNum, 2.5);
      un_d!(IrCmd::StoreDouble, 15, IrCmd::RoundNum, 2.5);
      un_d!(IrCmd::StoreDouble, 16, IrCmd::SqrtNum, 16.0);
      un_d!(IrCmd::StoreDouble, 17, IrCmd::AbsNum, -4.0);

      // NOT_ANY(tag, LOAD_DOUBLE(R1)) and NOT_ANY(tag, const)
      {
        let tag = b.const_tag(TNIL);
        let r1 = b.vm_reg(1);
        let load = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NotAny, tag, load);
        let r = b.vm_reg(18);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
      }
      {
        let tag = b.const_tag(TNUMBER);
        let r1 = b.vm_reg(1);
        let load = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NotAny, tag, load);
        let r = b.vm_reg(19);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
      }
      {
        let tag = b.const_tag(TBOOLEAN);
        let c0 = b.const_int(0);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NotAny, tag, c0);
        let r = b.vm_reg(20);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
      }
      {
        let tag = b.const_tag(TBOOLEAN);
        let c1 = b.const_int(1);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NotAny, tag, c1);
        let r = b.vm_reg(21);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);
      }

      un_d!(IrCmd::StoreDouble, 22, IrCmd::SignNum, -4.0);

      un_i!(IrCmd::StoreInt, 23, IrCmd::Sexti8Int, 0x7f);
      un_i!(IrCmd::StoreInt, 24, IrCmd::Sexti8Int, 0xf1);
      un_i!(IrCmd::StoreInt, 25, IrCmd::Sexti16Int, 0x7fff);
      un_i!(IrCmd::StoreInt, 26, IrCmd::Sexti16Int, 0xf111);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 30i\n   STORE_INT R1, -2147483648i\n   STORE_INT R2, -10i\n   STORE_INT R3, 2147483647i\n   STORE_DOUBLE R4, 7\n   STORE_DOUBLE R5, -3\n   STORE_DOUBLE R6, 10\n   STORE_DOUBLE R7, 0.40000000000000002\n   STORE_DOUBLE R8, 1\n   STORE_DOUBLE R10, 2\n   STORE_DOUBLE R11, 5\n   STORE_DOUBLE R12, -5\n   STORE_DOUBLE R13, 2\n   STORE_DOUBLE R14, 3\n   STORE_DOUBLE R15, 3\n   STORE_DOUBLE R16, 4\n   STORE_DOUBLE R17, 4\n   STORE_INT R18, 1i\n   STORE_INT R19, 0i\n   STORE_INT R20, 1i\n   STORE_INT R21, 0i\n   STORE_DOUBLE R22, -1\n   STORE_INT R23, 127i\n   STORE_INT R24, -15i\n   STORE_INT R25, 32767i\n   STORE_INT R26, -3823i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_numeric_conversions {
  #[cfg(test)]
  #[test]
  fn ir_builder_numeric_conversions() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let c8 = b.const_int(8);
      let op = b.inst_ir_cmd_ir_op(IrCmd::IntToNum, c8);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c = b.const_int(0xdeee0000u32 as i32);
      let op = b.inst_ir_cmd_ir_op(IrCmd::UintToNum, c);
      let r = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c = b.const_double(200.0);
      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, c);
      let r = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);

      let c = b.const_double(3740139520.0);
      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, c);
      let r = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);

      let c = b.const_double(-10.0);
      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, c);
      let r = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);

      let c = b.const_double(-12345678901234.0);
      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, c);
      let r = b.vm_reg(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 8\n   STORE_DOUBLE R1, 3740139520\n   STORE_INT R2, 200i\n   STORE_INT R3, -554827776i\n   STORE_INT R4, -10i\n   STORE_INT R5, -1942892530i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_numeric_conversions_blocked {
  #[cfg(test)]
  #[test]
  fn ir_builder_numeric_conversions_blocked() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let z1 = b.const_double(0.0);
      let z2 = b.const_double(0.0);
      let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, z1, z2);

      let c = b.const_double(1e20);
      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, c);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);

      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToInt, nan);
      let r = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);

      let op = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, nan);
      let r = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %1 = NUM_TO_INT 1e+20\n   STORE_INT R0, %1\n   %3 = NUM_TO_INT nan\n   STORE_INT R1, %3\n   %5 = NUM_TO_UINT nan\n   STORE_INT R2, %5\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_numeric_nan {
  #[cfg(test)]
  #[test]
  fn ir_builder_numeric_nan() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let z1 = b.const_double(0.0);
      let z2 = b.const_double(0.0);
      let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, z1, z2);

      let c2 = b.const_double(2.0);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MinNum, nan, c2);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c1 = b.const_double(1.0);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MinNum, c1, nan);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c2 = b.const_double(2.0);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MaxNum, nan, c2);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c1 = b.const_double(1.0);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MaxNum, c1, nan);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 2\n   STORE_DOUBLE R0, nan\n   STORE_DOUBLE R0, 2\n   STORE_DOUBLE R0, nan\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_numeric_simplifications {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3474:ir_builder_numeric_simplifications`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_numeric_simplifications

  #[cfg(test)]
  #[test]
  fn ir_builder_numeric_simplifications() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);

      let zero = b.const_double(0.0);
      let sub = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SubNum, value, zero);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, sub);
      let neg_zero = b.const_double(-0.0);
      let add = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, value, neg_zero);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, add);

      let one = b.const_double(1.0);
      let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, value, one);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r3, mul);
      let two = b.const_double(2.0);
      let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, value, two);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r4, mul);
      let neg_one = b.const_double(-1.0);
      let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, value, neg_one);
      let r5 = b.vm_reg(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r5, mul);
      let three = b.const_double(3.0);
      let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, value, three);
      let r6 = b.vm_reg(6);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r6, mul);

      let one = b.const_double(1.0);
      let div = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, value, one);
      let r7 = b.vm_reg(7);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r7, div);
      let neg_one = b.const_double(-1.0);
      let div = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, value, neg_one);
      let r8 = b.vm_reg(8);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r8, div);
      let thirty_two = b.const_double(32.0);
      let div = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, value, thirty_two);
      let r9 = b.vm_reg(9);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r9, div);
      let six = b.const_double(6.0);
      let div = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, value, six);
      let r10 = b.vm_reg(10);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r10, div);

      let r1 = b.vm_reg(1);
      let nine = b.const_int(9);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, nine);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   STORE_DOUBLE R1, %0\n   STORE_DOUBLE R2, %0\n   STORE_DOUBLE R3, %0\n   %7 = ADD_NUM %0, %0\n   STORE_DOUBLE R4, %7\n   %9 = UNM_NUM %0\n   STORE_DOUBLE R5, %9\n   %11 = MUL_NUM %0, 3\n   STORE_DOUBLE R6, %11\n   STORE_DOUBLE R7, %0\n   %15 = UNM_NUM %0\n   STORE_DOUBLE R8, %15\n   %17 = MUL_NUM %0, 0.03125\n   STORE_DOUBLE R9, %17\n   %19 = DIV_NUM %0, 6\n   STORE_DOUBLE R10, %19\n   RETURN R1, 9i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_partial_over_full_value {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6852:ir_builder_partial_over_full_value`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_partial_over_full_value

  #[cfg(test)]
  #[test]
  fn ir_builder_partial_over_full_value() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TSTRING: u8 = 6;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r0, tnumber, one);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r0 = b.vm_reg(0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, four);
      let sixteen = b.const_uint(16);
      let thirty_two = b.const_uint(32);
      let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, sixteen, thirty_two);
      let r0 = b.vm_reg(0);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r0, ttable, table);
      let eight = b.const_uint(8);
      let sixteen = b.const_uint(16);
      let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, eight, sixteen);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r0, table);
      let four = b.const_uint(4);
      let eight = b.const_uint(8);
      let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, four, eight);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r0, table);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r0, tnumber, one);
      let r0 = b.vm_reg(0);
      let tstring = b.const_tag(TSTRING);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tstring);
      let sixteen = b.const_uint(16);
      let thirty_two = b.const_uint(32);
      let newtable = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, sixteen, thirty_two);
      let r0 = b.vm_reg(0);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, ttable);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r0, newtable);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %11 = NEW_TABLE 16u, 32u\n   STORE_SPLIT_TVALUE R0, ttable, %11\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_partial_store_invalidation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4015:ir_builder_partial_store_invalidation`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_partial_store_invalidation

  #[cfg(test)]
  #[test]
  fn ir_builder_partial_store_invalidation() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let tv = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, tv);
      let r0 = b.vm_reg(0);
      let half = b.const_double(0.5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, half);
      let r0 = b.vm_reg(0);
      let tv = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, tv);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let tv = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, tv);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TVALUE R0\n   STORE_TVALUE R1, %0\n   STORE_DOUBLE R0, 0.5\n   %3 = LOAD_TVALUE R0\n   STORE_TVALUE R1, %3\n   STORE_TAG R0, tnumber\n   STORE_SPLIT_TVALUE R1, tnumber, 0.5\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_partial_vs_full_stores_no_removal_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6296:ir_builder_partial_vs_full_stores_no_removal_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_partial_vs_full_stores_no_removal_1

  #[cfg(test)]
  #[test]
  fn ir_builder_partial_vs_full_stores_no_removal_1() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let zero = b.const_int(0);
      let tnumber = b.const_tag(TNUMBER);
      let value = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, r1, zero, tnumber);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, value);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, one);
      let r2 = b.vm_reg(2);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r2);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, value);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R1, R2\n   %3 = LOAD_TVALUE R2\n   STORE_TVALUE R0, %3\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_partial_vs_full_stores_no_removal_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6321:ir_builder_partial_vs_full_stores_no_removal_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_partial_vs_full_stores_no_removal_2

  #[cfg(test)]
  #[test]
  fn ir_builder_partial_vs_full_stores_no_removal_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let zero = b.const_int(0);
      let tnumber = b.const_tag(TNUMBER);
      let value = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, r1, zero, tnumber);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, value);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, one);
      let r2 = b.vm_reg(2);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r0, tnumber, value);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R1, R2\n   %3 = LOAD_DOUBLE R2\n   STORE_SPLIT_TVALUE R0, tnumber, %3\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_partial_vs_full_stores_with_recombination {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6273:ir_builder_partial_vs_full_stores_with_recombination`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_partial_vs_full_stores_with_recombination

  #[cfg(test)]
  #[test]
  fn ir_builder_partial_vs_full_stores_with_recombination() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let loaded = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, loaded);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_SPLIT_TVALUE R0, tnumber, 1\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_propagate_through_tvalue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2613:ir_builder_propagate_through_tvalue`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_propagate_through_tvalue

  #[cfg(test)]
  #[test]
  fn ir_builder_propagate_through_tvalue() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let half = b.const_double(0.5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, half);

      let r0 = b.vm_reg(0);
      let tv = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r1, tv);

      let r1 = b.vm_reg(1);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r3, tag);
      let r1 = b.vm_reg(1);
      let double = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r3, double);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   STORE_DOUBLE R0, 0.5\n   STORE_SPLIT_TVALUE R1, tnumber, 0.5\n   STORE_TAG R3, tnumber\n   STORE_DOUBLE R3, 0.5\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_recursive_scc_use_removal_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3278:ir_builder_recursive_scc_use_removal_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_recursive_scc_use_removal_1

  #[cfg(test)]
  #[test]
  fn ir_builder_recursive_scc_use_removal_1() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let block = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);
      let repeat = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, r0, exit, repeat);

      b.begin_block(exit);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(repeat);
      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, zero);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_recursive_scc_use_removal_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3309:ir_builder_recursive_scc_use_removal_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_recursive_scc_use_removal_2

  #[cfg(test)]
  #[test]
  fn ir_builder_recursive_scc_use_removal_2() {
    use ulua_code_gen::{
      enums::{
        include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
        ir_condition::IrCondition,
      },
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let exit1 = b.block(IrBlockKind::Internal);
      let block = b.block(IrBlockKind::Internal);
      let exit2 = b.block(IrBlockKind::Internal);
      let repeat = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let zero = b.const_int(0);
      let one = b.const_int(1);
      let equal = b.cond(IrCondition::Equal);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpInt,
        zero,
        one,
        equal,
        block,
        exit1,
      );

      b.begin_block(exit1);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(block);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, r0, exit2, repeat);

      b.begin_block(exit2);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(repeat);
      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, zero);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n; glued to: bb_1\n\nbb_1:\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_redundant_store_check_constant_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2913:ir_builder_redundant_store_check_constant_type`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_redundant_store_check_constant_type

  #[cfg(test)]
  #[test]
  fn ir_builder_redundant_store_check_constant_type() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let ten = b.const_int(10);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, ten);
      let r0 = b.vm_reg(0);
      let half = b.const_double(0.5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, half);
      let r0 = b.vm_reg(0);
      let ten = b.const_int(10);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, ten);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 10i\n   STORE_DOUBLE R0, 0.5\n   STORE_INT R0, 10i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_register_versioning {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5477:ir_builder_register_versioning`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_register_versioning

  #[cfg(test)]
  #[test]
  fn ir_builder_register_versioning() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let op2 = b.inst_ir_cmd_ir_op(IrCmd::UnmNum, op1);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, op2);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let op3 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let op4 = b.inst_ir_cmd_ir_op(IrCmd::UnmNum, op3);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, op4);
      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   %1 = UNM_NUM %0\n   STORE_DOUBLE R0, %1\n   STORE_TAG R0, tnumber\n   %5 = UNM_NUM %1\n   STORE_DOUBLE R1, %5\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_remember_int_64_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2488:ir_builder_remember_int_64_values`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt64 (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_remember_int_64_values

  #[cfg(test)]
  #[test]
  fn ir_builder_remember_int_64_values() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let c42 = b.const_int_64(42);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r0, c42);

      let r0 = b.vm_reg(0);
      let loaded = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r1, loaded);

      let r0 = b.vm_reg(0);
      let c42 = b.const_int_64(42);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r0, c42);

      let r5 = b.vm_reg(5);
      let unknown = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r5);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r0, unknown);

      let r0 = b.vm_reg(0);
      let loaded = b.inst_ir_cmd_ir_op(IrCmd::LoadInt64, r0);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, r2, loaded);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 42i\n   STORE_INT64 R1, 42i\n   %4 = LOAD_INT64 R5\n   STORE_INT64 R0, %4\n   STORE_INT64 R2, %4\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_remember_new_table_state {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2745:ir_builder_remember_new_table_state`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_remember_new_table_state

  #[cfg(test)]
  #[test]
  fn ir_builder_remember_new_table_state() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let c16 = b.const_uint(16);
      let c32 = b.const_uint(32);
      let newtable = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, c16, c32);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r0, newtable);

      let r0 = b.vm_reg(0);
      let table = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);
      let c14 = b.const_int(14);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table, c14, fallback);

      let r1 = b.vm_reg(1);
      let r0 = b.vm_reg(0);
      let c13 = b.const_uint(13);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SetTable, r1, r0, c13);

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);
      let c14 = b.const_int(14);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table, c14, fallback);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

      b.begin_block(fallback);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = NEW_TABLE 16u, 32u\n   STORE_POINTER R0, %0\n   SET_TABLE R1, R0, 13u\n   CHECK_NO_METATABLE %0, bb_fallback_1\n   CHECK_READONLY %0, bb_fallback_1\n   CHECK_ARRAY_SIZE %0, 14i, bb_fallback_1\n   RETURN 0u\n\nbb_fallback_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_remember_table_state {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2701:ir_builder_remember_table_state`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_remember_table_state

  #[cfg(test)]
  #[test]
  fn ir_builder_remember_table_state() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let table = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);

      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, r1, r2);

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, table, fallback);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, fallback);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

      b.begin_block(fallback);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R0\n   CHECK_NO_METATABLE %0, bb_fallback_1\n   CHECK_READONLY %0, bb_fallback_1\n   DO_LEN R1, R2\n   CHECK_NO_METATABLE %0, bb_fallback_1\n   CHECK_READONLY %0, bb_fallback_1\n   RETURN 0u\n\nbb_fallback_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_remember_tags_and_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2430:ir_builder_remember_tags_and_values`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_remember_tags_and_values

  #[cfg(test)]
  #[test]
  fn ir_builder_remember_tags_and_values() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r1 = b.vm_reg(1);
      let ten = b.const_int(10);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, ten);
      let r2 = b.vm_reg(2);
      let half = b.const_double(0.5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, half);

      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r3, tag);
      let r1 = b.vm_reg(1);
      let int = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r1);
      let r4 = b.vm_reg(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r4, int);
      let r2 = b.vm_reg(2);
      let double = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
      let r5 = b.vm_reg(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r5, double);

      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r1 = b.vm_reg(1);
      let ten = b.const_int(10);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, ten);
      let r2 = b.vm_reg(2);
      let half = b.const_double(0.5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, half);

      let r6 = b.vm_reg(6);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r6);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tag);
      let r7 = b.vm_reg(7);
      let int = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r7);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r1, int);
      let r8 = b.vm_reg(8);
      let double = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r8);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, double);

      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let r9 = b.vm_reg(9);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r9, tag);
      let r1 = b.vm_reg(1);
      let int = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r1);
      let r10 = b.vm_reg(10);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r10, int);
      let r2 = b.vm_reg(2);
      let double = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
      let r11 = b.vm_reg(11);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r11, double);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   STORE_INT R1, 10i\n   STORE_DOUBLE R2, 0.5\n   STORE_TAG R3, tnumber\n   STORE_INT R4, 10i\n   STORE_DOUBLE R5, 0.5\n   %12 = LOAD_TAG R6\n   STORE_TAG R0, %12\n   %14 = LOAD_INT R7\n   STORE_INT R1, %14\n   %16 = LOAD_DOUBLE R8\n   STORE_DOUBLE R2, %16\n   %18 = LOAD_TAG R0\n   STORE_TAG R9, %18\n   STORE_INT R10, %14\n   STORE_DOUBLE R11, %16\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_remove_duplicate_calculation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5411:ir_builder_remove_duplicate_calculation`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_remove_duplicate_calculation

  #[cfg(test)]
  #[test]
  fn ir_builder_remove_duplicate_calculation() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let op2 = b.inst_ir_cmd_ir_op(IrCmd::UnmNum, op1);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, op2);
      let r0 = b.vm_reg(0);
      let op3 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let op4 = b.inst_ir_cmd_ir_op(IrCmd::UnmNum, op3);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, op4);
      let r1 = b.vm_reg(1);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   %1 = UNM_NUM %0\n   STORE_DOUBLE R1, %1\n   STORE_DOUBLE R2, %1\n   RETURN R1, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_replacement_preserves_uses {
  #[cfg(test)]
  #[test]
  fn ir_builder_replacement_preserves_uses() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let unk = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, r0);
      let mask = b.const_int(!0u32 as i32);
      let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BitxorUint, unk, mask);
      let r = b.vm_reg(8);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::Yes);
    let expected = "\nbb_0:                                                       ; useCount: 0\n   %0 = LOAD_INT R0                                          ; useCount: 1, lastUse: %0\n   %1 = BITNOT_UINT %0                                       ; useCount: 1, lastUse: %0\n   STORE_INT R8, %1                                          ; %2\n   RETURN 0u                                                 ; %3\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_safe_partial_value_stores_with_preserved_tag {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6680:ir_builder_safe_partial_value_stores_with_preserved_tag`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_safe_partial_value_stores_with_preserved_tag

  #[cfg(test)]
  #[test]
  fn ir_builder_safe_partial_value_stores_with_preserved_tag() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);
      let last = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r1, tnumber, one);
      b.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, fallback);
      let r1 = b.vm_reg(1);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, two);
      let r1 = b.vm_reg(1);
      let three = b.const_double(3.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, three);
      let r1 = b.vm_reg(1);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, four);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(fallback);
      b.inst_ir_cmd(IrCmd::CheckGc);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(last);
      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_2\n; in regs: R0\n; out regs: R0, R1\n   STORE_SPLIT_TVALUE R1, tnumber, 1\n   CHECK_SAFE_ENV bb_fallback_1\n   STORE_DOUBLE R1, 4\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0\n; successors: bb_2\n; in regs: R0, R1\n; out regs: R0, R1\n   CHECK_GC\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0, R1\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_safe_partial_value_stores_with_preserved_tag_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6733:ir_builder_safe_partial_value_stores_with_preserved_tag_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_safe_partial_value_stores_with_preserved_tag_2

  #[cfg(test)]
  #[test]
  fn ir_builder_safe_partial_value_stores_with_preserved_tag_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);
      let last = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r1, tnumber, one);
      b.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, fallback);
      let r1 = b.vm_reg(1);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, two);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r1, tnumber, four);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(fallback);
      b.inst_ir_cmd(IrCmd::CheckGc);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(last);
      let r0 = b.vm_reg(0);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_2\n; in regs: R0\n; out regs: R0, R1\n   STORE_SPLIT_TVALUE R1, tnumber, 1\n   CHECK_SAFE_ENV bb_fallback_1\n   STORE_SPLIT_TVALUE R1, tnumber, 4\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0\n; successors: bb_2\n; in regs: R0, R1\n; out regs: R0, R1\n   CHECK_GC\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0, R1\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_select_if_truthy {
  #[cfg(test)]
  #[test]
  fn ir_builder_select_if_truthy() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let unknown_tv1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r2 = b.vm_reg(2);
      let unknown_tv2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r2);

      let op = b.inst_ir_cmd_ir_op_ir_op_ir_op(
        IrCmd::SelectIfTruthy,
        unknown_tv1,
        unknown_tv2,
        unknown_tv2,
      );
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %1 = LOAD_TVALUE R2\n   STORE_TVALUE R0, %1\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_select_number {
  #[cfg(test)]
  #[test]
  fn ir_builder_select_number() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let zero_num = b.const_double(0.0);
      let one_num = b.const_double(1.0);
      let r0 = b.vm_reg(0);
      let unknown_num = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);

      let c4 = b.const_double(4.0);
      let c8 = b.const_double(8.0);
      let op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectNum, c4, c8, zero_num, zero_num);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c4 = b.const_double(4.0);
      let c8 = b.const_double(8.0);
      let op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectNum, c4, c8, zero_num, one_num);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c4a = b.const_double(4.0);
      let c4b = b.const_double(4.0);
      let op =
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectNum, c4a, c4b, zero_num, unknown_num);
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 8\n   STORE_DOUBLE R0, 4\n   STORE_DOUBLE R0, 4\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_select_vector {
  #[cfg(test)]
  #[test]
  fn ir_builder_select_vector() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts},
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let unknown_vec1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r2 = b.vm_reg(2);
      let unknown_vec2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r2);
      let r3 = b.vm_reg(3);
      let unknown_vec3 = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r3);

      let op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::SelectVec,
        unknown_vec3,
        unknown_vec3,
        unknown_vec1,
        unknown_vec2,
      );
      let r = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r, op);

      let c0 = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %2 = LOAD_TVALUE R3\n   STORE_TVALUE R0, %2\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_set_list_is_a_blocker {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5508:ir_builder_set_list_is_a_blocker`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_set_list_is_a_blocker

  #[cfg(test)]
  #[test]
  fn ir_builder_set_list_is_a_blocker() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let zero = b.const_uint(0);
      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      let one_int = b.const_int(1);
      let one_uint = b.const_uint(1);
      let undef = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::SETLIST,
        zero,
        r1,
        r2,
        one_int,
        one_uint,
        undef,
      );
      let r0 = b.vm_reg(0);
      let op2 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, op1, op2);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, sum);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   SETLIST 0u, R1, R2, 1i, 1u, undef\n   %2 = LOAD_DOUBLE R0\n   %3 = ADD_NUM %0, %2\n   STORE_DOUBLE R0, %3\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_set_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5337:ir_builder_set_table`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
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
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_set_table

  #[cfg(test)]
  #[test]
  fn ir_builder_set_table() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let r1 = b.vm_reg(1);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SetTable, r0, r1, one);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R1\n   SET_TABLE R0, R1, 1u\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_simple_diamond {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4967:ir_builder_simple_diamond`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_simple_diamond

  #[cfg(test)]
  #[test]
  fn ir_builder_simple_diamond() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let a = b.block(IrBlockKind::Internal);
      let branch_b = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, tag, tnumber, a, branch_b);

      b.begin_block(a);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, value);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(branch_b);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r2 = b.vm_reg(2);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1, bb_2\n; in regs: R0, R1, R2, R3\n; out regs: R1, R2, R3\n   %0 = LOAD_TAG R0\n   JUMP_EQ_TAG %0, tnumber, bb_1, bb_2\n\nbb_1:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R1, R3\n; out regs: R2, R3\n   %2 = LOAD_TVALUE R1\n   STORE_TVALUE R2, %2\n   JUMP bb_3\n\nbb_2:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R1, R2\n; out regs: R2, R3\n   %5 = LOAD_TVALUE R1\n   STORE_TVALUE R3, %5\n   JUMP bb_3\n\nbb_3:\n; predecessors: bb_1, bb_2\n; in regs: R2, R3\n   RETURN R2, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_simple_double_store {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5944:ir_builder_simple_double_store`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_simple_double_store

  #[cfg(test)]
  #[test]
  fn ir_builder_simple_double_store() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r1 = b.vm_reg(1);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, two);

      let r2 = b.vm_reg(2);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r2, one);
      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let r2 = b.vm_reg(2);
      let four = b.const_int(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r2, four);
      let r2 = b.vm_reg(2);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tboolean);

      let r3 = b.vm_reg(3);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r3, tnil);
      let r3 = b.vm_reg(3);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r3, tnumber);
      let r3 = b.vm_reg(3);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r3, four);

      let r4 = b.vm_reg(4);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r4, tnil);
      let r4 = b.vm_reg(4);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r4, one);
      let r4 = b.vm_reg(4);
      let tnumber = b.const_tag(TNUMBER);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r4, tnumber, two);

      let r0 = b.vm_reg(0);
      let some_tv = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r5 = b.vm_reg(5);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r5, tnil);
      let r5 = b.vm_reg(5);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r5, one);
      let r5 = b.vm_reg(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r5, some_tv);

      let r1 = b.vm_reg(1);
      let five = b.const_int(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, five);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   STORE_SPLIT_TVALUE R1, tnumber, 2\n   STORE_SPLIT_TVALUE R2, tboolean, 4i\n   STORE_TAG R3, tnumber\n   STORE_DOUBLE R3, 4\n   STORE_SPLIT_TVALUE R4, tnumber, 2\n   %13 = LOAD_TVALUE R0\n   STORE_TVALUE R5, %13\n   RETURN R1, 5i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_simple_path_extraction {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3835:ir_builder_simple_path_extraction`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function createLinearBlocks (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_simple_path_extraction

  #[cfg(test)]
  #[test]
  fn ir_builder_simple_path_extraction() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains,
        create_linear_blocks::create_linear_blocks, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block1 = b.block(IrBlockKind::Internal);
      let fallback1 = b.fallback_block(0);
      let block2 = b.block(IrBlockKind::Internal);
      let fallback2 = b.fallback_block(0);
      let block3 = b.block(IrBlockKind::Internal);
      let block4 = b.block(IrBlockKind::Internal);

      b.begin_block(block1);
      let r2 = b.vm_reg(2);
      let tag1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag1, tnumber, fallback1);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

      b.begin_block(fallback1);
      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, r1, r2);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

      b.begin_block(block2);
      let r2 = b.vm_reg(2);
      let tag2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag2, tnumber, fallback2);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block3);

      b.begin_block(fallback2);
      let r0 = b.vm_reg(0);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, r0, r2);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block3);

      b.begin_block(block3);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, block4);

      b.begin_block(block4);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    create_linear_blocks(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R2\n   CHECK_TAG %0, tnumber, bb_fallback_1\n   JUMP bb_linear_6\n; glued to: bb_linear_6\n\nbb_fallback_1:\n   DO_LEN R1, R2\n   JUMP bb_2\n\nbb_2:\n   %5 = LOAD_TAG R2\n   CHECK_TAG %5, tnumber, bb_fallback_3\n   JUMP bb_4\n\nbb_fallback_3:\n   DO_LEN R0, R2\n   JUMP bb_4\n\nbb_4:\n   JUMP bb_5\n; glued to: bb_5\n\nbb_5:\n   RETURN R0, 0i\n\nbb_linear_6:\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_skip_check_tag {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2646:ir_builder_skip_check_tag`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_skip_check_tag

  #[cfg(test)]
  #[test]
  fn ir_builder_skip_check_tag() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

      b.begin_block(fallback);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_skip_once_per_block_checks {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2671:ir_builder_skip_once_per_block_checks`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_skip_once_per_block_checks

  #[cfg(test)]
  #[test]
  fn ir_builder_skip_once_per_block_checks() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      b.inst_ir_cmd(IrCmd::CheckSafeEnv);
      b.inst_ir_cmd(IrCmd::CheckSafeEnv);
      b.inst_ir_cmd(IrCmd::CheckGc);
      b.inst_ir_cmd(IrCmd::CheckGc);

      let r1 = b.vm_reg(1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, r1, r2);
      b.inst_ir_cmd(IrCmd::CheckSafeEnv);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   CHECK_SAFE_ENV\n   CHECK_GC\n   DO_LEN R1, R2\n   CHECK_SAFE_ENV\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_skip_useless_barriers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2791:ir_builder_skip_useless_barriers`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_skip_useless_barriers

  #[cfg(test)]
  #[test]
  fn ir_builder_skip_useless_barriers() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r1 = b.vm_reg(1);
      let table = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let r0 = b.vm_reg(0);
      let undef = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, table, r0, undef);
      let r2 = b.vm_reg(2);
      let something = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r2);
      let r0 = b.vm_reg(0);
      let undef = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierObj, something, r0, undef);
      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_store_cannot_be_replaced_with_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6445:ir_builder_store_cannot_be_replaced_with_check`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_store_cannot_be_replaced_with_check

  #[cfg(test)]
  #[test]
  fn ir_builder_store_cannot_be_replaced_with_check() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ir_builder_fixture::IrBuilderFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const TNIL: u8 = 0;
    const TTABLE: u8 = 7;

    let _debug_luau_aborting_checks = ScopedFastFlag::new(&FFlag::DebugLuauAbortingChecks, true);

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);
      let last = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r1 = b.vm_reg(1);
      let ptr = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);

      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r2, ptr);
      let r2 = b.vm_reg(2);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, ttable);

      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, ptr, fallback);

      let r0 = b.vm_reg(0);
      let load = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r2, load);
      let r2 = b.vm_reg(2);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, ttable);

      let r2 = b.vm_reg(2);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnil);

      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(fallback);
      let r1 = b.vm_reg(1);
      let fallback_ptr = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r2, fallback_ptr);
      let r2 = b.vm_reg(2);
      let ttable = b.const_tag(TTABLE);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, ttable);
      b.inst_ir_cmd(IrCmd::CheckGc);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

      b.begin_block(last);
      let r0 = b.vm_reg(0);
      let three = b.const_int(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, three);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_2\n; in regs: R0, R1\n; out regs: R0, R1, R2\n   %0 = LOAD_POINTER R1\n   CHECK_READONLY %0, bb_fallback_1\n   STORE_TAG R2, tnil\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0\n; successors: bb_2\n; in regs: R0, R1\n; out regs: R0, R1, R2\n   %9 = LOAD_POINTER R1\n   STORE_POINTER R2, %9\n   STORE_TAG R2, ttable\n   CHECK_GC\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0, R1, R2\n   RETURN R0, 3i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_t_value_load_to_split_store {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5652:ir_builder_t_value_load_to_split_store`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> type_alias TValue (VM/src/lobject.h)
  //!   - calls -> function split (Common/src/StringUtils.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_t_value_load_to_split_store

  #[cfg(test)]
  #[test]
  fn ir_builder_t_value_load_to_split_store() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let four = b.const_double(4.0);
      let op1v2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, op1, four);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, op1v2);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);

      let r1 = b.vm_reg(1);
      let tv = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, tv);

      let r2 = b.vm_reg(2);
      let tag2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag2, tnumber, fallback);
      let r2 = b.vm_reg(2);
      let op2 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
      let r3 = b.vm_reg(3);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r3, op2);
      let r3 = b.vm_reg(3);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r3, tnumber);

      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);

      b.begin_block(fallback);
      let r2 = b.vm_reg(2);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   %1 = ADD_NUM %0, 4\n   STORE_DOUBLE R1, %1\n   STORE_TAG R1, tnumber\n   STORE_SPLIT_TVALUE R2, tnumber, %1\n   STORE_DOUBLE R3, %1\n   STORE_TAG R3, tnumber\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_and_value_over_tvalue_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7126:ir_builder_tag_and_value_over_tvalue_1`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> type_alias TValue (VM/src/lobject.h)
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_and_value_over_tvalue_1

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_and_value_over_tvalue_1() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, value);
      let r0 = b.vm_reg(0);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tboolean);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, one);
      let r0 = b.vm_reg(0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, four);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R1\n   %0 = LOAD_TVALUE R1\n   STORE_TVALUE R0, %0\n   STORE_SPLIT_TVALUE R0, tnumber, 4\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_and_value_over_tvalue_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7155:ir_builder_tag_and_value_over_tvalue_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> type_alias TValue (VM/src/lobject.h)
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_and_value_over_tvalue_2

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_and_value_over_tvalue_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, value);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, one);
      let r0 = b.vm_reg(0);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tboolean);
      let r0 = b.vm_reg(0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, four);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R1\n   %0 = LOAD_TVALUE R1\n   STORE_TVALUE R0, %0\n   STORE_SPLIT_TVALUE R0, tnumber, 4\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_check_propagation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2938:ir_builder_tag_check_propagation`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_check_propagation

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_check_propagation() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let unknown = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);

      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, unknown, tnumber, fallback);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, unknown, tnumber, fallback);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

      b.begin_block(fallback);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R0\n   CHECK_TAG %0, tnumber, bb_fallback_1\n   RETURN 0u\n\nbb_fallback_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_check_propagation_conflicting {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2970:ir_builder_tag_check_propagation_conflicting`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_check_propagation_conflicting

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_check_propagation_conflicting() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let unknown = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);

      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, unknown, tnumber, fallback);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, unknown, tnil, fallback);

      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

      b.begin_block(fallback);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R0\n   CHECK_TAG %0, tnumber, bb_fallback_1\n   JUMP bb_fallback_1\n\nbb_fallback_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_eq_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3082:ir_builder_tag_eq_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_eq_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_eq_removal() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckTag, tag, tboolean);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        tag,
        tnumber,
        true_block,
        false_block,
      );

      b.begin_block(true_block);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);

      b.begin_block(false_block);
      let two = b.const_uint(2);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R1\n   CHECK_TAG %0, tboolean\n   JUMP bb_2\n; glued to: bb_2\n\nbb_2:\n   RETURN 2u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_self_equality_check_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5787:ir_builder_tag_self_equality_check_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_self_equality_check_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_self_equality_check_removal() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tag1 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let r0 = b.vm_reg(0);
      let tag2 = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, tag1, tag2, true_block, false_block);

      b.begin_block(true_block);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);

      b.begin_block(false_block);
      let two = b.const_uint(2);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n; glued to: bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_store_updates_set_upval {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5761:ir_builder_tag_store_updates_set_upval`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmUpvalue (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_store_updates_set_upval

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_store_updates_set_upval() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let half = b.const_double(0.5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, half);

      let u0 = b.vm_upvalue(0);
      let r0 = b.vm_reg(0);
      let undef = b.undef();
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SetUpvalue, u0, r0, undef);

      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   STORE_DOUBLE R0, 0.5\n   SET_UPVALUE U0, R0, tnumber\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_store_updates_value_version {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5696:ir_builder_tag_store_updates_value_version`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_store_updates_value_version

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_store_updates_value_version() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TSTRING: u8 = 6;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let op1 = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r0);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r1, op1);
      let r1 = b.vm_reg(1);
      let tstring = b.const_tag(TSTRING);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tstring);

      let r1 = b.vm_reg(1);
      let str_op = b.inst_ir_cmd_ir_op(IrCmd::LoadPointer, r1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, r2, str_op);
      let r2 = b.vm_reg(2);
      let tstring = b.const_tag(TSTRING);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tstring);

      let r1 = b.vm_reg(1);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R0\n   STORE_POINTER R1, %0\n   STORE_TAG R1, tstring\n   STORE_POINTER R2, %0\n   STORE_TAG R2, tstring\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tag_vector_skip_error_fix {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4723:ir_builder_tag_vector_skip_error_fix`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tag_vector_skip_error_fix

  #[cfg(test)]
  #[test]
  fn ir_builder_tag_vector_skip_error_fix() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r0 = b.vm_reg(0);
      let a = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r1 = b.vm_reg(1);
      let b_value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);

      let mul_vec = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MulVec, a, b_value);
      let mul = b.inst_ir_cmd_ir_op(IrCmd::TagVector, mul_vec);

      let add_vec = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddVec, mul, mul);
      let t1 = b.inst_ir_cmd_ir_op(IrCmd::TagVector, add_vec);
      let sub_vec = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SubVec, mul, mul);
      let t2 = b.inst_ir_cmd_ir_op(IrCmd::TagVector, sub_vec);

      let unm_vec = b.inst_ir_cmd_ir_op(IrCmd::UnmVec, t2);
      let div_vec = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DivVec, t1, unm_vec);
      let t3 = b.inst_ir_cmd_ir_op(IrCmd::TagVector, div_vec);

      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r0, t3);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::Yes);
    let expected = "\nbb_0:                                                       ; useCount: 0\n   %0 = LOAD_TVALUE R0                                       ; useCount: 1, lastUse: %0\n   %1 = LOAD_TVALUE R1                                       ; useCount: 1, lastUse: %0\n   %2 = MUL_VEC %0, %1                                       ; useCount: 4, lastUse: %0\n   %4 = ADD_VEC %2, %2                                       ; useCount: 1, lastUse: %0\n   %6 = SUB_VEC %2, %2                                       ; useCount: 1, lastUse: %0\n   %8 = UNM_VEC %6                                           ; useCount: 1, lastUse: %0\n   %9 = DIV_VEC %4, %8                                       ; useCount: 1, lastUse: %0\n   %10 = TAG_VECTOR %9                                       ; useCount: 1, lastUse: %0\n   STORE_TVALUE R0, %10                                      ; %11\n   RETURN R0, 1u                                             ; %12\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tagged_value_propagation_into_tvalue_checks_register_version {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5819:ir_builder_tagged_value_propagation_into_tvalue_checks_register_version`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_tagged_value_propagation_into_tvalue_checks_register_version

  #[cfg(test)]
  #[test]
  fn ir_builder_tagged_value_propagation_into_tvalue_checks_register_version() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let a1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r0);
      let r1 = b.vm_reg(1);
      let b1 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r1);
      let sum1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a1, b1);

      let r7 = b.vm_reg(7);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r7, sum1);
      let r7 = b.vm_reg(7);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r7, tnumber);

      let r2 = b.vm_reg(2);
      let a2 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r2);
      let r3 = b.vm_reg(3);
      let b2 = b.inst_ir_cmd_ir_op(IrCmd::LoadDouble, r3);
      let sum2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, a2, b2);

      let r8 = b.vm_reg(8);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r8, sum2);
      let r8 = b.vm_reg(8);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r8, tnumber);

      let r7 = b.vm_reg(7);
      let zero = b.const_int(0);
      let tnumber = b.const_tag(TNUMBER);
      let old7 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, r7, zero, tnumber);
      let r8 = b.vm_reg(8);
      let zero = b.const_int(0);
      let tnumber = b.const_tag(TNUMBER);
      let old8 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, r8, zero, tnumber);

      let r8 = b.vm_reg(8);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r8, old7);
      let r9 = b.vm_reg(9);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r9, old8);

      let r8 = b.vm_reg(8);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r8, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R1, R2, R3\n   %0 = LOAD_DOUBLE R0\n   %1 = LOAD_DOUBLE R1\n   %2 = ADD_NUM %0, %1\n   STORE_DOUBLE R7, %2\n   STORE_TAG R7, tnumber\n   %5 = LOAD_DOUBLE R2\n   %6 = LOAD_DOUBLE R3\n   %7 = ADD_NUM %5, %6\n   STORE_DOUBLE R8, %7\n   STORE_TAG R8, tnumber\n   %11 = LOAD_TVALUE R8, 0i, tnumber\n   STORE_SPLIT_TVALUE R8, tnumber, %2\n   STORE_TVALUE R9, %11\n   RETURN R8, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tags_are_joined_from_predecessors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3685:ir_builder_tags_are_joined_from_predecessors`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_tags_are_joined_from_predecessors

  #[cfg(test)]
  #[test]
  fn ir_builder_tags_are_joined_from_predecessors() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TSTRING: u8 = 6;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry1 = b.block(IrBlockKind::Internal);
      let entry2 = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);

      b.begin_block(entry1);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r2 = b.vm_reg(2);
      let cond_tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        cond_tag,
        tnumber,
        true_block,
        false_block,
      );

      b.begin_block(entry2);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r1 = b.vm_reg(1);
      let tstring = b.const_tag(TSTRING);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tstring);
      let r2 = b.vm_reg(2);
      let cond_tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        cond_tag,
        tnumber,
        true_block,
        false_block,
      );

      b.begin_block(true_block);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r1 = b.vm_reg(1);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(false_block);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r1 = b.vm_reg(1);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_2, bb_3\n; in regs: R2\n; out regs: R0, R1\n   STORE_TAG R0, tnumber\n   STORE_TAG R1, tnumber\n   %2 = LOAD_TAG R2\n   JUMP_EQ_TAG %2, tnumber, bb_2, bb_3\n\nbb_1:\n; successors: bb_2, bb_3\n; in regs: R2\n; out regs: R0, R1\n   STORE_TAG R0, tnumber\n   STORE_TAG R1, tstring\n   %6 = LOAD_TAG R2\n   JUMP_EQ_TAG %6, tnumber, bb_2, bb_3\n\nbb_2:\n; predecessors: bb_0, bb_1\n; in regs: R0, R1\n   %10 = LOAD_TAG R1\n   CHECK_TAG %10, tnumber, exit(0)\n   RETURN R0, 0i\n\nbb_3:\n; predecessors: bb_0, bb_1\n; in regs: R0, R1\n   %15 = LOAD_TAG R1\n   CHECK_TAG %15, tnumber, exit(0)\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tags_are_joined_from_predecessors_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3758:ir_builder_tags_are_joined_from_predecessors_2`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_tags_are_joined_from_predecessors_2

  #[cfg(test)]
  #[test]
  fn ir_builder_tags_are_joined_from_predecessors_2() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TSTRING: u8 = 6;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry1 = b.block(IrBlockKind::Internal);
      let entry2 = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);

      b.begin_block(entry1);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let r0 = b.vm_reg(0);
      let cond_tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        cond_tag,
        tnumber,
        true_block,
        false_block,
      );

      b.begin_block(entry2);
      let r1 = b.vm_reg(1);
      let tstring = b.const_tag(TSTRING);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tstring);
      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let r0 = b.vm_reg(0);
      let cond_tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        cond_tag,
        tnumber,
        true_block,
        false_block,
      );

      b.begin_block(true_block);
      let r1 = b.vm_reg(1);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r2 = b.vm_reg(2);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(false_block);
      let r1 = b.vm_reg(1);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r2 = b.vm_reg(2);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r2);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_2, bb_3\n; in regs: R0\n; out regs: R1, R2\n   STORE_TAG R1, tnumber\n   STORE_TAG R2, tnumber\n   %2 = LOAD_TAG R0\n   JUMP_EQ_TAG %2, tnumber, bb_2, bb_3\n\nbb_1:\n; successors: bb_2, bb_3\n; in regs: R0\n; out regs: R1, R2\n   STORE_TAG R1, tstring\n   STORE_TAG R2, tnumber\n   %6 = LOAD_TAG R0\n   JUMP_EQ_TAG %6, tnumber, bb_2, bb_3\n\nbb_2:\n; predecessors: bb_0, bb_1\n; in regs: R1, R2\n   %8 = LOAD_TAG R1\n   CHECK_TAG %8, tnumber, exit(0)\n   RETURN R0, 0i\n\nbb_3:\n; predecessors: bb_0, bb_1\n; in regs: R1, R2\n   %13 = LOAD_TAG R1\n   CHECK_TAG %13, tnumber, exit(0)\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_tags_flow_from_single_predecessor {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3637:ir_builder_tags_flow_from_single_predecessor`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_tags_flow_from_single_predecessor

  #[cfg(test)]
  #[test]
  fn ir_builder_tags_flow_from_single_predecessor() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r1 = b.vm_reg(1);
      let cond_tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        cond_tag,
        tnumber,
        true_block,
        false_block,
      );

      b.begin_block(true_block);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

      b.begin_block(false_block);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      let fallback = b.vm_exit(0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag, tnumber, fallback);
      let r0 = b.vm_reg(0);
      let zero = b.const_int(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1, bb_2\n; in regs: R1\n; out regs: R0\n   STORE_TAG R0, tnumber\n   %1 = LOAD_TAG R1\n   JUMP_EQ_TAG %1, tnumber, bb_1, bb_2\n\nbb_1:\n; predecessors: bb_0\n; in regs: R0\n   RETURN R0, 0i\n\nbb_2:\n; predecessors: bb_0\n; in regs: R0\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_to_dot {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7695:ir_builder_to_dot`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function toDotCfg (CodeGen/src/IrDump.cpp)
  //!   - calls -> function toDotDjGraph (CodeGen/src/IrDump.cpp)
  //!   - translates_to -> rust_item ir_builder_to_dot

  #[cfg(test)]
  #[test]
  fn ir_builder_to_dot() {
    use ulua_code_gen::{
      enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_dot::to_dot, to_dot_cfg::to_dot_cfg,
        to_dot_dj_graph::to_dot_dj_graph, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let a = b.block(IrBlockKind::Internal);
      let branch_b = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, tag, tnumber, a, branch_b);

      b.begin_block(a);
      let r2 = b.vm_reg(2);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, value);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(branch_b);
      let r3 = b.vm_reg(3);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r3, value);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r2 = b.vm_reg(2);
      let two = b.const_int(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let _ = to_dot(&fix.build.function, true);
    let _ = to_dot_cfg(&fix.build.function);
    let _ = to_dot_dj_graph(&fix.build.function);
  }
}

mod ir_builder_truthy_test_removal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3002:ir_builder_truthy_test_removal`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_truthy_test_removal

  #[cfg(test)]
  #[test]
  fn ir_builder_truthy_test_removal() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);
      let true_block = b.block(IrBlockKind::Internal);
      let false_block = b.block(IrBlockKind::Internal);
      let fallback = b.fallback_block(0);

      b.begin_block(block);
      let r1 = b.vm_reg(1);
      let unknown = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, unknown, tnumber, fallback);
      let r1 = b.vm_reg(1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, r1, true_block, false_block);

      b.begin_block(true_block);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, one);

      b.begin_block(false_block);
      let two = b.const_uint(2);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, two);

      b.begin_block(fallback);
      let three = b.const_uint(3);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, three);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R1\n   CHECK_TAG %0, tnumber, bb_fallback_3\n   JUMP bb_1\n; glued to: bb_1\n\nbb_1:\n   RETURN 1u\n\nbb_fallback_3:\n   RETURN 3u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_unused_at_return {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5993:ir_builder_unused_at_return`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_unused_at_return

  #[cfg(test)]
  #[test]
  fn ir_builder_unused_at_return() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r1 = b.vm_reg(1);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r1, tnumber);
      let r2 = b.vm_reg(2);
      let four = b.const_int(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r2, four);
      let r2 = b.vm_reg(2);
      let tboolean = b.const_tag(TBOOLEAN);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tboolean);
      let r4 = b.vm_reg(4);
      let tnumber = b.const_tag(TNUMBER);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreSplitTvalue, r4, tnumber, two);

      let r0 = b.vm_reg(0);
      let some_tv = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r5 = b.vm_reg(5);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r5, some_tv);

      let r6 = b.vm_reg(6);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r6, tnil);

      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_unused_at_return_partial {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6024:ir_builder_unused_at_return_partial`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_unused_at_return_partial

  #[cfg(test)]
  #[test]
  fn ir_builder_unused_at_return_partial() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, const_prop_in_block_chains::const_prop_in_block_chains,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r1, one);
      let r2 = b.vm_reg(2);
      let four = b.const_int(4);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r2, four);
      let r3 = b.vm_reg(3);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r3, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   STORE_DOUBLE R1, 1\n   STORE_INT R2, 4i\n   STORE_TAG R3, tnumber\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_userdata_buffer_store_forwarding_invalidation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7725:ir_builder_userdata_buffer_store_forwarding_invalidation`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_userdata_buffer_store_forwarding_invalidation

  #[cfg(test)]
  #[test]
  fn ir_builder_userdata_buffer_store_forwarding_invalidation() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TUSERDATA: u8 = 9;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let sixteen = b.const_int(16);
      let one = b.const_int(1);
      let ud = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NewUserdata, sixteen, one);

      let four = b.const_int(4);
      let forty_two = b.const_int(42);
      let tuserdata = b.const_tag(TUSERDATA);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei32, ud, four, forty_two, tuserdata);

      let four = b.const_int(4);
      let ninety_nine = b.const_int(99);
      let tuserdata = b.const_tag(TUSERDATA);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::BufferWritei32,
        ud,
        four,
        ninety_nine,
        tuserdata,
      );

      let four = b.const_int(4);
      let tuserdata = b.const_tag(TUSERDATA);
      let loaded = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BufferReadi32, ud, four, tuserdata);
      let r0 = b.vm_reg(0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, r0, loaded);
      let r0 = b.vm_reg(0);
      let one = b.const_uint(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = NEW_USERDATA 16i, 1i\n   BUFFER_WRITEI32 %0, 4i, 42i, tuserdata\n   BUFFER_WRITEI32 %0, 4i, 99i, tuserdata\n   STORE_INT R0, 99i\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_variadic_sequence_peeling {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5183:ir_builder_variadic_sequence_peeling`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_variadic_sequence_peeling

  #[cfg(test)]
  #[test]
  fn ir_builder_variadic_sequence_peeling() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let a = b.block(IrBlockKind::Internal);
      let branch_b = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let zero = b.const_uint(0);
      let r3 = b.vm_reg(3);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackGetvarargs, zero, r3, minus_one);
      let r0 = b.vm_reg(0);
      let tag = b.inst_ir_cmd_ir_op(IrCmd::LoadTag, r0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpEqTag, tag, tnumber, a, branch_b);

      b.begin_block(a);
      let r0 = b.vm_reg(0);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r0);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, value);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(branch_b);
      let r1 = b.vm_reg(1);
      let value = b.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, r1);
      let r2 = b.vm_reg(2);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, r2, value);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r2 = b.vm_reg(2);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, minus_one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1, bb_2\n; in regs: R0, R1\n; out regs: R0, R1, R3...\n   FALLBACK_GETVARARGS 0u, R3, -1i\n   %1 = LOAD_TAG R0\n   JUMP_EQ_TAG %1, tnumber, bb_1, bb_2\n\nbb_1:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R0, R3...\n; out regs: R2...\n   %3 = LOAD_TVALUE R0\n   STORE_TVALUE R2, %3\n   JUMP bb_3\n\nbb_2:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R1, R3...\n; out regs: R2...\n   %6 = LOAD_TVALUE R1\n   STORE_TVALUE R2, %6\n   JUMP bb_3\n\nbb_3:\n; predecessors: bb_1, bb_2\n; in regs: R2...\n   RETURN R2, -1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_variadic_sequence_restart {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5101:ir_builder_variadic_sequence_restart`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - translates_to -> rust_item ir_builder_variadic_sequence_restart

  #[cfg(test)]
  #[test]
  fn ir_builder_variadic_sequence_restart() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);
      let exit = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r1 = b.vm_reg(1);
      let zero = b.const_int(0);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, r1, zero, minus_one);
      let r0 = b.vm_reg(0);
      let minus_one_params = b.const_int(-1);
      let minus_one_results = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, r0, minus_one_params, minus_one_results);
      b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

      b.begin_block(exit);
      let r0 = b.vm_reg(0);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, minus_one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1\n; in regs: R0, R1\n; out regs: R0...\n   CALL R1, 0i, -1i\n   CALL R0, -1i, -1i\n   JUMP bb_1\n\nbb_1:\n; predecessors: bb_0\n; in regs: R0...\n   RETURN R0, -1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_varidic_register_range_invalidation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4046:ir_builder_varidic_register_range_invalidation`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
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
  //!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_varidic_register_range_invalidation

  #[cfg(test)]
  #[test]
  fn ir_builder_varidic_register_range_invalidation() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        const_prop_in_block_chains::const_prop_in_block_chains, to_string_ir_dump_alt_g::to_string,
        update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let block = b.block(IrBlockKind::Internal);

      b.begin_block(block);

      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let zero = b.const_uint(0);
      let r1 = b.vm_reg(1);
      let minus_one = b.const_int(-1);
      b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackGetvarargs, zero, r1, minus_one);
      let r2 = b.vm_reg(2);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r2, tnumber);
      let zero = b.const_uint(0);
      b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R2, tnumber\n   FALLBACK_GETVARARGS 0u, R1, -1i\n   STORE_TAG R2, tnumber\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_vector_over_combined_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7048:ir_builder_vector_over_combined_number`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_vector_over_combined_number

  #[cfg(test)]
  #[test]
  fn ir_builder_vector_over_combined_number() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, four);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let eight = b.const_double(8.0);
      let sixteen = b.const_double(16.0);
      let thirty_two = b.const_double(32.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, eight, sixteen, thirty_two);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_VECTOR R0, 8, 16, 32, tvector\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_vector_over_combined_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7023:ir_builder_vector_over_combined_vector`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_vector_over_combined_vector

  #[cfg(test)]
  #[test]
  fn ir_builder_vector_over_combined_vector() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      let two = b.const_double(2.0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, one, two, four);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let eight = b.const_double(8.0);
      let sixteen = b.const_double(16.0);
      let thirty_two = b.const_double(32.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, eight, sixteen, thirty_two);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_VECTOR R0, 8, 16, 32, tvector\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_vector_over_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6976:ir_builder_vector_over_nil`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_vector_over_nil

  #[cfg(test)]
  #[test]
  fn ir_builder_vector_over_nil() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNIL: u8 = 0;
    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let tnil = b.const_tag(TNIL);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnil);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      let two = b.const_double(2.0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, one, two, four);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_VECTOR R0, 1, 2, 4, tvector\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_vector_over_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6885:ir_builder_vector_over_number`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_vector_over_number

  #[cfg(test)]
  #[test]
  fn ir_builder_vector_over_number() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TNUMBER: u8 = 3;
    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let two = b.const_double(2.0);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, r0, two);
      let r0 = b.vm_reg(0);
      let tnumber = b.const_tag(TNUMBER);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tnumber);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      let two = b.const_double(2.0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, one, two, four);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_VECTOR R0, 1, 2, 4, tvector\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}

mod ir_builder_vector_over_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6908:ir_builder_vector_over_vector`
  //! Source: `tests/IrBuilder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrBuilder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
  //!   - includes -> source_file CodeGen/include/Luau/IrDump.h
  //!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
  //!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/IrBuilder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
  //!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
  //!   - translates_to -> rust_item ir_builder_vector_over_vector

  #[cfg(test)]
  #[test]
  fn ir_builder_vector_over_vector() {
    use ulua_code_gen::{
      enums::{include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
      functions::{
        compute_cfg_info::compute_cfg_info,
        mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
        to_string_ir_dump_alt_g::to_string, update_use_counts::update_use_counts,
      },
    };
    use ulua_unit_test::records::ir_builder_fixture::IrBuilderFixture;

    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
      let b = &mut fix.build;
      let entry = b.block(IrBlockKind::Internal);

      b.begin_block(entry);
      let r0 = b.vm_reg(0);
      let four = b.const_double(4.0);
      let two = b.const_double(2.0);
      let one = b.const_double(1.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, four, two, one);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let one = b.const_double(1.0);
      let two = b.const_double(2.0);
      let four = b.const_double(4.0);
      b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, r0, one, two, four);
      let r0 = b.vm_reg(0);
      let tvector = b.const_tag(TVECTOR);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, r0, tvector);
      let r0 = b.vm_reg(0);
      let one = b.const_int(1);
      b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_VECTOR R0, 1, 2, 4, tvector\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
  }
}
