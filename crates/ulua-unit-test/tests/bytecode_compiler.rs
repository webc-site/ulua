extern crate alloc;

mod bytecode_compiler_bytecode_roundtrip {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:768:bytecode_compiler_bytecode_roundtrip`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method BytecodeCompilerFixture::checkRoundtrip (tests/BytecodeCompiler.test.cpp)
  //!   - translates_to -> rust_item bytecode_compiler_bytecode_roundtrip

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_bytecode_roundtrip() {
    use ulua_unit_test::records::bytecode_compiler_fixture::BytecodeCompilerFixture;

    let snippets = [
      r#"
        function fn(a, b)
            local extra = 0
            if a > b then extra = 1 end
            return extra + a + b
        end
    "#,
      r#"
        function fn()
            local var = 0
            repeat var += 1 until var < 10
        end
    "#,
      r#"
        function fn()
            local var = 3
            for i = 1, 10 do
                if var > 0 then print(i) end
                var -= 1;
            end
        end
    "#,
      r#"
        function fn()
            local res = 0
            local var = 0
            repeat
                local i = 0
                repeat
                    res += i * var
                    i += 1
                until i < 5
                var += 1
            until var < 10
        end
    "#,
      r#"
        local function x()
            local a, b = f()
            return b, a
        end
    "#,
      r#"
        local function fn(n)
            if n > 0 then
                return 0, 1
            else
                local a, b = fn(n - 1)
                return a + b, fn(n)
            end
        end
    "#,
      r#"
        local function fn(a, ...)
            local b, c = ...
            local l = {...}
            return a + b + c + l[1], ...
        end
    "#,
      r#"
        local function fn(x)
            local f = function (a, b) return a .. " and " .. b .. " and agian " .. b end
            return f(x, "eleven")
        end
    "#,
      r#"
        local tt = {}
        local function fn(x)
            local t = { a = x, b = x .. 42 }
            return table.insert({t}, tt)
        end
    "#,
    ];

    let mut fixture = BytecodeCompilerFixture::new();
    for snippet in snippets {
      fixture.check_roundtrip(snippet);
    }
  }
}

mod bytecode_compiler_classes_bytecode_roundtrips {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:849:bytecode_compiler_classes_bytecode_roundtrips`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeCompilerFixture::checkRoundtrip (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item bytecode_compiler_classes_bytecode_roundtrips

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_classes_bytecode_roundtrips() {
    use ulua_common::FFlag::DebugLuauUserDefinedClasses;
    use ulua_unit_test::{
      records::bytecode_compiler_fixture::BytecodeCompilerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);
    let mut fixture = BytecodeCompilerFixture::new();
    fixture.check_roundtrip(
      r#"
        class Point
            public x
            public y

            function magnitude(self)
                return math.sqrt(self.x * self.x + self.y * self.y)
            end

            function __mul(self, other)
                return Point { x = self.x * other.x, y = self.y * other.y }
            end

            function __add(self, other)
                return Point { x = self.x + other.x, y = self.y + other.y }
            end

            function __eq(self, other)
                return self.x == other.x and self.y == other.y
            end

            function zero()
                return Point { x = 0, y = 0 }
            end

            function asserttriple(self)
                local mag = self:magnitude()
                assert(mag == math.ceil(mag), "Not a pythagorean triple!")
            end

            function __tostring(self)
                return `Point(x={self.x}, y={self.y})`
            end

        end

        print(Point)

        return { Point = Point }
    "#,
    );
  }
}

mod bytecode_compiler_for_loop_and_backward_input {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:343:bytecode_compiler_for_loop_and_backward_input`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function fallthroughOp (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> record Loop (Compiler/src/Compiler.cpp)
  //!   - calls -> function fallthroughBlock (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function branchOp (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function loopOp (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function isPhiOf (tests/BytecodeCompiler.test.cpp)
  //!   - translates_to -> rust_item bytecode_compiler_for_loop_and_backward_input

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_for_loop_and_backward_input() {
    use ulua_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
    use ulua_common::{FFlag::LuauEmitCallFeedback, enums::luau_opcode::LuauOpcode};
    use ulua_unit_test::{
      functions::{
        branch_op::branch_op, check_edges::check_edges, check_ops::check_ops,
        fallthrough_op::fallthrough_op, get_op::get_op, is_phi_of::is_phi_of, loop_op::loop_op,
      },
      records::bytecode_compiler_fixture::BytecodeCompilerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
      .build_bytecode(
        r#"
        function fn()
            local var = 3
            for i = 1, 10 do
                if var > 0 then print(i) end
                var -= 1;
            end
        end
    "#,
        0,
      )
      .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 6);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert_eq!(entry.successors.len(), 2);
    assert!(check_edges(
      &entry.successors,
      &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let loop_enter_op = fallthrough_op(&entry.successors);
    let loop_enter = fn_.block_op(loop_enter_op).clone();
    assert!(check_edges(
      &loop_enter.predecessors,
      &[BcBlockEdgeKind::Fallthrough, BcBlockEdgeKind::Loop]
    ));
    assert!(check_edges(
      &loop_enter.successors,
      &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let loop_cond = fn_.block_op(fallthrough_op(&loop_enter.successors)).clone();
    assert!(check_edges(
      &loop_cond.successors,
      &[BcBlockEdgeKind::Fallthrough]
    ));
    let loop_epllog_op = branch_op(&loop_enter.successors);
    assert_eq!(fallthrough_op(&loop_cond.successors), loop_epllog_op);
    let loop_epllog = fn_.block_op(loop_epllog_op).clone();
    assert!(check_edges(
      &loop_epllog.successors,
      &[BcBlockEdgeKind::Loop, BcBlockEdgeKind::Fallthrough]
    ));
    assert_eq!(loop_op(&loop_epllog.successors), loop_enter_op);
    let ret = fn_.block_op(loop_epllog.successors[1].target).clone();

    assert!(check_ops(
      &mut fn_,
      &entry.ops,
      &[
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_LOADN,
        LuauOpcode::LOP_FORNPREP,
      ]
    ));
    assert!(check_ops(
      &mut fn_,
      &loop_enter.ops,
      &[LuauOpcode::LOP_LOADK, LuauOpcode::LOP_JUMPIFNOTLT]
    ));

    let var_init_op = get_op(&entry, 0);
    let sub_var_op = get_op(&loop_epllog, 1);
    let jump_if_not_lt = fn_.inst_op(get_op(&loop_enter, 1)).clone();
    assert_eq!(jump_if_not_lt.ops.len(), 3);
    assert_eq!(jump_if_not_lt.ops[0], get_op(&loop_enter, 0));
    assert!(is_phi_of(
      &mut fn_,
      jump_if_not_lt.ops[1],
      var_init_op,
      sub_var_op
    ));
    assert_eq!(jump_if_not_lt.ops[2], loop_epllog_op);

    let sub_var = fn_.inst_op(sub_var_op).clone();
    assert_eq!(sub_var.ops.len(), 2);
    assert!(is_phi_of(&mut fn_, sub_var.ops[0], var_init_op, sub_var_op));
    assert_eq!(sub_var.ops[1], get_op(&loop_epllog, 0));

    assert!(check_ops(
      &mut fn_,
      &loop_cond.ops,
      &[
        LuauOpcode::LOP_GETGLOBAL,
        LuauOpcode::LOP_MOVE,
        LuauOpcode::LOP_CALLFB,
      ]
    ));
    assert!(check_ops(
      &mut fn_,
      &loop_epllog.ops,
      &[
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_SUB,
        LuauOpcode::LOP_FORNLOOP,
      ]
    ));
    assert!(check_ops(&mut fn_, &ret.ops, &[LuauOpcode::LOP_RETURN]));
  }
}

mod bytecode_compiler_from_function_bytecode {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:220:bytecode_compiler_from_function_bytecode`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SubtypeFixture::meta (tests/Subtyping.test.cpp)
  //!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function branchOp (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function fallthroughOp (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> enum BcVmConstKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
  //!   - translates_to -> rust_item bytecode_compiler_from_function_bytecode

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_from_function_bytecode() {
    use ulua_bytecode::enums::{
      bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind, bc_vm_const_kind::BcVmConstKind,
    };
    use ulua_common::enums::luau_opcode::LuauOpcode;
    use ulua_unit_test::{
      functions::{
        branch_op::branch_op, check_edges::check_edges, check_ops::check_ops,
        fallthrough_op::fallthrough_op,
      },
      records::bytecode_compiler_fixture::BytecodeCompilerFixture,
    };

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
      .build_bytecode(
        r#"
        function fn(a, b)
            local extra = 0
            if a > b then extra = 1 end
            return extra + a + b
        end
    "#,
        0,
      )
      .expect("expected bytecode");

    assert_eq!(fn_.nups, 0);
    assert_eq!(fn_.numparams, 2);
    assert_eq!(fn_.constants.len(), 2);

    assert_eq!(fn_.blocks.len(), 4);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_edges(
      &entry.successors,
      &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let cond_false_op = branch_op(&entry.successors);
    let cond_true = fn_.block_op(entry.successors[1].target).clone();
    assert!(check_edges(
      &cond_true.successors,
      &[BcBlockEdgeKind::Fallthrough]
    ));
    assert_eq!(fallthrough_op(&cond_true.successors), cond_false_op);

    let cond_false = fn_.block_op(cond_false_op).clone();
    assert!(check_edges(
      &cond_false.successors,
      &[BcBlockEdgeKind::Fallthrough]
    ));
    assert_eq!(fallthrough_op(&cond_false.successors), fn_.exit_block);
    let exit_op = fn_.exit_block;
    let exit = fn_.block_op(exit_op).clone();

    assert_eq!(entry.ops.len(), 2);
    let mut ops = entry.ops.iter();
    let load_k_op = *ops.next().expect("entry loadk");
    let load_k = fn_.inst_op(load_k_op).clone();
    assert_eq!(load_k.op, LuauOpcode::LOP_LOADK);
    assert_eq!(load_k.ops.len(), 1);
    assert_eq!(load_k.ops[0].kind, BcOpKind::VmConst);
    assert_eq!(load_k.ops[0].index, 0);
    assert_eq!(fn_.constants[0].kind, BcVmConstKind::Number);
    assert_eq!(unsafe { fn_.constants[0].value.value_number }, 0.0);

    let jump_if_not_lt = fn_.inst_op(*ops.next().expect("entry jump")).clone();
    assert_eq!(jump_if_not_lt.op, LuauOpcode::LOP_JUMPIFNOTLT);
    assert_eq!(jump_if_not_lt.ops.len(), 3);

    assert!(check_ops(
      &mut fn_,
      &cond_true.ops,
      &[LuauOpcode::LOP_LOADK]
    ));
    assert!(check_ops(
      &mut fn_,
      &cond_false.ops,
      &[
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_RETURN,
      ]
    ));
    assert!(check_ops(&mut fn_, &exit.ops, &[]));
  }
}

mod bytecode_compiler_multi_call_fixed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:514:bytecode_compiler_multi_call_fixed`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record BcProj (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> method BcFunction::projOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record BcImm (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> method BcFunction::immOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> enum BcImmKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - translates_to -> rust_item bytecode_compiler_multi_call_fixed

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_multi_call_fixed() {
    use ulua_bytecode::enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind};
    use ulua_common::{FFlag::LuauEmitCallFeedback, enums::luau_opcode::LuauOpcode};
    use ulua_unit_test::{
      functions::{check_ops::check_ops, get_op::get_op},
      records::bytecode_compiler_fixture::BytecodeCompilerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
      .build_bytecode(
        r#"
        local function x()
            local a, b = f()
            return b, a
        end
    "#,
        0,
      )
      .expect("expected bytecode");

    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_ops(
      &mut fn_,
      &entry.ops,
      &[
        LuauOpcode::LOP_GETGLOBAL,
        LuauOpcode::LOP_CALLFB,
        LuauOpcode::LOP_MOVE,
        LuauOpcode::LOP_MOVE,
        LuauOpcode::LOP_RETURN,
      ]
    ));

    let call_op = get_op(&entry, 1);
    let move1_op = get_op(&entry, 2);
    let move1 = fn_.inst_op(move1_op).clone();
    assert_eq!(move1.ops.len(), 1);
    assert_eq!(move1.ops[0].kind, BcOpKind::Proj);
    let move1_proj = *fn_.proj_op(move1.ops[0]);
    assert_eq!(move1_proj.op, call_op);
    assert_eq!(move1_proj.index, 1);

    let move2_op = get_op(&entry, 3);
    let move2 = fn_.inst_op(move2_op).clone();
    assert_eq!(move2.ops.len(), 1);
    assert_eq!(move2.ops[0].kind, BcOpKind::Proj);
    let move2_proj = *fn_.proj_op(move2.ops[0]);
    assert_eq!(move2_proj.op, call_op);
    assert_eq!(move2_proj.index, 0);

    let ret = fn_.inst_op(get_op(&entry, 4)).clone();
    assert_eq!(ret.ops.len(), 3);
    assert_eq!(ret.ops[0].kind, BcOpKind::Imm);
    let ret_count = *fn_.imm_op(ret.ops[0]);
    assert_eq!(ret_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { ret_count.value.value_int }, 2);
    assert_eq!(ret.ops[1], move1_op);
    assert_eq!(ret.ops[2], move2_op);
  }
}

mod bytecode_compiler_multi_call_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:565:bytecode_compiler_multi_call_variadic`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function fallthroughBlock (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function branchBlock (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record BcImm (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> method BcFunction::immOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> enum BcImmKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - translates_to -> rust_item bytecode_compiler_multi_call_variadic

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_multi_call_variadic() {
    use ulua_bytecode::enums::{
      bc_block_edge_kind::BcBlockEdgeKind, bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind,
    };
    use ulua_common::{FFlag::LuauEmitCallFeedback, enums::luau_opcode::LuauOpcode};
    use ulua_unit_test::{
      functions::{
        branch_op::branch_op, check_edges::check_edges, check_ops::check_ops,
        fallthrough_op::fallthrough_op, get_op::get_op,
      },
      records::bytecode_compiler_fixture::BytecodeCompilerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emit_call_feedback = ScopedFastFlag::new(&LuauEmitCallFeedback, true);
    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
      .build_bytecode(
        r#"
        local function fn(n)
            if n > 0 then
                return 0, 1
            else
                local a, b = fn(n - 1)
                return a + b, fn(n)
            end
        end
    "#,
        0,
      )
      .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 4);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_edges(
      &entry.successors,
      &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));
    let if_true = fn_.block_op(fallthrough_op(&entry.successors)).clone();
    assert!(check_edges(
      &if_true.successors,
      &[BcBlockEdgeKind::Fallthrough]
    ));
    let if_false = fn_.block_op(branch_op(&entry.successors)).clone();
    assert!(check_edges(
      &if_true.successors,
      &[BcBlockEdgeKind::Fallthrough]
    ));

    assert!(check_ops(
      &mut fn_,
      &entry.ops,
      &[LuauOpcode::LOP_LOADK, LuauOpcode::LOP_JUMPIFNOTLT]
    ));
    assert!(check_ops(
      &mut fn_,
      &if_true.ops,
      &[
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_RETURN,
      ]
    ));
    assert!(check_ops(
      &mut fn_,
      &if_false.ops,
      &[
        LuauOpcode::LOP_GETUPVAL,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_SUB,
        LuauOpcode::LOP_CALLFB,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_GETUPVAL,
        LuauOpcode::LOP_MOVE,
        LuauOpcode::LOP_CALL,
        LuauOpcode::LOP_RETURN,
      ]
    ));

    let ret = fn_.inst_op(get_op(&if_false, 8)).clone();
    assert_eq!(ret.ops.len(), 3);
    assert_eq!(ret.ops[0].kind, BcOpKind::Imm);
    let ret_count = *fn_.imm_op(ret.ops[0]);
    assert_eq!(ret_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { ret_count.value.value_int }, -1);
    let add_op = get_op(&if_false, 4);
    assert_eq!(ret.ops[1], add_op);
    let multi_call_op = get_op(&if_false, 7);
    assert_eq!(ret.ops[2], multi_call_op);
  }
}

mod bytecode_compiler_nested_loops {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:427:bytecode_compiler_nested_loops`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function fallthroughOp (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> record Loop (Compiler/src/Compiler.cpp)
  //!   - calls -> function fallthroughBlock (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function loopOp (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function branchBlock (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function isPhiOf (tests/BytecodeCompiler.test.cpp)
  //!   - translates_to -> rust_item bytecode_compiler_nested_loops

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_nested_loops() {
    use ulua_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
    use ulua_common::enums::luau_opcode::LuauOpcode;
    use ulua_unit_test::{
      functions::{
        branch_op::branch_op, check_edges::check_edges, check_ops::check_ops,
        fallthrough_op::fallthrough_op, get_op::get_op, is_phi_of::is_phi_of, loop_op::loop_op,
      },
      records::bytecode_compiler_fixture::BytecodeCompilerFixture,
    };

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
      .build_bytecode(
        r#"
        function fn()
            local res = 0
            local var = 0
            repeat
                local i = 0
                repeat
                    res += i * var
                    i += 1
                until i < 5
                var += 1
            until var < 10
        end
    "#,
        0,
      )
      .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 8);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_edges(
      &entry.successors,
      &[BcBlockEdgeKind::Fallthrough]
    ));

    let outer_entry_op = fallthrough_op(&entry.successors);
    let outer_entry = fn_.block_op(outer_entry_op).clone();
    assert!(check_edges(
      &outer_entry.predecessors,
      &[BcBlockEdgeKind::Fallthrough, BcBlockEdgeKind::Loop]
    ));
    assert!(check_edges(
      &outer_entry.successors,
      &[BcBlockEdgeKind::Fallthrough]
    ));

    let inner_entry_op = fallthrough_op(&outer_entry.successors);
    let inner_entry = fn_.block_op(inner_entry_op).clone();
    assert!(check_edges(
      &inner_entry.predecessors,
      &[BcBlockEdgeKind::Fallthrough, BcBlockEdgeKind::Loop]
    ));
    assert!(check_edges(
      &inner_entry.successors,
      &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let inner_back_loop = fn_
      .block_op(fallthrough_op(&inner_entry.successors))
      .clone();
    assert!(check_edges(
      &inner_back_loop.successors,
      &[BcBlockEdgeKind::Loop]
    ));
    assert_eq!(loop_op(&inner_back_loop.successors), inner_entry_op);

    let outer_epllog = fn_.block_op(branch_op(&inner_entry.successors)).clone();
    assert!(check_edges(
      &outer_epllog.successors,
      &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));
    let outer_back_loop = fn_
      .block_op(fallthrough_op(&outer_epllog.successors))
      .clone();
    assert!(check_edges(
      &outer_back_loop.successors,
      &[BcBlockEdgeKind::Loop]
    ));
    assert_eq!(loop_op(&outer_back_loop.successors), outer_entry_op);
    let ret = fn_.block_op(branch_op(&outer_epllog.successors)).clone();

    assert!(check_ops(
      &mut fn_,
      &entry.ops,
      &[LuauOpcode::LOP_LOADK, LuauOpcode::LOP_LOADK]
    ));
    assert!(check_ops(
      &mut fn_,
      &outer_entry.ops,
      &[LuauOpcode::LOP_LOADK]
    ));
    assert!(check_ops(
      &mut fn_,
      &inner_entry.ops,
      &[
        LuauOpcode::LOP_MUL,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_JUMPIFLT,
      ]
    ));
    assert!(check_ops(
      &mut fn_,
      &inner_back_loop.ops,
      &[LuauOpcode::LOP_JUMPBACK]
    ));
    assert!(check_ops(
      &mut fn_,
      &outer_epllog.ops,
      &[
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_JUMPIFLT,
      ]
    ));
    assert!(check_ops(
      &mut fn_,
      &outer_back_loop.ops,
      &[LuauOpcode::LOP_JUMPBACK]
    ));
    assert!(check_ops(&mut fn_, &ret.ops, &[LuauOpcode::LOP_RETURN]));

    let var_init_op = get_op(&entry, 1);
    let var_inc_op = get_op(&outer_epllog, 1);
    let i_init_op = get_op(&outer_entry, 0);
    let i_inc_op = get_op(&inner_entry, 3);
    let i_times_var_op = get_op(&inner_entry, 0);
    let i_times_var = fn_.inst_op(i_times_var_op).clone();
    assert_eq!(i_times_var.ops.len(), 2);
    assert!(is_phi_of(&mut fn_, i_times_var.ops[0], i_init_op, i_inc_op));
    assert!(is_phi_of(
      &mut fn_,
      i_times_var.ops[1],
      var_init_op,
      var_inc_op
    ));
  }
}

mod bytecode_compiler_repeat_until_loop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:289:bytecode_compiler_repeat_until_loop`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function fallthroughBlock (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - type_ref -> record Loop (Compiler/src/Compiler.cpp)
  //!   - calls -> function branchBlock (tests/BytecodeCompiler.test.cpp)
  //!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - type_ref -> record BcPhi (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> method BcFunction::phiOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - translates_to -> rust_item bytecode_compiler_repeat_until_loop

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_repeat_until_loop() {
    use ulua_bytecode::enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind};
    use ulua_common::enums::luau_opcode::LuauOpcode;
    use ulua_unit_test::{
      functions::{
        branch_op::branch_op, check_edges::check_edges, check_ops::check_ops,
        fallthrough_op::fallthrough_op, get_op::get_op,
      },
      records::bytecode_compiler_fixture::BytecodeCompilerFixture,
    };

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
      .build_bytecode(
        r#"
        function fn()
            local var = 0
            repeat var += 1 until var < 10
            --return var
        end
    "#,
        0,
      )
      .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 5);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_edges(
      &entry.successors,
      &[BcBlockEdgeKind::Fallthrough]
    ));

    let loop_body_op = fallthrough_op(&entry.successors);
    let loop_body = fn_.block_op(loop_body_op).clone();
    assert!(check_edges(
      &loop_body.predecessors,
      &[BcBlockEdgeKind::Fallthrough, BcBlockEdgeKind::Loop]
    ));
    assert!(check_edges(
      &loop_body.successors,
      &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let loop_jump_back = fn_.block_op(fallthrough_op(&loop_body.successors)).clone();
    assert!(check_edges(
      &loop_jump_back.successors,
      &[BcBlockEdgeKind::Loop]
    ));
    let ret = fn_.block_op(branch_op(&loop_body.successors)).clone();

    assert!(check_ops(&mut fn_, &entry.ops, &[LuauOpcode::LOP_LOADK]));
    assert!(check_ops(
      &mut fn_,
      &loop_body.ops,
      &[
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_JUMPIFLT,
      ]
    ));
    assert!(check_ops(
      &mut fn_,
      &loop_jump_back.ops,
      &[LuauOpcode::LOP_JUMPBACK]
    ));
    assert!(check_ops(&mut fn_, &ret.ops, &[LuauOpcode::LOP_RETURN]));

    let var_init_op = get_op(&entry, 0);
    let load_k_one_op = get_op(&loop_body, 0);
    let add_var_op = get_op(&loop_body, 1);
    let add_var = fn_.inst_op(add_var_op).clone();
    assert_eq!(add_var.ops.len(), 2);
    assert_eq!(add_var.ops[0].kind, BcOpKind::Phi);
    let add_var_phi = fn_.phi_op(add_var.ops[0]).clone();
    assert_eq!(add_var_phi.ops[0], var_init_op);
    assert_eq!(add_var_phi.ops[1], add_var_op);
    assert_eq!(add_var.ops[1], load_k_one_op);
  }
}

mod bytecode_compiler_tables_strings_and_fastcall {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:709:bytecode_compiler_tables_strings_and_fastcall`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
  //!   - translates_to -> rust_item bytecode_compiler_tables_strings_and_fastcall

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_tables_strings_and_fastcall() {
    use ulua_common::enums::luau_opcode::LuauOpcode;
    use ulua_unit_test::{
      functions::check_ops::check_ops, records::bytecode_compiler_fixture::BytecodeCompilerFixture,
    };

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
      .build_bytecode(
        r#"
        local tt = {}
        local function fn(x)
            local t = { a = x, b = x .. 42 }
            return table.insert({t}, tt)
        end
    "#,
        1,
      )
      .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 2);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_ops(
      &mut fn_,
      &entry.ops,
      &[
        LuauOpcode::LOP_DUPTABLE,
        LuauOpcode::LOP_SETTABLEKS,
        LuauOpcode::LOP_MOVE,
        LuauOpcode::LOP_LOADN,
        LuauOpcode::LOP_CONCAT,
        LuauOpcode::LOP_SETTABLEKS,
        LuauOpcode::LOP_NEWTABLE,
        LuauOpcode::LOP_MOVE,
        LuauOpcode::LOP_SETLIST,
        LuauOpcode::LOP_GETUPVAL,
        LuauOpcode::LOP_FASTCALL2,
        LuauOpcode::LOP_GETIMPORT,
        LuauOpcode::LOP_CALL,
        LuauOpcode::LOP_RETURN,
      ]
    ));
  }
}

mod bytecode_compiler_variadic_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:628:bytecode_compiler_variadic_function`
  //! Source: `tests/BytecodeCompiler.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BytecodeCompiler.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
  //!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
  //!   - includes -> source_file Common/include/Luau/BytecodeWire.h
  //!   - includes -> source_file Compiler/include/Luau/Compiler.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BytecodeCompiler.test.cpp
  //! - outgoing:
  //!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
  //!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record BcImm (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - calls -> method BcFunction::immOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> enum BcImmKind (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
  //!   - translates_to -> rust_item bytecode_compiler_variadic_function

  #[cfg(test)]
  #[test]
  fn bytecode_compiler_variadic_function() {
    use ulua_bytecode::enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind};
    use ulua_common::enums::luau_opcode::LuauOpcode;
    use ulua_unit_test::{
      functions::{check_ops::check_ops, get_op::get_op},
      records::bytecode_compiler_fixture::BytecodeCompilerFixture,
    };

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
      .build_bytecode(
        r#"
        local function fn(a, ...)
            local b, c = ...
            local l = {...}
            return a + b + c + l[1], ...
        end
    "#,
        0,
      )
      .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 2);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_ops(
      &mut fn_,
      &entry.ops,
      &[
        LuauOpcode::LOP_PREPVARARGS,
        LuauOpcode::LOP_GETVARARGS,
        LuauOpcode::LOP_NEWTABLE,
        LuauOpcode::LOP_GETVARARGS,
        LuauOpcode::LOP_SETLIST,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_LOADK,
        LuauOpcode::LOP_GETTABLE,
        LuauOpcode::LOP_ADD,
        LuauOpcode::LOP_GETVARARGS,
        LuauOpcode::LOP_RETURN,
      ]
    ));

    let get_var_args1 = fn_.inst_op(get_op(&entry, 1)).clone();
    assert_eq!(get_var_args1.ops.len(), 2);
    assert_eq!(get_var_args1.ops[0].kind, BcOpKind::VmReg);
    assert_eq!(get_var_args1.ops[0].index, 1);
    assert_eq!(get_var_args1.ops[1].kind, BcOpKind::Imm);
    let get_var_args1_count = *fn_.imm_op(get_var_args1.ops[1]);
    assert_eq!(get_var_args1_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { get_var_args1_count.value.value_int }, 2);

    let get_var_args2_op = get_op(&entry, 3);
    let get_var_args2 = fn_.inst_op(get_var_args2_op).clone();
    assert_eq!(get_var_args2.ops.len(), 2);
    assert_eq!(get_var_args2.ops[0].kind, BcOpKind::VmReg);
    assert_eq!(get_var_args2.ops[0].index, 4);
    assert_eq!(get_var_args2.ops[1].kind, BcOpKind::Imm);
    let get_var_args2_count = *fn_.imm_op(get_var_args2.ops[1]);
    assert_eq!(get_var_args2_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { get_var_args2_count.value.value_int }, -1);

    let set_list = fn_.inst_op(get_op(&entry, 4)).clone();
    assert_eq!(set_list.ops.len(), 4);
    let set_list_start_idx = *fn_.imm_op(set_list.ops[0]);
    assert_eq!(set_list_start_idx.kind, BcImmKind::Int);
    assert_eq!(unsafe { set_list_start_idx.value.value_int }, 1);
    let set_list_count = *fn_.imm_op(set_list.ops[1]);
    assert_eq!(set_list_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { set_list_count.value.value_int }, -1);
    let new_table_op = get_op(&entry, 2);
    assert_eq!(set_list.ops[2], new_table_op);
    assert_eq!(set_list.ops[3], get_var_args2_op);
  }
}
