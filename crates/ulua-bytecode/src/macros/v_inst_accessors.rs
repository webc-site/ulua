//! VM 指令操作数访问器断言宏宿主：收编原 `vreg`/`vregrange`/`vconst`/`vconstany`/
//! `vjump`/`vupval` 6 枚单宏碎片文件（仿 `ulua-common` 的 `macros/luau_assert.rs`
//! 宿主先例）。宏体逐行原文照搬，语义不变；唯一消费点为 `records/bytecode_builder.rs`。

macro_rules! VREG {
  ($v:expr, $func:expr) => {
    LUAU_ASSERT!(($v as u32) < ($func.maxstacksize as u32))
  };
}

macro_rules! VREGRANGE {
  ($v:expr, $count:expr, $func:expr) => {
    LUAU_ASSERT!(
      (($v as i32)
        + (if ($count as i32) < 0 {
          0
        } else {
          $count as i32
        })) as u32
        <= ($func.maxstacksize as u32)
    )
  };
}

macro_rules! VCONST {
    (@kind Number) => {
        crate::enums::r#type::Type::Number
    };
    (@kind String) => {
        crate::enums::r#type::Type::String
    };
    (@kind Import) => {
        crate::enums::r#type::Type::Import
    };
    (@kind Table) => {
        crate::enums::r#type::Type::Table
    };
    (@kind Closure) => {
        crate::enums::r#type::Type::Closure
    };
    (@kind ClassShape) => {
        crate::enums::r#type::Type::ClassShape
    };
    ($v:expr, $kind:ident, $constants:expr) => {
        LUAU_ASSERT!(
            ($v as usize) < $constants.len()
                && $constants[$v as usize].kind() == VCONST!(@kind $kind)
        )
    };
}

macro_rules! VCONSTANY {
  ($v:expr, $constants:expr) => {
    LUAU_ASSERT!(($v as usize) < $constants.len())
  };
}

macro_rules! VJUMP {
  ($v:expr, $i:expr, $insns:expr, $insnvalid:expr) => {
    LUAU_ASSERT!(
      (($i as isize) + 1 + ($v as isize)) >= 0
        && (($i as isize) + 1 + ($v as isize)) < ($insns.len() as isize)
        && $insnvalid[($i as isize + 1 + ($v as isize)) as usize] != 0
    )
  };
}

macro_rules! VUPVAL {
  ($v:expr, $func:expr) => {
    LUAU_ASSERT!(($v as u32) < ($func.numupvalues as u32))
  };
}

pub(crate) use {VCONST, VCONSTANY, VJUMP, VREG, VREGRANGE, VUPVAL};
