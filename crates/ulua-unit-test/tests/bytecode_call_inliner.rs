extern crate alloc;

mod bytecode_call_inliner_early_return_inlining {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:347:bytecode_call_inliner_early_return_inlining`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_early_return_inlining

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_early_return_inlining() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a, b)
            if b < 0 then return a - b end
            return a + b
        end

        local function caller(x)
            local result = inlinee(x, 42)
            return result + 2
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
LOADK R3 K0 [42]
CMPPROTO R1 #0 L1
LOADK R4 K2 [0]
JUMPIFNOTLT R3 R4 L0
SUB R4 R2 R3
MOVE R1 R4
JUMP L2
L0: ADD R4 R2 R3
MOVE R1 R4
JUMP L2
L1: CALLFB R1 2 1 [0]
L2: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_loop_phis {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:761:bytecode_call_inliner_loop_phis`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_loop_phis

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_loop_phis() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(n)
            local sum = 0
            for i = 1, n do
                for j = 1, i do
                    sum = sum + j
                end
            end
            return sum
        end

        local function caller(x)
            local r = inlinee(x)
            return r
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
CMPPROTO R1 #0 L4
LOADK R3 K0 [0]
LOADK R6 K1 [1]
MOVE R4 R2
LOADN R5 1
FORNPREP R4 L3
L0: LOADK R9 K1 [1]
MOVE R7 R6
LOADN R8 1
FORNPREP R7 L2
L1: ADD R3 R3 R9
FORNLOOP R7 L1
L2: FORNLOOP R4 L0
L3: MOVE R1 R3
RETURN R1 1
L4: CALLFB R1 1 1 [0]
RETURN R1 1
"#
    );
  }
}

mod bytecode_call_inliner_mixed_vararg_func_inlining {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:542:bytecode_call_inliner_mixed_vararg_func_inlining`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_mixed_vararg_func_inlining

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_mixed_vararg_func_inlining() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a, ...)
            local b = ...
            return a + b
        end
        local function caller(x)
            local result = inlinee(x, 100)
            return result + 2
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
LOADK R3 K0 [100]
CMPPROTO R1 #0 L0
MOVE R5 R3
ADD R6 R2 R5
MOVE R1 R6
JUMP L1
L0: CALLFB R1 2 1 [0]
L1: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:591:bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a, b, ...)
            local c, d = ...
            return a + b + c + d
        end
        local function caller(x)
            local result = inlinee(x, 100)
            return result + 2
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
LOADK R3 K0 [100]
CMPPROTO R1 #0 L0
LOADNIL R6
LOADNIL R7
ADD R10 R2 R3
ADD R9 R10 R6
ADD R8 R9 R7
MOVE R1 R8
JUMP L1
L0: CALLFB R1 2 1 [0]
L1: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_multi_return_inlining {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:403:bytecode_call_inliner_multi_return_inlining`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_multi_return_inlining

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_multi_return_inlining() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a, b)
            if b < 0 then return a - b end
            return a + b, 12
        end

        local function caller(x)
            local result = inlinee(x, 42)
            return result + 2
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
LOADK R3 K0 [42]
CMPPROTO R1 #0 L1
LOADK R4 K2 [0]
JUMPIFNOTLT R3 R4 L0
SUB R4 R2 R3
MOVE R1 R4
JUMP L2
L0: ADD R4 R2 R3
LOADK R5 K3 [12]
MOVE R1 R4
MOVE R2 R5
JUMP L2
L1: CALLFB R1 2 1 [0]
L2: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_namecall_inlining {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:284:bytecode_call_inliner_namecall_inlining`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_namecall_inlining

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_namecall_inlining() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(t, x)
            return t.v + x
        end

        local function caller(x)
            local t = {v = 7, inlinee = inlinee}
            local result = t:inlinee(42)
            return result + 2
        end
    "#,
          0,
        )
      ),
      r#"
DUPTABLE R1 2
LOADK R2 K0 ['v']
LOADK R3 K3 [7]
SETTABLE R3 R1 R2
LOADK R2 K1 ['inlinee']
GETUPVAL R3 0
SETTABLE R3 R1 R2
LOADK R4 K4 [42]
MOVE R3 R1
GETTABLEKS R2 R3 K1 ['inlinee']
CMPPROTO R2 #0 L0
GETTABLEKS R6 R3 K0 ['v']
ADD R5 R6 R4
MOVE R2 R5
JUMP L1
L0: NAMECALL R2 R1 K1 ['inlinee']
CALLFB R2 2 1 [0]
L1: LOADK R4 K5 [2]
ADD R3 R2 R4
RETURN R3 1
"#
    );
  }
}

mod bytecode_call_inliner_retain_target_on_block_split {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:829:bytecode_call_inliner_retain_target_on_block_split`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_retain_target_on_block_split

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_retain_target_on_block_split() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a)
            return a + 1
        end

        local function caller(n)
            local sum = 0
            for i = 1, n do
                sum = sum + inlinee(i)
            end
            return sum
        end
    "#,
          0,
        )
      ),
      r#"
LOADK R1 K0 [0]
LOADK R4 K1 [1]
MOVE R2 R0
LOADN R3 1
FORNPREP R2 L3
L0: GETUPVAL R5 0
MOVE R6 R4
CMPPROTO R5 #0 L1
LOADK R8 K1 [1]
ADD R7 R6 R8
MOVE R5 R7
JUMP L2
L1: CALLFB R5 1 1 [0]
L2: ADD R1 R1 R5
FORNLOOP R2 L0
L3: RETURN R1 1
"#
    );
  }
}

mod bytecode_call_inliner_simple_inlining {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:150:bytecode_call_inliner_simple_inlining`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_simple_inlining

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_simple_inlining() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a, b)
            return a + b
        end

        local function caller(x)
            local result = inlinee(x, 42)
            return result + 2
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
LOADK R3 K0 [42]
CMPPROTO R1 #0 L0
ADD R4 R2 R3
MOVE R1 R4
JUMP L1
L0: CALLFB R1 2 1 [0]
L1: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_simple_inlining_under_return {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:245:bytecode_call_inliner_simple_inlining_under_return`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record BytecodeBuilder (Bytecode/include/Luau/BytecodeBuilder.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_simple_inlining_under_return

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_simple_inlining_under_return() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a)
            return a
        end

        local function caller()
            local r1, r2 = inlinee(10)
            return r2
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
LOADK R1 K0 [10]
CMPPROTO R0 #0 L0
MOVE R0 R1
LOADNIL R1
RETURN R1 1
L0: CALLFB R0 1 2 [0]
RETURN R1 1
"#
    );
  }
}

mod bytecode_call_inliner_simple_inlining_undercall {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:195:bytecode_call_inliner_simple_inlining_undercall`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_simple_inlining_undercall

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_simple_inlining_undercall() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a, b)
            return a + (b or 42)
        end

        local function caller(x)
            local result = inlinee(x)
            return result + 2
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
CMPPROTO R1 #0 L1
LOADNIL R3
MOVE R5 R3
JUMPIF R5 L0
LOADK R5 K1 [42]
L0: ADD R4 R2 R5
MOVE R1 R4
JUMP L2
L1: CALLFB R1 1 1 [0]
L2: LOADK R3 K0 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_var_return_inlining {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:462:bytecode_call_inliner_var_return_inlining`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::compileAndInline (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_var_return_inlining

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_var_return_inlining() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert!(
      fixture
        .compile_and_inline(
          r#"
        local function inlinee(a, b)
            return g(a, b)
        end

        local function caller(x)
            local a, b = inlinee(x, 42)
            return a + b
        end
    "#,
          0,
        )
        .is_none()
    );
  }
}

mod bytecode_call_inliner_vararg_func_inlining {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:479:bytecode_call_inliner_vararg_func_inlining`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_vararg_func_inlining

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_vararg_func_inlining() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(...)
            local x = 12
            local a, b = ...
            if b < 0 then return a - b end
            return a + b
        end

        local function caller(x)
            local result = inlinee(x, 42)
            return result + 2
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
LOADK R3 K0 [42]
CMPPROTO R1 #0 L1
LOADK R4 K2 [12]
MOVE R5 R2
MOVE R6 R3
LOADK R7 K3 [0]
JUMPIFNOTLT R6 R7 L0
SUB R7 R5 R6
MOVE R1 R7
JUMP L2
L0: ADD R7 R5 R6
MOVE R1 R7
JUMP L2
L1: CALLFB R1 2 1 [0]
L2: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_vararg_func_vararg_multi_usage {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:644:bytecode_call_inliner_vararg_func_vararg_multi_usage`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_vararg_func_vararg_multi_usage

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_vararg_func_vararg_multi_usage() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(...)
            local t = {1, 2, ...}
            return t[3]
        end

        local function caller()
            local result = inlinee(10, 20, 30)
            return result
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
LOADK R1 K0 [10]
LOADK R2 K1 [20]
LOADK R3 K2 [30]
CMPPROTO R0 #0 L0
NEWTABLE R4 0 2
LOADK R5 K3 [1]
LOADK R6 K4 [2]
MOVE R7 R1
MOVE R8 R2
MOVE R9 R3
SETLIST R4 R5 6 [1]
LOADK R6 K5 [3]
GETTABLE R5 R4 R6
MOVE R0 R5
RETURN R0 1
L0: CALLFB R0 3 1 [0]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_vararg_func_vararg_multi_usage_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:703:bytecode_call_inliner_vararg_func_vararg_multi_usage_2`
  //! Source: `tests/BytecodeCallInliner.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
  //!   - translates_to -> rust_item bytecode_call_inliner_vararg_func_vararg_multi_usage_2

  #[cfg(test)]
  #[test]
  fn bytecode_call_inliner_vararg_func_vararg_multi_usage_2() {
    use ulua_common::FFlag::LuauEmitCallFeedback;
    use ulua_unit_test::{
      records::bytecode_inliner_fixture::BytecodeInlinerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
      alloc::format!(
        "\n{}",
        fixture.inline_and_print(
          r#"
        local function inlinee(a, ...)
            local t = {1, a, ...}
            return t[3]
        end

        local function caller()
            local result = inlinee(10, 20, 30)
            return result
        end
    "#,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
LOADK R1 K0 [10]
LOADK R2 K1 [20]
LOADK R3 K2 [30]
CMPPROTO R0 #0 L0
NEWTABLE R5 0 2
LOADK R6 K3 [1]
MOVE R7 R1
MOVE R8 R2
MOVE R9 R3
SETLIST R5 R6 5 [1]
LOADK R7 K4 [3]
GETTABLE R6 R5 R7
MOVE R0 R6
RETURN R0 1
L0: CALLFB R0 3 1 [0]
RETURN R0 1
"#
    );
  }
}
