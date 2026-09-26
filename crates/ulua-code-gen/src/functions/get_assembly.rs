use alloc::{string::String, vec::Vec};

use ulua_vm::{
  functions::{lua_a_toobject::lua_a_toobject, lua_is_lfunction::lua_is_lfunction},
  records::lua_state::LuaState,
};

use crate::{
  enums::{abix_64::ABIX64, features_a_64::FeaturesA64, target::Target},
  functions::get_assembly_impl::{get_assembly_impl_a_64, get_assembly_impl_x_64},
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, lowering_stats::LoweringStats,
  },
};

/// 把 `get_assembly` / `get_assembly_from_ir` 的文本模式（`output_binary=false`）
/// 输出还原为 `String`：该分支下字节来自构建器内的 `String`，恒为合法 UTF-8。
/// 机器码（`output_binary=true`）不得走此入口，调用方应直接消费 `Vec<u8>`。
pub fn assembly_text(output: Vec<u8>) -> String {
  String::from_utf8(output).expect("codegen 文本输出应为合法 UTF-8")
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_assembly(
  l: *mut LuaState,
  idx: i32,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Vec<u8> {
  // Safety: 契约保证 l 为存活 LuaState*、idx 为界内栈索引，lua_is_lfunction/lua_a_toobject
  // 依 Lua C-ABI 合法读取该栈位，func 为其 LClosure TValue 指针。
  let func = unsafe {
    debug_assert!(lua_is_lfunction(l, idx) != 0);
    lua_a_toobject(l, idx)
  };

  // 各分支仅做构建器装配（安全代码），impl 调用为原样转发的 (b) 类边界：
  // func/stats 依上述契约存活或为 null，逐点就地窄化 unsafe。
  match options.target {
    Target::Host => {
      #[cfg(target_arch = "aarch64")]
      {
        use crate::functions::get_cpu_features_a_64::get_cpu_features_a_64;

        let cpu_features = get_cpu_features_a_64();
        // 复用构造函数，消除与下方 A64/A64NoFeatures 分支重复的字面量初始化
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(
          options.include_assembly,
          cpu_features,
        );

        // Safety: build 为本函数局部可变借用，func/stats 原样转发（契约见函数头）。
        unsafe { get_assembly_impl_a_64(&mut build, func, options, stats) }
      }

      #[cfg(not(target_arch = "aarch64"))]
      {
        let cpu_features = crate::functions::get_cpu_features_x_64::get_cpu_features_x_64();
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(
          options.include_assembly,
          cpu_features,
        );

        // Safety: 同上——局部 build，func/stats 依契约转发。
        unsafe { get_assembly_impl_x_64(&mut build, func, options, stats) }
      }
    }

    Target::A64 => {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(
        options.include_assembly,
        FeaturesA64::FeatureJscvt as u32,
      );

      // Safety: 同上——局部 build，func/stats 依契约转发。
      unsafe { get_assembly_impl_a_64(&mut build, func, options, stats) }
    }

    Target::A64NoFeatures => {
      let mut build =
        AssemblyBuilderA64::assembly_builder_a_64_bool_i32(options.include_assembly, 0);

      // Safety: 同上——局部 build，func/stats 依契约转发。
      unsafe { get_assembly_impl_a_64(&mut build, func, options, stats) }
    }

    Target::X64Windows => {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
        options.include_assembly,
        ABIX64::WINDOWS,
        0,
      );

      // Safety: 同上——局部 build，func/stats 依契约转发。
      unsafe { get_assembly_impl_x_64(&mut build, func, options, stats) }
    }

    Target::X64SystemV => {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
        options.include_assembly,
        ABIX64::SYSTEM_V,
        0,
      );

      // Safety: 同上——局部 build，func/stats 依契约转发。
      unsafe { get_assembly_impl_x_64(&mut build, func, options, stats) }
    }
  }
}
