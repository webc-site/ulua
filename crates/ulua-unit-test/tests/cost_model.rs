extern crate alloc;

mod cost_model_conditional {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:134:cost_model_conditional`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_conditional

  #[cfg(test)]
  #[test]
  fn cost_model_conditional() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a)
    return if a < 0 then -a else a
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(4, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(2, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_cost_overflow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:171:cost_model_cost_overflow`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_cost_overflow
  use core::ptr::null;

  #[cfg(test)]
  #[test]
  fn cost_model_cost_overflow() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test()
    return {{{{{{{{{{{{{{{}}}}}}}}}}}}}}}
end
"#;
    let model = model_function(source);

    assert_eq!(127, unsafe { compute_cost(model, null(), 0) });
  }
}

mod cost_model_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:37:cost_model_expression`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_expression

  #[cfg(test)]
  #[test]
  fn cost_model_expression() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a, b, c)
    return a + (b + 1) * (b + 1) - c
end
"#;
    let model = model_function(source);

    let args1 = [false, false, false];
    let args2 = [false, true, false];

    assert_eq!(5, unsafe { compute_cost(model, args1.as_ptr(), 3) });
    assert_eq!(2, unsafe { compute_cost(model, args2.as_ptr(), 3) });
  }
}

mod cost_model_fast_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:118:cost_model_fast_call`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_fast_call

  #[cfg(test)]
  #[test]
  fn cost_model_fast_call() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a)
    return math.abs(a + 1)
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(6, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(5, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_import_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:103:cost_model_import_call`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_import_call

  #[cfg(test)]
  #[test]
  fn cost_model_import_call() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a)
    return Instance.new(a)
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(6, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(6, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_interp_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:199:cost_model_interp_string`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_interp_string

  #[cfg(test)]
  #[test]
  fn cost_model_interp_string() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a)
    return `hello, {a}!`
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(3, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(3, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_loop_assign {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:68:cost_model_loop_assign`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_loop_assign

  #[cfg(test)]
  #[test]
  fn cost_model_loop_assign() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a)
    for i=1,3 do
        a[i] = i
    end
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(6, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(6, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_multiple_assignments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:214:cost_model_multiple_assignments`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_multiple_assignments

  #[cfg(test)]
  #[test]
  fn cost_model_multiple_assignments() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a)
    local x = 0
    x = a
    x = a + 1
    x, x, x = a
    x = a, a, a
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(8, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(7, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_mutable_variable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:86:cost_model_mutable_variable`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_mutable_variable

  #[cfg(test)]
  #[test]
  fn cost_model_mutable_variable() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a, b)
    local x = a * a
    x += b
    return x * x
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(3, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(2, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_propagate_variable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:52:cost_model_propagate_variable`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_propagate_variable

  #[cfg(test)]
  #[test]
  fn cost_model_propagate_variable() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a)
    local b = a * a * a
    return b * b
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(3, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(0, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_table_assign {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:182:cost_model_table_assign`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_table_assign

  #[cfg(test)]
  #[test]
  fn cost_model_table_assign() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(a)
    for i=1,#a do
        a[i] = i
    end
end
"#;
    let model = model_function(source);

    let args1 = [false];
    let args2 = [true];

    assert_eq!(7, unsafe { compute_cost(model, args1.as_ptr(), 1) });
    assert_eq!(6, unsafe { compute_cost(model, args2.as_ptr(), 1) });
  }
}

mod cost_model_tables_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:160:cost_model_tables_functions`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_tables_functions
  use core::ptr::null;

  #[cfg(test)]
  #[test]
  fn cost_model_tables_functions() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test()
    return { 42, op = function() end }
end
"#;
    let model = model_function(source);

    assert_eq!(22, unsafe { compute_cost(model, null(), 0) });
  }
}

mod cost_model_var_args {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:149:cost_model_var_args`
  //! Source: `tests/CostModel.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CostModel.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/CostModel.test.cpp
  //! - outgoing:
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> function modelFunction (tests/CostModel.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item cost_model_var_args
  use core::ptr::null;

  #[cfg(test)]
  #[test]
  fn cost_model_var_args() {
    use ulua_compiler::functions::compute_cost::compute_cost;
    use ulua_unit_test::functions::model_function::model_function;

    let source = r#"
function test(...)
    return select('#', ...) :: number
end
"#;
    let model = model_function(source);

    assert_eq!(8, unsafe { compute_cost(model, null(), 0) });
  }
}
