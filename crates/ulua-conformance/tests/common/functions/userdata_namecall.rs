use ulua_code_gen::{enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder};
use ulua_vm::{enums::lua_type::LuaType, macros::lua_multret::LUA_MULTRET};

use crate::common::{
  enums::userdata_kind::UserdataKind, records::vec_2_conformance_ir_hooks::Vec2,
};

fn adjust_multret(build: &mut IrBuilder, arg_res_reg: i32, results: i32) {
  if results == LUA_MULTRET {
    let reg = build.vm_reg(arg_res_reg as u8);
    let count = build.const_int(1);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, reg, count);
  }
}

pub fn userdata_namecall(
  build: &mut IrBuilder,
  r#type: u8,
  member: &str,
  arg_res_reg: i32,
  source_reg: i32,
  _params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  match UserdataKind::from_type(r#type) {
    Some(UserdataKind::Vec2) => match member {
      "Dot" => {
        let source = build.vm_reg(source_reg as u8);
        let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata1, pcpos);

        let arg = build.vm_reg((arg_res_reg + 2) as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(arg, LuaType::UserData as u8, exit);

        let arg = build.vm_reg((arg_res_reg + 2) as u8);
        let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, arg);
        Vec2::check_tag(build, udata2, pcpos);

        let mut x1 = Vec2::read_field(build, udata1, Vec2::OFFSET_X);
        let mut x2 = Vec2::read_field(build, udata2, Vec2::OFFSET_X);
        x1 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x1);
        x2 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x2);
        let xx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, x1, x2);

        let mut y1 = Vec2::read_field(build, udata1, Vec2::OFFSET_Y);
        let mut y2 = Vec2::read_field(build, udata2, Vec2::OFFSET_Y);
        y1 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, y1);
        y2 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, y2);
        let yy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, y1, y2);
        let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, xx, yy);

        let result = build.vm_reg(arg_res_reg as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, result, sum);
        let result = build.vm_reg(arg_res_reg as u8);
        let tag = build.const_tag(LuaType::Number as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, result, tag);
        adjust_multret(build, arg_res_reg, results);
        true
      }
      "Min" => {
        let source = build.vm_reg(source_reg as u8);
        let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata1, pcpos);

        let arg = build.vm_reg((arg_res_reg + 2) as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(arg, LuaType::UserData as u8, exit);

        let arg = build.vm_reg((arg_res_reg + 2) as u8);
        let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, arg);
        Vec2::check_tag(build, udata2, pcpos);

        let mut x1 = Vec2::read_field(build, udata1, Vec2::OFFSET_X);
        let mut x2 = Vec2::read_field(build, udata2, Vec2::OFFSET_X);
        x1 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x1);
        x2 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x2);
        let mx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinNum, x1, x2);

        let mut y1 = Vec2::read_field(build, udata1, Vec2::OFFSET_Y);
        let mut y2 = Vec2::read_field(build, udata2, Vec2::OFFSET_Y);
        y1 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, y1);
        y2 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, y2);
        let my = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinNum, y1, y2);

        let mx = build.inst_ir_cmd_ir_op(IrCmd::NumToFloat, mx);
        let my = build.inst_ir_cmd_ir_op(IrCmd::NumToFloat, my);

        let udata_result = Vec2::new_userdata(build);
        Vec2::write_field(build, udata_result, Vec2::OFFSET_X, mx);
        Vec2::write_field(build, udata_result, Vec2::OFFSET_Y, my);

        let result = build.vm_reg(arg_res_reg as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, result, udata_result);
        let result = build.vm_reg(arg_res_reg as u8);
        let tag = build.const_tag(LuaType::UserData as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, result, tag);
        adjust_multret(build, arg_res_reg, results);
        true
      }
      _ => false,
    },
    _ => false,
  }
}
