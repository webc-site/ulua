extern crate alloc;

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_break_comes_last() {
  use ulua_ast::records::{ast_stat_repeat::AstStatRepeat, parse_options::ParseOptions};
  use ulua_unit_test::{
    functions::{ast_node_ref::as_node_at, toposort::toposort},
    records::fixture::Fixture,
  };

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
repeat
local module = {}
local function confuseCompiler() return module.foo() end
module.foo = function() return "" end
break
until true
    "#,
    &ParseOptions::default(),
  );

  assert_eq!(program.body.len(), 1);

  let repeat = as_node_at::<AstStatRepeat, _>(&program.body, 0).unwrap();

  // Safety: repeat 指向 arena 存活的 AstStatRepeat；body 已句柄化为 Node
  // （parser 非空由类型层承载），as_ptr 桥交 toposort 所需的 &mut 视图，
  // 本测试内 arena 独占无别名；借用随即传入 toposort，生命周期互不重叠。
  let body = unsafe { &mut *repeat.body.as_ptr() };
  assert_eq!(body.body.len(), 4);

  let sorted = toposort(body);

  assert_eq!(sorted.len(), 4);
  assert_eq!(sorted[3], body.body.as_slice()[3]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_continue_comes_last() {
  use ulua_ast::records::{ast_stat_repeat::AstStatRepeat, parse_options::ParseOptions};
  use ulua_unit_test::{
    functions::{ast_node_ref::as_node_at, toposort::toposort},
    records::fixture::Fixture,
  };

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
repeat
local module = {}
local function confuseCompiler() return module.foo() end
module.foo = function() return "" end
continue
until true
    "#,
    &ParseOptions::default(),
  );

  assert_eq!(program.body.len(), 1);

  let repeat = as_node_at::<AstStatRepeat, _>(&program.body, 0).unwrap();

  // Safety: repeat 指向 arena 存活的 AstStatRepeat；body 已句柄化为 Node
  // （parser 非空由类型层承载），as_ptr 桥交 toposort 所需的 &mut 视图，
  // 本测试内 arena 独占无别名；借用随即传入 toposort，生命周期互不重叠。
  let body = unsafe { &mut *repeat.body.as_ptr() };
  assert_eq!(body.body.len(), 4);

  let sorted = toposort(body);

  assert_eq!(sorted.len(), 4);
  assert_eq!(sorted[3], body.body.as_slice()[3]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_cyclic_dependency_terminates() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        function A()
            return B()
        end

        function B()
            return A()
        end
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);
  assert_eq!(2, sorted.len());
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_doesnt_omit_statements_that_dont_need_sorting() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local X = {}

        function A()
            return B(5), B("Hi")
        end

        local Y = {}

        function B(x)
            return x
        end

        local Z = B()
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);
  assert_eq!(5, sorted.len());

  assert_eq!(5, program.body.len());

  let x = program.body.as_slice()[0];
  let a = program.body.as_slice()[1];
  let y = program.body.as_slice()[2];
  let b = program.body.as_slice()[3];
  let z = program.body.as_slice()[4];

  assert_eq!(sorted[0], x);
  assert_eq!(sorted[1], y);
  assert_eq!(sorted[2], b);
  assert_eq!(sorted[3], z);
  assert_eq!(sorted[4], a);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_dont_force_checking_until_an_ast_expr_call_needs_the_symbol() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
      r#"
        function A(obj)
            C(obj)
        end

        local B = A             -- It would be an error to force checking of A at this point just because the definition of B is an imperative

        function C(player)
        end

        local D = A(nil)        -- The real dependency on A is here, where A is invoked.
    "#,
      &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(4, sorted.len());

  let a = program.body.as_slice()[0];
  let b = program.body.as_slice()[1];
  let c = program.body.as_slice()[2];
  let d = program.body.as_slice()[3];

  assert_eq!(sorted[0], c);
  assert_eq!(sorted[1], a);
  assert_eq!(sorted[2], b);
  assert_eq!(sorted[3], d);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_dont_reorder_assigns() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local T = {}                -- 0

        function T.a()              -- 1 depends on (2)
            T.b()
        end

        function T.b()              -- 2 depends on (5)
            T.c()
        end

        function make_function()    -- 3
            return function() end
        end

        T.a()                       -- 4 depends on (1 -> 2 -> 5), but we cannot reorder it after 5!

        T.c = make_function()       -- 5 depends on (3)
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(6, sorted.len());
  assert_eq!(sorted[0], program.body.as_slice()[0]);
  assert_eq!(sorted[1], program.body.as_slice()[3]);
  assert_eq!(sorted[2], program.body.as_slice()[2]);
  assert_eq!(sorted[3], program.body.as_slice()[1]);
  assert_eq!(sorted[4], program.body.as_slice()[4]);
  assert_eq!(sorted[5], program.body.as_slice()[5]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_dont_reorder_function_after_assignment_to_global() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local f

        function g()
            f()
        end

        f = function() end
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(3, sorted.len());
  assert_eq!(sorted[0], program.body.as_slice()[0]);
  assert_eq!(sorted[1], program.body.as_slice()[1]);
  assert_eq!(sorted[2], program.body.as_slice()[2]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_dont_reorder_imperatives() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local temp = work
        work = arr
        arr = temp
        width = width * 2
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);
  assert_eq!(4, sorted.len());
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_function_return_type_depends_on_type_aliases() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        type callbackFn<K, V> = (element: V, key: K, map: Map<K, V>) -> ()

        export type Map<K, V> = {
            forEach: (callback: callbackFn<K, V>) -> (),
        }

        function foo<K, V>(key: K, value: V): Map<K, V>
        end
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(3, sorted.len());

  let callback_fn = program.body.as_slice()[0];
  let map = program.body.as_slice()[1];
  let foo = program.body.as_slice()[2];

  assert_eq!(sorted[0], callback_fn);
  assert_eq!(sorted[1], map);
  assert_eq!(sorted[2], foo);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_local_functions_need_sorting_too() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local a = nil                       -- 0

        local function f()                  -- 1 depends on 4
            a.c = 4
        end

        local function g()                  -- 2 depends on 1
            f()
        end

        a = {}                              -- 3
        a.c = nil                           -- 4
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(5, sorted.len());
  assert_eq!(sorted[0], program.body.as_slice()[0]);
  assert_eq!(sorted[1], program.body.as_slice()[3]);
  assert_eq!(sorted[2], program.body.as_slice()[4]);
  assert_eq!(sorted[3], program.body.as_slice()[1]);
  assert_eq!(sorted[4], program.body.as_slice()[2]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_nested_type_annotations_depends_on_later_typealiases() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        type Foo = A | B
        type B = number
        type A = string
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(3, sorted.len());

  let foo = program.body.as_slice()[0];
  let b = program.body.as_slice()[1];
  let a = program.body.as_slice()[2];

  assert_eq!(sorted[0], b);
  assert_eq!(sorted[1], a);
  assert_eq!(sorted[2], foo);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_reorder_functions_after_dependent_assigns() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local T = {}                -- 0

        function T.a()              -- 1 depends on (2)
            T.b()
        end

        function T.b()              -- 2 depends on (4)
            T.c()
        end

        function make_function()    -- 3
            return function() end
        end

        T.c = make_function()       -- 4 depends on (3)

        T.a()                       -- 5 depends on (1)
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(6, sorted.len());
  assert_eq!(sorted[0], program.body.as_slice()[0]);
  assert_eq!(sorted[1], program.body.as_slice()[3]);
  assert_eq!(sorted[2], program.body.as_slice()[4]);
  assert_eq!(sorted[3], program.body.as_slice()[2]);
  assert_eq!(sorted[4], program.body.as_slice()[1]);
  assert_eq!(sorted[5], program.body.as_slice()[5]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_return_comes_last() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local module = {}

        local function confuseCompiler() return module.foo() end

        module.foo = function() return "" end

        function module.bar(x:number)
            confuseCompiler()
            return true
        end

        return module
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(sorted[0], program.body.as_slice()[0]);
  assert_eq!(sorted[2], program.body.as_slice()[1]);
  assert_eq!(sorted[1], program.body.as_slice()[2]);
  assert_eq!(sorted[3], program.body.as_slice()[3]);
  assert_eq!(sorted[4], program.body.as_slice()[4]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_slightly_more_complex() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local T = {}

        function T:foo()
            return T:bar(999), T:bar("hi")
        end

        function T:bar(i)
            return i
        end
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(3, sorted.len());
  assert_eq!(sorted[0], program.body.as_slice()[0]);
  assert_eq!(sorted[1], program.body.as_slice()[2]);
  assert_eq!(sorted[2], program.body.as_slice()[1]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_sort_typealias_first() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        local foo: A = 1
        type A = number
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(2, sorted.len());

  let a = program.body.as_slice()[0];
  let b = program.body.as_slice()[1];

  assert_eq!(sorted[0], b);
  assert_eq!(sorted[1], a);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_sorts() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        function A()
            return B("high five!")
        end

        function B(x)
            return x
        end
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);
  assert_eq!(2, sorted.len());

  assert_eq!(2, program.body.len());

  assert_eq!(program.body.as_slice()[1], sorted[0]);
  assert_eq!(program.body.as_slice()[0], sorted[1]);
}

// Source: `tests/TopoSort.test.cpp`
#[test]
fn topo_sort_typealias_of_typeof_is_not_sorted() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let program = fixture.parse(
    r#"
        type Foo = typeof(foo)
        local function foo(x: number) end
    "#,
    &ParseOptions::default(),
  );

  let sorted = toposort(&mut *program);

  assert_eq!(2, sorted.len());

  let a = program.body.as_slice()[0];
  let b = program.body.as_slice()[1];

  assert_eq!(sorted[0], a);
  assert_eq!(sorted[1], b);
}
