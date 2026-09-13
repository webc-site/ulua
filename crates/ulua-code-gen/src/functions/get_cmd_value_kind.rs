use crate::enums::{ir_cmd::IrCmd, ir_value_kind::IrValueKind};

pub fn get_cmd_value_kind(cmd: IrCmd) -> IrValueKind {
  match cmd {
    IrCmd::NOP => IrValueKind::None,
    IrCmd::LoadTag => IrValueKind::Tag,
    IrCmd::LoadPointer => IrValueKind::Pointer,
    IrCmd::LoadDouble => IrValueKind::Double,
    IrCmd::LoadInt => IrValueKind::Int,
    IrCmd::LoadFloat => IrValueKind::Float,
    IrCmd::LoadTvalue => IrValueKind::Tvalue,
    IrCmd::LoadEnv
    | IrCmd::GetArrAddr
    | IrCmd::GetSlotNodeAddr
    | IrCmd::GetHashNodeAddr
    | IrCmd::GetClosureUpvalAddr => IrValueKind::Pointer,
    IrCmd::StoreTag
    | IrCmd::StoreExtra
    | IrCmd::StorePointer
    | IrCmd::StoreDouble
    | IrCmd::StoreInt
    | IrCmd::StoreInt64
    | IrCmd::StoreVector
    | IrCmd::StoreTvalue
    | IrCmd::StoreSplitTvalue
    | IrCmd::CheckDivInt64 => IrValueKind::None,
    IrCmd::LoadInt64
    | IrCmd::AddInt64
    | IrCmd::SubInt64
    | IrCmd::MulInt64
    | IrCmd::DivInt64
    | IrCmd::IdivInt64
    | IrCmd::UdivInt64
    | IrCmd::RemInt64
    | IrCmd::UremInt64
    | IrCmd::ModInt64
    | IrCmd::SelectInt64
    | IrCmd::BitandInt64
    | IrCmd::BitxorInt64
    | IrCmd::BitorInt64
    | IrCmd::BitnotInt64
    | IrCmd::BitlshiftInt64
    | IrCmd::BitrshiftInt64
    | IrCmd::BitarshiftInt64
    | IrCmd::BitlrotateInt64
    | IrCmd::BitrrotateInt64
    | IrCmd::BitcountlzInt64
    | IrCmd::BitcountrzInt64
    | IrCmd::ByteswapInt64 => IrValueKind::Int64,
    IrCmd::AddInt | IrCmd::SubInt | IrCmd::Sexti8Int | IrCmd::Sexti16Int => IrValueKind::Int,
    IrCmd::AddNum
    | IrCmd::SubNum
    | IrCmd::MulNum
    | IrCmd::DivNum
    | IrCmd::IdivNum
    | IrCmd::ModNum
    | IrCmd::MinNum
    | IrCmd::MaxNum
    | IrCmd::UnmNum
    | IrCmd::FloorNum
    | IrCmd::CeilNum
    | IrCmd::RoundNum
    | IrCmd::SqrtNum
    | IrCmd::AbsNum
    | IrCmd::SignNum
    | IrCmd::SelectNum
    | IrCmd::MuladdNum => IrValueKind::Double,
    IrCmd::AddFloat
    | IrCmd::SubFloat
    | IrCmd::MulFloat
    | IrCmd::DivFloat
    | IrCmd::MinFloat
    | IrCmd::MaxFloat
    | IrCmd::UnmFloat
    | IrCmd::FloorFloat
    | IrCmd::CeilFloat
    | IrCmd::SqrtFloat
    | IrCmd::AbsFloat
    | IrCmd::SignFloat => IrValueKind::Float,
    IrCmd::AddVec
    | IrCmd::SubVec
    | IrCmd::MulVec
    | IrCmd::DivVec
    | IrCmd::IdivVec
    | IrCmd::UnmVec
    | IrCmd::MinVec
    | IrCmd::MaxVec
    | IrCmd::FloorVec
    | IrCmd::CeilVec
    | IrCmd::AbsVec
    | IrCmd::SelectVec
    | IrCmd::SelectIfTruthy
    | IrCmd::MuladdVec => IrValueKind::Tvalue,
    IrCmd::DotVec | IrCmd::ExtractVec => IrValueKind::Float,
    IrCmd::NotAny
    | IrCmd::CmpAny
    | IrCmd::CmpInt
    | IrCmd::CmpInt64
    | IrCmd::CmpTag
    | IrCmd::CmpSplitTvalue => IrValueKind::Int,
    IrCmd::JUMP
    | IrCmd::JumpIfTruthy
    | IrCmd::JumpIfFalsy
    | IrCmd::JumpEqTag
    | IrCmd::JumpCmpInt
    | IrCmd::JumpEqPointer
    | IrCmd::JumpCmpNum
    | IrCmd::JumpCmpFloat
    | IrCmd::JumpFornLoopCond
    | IrCmd::JumpSlotMatch => IrValueKind::None,
    IrCmd::TableLen => IrValueKind::Int,
    IrCmd::TableSetnum => IrValueKind::Pointer,
    IrCmd::StringLen => IrValueKind::Int,
    IrCmd::NewTable | IrCmd::DupTable => IrValueKind::Pointer,
    IrCmd::TryNumToIndex => IrValueKind::Int,
    IrCmd::TryCallFastgettm | IrCmd::NewUserdata => IrValueKind::Pointer,
    IrCmd::Int64ToNum | IrCmd::IntToNum | IrCmd::UintToNum => IrValueKind::Double,
    IrCmd::UintToFloat => IrValueKind::Float,
    IrCmd::NumToInt | IrCmd::NumToUint => IrValueKind::Int,
    IrCmd::NumToInt64 => IrValueKind::Int64,
    IrCmd::FloatToNum => IrValueKind::Double,
    IrCmd::NumToFloat => IrValueKind::Float,
    IrCmd::FloatToVec | IrCmd::TagVector => IrValueKind::Tvalue,
    IrCmd::TruncateUint => IrValueKind::Int,
    IrCmd::AdjustStackToReg | IrCmd::AdjustStackToTop | IrCmd::FASTCALL => IrValueKind::None,
    IrCmd::InvokeFastcall => IrValueKind::Int,
    IrCmd::CheckFastcallRes
    | IrCmd::DoArith
    | IrCmd::DoLen
    | IrCmd::GetTable
    | IrCmd::SetTable
    | IrCmd::GetCachedImport
    | IrCmd::CONCAT => IrValueKind::None,
    IrCmd::GetUpvalue => IrValueKind::Tvalue,
    IrCmd::SetUpvalue
    | IrCmd::CheckTag
    | IrCmd::CheckTruthy
    | IrCmd::CheckReadonly
    | IrCmd::CheckNoMetatable
    | IrCmd::CheckSafeEnv
    | IrCmd::CheckArraySize
    | IrCmd::CheckSlotMatch
    | IrCmd::CheckNodeNoNext
    | IrCmd::CheckNodeValue
    | IrCmd::CheckBufferLen
    | IrCmd::CheckUserdataTag
    | IrCmd::CheckCmpNum
    | IrCmd::CheckCmpInt
    | IrCmd::CheckCmpInt64
    | IrCmd::INTERRUPT
    | IrCmd::CheckGc
    | IrCmd::BarrierObj
    | IrCmd::BarrierTableBack
    | IrCmd::BarrierTableForward
    | IrCmd::SetSavedpc
    | IrCmd::CloseUpvals
    | IrCmd::CAPTURE
    | IrCmd::SETLIST
    | IrCmd::CALL
    | IrCmd::RETURN
    | IrCmd::FORGLOOP
    | IrCmd::ForgloopFallback
    | IrCmd::ForgprepXnextFallback
    | IrCmd::COVERAGE
    | IrCmd::FallbackGetglobal
    | IrCmd::FallbackSetglobal
    | IrCmd::FallbackGettableks
    | IrCmd::FallbackSettableks
    | IrCmd::FallbackNamecall
    | IrCmd::FallbackPrepvarargs
    | IrCmd::FallbackGetvarargs => IrValueKind::None,
    IrCmd::NEWCLOSURE => IrValueKind::Pointer,
    IrCmd::FallbackDupclosure | IrCmd::FallbackForgprep => IrValueKind::None,
    IrCmd::SUBSTITUTE => IrValueKind::Unknown,
    IrCmd::MarkUsed | IrCmd::MarkDead => IrValueKind::None,
    IrCmd::BitandUint
    | IrCmd::BitxorUint
    | IrCmd::BitorUint
    | IrCmd::BitnotUint
    | IrCmd::BitlshiftUint
    | IrCmd::BitrshiftUint
    | IrCmd::BitarshiftUint
    | IrCmd::BitlrotateUint
    | IrCmd::BitrrotateUint
    | IrCmd::BitcountlzUint
    | IrCmd::BitcountrzUint
    | IrCmd::ByteswapUint => IrValueKind::Int,
    IrCmd::InvokeLibm => IrValueKind::Double,
    IrCmd::GetType | IrCmd::GetTypeof | IrCmd::FINDUPVAL => IrValueKind::Pointer,
    IrCmd::BufferReadi8
    | IrCmd::BufferReadu8
    | IrCmd::BufferReadi16
    | IrCmd::BufferReadu16
    | IrCmd::BufferReadi32 => IrValueKind::Int,
    IrCmd::BufferReadi64 => IrValueKind::Int64,
    IrCmd::BufferWritei8
    | IrCmd::BufferWritei16
    | IrCmd::BufferWritei32
    | IrCmd::BufferWritef32
    | IrCmd::BufferWritef64
    | IrCmd::BufferWritei64 => IrValueKind::None,
    IrCmd::BufferReadf32 => IrValueKind::Float,
    IrCmd::BufferReadf64 => IrValueKind::Double,
    IrCmd::JumpCmpProtoid => IrValueKind::None,
  }
}
