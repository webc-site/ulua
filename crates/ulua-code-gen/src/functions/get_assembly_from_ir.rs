use alloc::vec::Vec;

use crate::{
  enums::{abix_64::ABIX64, features_a_64::FeaturesA64, target::Target},
  functions::get_assembly_from_ir_impl::get_assembly_from_ir_impl,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, ir_builder::IrBuilder, lowering_stats::LoweringStats,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_assembly_from_ir(
  ir: &mut IrBuilder,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Vec<u8> {
  // ir 为安全 &mut 借用；stats 依 `# Safety` 契约为 null 或指向存活 LoweringStats 的可写
  // 指针。各分支仅做构建器装配（安全代码），impl 泛型骨架收口后统一转发 (b) 类边界。
  match options.target {
    Target::Host => {
      #[cfg(target_arch = "aarch64")]
      {
        use crate::functions::get_cpu_features_a_64::get_cpu_features_a_64;

        let cpu_features = get_cpu_features_a_64();
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(
          options.include_assembly,
          cpu_features,
        );

        // Safety: build 为本函数局部可变借用，ir/stats 原样转发（契约见函数头）。
        unsafe { get_assembly_from_ir_impl(&mut build, ir, options, stats) }
      }

      #[cfg(not(target_arch = "aarch64"))]
      {
        let cpu_features = crate::functions::get_cpu_features_x_64::get_cpu_features_x_64();
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(
          options.include_assembly,
          cpu_features,
        );

        // Safety: 同上——局部 build，ir/stats 依契约转发。
        unsafe { get_assembly_from_ir_impl(&mut build, ir, options, stats) }
      }
    }

    Target::A64 => {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(
        options.include_assembly,
        FeaturesA64::FeatureJscvt as u32,
      );

      // Safety: 同上——局部 build，ir/stats 依契约转发。
      unsafe { get_assembly_from_ir_impl(&mut build, ir, options, stats) }
    }

    Target::A64NoFeatures => {
      let mut build =
        AssemblyBuilderA64::assembly_builder_a_64_bool_i32(options.include_assembly, 0);

      // Safety: 同上——局部 build，ir/stats 依契约转发。
      unsafe { get_assembly_from_ir_impl(&mut build, ir, options, stats) }
    }

    Target::X64Windows => {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
        options.include_assembly,
        ABIX64::WINDOWS,
        0,
      );

      // Safety: 同上——局部 build，ir/stats 依契约转发。
      unsafe { get_assembly_from_ir_impl(&mut build, ir, options, stats) }
    }

    Target::X64SystemV => {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
        options.include_assembly,
        ABIX64::SYSTEM_V,
        0,
      );

      // Safety: 同上——局部 build，ir/stats 依契约转发。
      unsafe { get_assembly_from_ir_impl(&mut build, ir, options, stats) }
    }
  }
}
