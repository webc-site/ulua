use ulua_code_gen::{enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder};
use ulua_vm::enums::lua_type::LuaType;

pub fn vector_access(
  build: &mut IrBuilder,
  member: &str,
  result_reg: i32,
  source_reg: i32,
  _pcpos: i32,
) -> bool {
  match member {
    "Magnitude" => {
      let vm_reg = build.vm_reg(source_reg as u8);
      let c0 = build.const_int(0);
      let c4 = build.const_int(4);
      let c8 = build.const_int(8);
      let x = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, vm_reg, c0);
      let y = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, vm_reg, c4);
      let z = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, vm_reg, c8);

      let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, x);
      let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, y);
      let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z, z);

      let sum_xy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, x2, y2);
      let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, sum_xy, z2);

      let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
      let mag_num = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, mag);

      let res_reg = build.vm_reg(result_reg as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, res_reg, mag_num);
      let tag = build.const_tag(LuaType::Number as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, res_reg, tag);

      true
    }
    "Unit" => {
      let vm_reg = build.vm_reg(source_reg as u8);
      let c0 = build.const_int(0);
      let c4 = build.const_int(4);
      let c8 = build.const_int(8);
      let x = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, vm_reg, c0);
      let y = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, vm_reg, c4);
      let z = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, vm_reg, c8);

      let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, x);
      let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, y);
      let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z, z);

      let sum_xy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, x2, y2);
      let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, sum_xy, z2);

      let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
      let one = build.const_double(1.0);
      let inv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivFloat, one, mag);

      let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, inv);
      let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, inv);
      let zr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z, inv);

      let res_reg = build.vm_reg(result_reg as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, res_reg, xr, yr, zr);
      let tag = build.const_tag(LuaType::Vector as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, res_reg, tag);

      true
    }
    _ => false,
  }
}
