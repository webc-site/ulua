// 待接入锚注（R69 跨 crate 审计立案）：本出口当前仅测试消费（ulua-unit-test
// fixture / 本 crate tests），生产管线（CLI/rt）尚未接线——属 [[vm-pure-rust]]
// 分阶段策略的在途面，勿按孤儿收窄；接线时连同 cpp 对应 TEST_CASE 一并启用。
use std::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  bc_function::BcFunction, bc_vm_const::BcVmConst, bytecode_builder::BytecodeBuilder,
  comp_time_bytecode_graph_serializer::CompTimeBytecodeGraphSerializer, string_ref::StringRef,
};

/// cpp `toFunctionBytecode(BcFunction&)` 重载：自持 builder 的一参入口，
/// 转调下方双参重载（`BytecodeGraph.cpp:322` 起的单一序列化源）。
pub fn to_function_bytecode_comp_time_bc_function(fn_: &mut BcFunction<'_>) -> Vec<u8> {
  let mut bcb = BytecodeBuilder::new(None);
  to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut bcb, fn_)
}

pub fn to_function_bytecode_bytecode_builder_comp_time_bc_function(
  bcb: &mut BytecodeBuilder<'_>,
  fn_: &mut BcFunction<'_>,
) -> Vec<u8> {
  let function_id = bcb.begin_function(fn_.numparams, fn_.is_vararg);

  if !fn_.debugname.is_empty() {
    // 视图字节来自 `fn_` 自持的 `String`，而本函数后续还要 `&mut fn_`；借用模型无法
    // 表达上游 `string_view` 的字段级别名（`BytecodeGraph.cpp:322`），故拷贝进 builder。
    bcb.set_debug_function_name(StringRef::owned(fn_.debugname.as_bytes().to_vec()));
  }

  bcb.set_debug_function_line_defined(fn_.linedefined as i32);
  bcb.set_function_type_info(fn_.type_info.clone());

  for t in &fn_.upvalue_types {
    bcb.push_upval_type_info(*t);
  }

  for upval in &fn_.upvalue_names {
    bcb.push_debug_upval(StringRef::owned(upval.as_bytes().to_vec()));
  }

  // cpp `std::vector<uint32_t> consts`：addConstant* 的 int32_t id 隐式转为 u32。
  let mut consts: Vec<u32> = Vec::with_capacity(fn_.constants.len());

  for c in &fn_.constants {
    match *c {
      BcVmConst::Nil => consts.push(bcb.add_constant_nil() as u32),
      BcVmConst::Boolean(value) => consts.push(bcb.add_constant_boolean(value) as u32),
      BcVmConst::Number(value) => consts.push(bcb.add_constant_number(value) as u32),
      BcVmConst::Vector([x, y, z, w]) => {
        consts.push(bcb.add_constant_vector(x, y, z, w) as u32);
      }
      // cpp `BytecodeGraph.cpp:351-353` + `BytecodeBuilder.cpp:406-424`
      BcVmConst::Vectord([x, y, z, w]) => {
        consts.push(bcb.add_constant_vector_d(x, y, z, w) as u32);
      }
      // `owned` 原因同 debugname：视图源在 `fn_` 的 `strings` 借用里，builder 的
      // 生命周期与 `'f` 无静态关系，拷贝一次换取所有权闭环（cpp 直接存 string_view）。
      BcVmConst::String(bytes) => {
        consts.push(bcb.add_constant_string(StringRef::owned(bytes.to_vec())) as u32)
      }
      BcVmConst::Import(iid) => consts.push(bcb.add_import(iid) as u32),
      BcVmConst::Table(shape_idx) => {
        LUAU_ASSERT!(shape_idx < fn_.table_shapes.len() as u32);
        consts.push(bcb.add_constant_table(&fn_.table_shapes[shape_idx as usize]) as u32);
      }
      BcVmConst::Closure(fid) => consts.push(bcb.add_constant_closure(fid) as u32),
      BcVmConst::Integer(value) => consts.push(bcb.add_constant_integer(value) as u32),
      // cpp: `cpp/Bytecode/src/BytecodeGraph.cpp:378-386`
      BcVmConst::ClassShape(shape_idx) => {
        consts.push(bcb.add_class_shape(fn_.class_shapes[shape_idx as usize].clone()) as u32)
      }
    }
  }

  for fid in &fn_.protos {
    bcb.add_child_function(*fid);
  }

  let mut serializer = CompTimeBytecodeGraphSerializer::new(bcb, fn_, consts);
  let insns_pc = serializer.emit_bytecode();
  // error 只在 emitBytecode 期间置位，先取出以结束 serializer 对 bcb/fn_ 的可变借用
  let serializer_error = serializer.error();

  fn_.remap_local_pcs(&insns_pc, bcb.get_debug_pc());

  for local in &fn_.local_types {
    bcb.push_local_type_info(local.r#type, local.reg, local.startpc, local.endpc);
  }

  for local in &fn_.locals {
    bcb.push_debug_local(
      StringRef::owned(local.varname.as_bytes().to_vec()),
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
