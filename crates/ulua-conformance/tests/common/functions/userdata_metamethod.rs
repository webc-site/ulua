use ulua_code_gen::{
  enums::{host_metamethod::HostMetamethod, ir_cmd::IrCmd},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::common::{
  enums::userdata_kind::UserdataKind, records::vec_2_conformance_ir_hooks::Vec2,
};

pub fn userdata_metamethod(
  build: &mut IrBuilder,
  lhs_ty: u8,
  rhs_ty: u8,
  result_reg: i32,
  lhs: IrOp,
  rhs: IrOp,
  method: HostMetamethod,
  pcpos: i32,
) -> bool {
  match method {
    HostMetamethod::Add | HostMetamethod::Mul => {
      if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2)
        && UserdataKind::from_type(rhs_ty) == Some(UserdataKind::Vec2)
      {
        let vm_exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(lhs, LuaType::UserData as u8, vm_exit);
        let vm_exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(rhs, LuaType::UserData as u8, vm_exit);

        let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, lhs);
        Vec2::check_tag(build, udata1, pcpos);

        let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, rhs);
        Vec2::check_tag(build, udata2, pcpos);

        let x1 = Vec2::read_field(build, udata1, Vec2::OFFSET_X);
        let x2 = Vec2::read_field(build, udata2, Vec2::OFFSET_X);

        let cmd = if method == HostMetamethod::Add {
          IrCmd::AddFloat
        } else {
          IrCmd::MulFloat
        };
        let mx = build.inst_ir_cmd_ir_op_ir_op(cmd, x1, x2);

        let y1 = Vec2::read_field(build, udata1, Vec2::OFFSET_Y);
        let y2 = Vec2::read_field(build, udata2, Vec2::OFFSET_Y);
        let my = build.inst_ir_cmd_ir_op_ir_op(cmd, y1, y2);

        let udatar = Vec2::new_userdata(build);
        Vec2::write_field(build, udatar, Vec2::OFFSET_X, mx);
        Vec2::write_field(build, udatar, Vec2::OFFSET_Y, my);

        let vm_reg = build.vm_reg(result_reg as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, vm_reg, udatar);
        let tag_udata = build.const_tag(LuaType::UserData as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg, tag_udata);

        true
      } else {
        false
      }
    }
    HostMetamethod::Minus if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2) => {
      let vm_exit = build.vm_exit(pcpos as u32);
      build.load_and_check_tag(lhs, LuaType::UserData as u8, vm_exit);

      let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, lhs);
      Vec2::check_tag(build, udata1, pcpos);

      let x = Vec2::read_field(build, udata1, Vec2::OFFSET_X);
      let y = Vec2::read_field(build, udata1, Vec2::OFFSET_Y);

      let mx = build.inst_ir_cmd_ir_op(IrCmd::UnmFloat, x);
      let my = build.inst_ir_cmd_ir_op(IrCmd::UnmFloat, y);

      let udatar = Vec2::new_userdata(build);
      Vec2::write_field(build, udatar, Vec2::OFFSET_X, mx);
      Vec2::write_field(build, udatar, Vec2::OFFSET_Y, my);

      let vm_reg = build.vm_reg(result_reg as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, vm_reg, udatar);
      let tag_udata = build.const_tag(LuaType::UserData as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg, tag_udata);

      true
    }
    _ => false,
  }
}
