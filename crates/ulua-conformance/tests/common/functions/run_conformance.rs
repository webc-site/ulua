use alloc::string::String;
use core::{
  ffi::{c_char, c_int, c_void},
  ptr::{null, null_mut},
};
use std::{
  env::var,
  ffi::{CStr, CString},
  fs::read,
};

use ulua_code_gen::{
  enums::{function_stats_flags::FunctionStatsFlags, target::Target},
  functions::{
    get_assembly::get_assembly, luau_codegen_compile::luau_codegen_compile,
    luau_codegen_create::luau_codegen_create, luau_codegen_supported::luau_codegen_supported,
  },
  records::{
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    lowering_stats::LoweringStats,
  },
};
use ulua_compiler::{
  functions::luau_compile::luau_compile, records::lua_compile_options::LuaCompileOptions,
};
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_c_validate::lua_c_validate, lua_debugtrace::lua_debugtrace, lua_isstring::lua_isstring,
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_register::lua_l_register,
    lua_l_sandbox::lua_l_sandbox, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_pushvalue::lua_pushvalue, lua_resume::lua_resume, lua_resumeerror::lua_resumeerror,
    lua_setfield::lua_setfield, luau_load::luau_load,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop, lua_tostring::lua_tostring},
  records::{lua_l_reg::LuaLReg, lua_state::lua_State},
};

use crate::common::{
  functions::{
    default_codegen_options::default_codegen_options,
    find_conformance_source_dir::find_conformance_source_dir,
    lua_collectgarbage::lua_collectgarbage, lua_loadstring::lua_loadstring,
    lua_silence::lua_silence,
  },
  type_aliases::state_ref::StateRef,
};
unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

pub static mut VERBOSE: bool = false;
pub static mut CODEGEN: bool = false;
pub static mut OPTIMIZATION_LEVEL: c_int = 1;

fn default_lua_compile_options() -> LuaCompileOptions {
  LuaCompileOptions {
    optimization_level: unsafe { OPTIMIZATION_LEVEL },
    debug_level: 1,
    type_info_level: 1,
    coverage_level: 0,
    vector_lib: null(),
    vector_ctor: null(),
    vector_type: null(),
    mutable_globals: null(),
    userdata_types: null(),
    libraries_with_known_members: null(),
    library_member_type_cb: None,
    library_member_constant_cb: None,
    disabled_builtins: null(),
  }
}

/// # Safety
/// 调用方须保证裸指针参数有效（C 侧调用契约）。
pub unsafe fn run_conformance(
  name: *const c_char,
  setup: Option<unsafe extern "C-unwind" fn(*mut lua_State)>,
  yield_fn: Option<unsafe extern "C-unwind" fn(*mut lua_State) -> bool>,
  initial_lua_state: *mut lua_State,
  options: *mut LuaCompileOptions,
  skip_codegen: bool,
  codegen_options: *mut CompilationOptions,
) -> StateRef {
  let name_str = unsafe { CStr::from_ptr(name) }
    .to_string_lossy()
    .into_owned();

  let mut path =
    var("LUAU_CONFORMANCE_SOURCE_DIR").unwrap_or_else(|_| find_conformance_source_dir());
  if path.is_empty() {
    path = "Client/Luau/tests/conformance".to_owned();
  }
  if !path.ends_with('/') {
    path.push('/');
  }
  path.push_str(&name_str);

  let source = read(&path).unwrap_or_else(|_| {
        panic!(
            "File {path} is not found. Make sure you run tests from the root or specify custom directory using LUAU_CONFORMANCE_SOURCE_DIR env variable"
        )
    });

  let initial_lua_state = if initial_lua_state.is_null() {
    lua_l_newstate()
  } else {
    initial_lua_state
  };

  let global_state = StateRef::new(initial_lua_state).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    if CODEGEN && !skip_codegen && luau_codegen_supported() != 0 {
      luau_codegen_create(l);
    }

    lua_l_openlibs(l);

    let mut funcs = vec![
      LuaLReg {
        name: c"collectgarbage".as_ptr(),
        func: Some(lua_collectgarbage),
      },
      LuaLReg {
        name: c"loadstring".as_ptr(),
        func: Some(lua_loadstring),
      },
    ];

    if !VERBOSE {
      funcs.push(LuaLReg {
        name: c"print".as_ptr(),
        func: Some(lua_silence),
      });
    }

    funcs.push(LuaLReg {
      name: null(),
      func: None,
    });

    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_l_register(l, null(), funcs.as_ptr());
    lua_pop(l, 1);

    if let Some(setup_fn) = setup {
      setup_fn(l);
    }

    lua_l_sandbox(l);
    lua_l_sandboxthread(l);

    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_setfield(l, -1, c"_G".as_ptr());

    let chunkname = CString::new(format!("={name_str}")).expect("chunk name contains nul");

    let mut local_options;
    let options = if options.is_null() {
      local_options = default_lua_compile_options();
      &mut local_options as *mut LuaCompileOptions
    } else {
      options
    };

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      options,
      &mut bytecode_size,
    );
    let load_result = luau_load(l, chunkname.as_ptr(), bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);

    let native_opts = if codegen_options.is_null() {
      default_codegen_options()
    } else {
      (*codegen_options).clone()
    };

    if load_result == 0 && CODEGEN && !skip_codegen && luau_codegen_supported() != 0 {
      luau_codegen_compile(l, -1);
    }

    if load_result == 0 && luau_codegen_supported() != 0 {
      let mut assembly_options = AssemblyOptions {
        target: Target::A64,
        compilation_options: native_opts.clone(),
        output_binary: false,
        include_assembly: true,
        include_ir: true,
        include_outlined_code: true,
        include_ir_types: true,
        include_ir_prefix: Default::default(),
        include_use_info: Default::default(),
        include_cfg_info: Default::default(),
        include_reg_flow_info: Default::default(),
        annotator: None,
        annotator_context: null_mut(),
      };
      let mut stats = LoweringStats {
        function_stats_flags: FunctionStatsFlags::FunctionStatsEnable as u32,
        ..Default::default()
      };
      let a64 = get_assembly(l, -1, assembly_options.clone(), &mut stats);
      assert!(!a64.is_empty());
      assert_eq!(stats.reg_alloc_errors, 0);
      assert_eq!(stats.lowering_errors, 0);

      assembly_options.target = Target::X64SystemV;
      let x64 = get_assembly(l, -1, assembly_options, &mut stats);
      assert!(!x64.is_empty());
      assert_eq!(stats.reg_alloc_errors, 0);
      assert_eq!(stats.lowering_errors, 0);
    }

    let mut status = if load_result == 0 {
      lua_resume(l, null_mut(), 0)
    } else {
      LuaStatus::ErrSyntax as c_int
    };

    while let Some(yield_fn) = yield_fn {
      if status != LuaStatus::Yield as c_int && status != LuaStatus::Break as c_int {
        break;
      }

      let resume_error = yield_fn(l);
      status = if resume_error {
        lua_resumeerror(l, null_mut())
      } else {
        lua_resume(l, null_mut(), 0)
      };
    }

    lua_c_validate(l);

    if status == 0 {
      assert!(lua_isstring(l, -1) != 0);
      let result = CStr::from_ptr(lua_tostring!(l, -1)).to_string_lossy();
      assert_eq!(result.as_ref(), "OK");
      lua_pop(l, 1);
    } else {
      let error = if status == LuaStatus::Yield as c_int {
        String::from("thread yielded unexpectedly")
      } else {
        CStr::from_ptr(lua_tostring!(l, -1))
          .to_string_lossy()
          .into_owned()
      };
      let trace = CStr::from_ptr(lua_debugtrace(l)).to_string_lossy();
      panic!("{error}\nstacktrace:\n{trace}");
    }
  }

  global_state
}
