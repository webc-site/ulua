use ulua_code_gen::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::common::{
  enums::userdata_kind::UserdataKind,
  records::{vec_2_conformance_ir_hooks::Vec2, vertex::Vertex},
};

fn store_number(build: &mut IrBuilder, result_reg: i32, value: IrOp) {
  let result = build.vm_reg(result_reg as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, result, value);
  let result = build.vm_reg(result_reg as u8);
  let tag = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, result, tag);
}

fn store_userdata(build: &mut IrBuilder, result_reg: i32, value: IrOp) {
  let result = build.vm_reg(result_reg as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, result, value);
  let result = build.vm_reg(result_reg as u8);
  let tag = build.const_tag(LuaType::UserData as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, result, tag);
}

fn store_vector(build: &mut IrBuilder, result_reg: i32, x: IrOp, y: IrOp, z: IrOp) {
  let result = build.vm_reg(result_reg as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, result, x, y, z);
  let result = build.vm_reg(result_reg as u8);
  let tag = build.const_tag(LuaType::Vector as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, result, tag);
}

pub fn userdata_access(
  build: &mut IrBuilder,
  r#type: u8,
  member: &str,
  result_reg: i32,
  source_reg: i32,
  pcpos: i32,
) -> bool {
  match UserdataKind::from_type(r#type) {
    Some(UserdataKind::Vec2) => match member {
      "X" | "Y" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata, pcpos);

        let field_offset = if member == "X" {
          Vec2::OFFSET_X
        } else {
          Vec2::OFFSET_Y
        };
        let value = Vec2::read_field(build, udata, field_offset);
        let value = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, value);
        store_number(build, result_reg, value);
        true
      }
      "Magnitude" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata, pcpos);

        let x = Vec2::read_field(build, udata, Vec2::OFFSET_X);
        let y = Vec2::read_field(build, udata, Vec2::OFFSET_Y);
        let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, x);
        let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, y);
        let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, x2, y2);
        let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
        let mag = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, mag);
        store_number(build, result_reg, mag);
        true
      }
      "Unit" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata, pcpos);

        let x = Vec2::read_field(build, udata, Vec2::OFFSET_X);
        let y = Vec2::read_field(build, udata, Vec2::OFFSET_Y);
        let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, x);
        let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, y);
        let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, x2, y2);
        let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
        let one = build.const_double(1.0);
        let inv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivFloat, one, mag);
        let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, inv);
        let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, inv);

        let result = Vec2::new_userdata(build);
        Vec2::write_field(build, result, Vec2::OFFSET_X, xr);
        Vec2::write_field(build, result, Vec2::OFFSET_Y, yr);
        store_userdata(build, result_reg, result);
        true
      }
      _ => false,
    },
    Some(UserdataKind::Vertex) => match member {
      "pos" | "normal" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vertex::check_tag(build, udata, pcpos);

        let (x_off, y_off, z_off) = if member == "pos" {
          (
            Vertex::OFFSET_POS_X,
            Vertex::OFFSET_POS_Y,
            Vertex::OFFSET_POS_Z,
          )
        } else {
          (
            Vertex::OFFSET_NORMAL_X,
            Vertex::OFFSET_NORMAL_Y,
            Vertex::OFFSET_NORMAL_Z,
          )
        };

        let x = Vec2::read_field(build, udata, x_off);
        let y = Vec2::read_field(build, udata, y_off);
        let z = Vec2::read_field(build, udata, z_off);
        store_vector(build, result_reg, x, y, z);
        true
      }
      "uv" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vertex::check_tag(build, udata, pcpos);

        let x = Vec2::read_field(build, udata, Vertex::OFFSET_UV_X);
        let y = Vec2::read_field(build, udata, Vertex::OFFSET_UV_Y);

        let result = Vec2::new_userdata(build);
        Vec2::write_field(build, result, Vec2::OFFSET_X, x);
        Vec2::write_field(build, result, Vec2::OFFSET_Y, y);
        store_userdata(build, result_reg, result);
        true
      }
      _ => false,
    },
    _ => false,
  }
}
