extern crate alloc;

mod cost_model_conditional {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(4, compute_cost(model, &args1));
    assert_eq!(2, compute_cost(model, &args2));
  }
}

mod cost_model_cost_overflow {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(127, compute_cost(model, &[]));
  }
}

mod cost_model_expression {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(5, compute_cost(model, &args1));
    assert_eq!(2, compute_cost(model, &args2));
  }
}

mod cost_model_fast_call {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(6, compute_cost(model, &args1));
    assert_eq!(5, compute_cost(model, &args2));
  }
}

mod cost_model_import_call {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(6, compute_cost(model, &args1));
    assert_eq!(6, compute_cost(model, &args2));
  }
}

mod cost_model_interp_string {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(3, compute_cost(model, &args1));
    assert_eq!(3, compute_cost(model, &args2));
  }
}

mod cost_model_loop_assign {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(6, compute_cost(model, &args1));
    assert_eq!(6, compute_cost(model, &args2));
  }
}

mod cost_model_multiple_assignments {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(8, compute_cost(model, &args1));
    assert_eq!(7, compute_cost(model, &args2));
  }
}

mod cost_model_mutable_variable {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(3, compute_cost(model, &args1));
    assert_eq!(2, compute_cost(model, &args2));
  }
}

mod cost_model_propagate_variable {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(3, compute_cost(model, &args1));
    assert_eq!(0, compute_cost(model, &args2));
  }
}

mod cost_model_table_assign {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(7, compute_cost(model, &args1));
    assert_eq!(6, compute_cost(model, &args2));
  }
}

mod cost_model_tables_functions {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(22, compute_cost(model, &[]));
  }
}

mod cost_model_var_args {
  //! Source: `tests/CostModel.test.cpp`

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

    assert_eq!(8, compute_cost(model, &[]));
  }
}
