use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_vm_const_kind::BcVmConstKind,
  methods::bc_function_remap_local_pcs::bc_function_remap_local_pcs,
  records::{
    bc_function::BcFunction, bytecode_builder::BytecodeBuilder,
    comp_time_bytecode_graph_serializer::CompTimeBytecodeGraphSerializer, string_ref::StringRef,
  },
};

pub fn to_function_bytecode_bytecode_builder_comp_time_bc_function(
  bcb: &mut BytecodeBuilder,
  fn_: &mut BcFunction,
) -> Vec<u8> {
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

  // cpp `std::vector<uint32_t> consts`：addConstant* 的 int32_t id 隐式转为 u32。
  let mut consts: Vec<u32> = Vec::with_capacity(fn_.constants.len());

  for c in &fn_.constants {
    match c.kind {
      BcVmConstKind::Nil => consts.push(bcb.add_constant_nil() as u32),
      BcVmConstKind::Boolean => {
        consts.push(bcb.add_constant_boolean(unsafe { c.value.value_boolean }) as u32)
      }
      BcVmConstKind::Number => {
        consts.push(bcb.add_constant_number(unsafe { c.value.value_number }) as u32)
      }
      BcVmConstKind::Vector => {
        let value = unsafe { c.value.value_vector };
        consts.push(bcb.add_constant_vector(value[0], value[1], value[2], value[3]) as u32);
      }
      BcVmConstKind::String => consts
        .push(bcb.add_constant_string(StringRef::from(unsafe { c.value.value_string })) as u32),
      BcVmConstKind::Import => consts.push(bcb.add_import(unsafe { c.value.value_import }) as u32),
      BcVmConstKind::Table => {
        let value_table = unsafe { c.value.value_table };
        LUAU_ASSERT!(value_table < fn_.table_shapes.len() as u32);
        consts.push(bcb.add_constant_table(&fn_.table_shapes[value_table as usize]) as u32);
      }
      BcVmConstKind::Closure => {
        consts.push(bcb.add_constant_closure(unsafe { c.value.value_closure }) as u32)
      }
      BcVmConstKind::Integer => {
        consts.push(bcb.add_constant_integer(unsafe { c.value.value_integer }) as u32)
      }
      // cpp: `cpp/Bytecode/src/BytecodeGraph.cpp:378-386`
      BcVmConstKind::ClassShape => consts.push(
        bcb.add_class_shape(fn_.class_shapes[unsafe { c.value.value_class_shape } as usize].clone())
          as u32,
      ),
    }
  }

  for fid in &fn_.protos {
    bcb.add_child_function(*fid);
  }

  let mut serializer = CompTimeBytecodeGraphSerializer::new(bcb, fn_, consts);
  let insns_pc = serializer.emit_bytecode();
  // error 只在 emitBytecode 期间置位，先取出以结束 serializer 对 bcb/fn_ 的可变借用
  let serializer_error = serializer.error();

  bc_function_remap_local_pcs(fn_, &insns_pc, bcb.get_debug_pc());

  for local in &fn_.local_types {
    bcb.push_local_type_info(local.r#type, local.reg, local.startpc, local.endpc);
  }

  for local in &fn_.locals {
    bcb.push_debug_local(
      StringRef::from(local.varname.as_str()),
      local.reg,
      local.startpc,
      local.endpc,
    );
  }

  bcb.fold_jumps();
  // cpp BytecodeGraph.cpp:413-416：长跳转放不进 JUMPX 时放弃序列化
  if bcb.expand_jumps() {
    return Vec::new();
  }
  bcb.end_function(fn_.maxstacksize, fn_.nups, fn_.flags, 0);

  if serializer_error {
    return Vec::new();
  }

  bcb.get_function_data(function_id)
}
