// feedback vector（sealed / inline / namecall）用例
// 移植自 `cpp/tests/FeedbackVector.test.cpp`。
//
// Proto/反馈槽/ecbdata 的裸内存遍历已收口进 [`feedback_vector_api`] 门面。
// 每个用例顶部同形的三条 call-feedback 旗标（`LuauEmitCallFeedback` /
// `LuauCallFeedback` / `LuauInlineHitsThreshold=2`）已并入
// `FeedbackVectorFixture::new()`，用例侧不再各写一遍。

#[test]
fn feedback_vector_c_call_sealed() {
  use ulua_common::macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED;

  use crate::common::{
    functions::{feedback_vector_api as fva, id_inliner::id_inliner, safe_api},
    records::feedback_vector_fixture::FeedbackVectorFixture,
  };

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(
    r#"
        local function f(h) return h(1) + 1 end
        f(tostring)
    "#,
  );

  let top = fixture.load();

  let f = fva::child_proto(top, 0);
  let fbslot = fva::feedback_slot(f, 0);

  assert_eq!(fva::code_at(f, fva::slot_pc(fbslot) as usize + 1), 0);

  fixture.on_inline = Some(id_inliner);

  let l = fixture.lua_state();
  safe_api::pop(l, safe_api::open_base(l));

  fixture.run();

  assert_eq!(
    fva::code_at(f, fva::slot_pc(fbslot) as usize + 1),
    LUAU_INSN_FBSLOT_SEALED
  );
}

#[test]
fn feedback_vector_high_order_call() {
  use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use crate::common::{
    functions::{feedback_vector_api as fva, id_inliner_with_assert::id_inliner_with_assert},
    records::feedback_vector_fixture::FeedbackVectorFixture,
  };

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(
    r#"
        local function g() return 1 end
        local function f(h) return h() + 1 end
        f(g)
        f(g)
    "#,
  );

  assert_eq!(
    format!("\n{}", fixture.bcb.dump_function(1)),
    r#"
MOVE R2 R0
CALLFB R2 0 1 [0]
LOADK R3 K0 [1]
ADD R1 R2 R3
RETURN R1 1
"#
  );

  let top = fixture.load();

  let g = fva::child_proto(top, 0);
  let f = fva::child_proto(top, 1);

  assert_ne!(fva::proto_flags(g) & LuauProtoFlag::LPF_INLINABLE as u8, 0);
  assert_eq!(fva::feedbackvecsize(f), 1);

  let fbslot = fva::feedback_slot(f, 0);

  assert_eq!(fva::slot_kind(fbslot), FeedbackVectorSlotKind::CallTarget);
  assert_eq!(fva::slot_pc(fbslot), 1);
  assert_eq!(fva::slot_proto(fbslot), 0);
  assert_eq!(fva::slot_hits(fbslot), 0);
  assert_eq!(fva::code_at(f, fva::slot_pc(fbslot) as usize + 1), 0);

  let data = fva::assert_inliner_install(fixture.lua_state(), f, g, fva::slot_pc(fbslot));
  fixture.on_inline = Some(id_inliner_with_assert);

  fixture.run();

  assert_eq!(fva::slot_pc(fbslot), 1);
  assert_eq!(fva::slot_proto(fbslot), fva::proto_funid(g));
  assert_eq!(fva::slot_hits(fbslot), 2);
  assert!(fva::inliner_data_called(data));
}

#[test]
fn feedback_vector_metamethod_call_sealed() {
  use ulua_common::macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED;

  use crate::common::{
    functions::{feedback_vector_api as fva, id_inliner::id_inliner, safe_api},
    records::feedback_vector_fixture::FeedbackVectorFixture,
  };

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(
    r#"
        local function f(h) return h(1) + 1 end

        local callableTable = {}

        setmetatable(callableTable, { __call = function(self, arg) return arg + 42 end })

        f(callableTable)
    "#,
  );

  let top = fixture.load();

  let f = fva::child_proto(top, 0);
  let fbslot = fva::feedback_slot(f, 0);

  assert_eq!(fva::code_at(f, fva::slot_pc(fbslot) as usize + 1), 0);

  fixture.on_inline = Some(id_inliner);

  let l = fixture.lua_state();
  safe_api::pop(l, safe_api::open_base(l));

  fixture.run();

  assert_eq!(
    fva::code_at(f, fva::slot_pc(fbslot) as usize + 1),
    LUAU_INSN_FBSLOT_SEALED
  );
}

#[test]
fn feedback_vector_namecall() {
  use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use crate::common::{
    functions::{feedback_vector_api as fva, id_inliner_with_assert::id_inliner_with_assert},
    records::feedback_vector_fixture::FeedbackVectorFixture,
  };

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(
    r#"
        local t = { x = 1 }
        function t.g(self) return self.x end
        local function f(t) return t:g() + 1 end
        f(t)
        f(t)
    "#,
  );

  assert_eq!(
    format!("\n{}", fixture.bcb.dump_function(1)),
    r#"
NAMECALL R2 R0 K0 ['g']
CALLFB R2 1 1 [0]
LOADK R3 K1 [1]
ADD R1 R2 R3
RETURN R1 1
"#
  );

  let top = fixture.load();

  let g = fva::child_proto(top, 0);
  let f = fva::child_proto(top, 1);

  assert_ne!(fva::proto_flags(g) & LuauProtoFlag::LPF_INLINABLE as u8, 0);
  assert_eq!(fva::feedbackvecsize(f), 1);

  let fbslot = fva::feedback_slot(f, 0);

  assert_eq!(fva::slot_kind(fbslot), FeedbackVectorSlotKind::CallTarget);
  assert_eq!(fva::slot_pc(fbslot), 2);
  assert_eq!(fva::slot_proto(fbslot), 0);
  assert_eq!(fva::slot_hits(fbslot), 0);
  assert_eq!(fva::code_at(f, fva::slot_pc(fbslot) as usize + 1), 0);

  let data = fva::assert_inliner_install(fixture.lua_state(), f, g, fva::slot_pc(fbslot));
  fixture.on_inline = Some(id_inliner_with_assert);

  fixture.run();

  assert_eq!(fva::slot_proto(fbslot), fva::proto_funid(g));
  assert_eq!(fva::slot_hits(fbslot), 2);
  assert!(fva::inliner_data_called(data));
}

#[test]
fn feedback_vector_polymorphic_call_sealed() {
  use ulua_common::macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED;

  use crate::common::{
    functions::{feedback_vector_api as fva, id_inliner::id_inliner},
    records::feedback_vector_fixture::FeedbackVectorFixture,
  };

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(
    r#"
        local function g() return 1 end
        local function y() return 2 end
        local function f(h) return h() + 1 end
        f(g)
        f(y)
    "#,
  );

  let top = fixture.load();

  let f = fva::child_proto(top, 2);
  let fbslot = fva::feedback_slot(f, 0);

  assert_eq!(fva::code_at(f, fva::slot_pc(fbslot) as usize + 1), 0);

  fixture.on_inline = Some(id_inliner);

  fixture.run();

  assert_eq!(
    fva::code_at(f, fva::slot_pc(fbslot) as usize + 1),
    LUAU_INSN_FBSLOT_SEALED
  );
}

#[test]
fn feedback_vector_simple_call() {
  use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use crate::common::{
    functions::{feedback_vector_api as fva, id_inliner_with_assert::id_inliner_with_assert},
    records::feedback_vector_fixture::FeedbackVectorFixture,
  };

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(
    r#"
        local function g() return 1 end
        local function f() return g() + 1 end
        f()
        f()
    "#,
  );

  assert_eq!(
    format!("\n{}", fixture.bcb.dump_function(1)),
    r#"
GETUPVAL R1 0
CALLFB R1 0 1 [0]
LOADK R2 K0 [1]
ADD R0 R1 R2
RETURN R0 1
"#
  );

  let top = fixture.load();

  let g = fva::child_proto(top, 0);
  let f = fva::child_proto(top, 1);

  assert_ne!(fva::proto_flags(g) & LuauProtoFlag::LPF_INLINABLE as u8, 0);
  assert_eq!(fva::feedbackvecsize(f), 1);

  let fbslot = fva::feedback_slot(f, 0);

  assert_eq!(fva::slot_kind(fbslot), FeedbackVectorSlotKind::CallTarget);
  assert_eq!(fva::slot_pc(fbslot), 1);
  assert_eq!(fva::slot_proto(fbslot), 0);
  assert_eq!(fva::slot_hits(fbslot), 0);
  assert_eq!(fva::code_at(f, fva::slot_pc(fbslot) as usize + 1), 0);

  let data = fva::assert_inliner_install(fixture.lua_state(), f, g, fva::slot_pc(fbslot));
  fixture.on_inline = Some(id_inliner_with_assert);

  fixture.run();

  assert_eq!(fva::slot_pc(fbslot), 1);
  assert_eq!(fva::slot_proto(fbslot), fva::proto_funid(g));
  assert_eq!(fva::slot_hits(fbslot), 2);
  assert!(fva::inliner_data_called(data));
}

#[test]
fn feedback_vector_simple_call_sealed() {
  use ulua_common::macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED;

  use crate::common::{
    functions::{feedback_vector_api as fva, id_inliner::id_inliner},
    records::feedback_vector_fixture::FeedbackVectorFixture,
  };

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(
    r#"
        local function g() return 1 end
        local function f() return g() + 1 end
        f()
        f()
    "#,
  );

  let top = fixture.load();

  let f = fva::child_proto(top, 1);
  let fbslot = fva::feedback_slot(f, 0);

  assert_eq!(fva::code_at(f, fva::slot_pc(fbslot) as usize + 1), 0);
  let pc = fva::slot_pc(fbslot) as usize + 1;
  fva::set_code_at(f, pc, LUAU_INSN_FBSLOT_SEALED);

  fixture.on_inline = Some(id_inliner);

  fixture.run();

  assert_eq!(fva::slot_proto(fbslot), 0);
  assert_eq!(fva::slot_hits(fbslot), 0);
}

#[test]
fn feedback_vector_simple_call_sealed_on_inline() {
  use ulua_common::macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED;

  use crate::common::{
    functions::{feedback_vector_api as fva, sealing_inliner::sealing_inliner},
    records::feedback_vector_fixture::FeedbackVectorFixture,
  };

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(
    r#"
        local function g() return 1 end
        local function f() return g() + 1 end
        f()
        f()
    "#,
  );

  let top = fixture.load();

  let f = fva::child_proto(top, 1);
  let fbslot = fva::feedback_slot(f, 0);

  assert_eq!(fva::code_at(f, fva::slot_pc(fbslot) as usize + 1), 0);

  fixture.on_inline = Some(sealing_inliner);

  fixture.run();

  assert_eq!(
    fva::code_at(f, fva::slot_pc(fbslot) as usize + 1),
    LUAU_INSN_FBSLOT_SEALED
  );
}
