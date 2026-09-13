use alloc::{string::String, vec::Vec};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_vm_const_kind::BcVmConstKind,
  methods::bc_function_remap_local_pcs::bc_function_remap_local_pcs,
  records::{
    bytecode_builder::BytecodeBuilder,
    comp_time_bytecode_graph_serializer::CompTimeBytecodeGraphSerializer, string_ref::StringRef,
  },
  type_aliases::comp_time_bc_function::CompTimeBcFunction,
};

pub fn to_function_bytecode_bytecode_builder_comp_time_bc_function(
  bcb: &mut BytecodeBuilder,
  fn_: &mut CompTimeBcFunction,
) -> String {
  let function_id = bcb.begin_function(fn_.numparams, fn_.is_vararg);

  if !fn_.debugname.is_empty() {
    bcb.set_debug_function_name(StringRef::from(fn_.debugname.as_str()));
  }

  bcb.set_debug_function_line_defined(fn_.linedefined as i32);
  bcb.set_function_type_info(fn_.type_info.clone());

  for t in &fn_.upvalue_types {
    bcb.push_upval_type_info(*t);
  }

  for upval in &fn_.upvalue_names {
    bcb.push_debug_upval(StringRef::from(upval.as_str()));
  }

  let mut consts: Vec<u16> = Vec::with_capacity(fn_.constants.len());

  for c in &fn_.constants {
    match c.kind {
      BcVmConstKind::Nil => consts.push(bcb.add_constant_nil() as u16),
      BcVmConstKind::Boolean => {
        consts.push(bcb.add_constant_boolean(unsafe { c.value.value_boolean }) as u16)
      }
      BcVmConstKind::Number => {
        consts.push(bcb.add_constant_number(unsafe { c.value.value_number }) as u16)
      }
      BcVmConstKind::Vector => {
        let value = unsafe { c.value.value_vector };
        consts.push(bcb.add_constant_vector(value[0], value[1], value[2], value[3]) as u16);
      }
      BcVmConstKind::String => consts
        .push(bcb.add_constant_string(StringRef::from(unsafe { c.value.value_string })) as u16),
      BcVmConstKind::Import => consts.push(bcb.add_import(unsafe { c.value.value_import }) as u16),
      BcVmConstKind::Table => {
        let value_table = unsafe { c.value.value_table };
        LUAU_ASSERT!(value_table < fn_.table_shapes.len() as u32);
        consts.push(bcb.add_constant_table(&fn_.table_shapes[value_table as usize]) as u16);
      }
      BcVmConstKind::Closure => {
        consts.push(bcb.add_constant_closure(unsafe { c.value.value_closure }) as u16)
      }
      BcVmConstKind::Integer => {
        consts.push(bcb.add_constant_integer(unsafe { c.value.value_integer }) as u16)
      }
    }
  }

  for fid in &fn_.protos {
    bcb.add_child_function(*fid);
  }

  let mut serializer = CompTimeBytecodeGraphSerializer::comp_time_bytecode_graph_serializer_comp_time_bytecode_graph_serializer(bcb, fn_, &mut consts);
  let insns_pc = serializer.emit_bytecode();

  bc_function_remap_local_pcs(fn_, &insns_pc, bcb.get_debug_pc());

  for local in &fn_.local_types {
    bcb.push_local_type_info(local.r#type, local.reg, local.startpc, local.endpc);
  }

  for local in &fn_.locals {
    bcb.push_debug_local(
      StringRef::from(local.varname),
      local.reg,
      local.startpc,
      local.endpc,
    );
  }

  bcb.fold_jumps();
  bcb.expand_jumps();
  bcb.end_function(fn_.maxstacksize, fn_.nups, fn_.flags);

  if serializer.error() {
    return String::new();
  }

  bcb.get_function_data(function_id)
}
