use core::mem::transmute;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LuauOpcode {
  LopNop,
  LopBreak,
  LopLoadnil,
  LopLoadb,
  LopLoadn,
  LopLoadk,
  LopMove,
  LopGetglobal,
  LopSetglobal,
  LopGetupval,
  LopSetupval,
  LopCloseupvals,
  LopGetimport,
  LopGettable,
  LopSettable,
  LopGettableks,
  LopSettableks,
  LopGettablen,
  LopSettablen,
  LopNewclosure,
  LopNamecall,
  LopCall,
  LopReturn,
  LopJump,
  LopJumpback,
  LopJumpif,
  LopJumpifnot,
  LopJumpifeq,
  LopJumpifle,
  LopJumpiflt,
  LopJumpifnoteq,
  LopJumpifnotle,
  LopJumpifnotlt,
  LopAdd,
  LopSub,
  LopMul,
  LopDiv,
  LopMod,
  LopPow,
  LopAddk,
  LopSubk,
  LopMulk,
  LopDivk,
  LopModk,
  LopPowk,
  LopAnd,
  LopOr,
  LopAndk,
  LopOrk,
  LopConcat,
  LopNot,
  LopMinus,
  LopLength,
  LopNewtable,
  LopDuptable,
  LopSetlist,
  LopFornprep,
  LopFornloop,
  LopForgloop,
  LopForgprepInext,
  LopFastcall3,
  LopForgprepNext,
  LopNativecall,
  LopGetvarargs,
  LopDupclosure,
  LopPrepvarargs,
  LopLoadkx,
  LopJumpx,
  LopFastcall,
  LopCoverage,
  LopCapture,
  LopSubrk,
  LopDivrk,
  LopFastcall1,
  LopFastcall2,
  LopFastcall2k,
  LopForgprep,
  LopJumpxeqknil,
  LopJumpxeqkb,
  LopJumpxeqkn,
  LopJumpxeqks,
  LopIdiv,
  LopIdivk,
  LopGetudataks,
  LopSetudataks,
  LopNamecalludata,
  LopNewclassmember,
  LopCallfb,
  LopCmpproto,
  LopFastpcall,
  LopNewclass,
  LopCount,
}

impl LuauOpcode {
  pub const LOP_NOP: LuauOpcode = LuauOpcode::LopNop;
  pub const LOP_BREAK: LuauOpcode = LuauOpcode::LopBreak;
  pub const LOP_LOADNIL: LuauOpcode = LuauOpcode::LopLoadnil;
  pub const LOP_LOADB: LuauOpcode = LuauOpcode::LopLoadb;
  pub const LOP_LOADN: LuauOpcode = LuauOpcode::LopLoadn;
  pub const LOP_LOADK: LuauOpcode = LuauOpcode::LopLoadk;
  pub const LOP_MOVE: LuauOpcode = LuauOpcode::LopMove;
  pub const LOP_GETGLOBAL: LuauOpcode = LuauOpcode::LopGetglobal;
  pub const LOP_SETGLOBAL: LuauOpcode = LuauOpcode::LopSetglobal;
  pub const LOP_GETUPVAL: LuauOpcode = LuauOpcode::LopGetupval;
  pub const LOP_SETUPVAL: LuauOpcode = LuauOpcode::LopSetupval;
  pub const LOP_CLOSEUPVALS: LuauOpcode = LuauOpcode::LopCloseupvals;
  pub const LOP_GETIMPORT: LuauOpcode = LuauOpcode::LopGetimport;
  pub const LOP_GETTABLE: LuauOpcode = LuauOpcode::LopGettable;
  pub const LOP_SETTABLE: LuauOpcode = LuauOpcode::LopSettable;
  pub const LOP_GETTABLEKS: LuauOpcode = LuauOpcode::LopGettableks;
  pub const LOP_SETTABLEKS: LuauOpcode = LuauOpcode::LopSettableks;
  pub const LOP_GETTABLEN: LuauOpcode = LuauOpcode::LopGettablen;
  pub const LOP_SETTABLEN: LuauOpcode = LuauOpcode::LopSettablen;
  pub const LOP_NEWCLOSURE: LuauOpcode = LuauOpcode::LopNewclosure;
  pub const LOP_NAMECALL: LuauOpcode = LuauOpcode::LopNamecall;
  pub const LOP_CALL: LuauOpcode = LuauOpcode::LopCall;
  pub const LOP_RETURN: LuauOpcode = LuauOpcode::LopReturn;
  pub const LOP_JUMP: LuauOpcode = LuauOpcode::LopJump;
  pub const LOP_JUMPBACK: LuauOpcode = LuauOpcode::LopJumpback;
  pub const LOP_JUMPIF: LuauOpcode = LuauOpcode::LopJumpif;
  pub const LOP_JUMPIFNOT: LuauOpcode = LuauOpcode::LopJumpifnot;
  pub const LOP_JUMPIFEQ: LuauOpcode = LuauOpcode::LopJumpifeq;
  pub const LOP_JUMPIFLE: LuauOpcode = LuauOpcode::LopJumpifle;
  pub const LOP_JUMPIFLT: LuauOpcode = LuauOpcode::LopJumpiflt;
  pub const LOP_JUMPIFNOTEQ: LuauOpcode = LuauOpcode::LopJumpifnoteq;
  pub const LOP_JUMPIFNOTLE: LuauOpcode = LuauOpcode::LopJumpifnotle;
  pub const LOP_JUMPIFNOTLT: LuauOpcode = LuauOpcode::LopJumpifnotlt;
  pub const LOP_ADD: LuauOpcode = LuauOpcode::LopAdd;
  pub const LOP_SUB: LuauOpcode = LuauOpcode::LopSub;
  pub const LOP_MUL: LuauOpcode = LuauOpcode::LopMul;
  pub const LOP_DIV: LuauOpcode = LuauOpcode::LopDiv;
  pub const LOP_MOD: LuauOpcode = LuauOpcode::LopMod;
  pub const LOP_POW: LuauOpcode = LuauOpcode::LopPow;
  pub const LOP_ADDK: LuauOpcode = LuauOpcode::LopAddk;
  pub const LOP_SUBK: LuauOpcode = LuauOpcode::LopSubk;
  pub const LOP_MULK: LuauOpcode = LuauOpcode::LopMulk;
  pub const LOP_DIVK: LuauOpcode = LuauOpcode::LopDivk;
  pub const LOP_MODK: LuauOpcode = LuauOpcode::LopModk;
  pub const LOP_POWK: LuauOpcode = LuauOpcode::LopPowk;
  pub const LOP_AND: LuauOpcode = LuauOpcode::LopAnd;
  pub const LOP_OR: LuauOpcode = LuauOpcode::LopOr;
  pub const LOP_ANDK: LuauOpcode = LuauOpcode::LopAndk;
  pub const LOP_ORK: LuauOpcode = LuauOpcode::LopOrk;
  pub const LOP_CONCAT: LuauOpcode = LuauOpcode::LopConcat;
  pub const LOP_NOT: LuauOpcode = LuauOpcode::LopNot;
  pub const LOP_MINUS: LuauOpcode = LuauOpcode::LopMinus;
  pub const LOP_LENGTH: LuauOpcode = LuauOpcode::LopLength;
  pub const LOP_NEWTABLE: LuauOpcode = LuauOpcode::LopNewtable;
  pub const LOP_DUPTABLE: LuauOpcode = LuauOpcode::LopDuptable;
  pub const LOP_SETLIST: LuauOpcode = LuauOpcode::LopSetlist;
  pub const LOP_FORNPREP: LuauOpcode = LuauOpcode::LopFornprep;
  pub const LOP_FORNLOOP: LuauOpcode = LuauOpcode::LopFornloop;
  pub const LOP_FORGLOOP: LuauOpcode = LuauOpcode::LopForgloop;
  pub const LOP_FORGPREP_INEXT: LuauOpcode = LuauOpcode::LopForgprepInext;
  pub const LOP_FASTCALL3: LuauOpcode = LuauOpcode::LopFastcall3;
  pub const LOP_FORGPREP_NEXT: LuauOpcode = LuauOpcode::LopForgprepNext;
  pub const LOP_NATIVECALL: LuauOpcode = LuauOpcode::LopNativecall;
  pub const LOP_GETVARARGS: LuauOpcode = LuauOpcode::LopGetvarargs;
  pub const LOP_DUPCLOSURE: LuauOpcode = LuauOpcode::LopDupclosure;
  pub const LOP_PREPVARARGS: LuauOpcode = LuauOpcode::LopPrepvarargs;
  pub const LOP_LOADKX: LuauOpcode = LuauOpcode::LopLoadkx;
  pub const LOP_JUMPX: LuauOpcode = LuauOpcode::LopJumpx;
  pub const LOP_FASTCALL: LuauOpcode = LuauOpcode::LopFastcall;
  pub const LOP_COVERAGE: LuauOpcode = LuauOpcode::LopCoverage;
  pub const LOP_CAPTURE: LuauOpcode = LuauOpcode::LopCapture;
  pub const LOP_SUBRK: LuauOpcode = LuauOpcode::LopSubrk;
  pub const LOP_DIVRK: LuauOpcode = LuauOpcode::LopDivrk;
  pub const LOP_FASTCALL1: LuauOpcode = LuauOpcode::LopFastcall1;
  pub const LOP_FASTCALL2: LuauOpcode = LuauOpcode::LopFastcall2;
  pub const LOP_FASTCALL2K: LuauOpcode = LuauOpcode::LopFastcall2k;
  pub const LOP_FORGPREP: LuauOpcode = LuauOpcode::LopForgprep;
  pub const LOP_JUMPXEQKNIL: LuauOpcode = LuauOpcode::LopJumpxeqknil;
  pub const LOP_JUMPXEQKB: LuauOpcode = LuauOpcode::LopJumpxeqkb;
  pub const LOP_JUMPXEQKN: LuauOpcode = LuauOpcode::LopJumpxeqkn;
  pub const LOP_JUMPXEQKS: LuauOpcode = LuauOpcode::LopJumpxeqks;
  pub const LOP_IDIV: LuauOpcode = LuauOpcode::LopIdiv;
  pub const LOP_IDIVK: LuauOpcode = LuauOpcode::LopIdivk;
  pub const LOP_GETUDATAKS: LuauOpcode = LuauOpcode::LopGetudataks;
  pub const LOP_SETUDATAKS: LuauOpcode = LuauOpcode::LopSetudataks;
  pub const LOP_NAMECALLUDATA: LuauOpcode = LuauOpcode::LopNamecalludata;
  pub const LOP_NEWCLASSMEMBER: LuauOpcode = LuauOpcode::LopNewclassmember;
  pub const LOP_CALLFB: LuauOpcode = LuauOpcode::LopCallfb;
  pub const LOP_CMPPROTO: LuauOpcode = LuauOpcode::LopCmpproto;
  pub const LOP_FASTPCALL: LuauOpcode = LuauOpcode::LopFastpcall;
  pub const LOP_NEWCLASS: LuauOpcode = LuauOpcode::LopNewclass;
  pub const LOP__COUNT: LuauOpcode = LuauOpcode::LopCount;
}

impl From<u8> for LuauOpcode {
  /// C++ 将指令的操作码字节直接转换为 `LuauOpcode`
  /// (`LuauOpcode(LUAU_INSN_OP(insn))`)。有效字节码仅包含范围内的操作码；
  /// `repr(u8)` 使内存布局完全一致。
  fn from(v: u8) -> Self {
    unsafe { transmute(v) }
  }
}
