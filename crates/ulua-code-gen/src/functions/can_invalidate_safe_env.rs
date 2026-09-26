use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn can_invalidate_safe_env(cmd: IrCmd) -> bool {
  match cmd {
        IrCmd::CmpAny
        | IrCmd::DoArith
        | IrCmd::DoLen
        | IrCmd::GetTable
        | IrCmd::SetTable
        | IrCmd::CONCAT // TODO: if only strings and numbers are concatenated, there will be no user calls
        | IrCmd::CALL
        | IrCmd::ForgloopFallback
        | IrCmd::FallbackGetglobal
        | IrCmd::FallbackSetglobal
        | IrCmd::FallbackGettableks
        | IrCmd::FallbackSettableks
        | IrCmd::FallbackNamecall
        | IrCmd::FallbackForgprep => true,
        _ => false,
    }
}
