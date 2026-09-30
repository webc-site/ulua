// vector 库与 vector 常量用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_vector() {
  use ulua_common::fflag::LuauCompileNoFoldVectorEqW;
  use ulua_compiler::records::compile_options::CompileOptions;

  use crate::common::{
    functions::{
      codegen_ir_hook_callbacks::{
        vector_access_bytecode_type_callback, vector_access_callback,
        vector_namecall_bytecode_type_callback, vector_namecall_callback,
      },
      conformance_vector_setup::conformance_vector_setup,
      default_codegen_options::default_codegen_options,
      default_compile_options::default_compile_options,
      run_conformance::run_conformance,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // cpp `Conformance.test.cpp:1920-1923`：`ScopedFastFlag
  // luauCompileNoFoldVectorEqW{FFlag::LuauCompileNoFoldVectorEqW, true}`。
  // vector.luau 的「4th component is only visible to equality when vectors are 4-wide」
  // 用例要求优化级 2 下跳过「仅 w 不同」的两向量 `==`/`~=` 常量折叠，交给 3-wide
  // 运行时比较（luai_veceq 只比 x/y/z），否则折叠结果与运行时语义分歧。
  let _sff = ScopedFastFlag::new(&LuauCompileNoFoldVectorEqW, true);

  for use_ir_hooks in [false, true] {
    for optimization_level in 0..=2 {
      let copts = CompileOptions {
        optimization_level,
        ..default_compile_options()
      };
      let mut native_options = default_codegen_options();

      if use_ir_hooks {
        native_options.hooks.vector_access_bytecode_type =
          Some(vector_access_bytecode_type_callback);
        native_options.hooks.vector_namecall_bytecode_type =
          Some(vector_namecall_bytecode_type_callback);
        native_options.hooks.vector_access = Some(vector_access_callback);
        native_options.hooks.vector_namecall = Some(vector_namecall_callback);
      }

      // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
      run_conformance(
        "vector.luau",
        Some(conformance_vector_setup),
        None,
        None,
        Some(&copts),
        false,
        Some(&native_options),
      );
    }
  }
}

#[test]
fn conformance_vector_library() {
  use ulua_compiler::records::compile_options::CompileOptions;

  use crate::common::functions::{
    default_compile_options::default_compile_options, run_conformance::run_conformance,
    setup_native_helpers::setup_native_helpers,
  };

  for optimization_level in 0..=2 {
    let copts = CompileOptions {
      optimization_level,
      ..default_compile_options()
    };

    run_conformance(
      "vector_library.luau",
      Some(setup_native_helpers),
      None,
      None,
      Some(&copts),
      false,
      None,
    );
  }
}
