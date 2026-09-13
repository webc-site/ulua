extern crate alloc;

mod topo_sort_break_comes_last {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:392:topo_sort_break_comes_last`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record AstStatRepeat (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_break_comes_last

  #[cfg(test)]
  #[test]
  fn topo_sort_break_comes_last() {
    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_repeat::AstStatRepeat, parse_options::ParseOptions},
      rtti::ast_node_as,
    };
    use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

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

    let program = unsafe { &*program };
    assert_eq!(program.body.size, 1);

    let repeat = unsafe { ast_node_as::<AstStatRepeat>(*program.body.data.add(0) as *mut AstNode) };
    assert!(!repeat.is_null());

    let body = unsafe { &mut *(*repeat).body };
    assert_eq!(body.body.size, 4);

    let sorted = toposort(body);

    assert_eq!(sorted.len(), 4);
    assert_eq!(sorted[3], unsafe { *body.body.data.add(3) });
  }
}

mod topo_sort_continue_comes_last {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:416:topo_sort_continue_comes_last`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record AstStatRepeat (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_continue_comes_last

  #[cfg(test)]
  #[test]
  fn topo_sort_continue_comes_last() {
    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_repeat::AstStatRepeat, parse_options::ParseOptions},
      rtti::ast_node_as,
    };
    use ulua_unit_test::{functions::toposort::toposort, records::fixture::Fixture};

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

    let program = unsafe { &*program };
    assert_eq!(program.body.size, 1);

    let repeat = unsafe { ast_node_as::<AstStatRepeat>(*program.body.data.add(0) as *mut AstNode) };
    assert!(!repeat.is_null());

    let body = unsafe { &mut *(*repeat).body };
    assert_eq!(body.body.size, 4);

    let sorted = toposort(body);

    assert_eq!(sorted.len(), 4);
    assert_eq!(sorted[3], unsafe { *body.body.data.add(3) });
  }
}

mod topo_sort_cyclic_dependency_terminates {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:45:topo_sort_cyclic_dependency_terminates`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_cyclic_dependency_terminates

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    assert_eq!(2, sorted.len());
  }
}

mod topo_sort_doesnt_omit_statements_that_dont_need_sorting {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:61:topo_sort_doesnt_omit_statements_that_dont_need_sorting`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item topo_sort_doesnt_omit_statements_that_dont_need_sorting

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    assert_eq!(5, sorted.len());

    let block = unsafe { &*program };
    assert_eq!(5, block.body.size);

    let x = unsafe { *block.body.data.add(0) };
    let a = unsafe { *block.body.data.add(1) };
    let y = unsafe { *block.body.data.add(2) };
    let b = unsafe { *block.body.data.add(3) };
    let z = unsafe { *block.body.data.add(4) };

    assert_eq!(sorted[0], x);
    assert_eq!(sorted[1], y);
    assert_eq!(sorted[2], b);
    assert_eq!(sorted[3], z);
    assert_eq!(sorted[4], a);
  }
}

mod topo_sort_dont_force_checking_until_an_ast_expr_call_needs_the_symbol {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:239:topo_sort_dont_force_checking_until_an_ast_expr_call_needs_the_symbol`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_dont_force_checking_until_an_ast_expr_call_needs_the_symbol

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(4, sorted.len());

    let a = unsafe { *program.body.data.add(0) };
    let b = unsafe { *program.body.data.add(1) };
    let c = unsafe { *program.body.data.add(2) };
    let d = unsafe { *program.body.data.add(3) };

    assert_eq!(sorted[0], c);
    assert_eq!(sorted[1], a);
    assert_eq!(sorted[2], b);
    assert_eq!(sorted[3], d);
  }
}

mod topo_sort_dont_reorder_assigns {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:156:topo_sort_dont_reorder_assigns`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_dont_reorder_assigns

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(6, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(3) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(2) });
    assert_eq!(sorted[3], unsafe { *program.body.data.add(1) });
    assert_eq!(sorted[4], unsafe { *program.body.data.add(4) });
    assert_eq!(sorted[5], unsafe { *program.body.data.add(5) });
  }
}

mod topo_sort_dont_reorder_function_after_assignment_to_global {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:190:topo_sort_dont_reorder_function_after_assignment_to_global`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_dont_reorder_function_after_assignment_to_global

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(3, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(1) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(2) });
  }
}

mod topo_sort_dont_reorder_imperatives {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:269:topo_sort_dont_reorder_imperatives`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_dont_reorder_imperatives

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    assert_eq!(4, sorted.len());
  }
}

mod topo_sort_function_return_type_depends_on_type_aliases {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:340:topo_sort_function_return_type_depends_on_type_aliases`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_function_return_type_depends_on_type_aliases

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(3, sorted.len());

    let callback_fn = unsafe { *program.body.data.add(0) };
    let map = unsafe { *program.body.data.add(1) };
    let foo = unsafe { *program.body.data.add(2) };

    assert_eq!(sorted[0], callback_fn);
    assert_eq!(sorted[1], map);
    assert_eq!(sorted[2], foo);
  }
}

mod topo_sort_local_functions_need_sorting_too {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:211:topo_sort_local_functions_need_sorting_too`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_local_functions_need_sorting_too

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(5, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(3) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(4) });
    assert_eq!(sorted[3], unsafe { *program.body.data.add(1) });
    assert_eq!(sorted[4], unsafe { *program.body.data.add(2) });
  }
}

mod topo_sort_nested_type_annotations_depends_on_later_typealiases {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:319:topo_sort_nested_type_annotations_depends_on_later_typealiases`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_nested_type_annotations_depends_on_later_typealiases

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(3, sorted.len());

    let foo = unsafe { *program.body.data.add(0) };
    let b = unsafe { *program.body.data.add(1) };
    let a = unsafe { *program.body.data.add(2) };

    assert_eq!(sorted[0], b);
    assert_eq!(sorted[1], a);
    assert_eq!(sorted[2], foo);
  }
}

mod topo_sort_reorder_functions_after_dependent_assigns {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:122:topo_sort_reorder_functions_after_dependent_assigns`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_reorder_functions_after_dependent_assigns

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(6, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(3) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(4) });
    assert_eq!(sorted[3], unsafe { *program.body.data.add(2) });
    assert_eq!(sorted[4], unsafe { *program.body.data.add(1) });
    assert_eq!(sorted[5], unsafe { *program.body.data.add(5) });
  }
}

mod topo_sort_return_comes_last {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:366:topo_sort_return_comes_last`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_return_comes_last

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(1) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(2) });
    assert_eq!(sorted[3], unsafe { *program.body.data.add(3) });
    assert_eq!(sorted[4], unsafe { *program.body.data.add(4) });
  }
}

mod topo_sort_slightly_more_complex {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:99:topo_sort_slightly_more_complex`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_slightly_more_complex

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(3, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(2) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(1) });
  }
}

mod topo_sort_sort_typealias_first {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:283:topo_sort_sort_typealias_first`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_sort_typealias_first

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(2, sorted.len());

    let a = unsafe { *program.body.data.add(0) };
    let b = unsafe { *program.body.data.add(1) };

    assert_eq!(sorted[0], b);
    assert_eq!(sorted[1], a);
  }
}

mod topo_sort_sorts {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:21:topo_sort_sorts`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_sorts

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    assert_eq!(2, sorted.len());

    let block = unsafe { &*program };
    assert_eq!(2, block.body.size);

    assert_eq!(unsafe { *block.body.data.add(1) }, sorted[0]);
    assert_eq!(unsafe { *block.body.data.add(0) }, sorted[1]);
  }
}

mod topo_sort_typealias_of_typeof_is_not_sorted {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:301:topo_sort_typealias_of_typeof_is_not_sorted`
  //! Source: `tests/TopoSort.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TopoSort.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TopoSort.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item topo_sort_typealias_of_typeof_is_not_sorted

  #[cfg(test)]
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

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(2, sorted.len());

    let a = unsafe { *program.body.data.add(0) };
    let b = unsafe { *program.body.data.add(1) };

    assert_eq!(sorted[0], a);
    assert_eq!(sorted[1], b);
  }
}
