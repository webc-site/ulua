use core::slice::from_raw_parts;
use std::fs;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_code_gen::{
  functions::{
    assemble_helpers_x_64::assemble_helpers, create_native_function::create_native_function_x_64,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, compilation_options::CompilationOptions,
    module::ModuleHelpers,
  },
};
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{
  functions::{
    lua_a_toobject::lua_a_toobject, lua_close::lua_close, lua_l_newstate::lua_l_newstate,
    luau_load::luau_load,
  },
  records::proto::Proto,
};

#[test]
fn test_x64_codegen_no_unbound_labels_across_all_benchmarks() {
  let cases_dir = fs::read_dir("../../benchmarks/cases")
    .or_else(|_| fs::read_dir("benchmarks/cases"))
    .expect("read benchmarks/cases dir");

  let mut lua_files = Vec::new();
  for entry in cases_dir {
    let entry = entry.expect("valid entry");
    let path = entry.path();
    if path.extension().and_then(|s| s.to_str()) == Some("lua") {
      lua_files.push(path);
    }
  }
  assert!(!lua_files.is_empty(), "must find benchmark cases");

  for lua_path in &lua_files {
    let src = fs::read_to_string(lua_path)
      .unwrap_or_else(|e| panic!("failed to read {}: {e}", lua_path.display()));

    let bytecode = compile(
      src.as_bytes(),
      &CompileOptions::default(),
      &ParseOptions::default(),
      NoopEncoder,
    );

    unsafe {
      let l = lua_l_newstate();
      assert!(!l.is_null());
      let chunkname = format!("={}", lua_path.file_name().unwrap().to_str().unwrap());
      let rc = luau_load(l, &chunkname, &bytecode, 0);
      assert_eq!(rc, 0, "luau_load failed for {}", lua_path.display());

      let cl = (*lua_a_toobject(&*l, -1)).as_closure();
      let root_proto = cl.inner.l.p;

      let mut all_protos = vec![root_proto];
      fn collect_protos(p: *mut Proto, list: &mut Vec<*mut Proto>) {
        unsafe {
          if (*p).sizep > 0 && !(*p).p.is_null() {
            let sub_protos = from_raw_parts((*p).p, (*p).sizep as usize);
            for &sp in sub_protos {
              list.push(sp);
              collect_protos(sp, list);
            }
          }
        }
      }
      collect_protos(root_proto, &mut all_protos);

      let mut build = AssemblyBuilderX64::new(true, 0);
      let mut helpers = ModuleHelpers::default();
      let options = CompilationOptions::default();
      let mut total_inst = 0u32;

      for (i, &proto) in all_protos.iter().enumerate() {
        let res =
          create_native_function_x_64(&mut build, &mut helpers, proto, &mut total_inst, &options);
        assert!(
          res.is_ok(),
          "compile proto {} failed for {}: {:?}",
          i,
          lua_path.display(),
          res.err()
        );
      }

      assemble_helpers(&mut build, &mut helpers);

      assert!(
        !build.text.contains(" .L0\n")
          && !build.text.contains(" .L0\t")
          && !build.text.contains(" .L0,")
          && !build.text.contains(",.L0")
          && !build.text.contains(".L0:"),
        "Detected unallocated/unbound label .L0 in generated x86_64 assembly for {}!\n{}",
        lua_path.display(),
        build.text
      );

      lua_close(l);
    }
  }
}
