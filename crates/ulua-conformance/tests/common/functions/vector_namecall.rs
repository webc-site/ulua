use ulua_code_gen::{enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder};
use ulua_vm::{enums::lua_type::LuaType, macros::lua_multret::LUA_MULTRET};

pub fn vector_namecall(
  build: &mut IrBuilder,
  member: &str,
  arg_res_reg: i32,
  source_reg: i32,
  params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  if params != 2 || results > 1 {
    return false;
  }

  match member {
    "Dot" => {
      let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
      let exit = build.vm_exit(pcpos as u32);
      build.load_and_check_tag(arg_reg, LuaType::Vector as u8, exit);

      let src_reg = build.vm_reg(source_reg as u8);
      let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
      let c0 = build.const_int(0);
      let c4 = build.const_int(4);
      let c8 = build.const_int(8);

      let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c0);
      let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c0);
      let xx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x1, x2);

      let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c4);
      let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c4);
      let yy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y1, y2);

      let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c8);
      let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c8);
      let zz = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z1, z2);

      let sum_xy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, xx, yy);
      let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, sum_xy, zz);

      let res_reg = build.vm_reg(arg_res_reg as u8);
      let num = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, sum);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, res_reg, num);
      let tag = build.const_tag(LuaType::Number as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, res_reg, tag);

      if results == LUA_MULTRET {
        let adj_arg = build.vm_reg(arg_res_reg as u8);
        let adj_val = build.const_int(1);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, adj_arg, adj_val);
      }

      true
    }
    "Cross" => {
      let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
      let exit = build.vm_exit(pcpos as u32);
      build.load_and_check_tag(arg_reg, LuaType::Vector as u8, exit);

      let src_reg = build.vm_reg(source_reg as u8);
      let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
      let c0 = build.const_int(0);
      let c4 = build.const_int(4);
      let c8 = build.const_int(8);

      let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c0);
      let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c0);
      let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c4);
      let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c4);
      let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c8);
      let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c8);

      let y1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y1, z2);
      let z1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z1, y2);
      let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, y1z2, z1y2);

      let z1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z1, x2);
      let x1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x1, z2);
      let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, z1x2, x1z2);

      let x1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x1, y2);
      let y1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y1, x2);
      let zr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, x1y2, y1x2);

      let res_reg = build.vm_reg(arg_res_reg as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, res_reg, xr, yr, zr);
      let tag = build.const_tag(LuaType::Vector as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, res_reg, tag);

      if results == LUA_MULTRET {
        let adj_arg = build.vm_reg(arg_res_reg as u8);
        let adj_val = build.const_int(1);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, adj_arg, adj_val);
      }

      true
    }
    _ => false,
  }
}
