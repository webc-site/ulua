extern crate alloc;

mod bytecode_call_inliner_does_not_fold_runtime_arguments {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  //! 调用方传的是运行期实参时没有常量可折，内联出来的 ADD 必须原样保留。
  #[test]
  fn bytecode_call_inliner_does_not_fold_runtime_arguments() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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

        local function caller(x, y)
            local res = inlinee(x, y)
            return res
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R2 0
MOVE R3 R0
MOVE R4 R1
CMPPROTO R2 #0 L0
ADD R5 R3 R4
MOVE R2 R5
RETURN R2 1
L0: CALLFB R2 2 1 [-1]
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_empty_inlinee_with_vararg {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_empty_inlinee_with_vararg() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
        end

        local function caller()
            return {inlinee, (inlinee())}
        end
    "#,
          0,
          false,
          0,
        )
      ),
      r#"
NEWTABLE R0 0 2
GETUPVAL R1 0
GETUPVAL R2 0
CMPPROTO R2 #0 L0
LOADNIL R3
LOADNIL R2
JUMP L1
L0: CALLFB R2 0 1 [-1]
L1: SETLIST R0 R1 2 [1]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_empty_varargs_sequence_in_for_loop {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_empty_varargs_sequence_in_for_loop() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            for _ in ... do
                pcall += _
            end
        end

        local function caller()
            inlinee()
        end
    "#,
          0,
          false,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
CMPPROTO R0 #0 L2
LOADNIL R1
LOADNIL R3
LOADNIL R4
LOADNIL R5
FORGPREP R3 L1
L0: GETGLOBAL R8 K0 ['pcall']
ADD R8 R8 R6
SETGLOBAL R8 K0 ['pcall']
L1: FORGLOOP R3 L0 1
RETURN R0 0
L2: CALLFB R0 0 0 [-1]
RETURN R0 0
"#
    );
  }
}

mod bytecode_call_inliner_empty_varargs_sequence_in_setlist {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_empty_varargs_sequence_in_setlist() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            return {...}
        end

        local function caller()
            inlinee()
        end
    "#,
          0,
          false,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
CMPPROTO R0 #0 L0
NEWTABLE R1 0 0
MOVE R0 R1
RETURN R0 0
L0: CALLFB R0 0 0 [-1]
RETURN R0 0
"#
    );
  }
}

mod bytecode_call_inliner_early_return_inlining {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_early_return_inlining() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L1: CALLFB R1 2 1 [-1]
L2: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  //! 内联出的 ADD (5+42) 折成 LOADK 47，搬运它的 MOVE 也一并折叠；
  //! 第二个 ADD (result+2) 不能折，因为 R1 同时来自内联路径（常量 47）与 CALLFB 回退路径。
  #[test]
  fn bytecode_call_inliner_fold_constants() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            local result = inlinee(5, 42)
            return result + 2
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
LOADK R2 K0 [5]
LOADK R3 K1 [42]
CMPPROTO R1 #0 L0
LOADK R4 K3 [47]
LOADK R1 K3 [47]
JUMP L1
L0: CALLFB R1 2 1 [-1]
L1: ADDK R2 R1 K2 [2]
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_chained {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  //! 串联的多个算术指令应当全部折叠。
  #[test]
  fn bytecode_call_inliner_fold_constants_chained() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            return (a + b) * 2
        end

        local function caller(x)
            local result = inlinee(10, 11)
            return result + x
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
LOADK R2 K0 [10]
LOADK R3 K1 [11]
CMPPROTO R1 #0 L0
LOADK R5 K3 [21]
LOADK R6 K2 [2]
LOADK R4 K4 [42]
LOADK R1 K4 [42]
JUMP L1
L0: CALLFB R1 2 1 [-1]
L1: ADD R2 R1 R0
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_div_by_zero {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  //! 除零应折成 inf。
  #[test]
  fn bytecode_call_inliner_fold_constants_div_by_zero() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            return a / b
        end

        local function caller(x)
            local result = inlinee(10, 0)
            return result + x
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
LOADK R2 K0 [10]
LOADK R3 K1 [0]
CMPPROTO R1 #0 L0
LOADK R4 K2 [inf]
LOADK R1 K2 [inf]
JUMP L1
L0: CALLFB R1 2 1 [-1]
L1: ADD R2 R1 R0
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_nil_argument {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  //! 缺失的形参被当作 LOADNIL，SCCP 可据此折掉 `return 1` 分支。
  #[test]
  fn bytecode_call_inliner_fold_constants_nil_argument() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            if a == nil then
                return 0
            end
            return 1
        end

        local function caller()
            local res = inlinee()
            return res
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
CMPPROTO R0 #0 L0
LOADNIL R1
LOADNIL R2
LOADK R2 K0 [0]
LOADK R0 K0 [0]
RETURN R0 1
L0: CALLFB R0 0 1 [-1]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_prunes_equality_branch {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  //! JUMPIFEQ 相等比较折叠。
  #[test]
  fn bytecode_call_inliner_fold_constants_prunes_equality_branch() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            if a == b then
                return 1
            end
            return 2
        end

        local function caller()
            local res = inlinee(7, 7)
            return res
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
LOADK R1 K0 [7]
LOADK R2 K0 [7]
CMPPROTO R0 #0 L0
LOADK R3 K1 [1]
LOADK R0 K1 [1]
RETURN R0 1
L0: CALLFB R0 2 1 [-1]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_prunes_ordering_branch {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  //! 覆盖 evaluateComparisonCondition 的 JUMPIF(NOT)LT 路径。
  #[test]
  fn bytecode_call_inliner_fold_constants_prunes_ordering_branch() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            if a < b then
                return a
            end
            return b
        end

        local function caller()
            local res = inlinee(3, 10)
            return res
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
LOADK R1 K0 [3]
LOADK R2 K1 [10]
CMPPROTO R0 #0 L0
LOADK R0 K0 [3]
RETURN R0 1
L0: CALLFB R0 2 1 [-1]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_string_equality {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  //! 常量字符串实参与字符串字面量比较，覆盖 evaluateXeqkCondition。
  #[test]
  fn bytecode_call_inliner_fold_constants_string_equality() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
        local function inlinee(s)
            if s == "yes" then
                return 1
            end
            return 0
        end

        local function caller()
            local res = inlinee("yes")
            return res
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
LOADK R1 K0 ['yes']
CMPPROTO R0 #0 L0
LOADK R2 K0 ['yes']
LOADK R2 K1 [1]
LOADK R0 K1 [1]
RETURN R0 1
L0: CALLFB R0 1 1 [-1]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_with_branch {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_fold_constants_with_branch() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
                if a then
                    return 1
                else
                    return 2
                end
            end

            local function caller()
                local t = true
                local res = inlinee(t)
                return res
            end
            "#,
          0,
          true,
          0,
        )
      ),
      r#"
LOADB R0 1
GETUPVAL R1 0
LOADB R2 1
CMPPROTO R1 #0 L0
LOADK R3 K0 [1]
LOADK R1 K0 [1]
RETURN R1 1
L0: CALLFB R1 1 1 [-1]
RETURN R1 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_with_branches {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_fold_constants_with_branches() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
                if a then
                    if a > 1 then
                        return 3
                    else
                        return 1
                    end
                else
                    return 2
                end
            end

            local function caller()
                local res = inlinee(5)
                return res
            end
            "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
LOADK R1 K0 [5]
CMPPROTO R0 #0 L0
LOADK R2 K1 [1]
LOADK R2 K2 [3]
LOADK R0 K2 [3]
RETURN R0 1
L0: CALLFB R0 1 1 [-1]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_constants_with_for_loop {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_fold_constants_with_for_loop() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
                local sum = 0
                for i = 1, a do
                    sum = sum + i
                end
                return sum
            end

            local function caller()
                local t = 10
                local res = inlinee(t)
                return res
            end
            "#,
          0,
          true,
          0,
        )
      ),
      r#"
LOADK R0 K0 [10]
GETUPVAL R1 0
LOADK R2 K0 [10]
CMPPROTO R1 #0 L2
LOADK R3 K1 [0]
LOADK R6 K2 [1]
LOADK R4 K0 [10]
LOADN R5 1
FORNPREP R4 L1
L0: ADD R3 R3 R6
FORNLOOP R4 L0
L1: MOVE R1 R3
RETURN R1 1
L2: CALLFB R1 1 1 [-1]
RETURN R1 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_div_constant_lhs_one_is_reciprocal_not_move {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_fold_div_constant_lhs_one_is_reciprocal_not_move() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
        local function inlinee(a, x)
            return a / x
        end
        local function caller(x)
            local r = inlinee(1, x)
            return r + x
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
LOADK R2 K0 [1]
MOVE R3 R0
CMPPROTO R1 #0 L0
DIV R4 R2 R3
MOVE R1 R4
JUMP L1
L0: CALLFB R1 2 1 [-1]
L1: ADD R2 R1 R0
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_jumpxeqkb_bool_immediate_value {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_fold_jumpxeqkb_bool_immediate_value() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
        local function inlinee(flag)
            if flag == true then return 1 else return 2 end
        end
        local function caller(x)
            local r = inlinee(true)
            return r + x
        end
    "#,
          0,
          true,
          1,
        )
      ),
      r#"
GETUPVAL R1 0
LOADB R2 1
CMPPROTO R1 #0 L0
LOADN R3 1
LOADN R1 1
JUMP L1
L0: CALLFB R1 1 1 [-1]
L1: ADD R2 R1 R0
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_fold_sub_constant_lhs_is_negation_not_move {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_fold_sub_constant_lhs_is_negation_not_move() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
        local function inlinee(a, x)
            return a - x
        end
        local function caller(x)
            local r = inlinee(0, x)
            return r + x
        end
    "#,
          0,
          true,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
LOADK R2 K0 [0]
MOVE R3 R0
CMPPROTO R1 #0 L0
SUB R4 R2 R3
MOVE R1 R4
JUMP L1
L0: CALLFB R1 2 1 [-1]
L1: ADD R2 R1 R0
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_loop_phis {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_loop_phis() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L4: CALLFB R1 1 1 [-1]
RETURN R1 1
"#
    );
  }
}

mod bytecode_call_inliner_mixed_vararg_func_inlining {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_mixed_vararg_func_inlining() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L0: CALLFB R1 2 1 [-1]
L1: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L0: CALLFB R1 2 1 [-1]
L1: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_multi_return_inlining {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_multi_return_inlining() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L1: CALLFB R1 2 1 [-1]
L2: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_namecall_inlining {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_namecall_inlining() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
CALLFB R2 2 1 [-1]
L1: LOADK R4 K5 [2]
ADD R3 R2 R4
RETURN R3 1
"#
    );
  }
}

mod bytecode_call_inliner_retain_target_on_block_split {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_retain_target_on_block_split() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L1: CALLFB R5 1 1 [-1]
L2: ADD R1 R1 R5
FORNLOOP R2 L0
L3: RETURN R1 1
"#
    );
  }
}

mod bytecode_call_inliner_simple_inlining {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_simple_inlining() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L0: CALLFB R1 2 1 [-1]
L1: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_simple_inlining_under_return {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_simple_inlining_under_return() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L0: CALLFB R0 1 2 [-1]
RETURN R1 1
"#
    );
  }
}

mod bytecode_call_inliner_simple_inlining_undercall {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_simple_inlining_undercall() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L1: CALLFB R1 1 1 [-1]
L2: LOADK R3 K0 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_var_return_inlining {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_var_return_inlining() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          0,
        )
        .is_none()
    );
  }
}

mod bytecode_call_inliner_vararg_func_inlining {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_vararg_func_inlining() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
L1: CALLFB R1 2 1 [-1]
L2: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
  }
}

mod bytecode_call_inliner_vararg_func_vararg_multi_usage {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_vararg_func_vararg_multi_usage() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
SETLIST R4 R5 5 [1]
LOADK R6 K5 [3]
GETTABLE R5 R4 R6
MOVE R0 R5
RETURN R0 1
L0: CALLFB R0 3 1 [-1]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_vararg_func_vararg_multi_usage_2 {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_vararg_func_vararg_multi_usage_2() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
          false,
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
SETLIST R5 R6 4 [1]
LOADK R7 K4 [3]
GETTABLE R6 R5 R7
MOVE R0 R6
RETURN R0 1
L0: CALLFB R0 3 1 [-1]
RETURN R0 1
"#
    );
  }
}

mod bytecode_call_inliner_vararg_in_loops_phi {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_vararg_in_loops_phi() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
                repeat
                    local a = ...
                    while a do break end
                until false
            end

            local function caller()
                inlinee()
            end
    "#,
          0,
          false,
          0,
        )
      ),
      r#"
GETUPVAL R0 0
CMPPROTO R0 #0 L3
L0: LOADNIL R1
L1: JUMPIFNOT R1 L2
JUMP L2
JUMPBACK L1
L2: LOADB R2 0
JUMPIF R2 L4
JUMPBACK L0
RETURN R0 0
L3: CALLFB R0 0 0 [-1]
L4: RETURN R0 0
"#
    );
  }
}

mod bytecode_call_inliner_vararg_projection_in_return_phi {
  //! Source: `tests/BytecodeCallInliner.test.cpp`

  #[test]
  fn bytecode_call_inliner_vararg_projection_in_return_phi() {
    use ulua_common::fflag::LuauEmitCallFeedback;
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
            return a and ...
        end

        local function caller(x)
            local r = inlinee(x)
            return r
        end
    "#,
          0,
          false,
          0,
        )
      ),
      r#"
GETUPVAL R1 0
MOVE R2 R0
CMPPROTO R1 #0 L1
MOVE R4 R2
JUMPIFNOT R4 L0
LOADNIL R4
L0: MOVE R1 R4
RETURN R1 1
L1: CALLFB R1 1 1 [-1]
RETURN R1 1
"#
    );
  }
}
