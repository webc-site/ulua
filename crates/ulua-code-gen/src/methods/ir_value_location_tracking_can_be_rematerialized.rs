use crate::{enums::ir_cmd::IrCmd, records::ir_value_location_tracking::IrValueLocationTracking};

impl IrValueLocationTracking {
  pub fn can_be_rematerialized(&self, cmd: IrCmd) -> bool {
    cmd == IrCmd::UintToNum || cmd == IrCmd::IntToNum
  }
}
