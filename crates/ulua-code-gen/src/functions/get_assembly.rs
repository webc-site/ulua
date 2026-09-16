//! @interface-stub
use alloc::{string::String, vec::Vec};
use core::ptr::null_mut;

use ulua_vm::{
  functions::{lua_a_toobject::luaA_toobject, lua_is_lfunction::lua_is_lfunction},
  records::lua_state::lua_State,
};

use crate::{
  enums::{abix_64::ABIX64, features_a_64::FeaturesA64, target::Target},
  functions::{
    get_assembly_impl::{get_assembly_impl_a_64, get_assembly_impl_x_64},
    get_cpu_features_a_64::get_cpu_features_a_64,
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, lowering_stats::LoweringStats,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_assembly(
  l: *mut lua_State,
  idx: i32,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> String {
  unsafe {
    debug_assert!(lua_is_lfunction(l, idx) != 0);
    let func = luaA_toobject(l, idx);

    match options.target {
      Target::Host => {
        #[cfg(target_arch = "aarch64")]
        {
          let cpu_features = get_cpu_features_a_64();
          let mut build = AssemblyBuilderA64 {
            data: Vec::new(),
            code: Vec::new(),
            text: String::new(),
            log_text: false,
            features: 0,
            next_label: 1,
            pending_labels: Vec::new(),
            label_locations: Vec::new(),
            finalized: false,
            overflowed: false,
            data_pos: 0,
            code_pos: null_mut(),
            code_end: null_mut(),
          };
          build.assembly_builder_a_64_assembly_builder_a_64(options.include_assembly, cpu_features);

          get_assembly_impl_a_64(&mut build, func, options, stats)
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
          let cpu_features = crate::functions::get_cpu_features_x_64::get_cpu_features_x_64();
          let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(
            options.include_assembly,
            cpu_features,
          );

          get_assembly_impl_x_64(&mut build, func, options, stats)
        }
      }

      Target::A64 => {
        let mut build = AssemblyBuilderA64 {
          data: Vec::new(),
          code: Vec::new(),
          text: String::new(),
          log_text: false,
          features: 0,
          next_label: 1,
          pending_labels: Vec::new(),
          label_locations: Vec::new(),
          finalized: false,
          overflowed: false,
          data_pos: 0,
          code_pos: null_mut(),
          code_end: null_mut(),
        };
        build.assembly_builder_a_64_assembly_builder_a_64(
          options.include_assembly,
          FeaturesA64::FeatureJscvt as u32,
        );

        get_assembly_impl_a_64(&mut build, func, options, stats)
      }

      Target::A64NoFeatures => {
        let mut build = AssemblyBuilderA64 {
          data: Vec::new(),
          code: Vec::new(),
          text: String::new(),
          log_text: false,
          features: 0,
          next_label: 1,
          pending_labels: Vec::new(),
          label_locations: Vec::new(),
          finalized: false,
          overflowed: false,
          data_pos: 0,
          code_pos: null_mut(),
          code_end: null_mut(),
        };
        build.assembly_builder_a_64_assembly_builder_a_64(options.include_assembly, 0);

        get_assembly_impl_a_64(&mut build, func, options, stats)
      }

      Target::X64Windows => {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
          options.include_assembly,
          ABIX64::WINDOWS,
          0,
        );

        get_assembly_impl_x_64(&mut build, func, options, stats)
      }

      Target::X64SystemV => {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
          options.include_assembly,
          ABIX64::SYSTEM_V,
          0,
        );

        get_assembly_impl_x_64(&mut build, func, options, stats)
      }
    }
  }
}
