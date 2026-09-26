export default {
  "meta.title": "ulua — Luau herschreven in Rust",
  "meta.description":
    "Luau volledig herschreven in modern Rust. Directe uitvoering en statische typecontrole in de browser-sandbox.",
  "nav.playground": "Playground",
  "nav.benchmark": "Benchmarks",
  "nav.about": "Over",
  "nav.features": "Functies",
  "nav.lua_syntax": "Lua 5.1 Syntaxis",
  "nav.luau_syntax": "Luau Extensies",
  "nav.embed": "Inbedding",
  "nav.checker": "Typecontrole",
  "nav.crates": "Crates",
  "nav.lang": "Taal",
  "bench.title": "Performance Benchmarks",
  "bench.zone_jit": "JIT Mode",
  "bench.zone_interp": "Interpreter Mode",
  "bench.desc":
    "ulua benchmarked in both interpreter and JIT modes against official <code>mlua (C++ Luau)</code>, LuaJIT, and Lua 5.4.",
  "bench.env_label": "Test Environment",
  "bench.tab_overview": "Geometric Mean Time",
  "bench.tab_single": "Individual Benchmarks",
  "bench.col_bench": "Benchmark",
  "bench.toggle_table_show": "Show Benchmark Table ▼",
  "bench.toggle_table_hide": "Hide Benchmark Table ▲",
  "bench.tip_faster": "Lower is faster",
  "bench.current": "this",
  "bench.lang.ulua": "Luau (Pure Rust)",
  "bench.lang.mlua_luau": "Luau (C++)",
  "bench.lang.mlua_luajit": "LuaJIT 2.1",
  "bench.lang.mlua_lua54": "Lua 5.4",
  "bench.item.fib": "Recursive Fibonacci",
  "bench.item.fib_desc": "Deep function recursion and call stack overhead",
  "bench.param.fib": "Scale: Recursion depth N = 33",
  "bench.item.nbody": "Celestial Orbit Simulation",
  "bench.item.nbody_desc": "Heavy floating-point operations and local registers",
  "bench.param.nbody": "Scale: 450,000 steps multi-body orbit simulation",
  "bench.item.mandel": "Mandelbrot Fractal Set",
  "bench.item.mandel_desc": "Nested loop execution and branch prediction",
  "bench.param.mandel": "Scale: 400 × 400 complex grid, 256 iterations",
  "bench.item.matmul": "Dense Matrix Multiplication",
  "bench.item.matmul_desc": "Two-dimensional nested tables and array indexing",
  "bench.param.matmul": "Scale: 320 × 320 matrix multiply, O(N³) operations",
  "bench.item.tablesort": "In-place Table Sort",
  "bench.item.tablesort_desc": "Quicksort on tables with comparison closures",
  "bench.param.tablesort": "Scale: 140,000 pseudo-random numbers in-place sort",
  "bench.item.strings": "String Formatting & Concat",
  "bench.item.strings_desc": "String interning, allocation, and GC throughput",
  "bench.param.strings": "Scale: 320,000 string format and join iterations",
  "bench.item.binarytrees": "Binary Trees GC Stress",
  "bench.item.binarytrees_desc": "Tree node allocation and recursive GC lifecycle",
  "bench.param.binarytrees": "Scale: Maximum depth 13 tree allocation & GC reclaim",
  "bench.item.spectralnorm": "Spectral Norm Calculation",
  "bench.item.spectralnorm_desc": "Intense numeric approximation and array math",
  "bench.param.spectralnorm": "Scale: 1000-order matrix eigenvalue iteration",
  "hero.title_pre": "Van de grond af herschreven in Rust:",
  "hero.title_grad": "Moderne Luau-Engine",
  "pg.example": "Voorbeelden",
  "pg.run": "Uitvoeren",
  "pg.check": "Controleren",
  "pg.clear": "Wissen",
  "pg.editor_tab": "Editor",
  "pg.diag_output": "Diagnostiek / Uitvoer",
  "pg.loading_engine": "WASM-engine laden…",
  "pg.ready_prompt": "Voer code in en klik op «Uitvoeren (Ctrl+Enter)» of «Controleren».",
  "pg.note":
    '<strong>Type checking</strong> runs automatically after typing (click <span class="kbd-inline">Lnn</span> to jump); press <kbd>Ctrl+Enter</kbd> to <strong>run</strong>.',
  "about.title": "Wat is Luau? Waarom herschrijven in Rust",
  "about.lead":
    '<a href="https://luau.org" target="_blank" rel="noopener" class="luau-link">Luau ↗</a> is an embeddable scripting language with gradual typing developed by Roblox, derived from Lua 5.1. It adds static type inference, syntax extensions, and performance optimizations while retaining Lua\'s simplicity.',
  "about.text":
    'Official Luau is built with C++. <a href="https://crates.io/crates/ulua" target="_blank" rel="noopener" class="crate-link"><strong>ulua</strong> ↗</a> implements the Luau parser, compiler, VM, and type solver in Rust, providing memory safety, mlua-style bindings, and WebAssembly support.',
  "about.p1_title": "Memory Safety",
  "about.p1_desc":
    "Leverages Rust's ownership model to eliminate dangling pointers and buffer overflows at compile time.",
  "about.p2_title": "Cargo Integration",
  "about.p2_desc":
    'Add with <a href="https://crates.io/crates/ulua" target="_blank" rel="noopener" class="crate-code-link"><code>cargo add ulua</code> ↗</a> without C/C++ build tools, featuring an API compatible with mlua.',
  "about.p3_title": "Native & WASM",
  "about.p3_desc":
    "Embed in Rust backend and desktop applications, or compile to WebAssembly for browser environments.",
  "features.title": "Ondersteuning voor WebAssembly",
  "features.desc":
    "The compiler, VM, and type solver compile to WebAssembly for client-side execution and validation.",
  "features.f1_title": "Client-side Diagnostics",
  "features.f1_desc":
    "Perform type inference and line-level diagnostics directly in the browser without remote servers.",
  "features.f2_title": "Pure Rust",
  "features.f2_desc":
    "Free of external C/C++ dependencies, ready to integrate into Rust codebases.",
  "features.f3_title": "Sandbox & Offline",
  "features.f3_desc":
    "Executes inside the browser sandbox with no network dependencies, preserving privacy.",
  "syntax.expand_all": "Alles uitklappen",
  "syntax.collapse_all": "Alles inklappen",
  "syntax.lua_title": "Lua 5.1 Syntax",
  "syntax.luau_title": "Luau Extensions",
  "syntax.lua_intro":
    "Lua 5.1 is known for its lightweight simplicity, featuring tables, first-class functions, closures, and coroutines. ulua maintains compatibility with this specification.",
  "syntax.luau_intro":
    "Luau extends Lua 5.1 with a gradual type system, vector and buffer types, modern control flow, and native compilation optimizations.",
  "syntax.lua_basics": "Variabelen, Bereik & Scalairen",
  "syntax.lua_variables_title": "Local Variables & Lexical Scope",
  "syntax.lua_variables_desc":
    "Variables follow lexical scoping via local; supports block-level shadowing with do...end and interaction with the global table _G:",
  "syntax.lua_types_title": "Primitive Types & Dynamic Inspection",
  "syntax.lua_types_desc":
    "Lua 5.1 employs dynamic typing with primitives nil, boolean, number, and string; inspect runtime types with type(v):",
  "syntax.lua_strings_title": "String Literals & Long Brackets",
  "syntax.lua_strings_desc":
    "Supports single/double quoted strings and [[...]] raw multiline literals; concatenate via .. and get byte lengths via #:",
  "syntax.lua_control": "Besturingsstroom & Lussen",
  "syntax.lua_conditionals_title": "Conditional Branches & Truthiness",
  "syntax.lua_conditionals_desc":
    'Standard if-then-elseif-else logic; in Lua, only false and nil are falsy, while 0, empty string "", and {} are strictly truthy:',
  "syntax.lua_loops_title": "Conditional Loops & Numeric For",
  "syntax.lua_loops_desc":
    "Includes while loops, post-conditioned repeat...until loops, and numeric for loops with start, stop, and step:",
  "syntax.lua_pairs_title": "Generic Iteration: pairs & ipairs",
  "syntax.lua_pairs_desc":
    "pairs iterates over all key-value entries in a table; ipairs iterates sequentially over numeric keys 1..n until the first nil:",
  "syntax.lua_break_title": "break Statements & Block End Rules",
  "syntax.lua_break_desc":
    "break exits the innermost loop; standard grammar requires break at the end of a block, wrapping in do break end when breaking early:",
  "syntax.lua_functions": "Functies & Closures",
  "syntax.lua_functions_title": "First-Class Functions & Multi-Returns",
  "syntax.lua_functions_desc":
    "Functions are first-class values with multi-return expansion and truncation; wrapping in parentheses (fn()) forces truncation to a single value:",
  "syntax.lua_varargs_title": "Variadic Arguments (...)",
  "syntax.lua_varargs_desc":
    "Capture arbitrary arguments with ...; count inputs via select('#', ...) or pack them into tables via { ... }:",
  "syntax.lua_closures_title": "Lexical Closures & State Encapsulation",
  "syntax.lua_closures_desc":
    "Inner functions can capture outer local variables (upvalues), sharing state across closures:",
  "syntax.lua_errors_title": "Protected Calls & Error Handling (pcall / xpcall)",
  "syntax.lua_errors_desc":
    "Lua has no try-catch; use pcall for safe execution, or xpcall to capture debug.traceback before the call stack unwinds:",
  "syntax.lua_tables": "Tabellen, Metatabellen & OOP",
  "syntax.lua_tables_title": "Tables: Arrays & Hash Dictionaries",
  "syntax.lua_tables_desc":
    "Tables are Lua's sole compound data structure for arrays, dictionaries, and records; supports # length and table manipulation APIs:",
  "syntax.lua_metatables_title": "Metatable System & Event Dispatch",
  "syntax.lua_metatables_desc":
    "Associate metatables via setmetatable to customize __index fallback lookups, __newindex writes, and __call invocations:",
  "syntax.lua_weak_tables_title": "Weak Tables & Caching (__mode)",
  "syntax.lua_weak_tables_desc":
    "Metatable __mode supports 'k' (weak keys), 'v' (weak values), or 'kv'. Garbage collector cleans entries when no strong references remain:",
  "syntax.lua_op_overload_title": "Operator Overloading & Comparison",
  "syntax.lua_op_overload_desc":
    "Define __add, __sub, __mul, __eq, __tostring metamethods to give custom table structures native arithmetic and comparison support:",
  "syntax.lua_oop_title": "Prototype OOP & Colon Method Syntax",
  "syntax.lua_oop_desc":
    "Simulate class hierarchies via __index fallback chains; use object:method() colon syntax to implicitly pass self:",
  "syntax.lua_coroutines": "Coöperatieve Multitasking & Coroutines",
  "syntax.lua_coroutines_title": "Cooperative Coroutines (coroutine)",
  "syntax.lua_coroutines_desc":
    "Features suspended, running, normal, and dead states, using yield and resume for cooperative task switching and bidirectional data passing:",
  "syntax.lua_generator_title": "Coroutine Wrappers & Generators",
  "syntax.lua_generator_desc":
    "Use coroutine.wrap to transform stateful generators into standard iterators, enabling synchronous-style streaming:",
  "syntax.luau_syntax": "Moderne Syntaxisuitbreidingen",
  "syntax.compound_title": "Compound Assignments & Floor Division",
  "syntax.compound_desc":
    "Statements +=, -=, *=, /=, //=, %=, ^=, ..= with strict single evaluation on left-hand expressions, and // via __idiv:",
  "syntax.number_lit_title": "Number Literals & Digit Separators",
  "syntax.number_lit_desc":
    "Unified 64-bit IEEE754 double floats, supporting 0x hex, 0b binary, scientific notation, and underscore separators 1_000_000:",
  "syntax.string_escape_title": "Extended Escapes & Whitespace Trimming",
  "syntax.string_escape_desc":
    "Single/double quotes, raw literals, \\xHH hex bytes, \\u{...} Unicode code points, and \\z whitespace trimming:",
  "syntax.interp_title": "String Interpolation & Constraints",
  "syntax.interp_desc":
    "Backtick template strings evaluate embedded {expr} expressions, supporting multiline text and escapes:",
  "syntax.const_binding_title": "Const Bindings & Immutability (const)",
  "syntax.const_binding_desc":
    "const declares immutable variable bindings, combined with table.freeze to enforce read-only semantics:",
  "syntax.control_flow_title": "Control Flow, Loops & continue",
  "syntax.control_flow_desc":
    "Standard if-then-elseif-else, while, repeat..until, for loops, and the continue loop jump statement:",
  "syntax.ifexpr_title": "If-Else Expressions (Ternary Alternative)",
  "syntax.ifexpr_desc": "Expression-based conditional branch, requiring an else clause:",
  "syntax.iter_title": "Generalized Iteration & __iter",
  "syntax.iter_desc":
    "Direct for..in iteration over tables without pairs/ipairs, dense array insertion guarantees, and custom __iter metamethods:",
  "syntax.luau_types": "Geleidelijk Statisch Typesysteem",
  "syntax.type_modes_title": "Type Inference Modes & Directives",
  "syntax.type_modes_desc":
    "Script top comments including --!strict, --!nonstrict, --!nocheck, and --!native machine compilation directives:",
  "syntax.type_annot_title": "Basic Annotations & Top/Bottom Types",
  "syntax.type_annot_desc":
    "Explicit type hints covering number, string, boolean, nil, thread, buffer, vector, alongside any, unknown, and never:",
  "syntax.func_types_title": "Function Types & Parameter Hints",
  "syntax.func_types_desc":
    "Define signatures via (A, B) -> (R1, R2), supporting multiple returns, void (), and documentation parameter names:",
  "syntax.union_inter_title": "Union, Optional & Intersection Types",
  "syntax.union_inter_desc":
    "Express variants with |, optional types with ?, and merge table interfaces or define function overloads with &:",
  "syntax.casts_title": "Type Casts & Static Reflection typeof()",
  "syntax.casts_desc":
    "Safely assert types with the :: operator and query inferred expression structures at compile time with typeof():",
  "syntax.luau_tables": "Gestructureerde Tabellen & Typevernauwing",
  "syntax.tables_types_title": "Table Types & read/write Modifiers",
  "syntax.tables_types_desc":
    "Define structured table shapes, shorthand array types {T}, table indexers, and fine-grained read / write property modifiers:",
  "syntax.table_states_title": "Table States: Unsealed, Sealed & Width Subtyping",
  "syntax.table_states_desc":
    "Unsealed table literals dynamically expand properties, seal upon scope exit or annotation, and support width subtyping:",
  "syntax.refinements_title": "Tagged Unions & Type Refinements",
  "syntax.refinements_desc":
    "Discriminate variants with literal tags and refine types automatically inside control flow guards or assert():",
  "syntax.tables_freeze_title": "Table Freezing & Deep Immutability",
  "syntax.tables_freeze_desc":
    "Use table.freeze to lock tables against modification or key addition:",
  "syntax.luau_generics": "Generics & Typepakketten",
  "syntax.generics_title": "Type Aliases & Generic Defaults",
  "syntax.generics_desc":
    "Declare reusable aliases via type and export type with generic parameters and type parameter defaults:",
  "syntax.instantiate_title": "Generic Instantiation (<<T>>)",
  "syntax.instantiate_desc":
    "Explicitly pass type arguments at call sites using <<Type>> to resolve factory ambiguity and narrow singletons (type-erased at runtime):",
  "syntax.type_packs_title": "Type Packs (T...) & Variadics (...T)",
  "syntax.type_packs_desc":
    "Define variable-length type packs with T... for perfect forwarding, distinct from homogeneous variadics ...T:",
  "syntax.luau_host": "Modules, Klassen & Hostcontracten",
  "syntax.oop_title": "Typed Object-Oriented Metatables",
  "syntax.oop_desc":
    "Combine setmetatable and typeof to construct strongly-typed class structures with typed self receivers:",
  "syntax.modules_export_title": "Module Isolation & export Syntax",
  "syntax.modules_export_desc":
    "Type aliases are file-private by default; export type exposes types to require; export local/function exports values:",
  "syntax.declare_title": "External Declarations (Host & .d.luau)",
  "syntax.declare_desc":
    "declare global, declare function, and declare extern type for describing host runtime objects and class hierarchies:",
  "syntax.luau_native": "Moedertaal Typen & Prestatieversnelling",
  "syntax.vector_title": "Native Vector & SIMD Hardware Acceleration",
  "syntax.vector_desc":
    "Built-in 3D/4D vector type with SIMD arithmetic operations and component-wise division:",
  "syntax.buffer_title": "Native Contiguous Memory Buffers (buffer)",
  "syntax.buffer_desc":
    "Contiguous memory buffer library supporting numeric types, string serialization, and buffer copies:",
  "syntax.attr_title": "Function Attributes (@native, @deprecated)",
  "syntax.attr_desc":
    "Built-in directives: non-recursive @native machine compilation, @checked runtime assertion, and parameterized @[deprecated {...}]:",
  "syntax.type_func_title": "Compile-Time Type Functions (type function)",
  "syntax.type_func_desc":
    "Metaprogramming functions executed during static analysis using the types library to inspect, transform, and generate types:",
  "embed.title": "Embed in Rust",
  "embed.desc":
    '<code>ulua-rt</code> provides an API matching <a href="https://github.com/mlua-rs/mlua" target="_blank" rel="noopener">mlua</a> to expose Rust functions and types to Luau. Panics and errors convert into catchable Lua errors. Pure Rust with no C/FFI dependencies, supporting native and WebAssembly.',
  "checker.title": "Native Static Type Checker",
  "checker.desc":
    "Provides a static type checker to validate scripts against host declarations before execution:",
  "crates.title": "Crate Architecture",
  "crates.desc": "ulua is published as modular crates, allowing selective dependencies.",
  "crates.ulua":
    "Entry-point crate providing mlua-style API and top-level <code>compile</code>/<code>eval</code>/<code>check</code> functions, re-exporting submodules",
  "crates.ulua-rt":
    "<strong>mlua-style runtime bindings</strong>, featuring <code>Lua</code>, <code>create_function</code>, <code>UserData</code>, and <code>FromLua</code>/<code>IntoLua</code>",
  "crates.ulua-common":
    "Foundational primitives: <code>SmallVector</code>, <code>DenseHashMap</code>, <code>Variant</code>, FastFlags switches",
  "crates.ulua-ast": "Lexer, parser, and Abstract Syntax Tree (AST) definitions",
  "crates.ulua-bytecode": "Bytecode instruction specifications, serializer, and builder",
  "crates.ulua-compiler": "Source-to-bytecode optimizing compiler",
  "crates.ulua-codegen": "Native machine code generation backend (A64 / X64)",
  "crates.ulua-vm": "Register-based virtual machine and standard libraries",
  "crates.ulua-analysis": "Bidirectional static type checker and type inference engine",
  "crates.ulua-config": "<code>.uluac</code> configuration parser",
  "crates.ulua-require": "String-based require dependency module resolver",
  "crates.ulua-web": "<code>wasm32</code> bindings — execute and typecheck Luau in the browser",
  "crates.foot":
    "Command-line utilities: <code>ulua-repl-cli</code> (interactive REPL), <code>ulua-analyze-cli</code> (type checker), and compilation tools.",
  "foot.license":
    "MIT License. ulua is a rewrite of Luau (© Roblox Corporation), which derives from Lua (© Lua.org, PUC-Rio); upstream copyrights fully preserved.",
  "foot.commit": "Demos and compilers execute inside the browser WebAssembly environment.",
  "foot.source": "Source code & issues:",
  "example.hello": "Hello, world (Basic syntax)",
  "example.fibonacci": "Fibonacci (Recursion & loop)",
  "example.tables": "Tables (Arrays & dicts)",
  "example.metatables": "Metatables (OOP & operators)",
  "example.strings": "Strings (Standard string library)",
  "example.coroutines": "Coroutines (Generator cooperative)",
  "example.typed": "Gradual Typing (Static type hints)",
  "example.globals": "Inspect _G (Global environment)",
  "example.type_error": "Type Error (Trigger the checker!)",
  "status.ready": "Gereed",
  "status.running": "Wordt uitgevoerd…",
  "status.checking": "Types controleren…",
  "status.loading_wasm": "Loading wasm…",
  "status.wasm_failed": "wasm load failed",
  "status.runtime_error": "runtime error",
  "status.error": "Error",
  "status.no_errors": "No errors",
  "status.ran_ok": "Succesvol voltooid",
  "status.errors": "error(s)",
  "out.no_errors": "Geen fouten. Typecontrole geslaagd!",
  "out.no_output": "(Geen uitvoer)",
  "out.wasm_failed": "Failed to load WebAssembly engine.\n",
};
