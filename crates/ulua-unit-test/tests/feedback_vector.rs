//! Port of `cpp/tests/FeedbackVector.test.cpp`（389 行，8 个 TEST_CASE）。
//!
//! 被测对象：调用反馈向量（编译器 `CALLFB` 发射 + VM 侧命中记录，
//! `LuauEmitCallFeedback` / `LuauCallFeedback` / `LuauInlineHitsThreshold`）。
//!
//! 移植说明：
//! - C++ `FeedbackVectorFixture`（bcb + lua_State + onInline 回调）镜像为
//!   同名结构体；`alloc` 用 `free`/`realloc` extern，与 cpp 语义一致。
//! - C++ `AssertInlinerData` 经 `L->global->ecbdata` 传递；Rust 对应
//!   `LuaExecutionCallbackStorage`（512 字节对齐缓冲），以
//!   `as_mut_ptr` 转成 `*mut AssertInlinerData` 原地读写，与 cpp 的
//!   `reinterpret_cast` 等价。
//! - `f->code[pc+1]` 的 0/0xFFFFFFFF 密封标记用常量
//!   `LUAU_INSN_FBSLOT_SEALED` 表达。

use ulua_vm::macros::{clvalue::clvalue, lua_isfunction::lua_isfunction};
extern crate alloc;

use alloc::string::String;
use core::{
  ffi::{c_char, c_void},
  ptr::{NonNull, null_mut},
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
use ulua_vm::{
  functions::{
    lua_close::lua_close, lua_newstate::lua_newstate, lua_resume::lua_resume, luau_load::luau_load,
  },
  records::{Proto::Proto, closure::Closure, lua_state::lua_State},
};

/// cpp `alloc`：free/realloc 语义的分配回调
unsafe extern "C-unwind" fn luau_test_alloc(
  _ud: *mut c_void,
  ptr: *mut u8,
  _osize: usize,
  nsize: usize,
) -> *mut u8 {
  unsafe extern "C" {
    fn free(ptr: *mut c_void);
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
  }

  unsafe {
    if nsize == 0 {
      free(ptr as *mut c_void);
      null_mut()
    } else {
      realloc(ptr as *mut c_void, nsize) as *mut u8
    }
  }
}

/// cpp `std::unique_ptr<lua_State, void (*)(lua_State*)>` 的镜像
struct StateGuard(NonNull<lua_State>);

impl StateGuard {
  /// 测试环境内存充足，`lua_newstate` 失败即环境失效，集中一处 expect
  fn new() -> Self {
    Self(
      NonNull::new(unsafe { lua_newstate(Some(luau_test_alloc), null_mut()) })
        .expect("lua state allocation failed"),
    )
  }

  fn as_ptr(&self) -> *mut lua_State {
    self.0.as_ptr()
  }
}

impl Drop for StateGuard {
  fn drop(&mut self) {
    unsafe {
      lua_close(self.as_ptr());
    }
  }
}

/// cpp `FeedbackVectorFixture`
struct FeedbackVectorFixture {
  bcb: BytecodeBuilder,
  l: StateGuard,
  on_inline: Option<
    unsafe extern "C-unwind" fn(*mut lua_State, *mut Closure, *mut Closure, u32) -> *mut Proto,
  >,
}

impl FeedbackVectorFixture {
  fn new() -> Self {
    Self {
      bcb: BytecodeBuilder::new(None),
      l: StateGuard::new(),
      on_inline: None,
    }
  }

  fn lua_state(&self) -> *mut lua_State {
    self.l.as_ptr()
  }

  /// cpp `compile`：Dump_Code + optimizationLevel 0
  fn compile(&mut self, source: &str) {
    self.bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let opts = CompileOptions {
      optimization_level: 0,
      ..CompileOptions::default()
    };

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
      &mut self.bcb,
      &String::from(source),
      &opts,
      &ParseOptions::default(),
    );
  }

  /// cpp `load`：取字节码 → luau_load → 返回栈顶 Closure 的 Proto
  fn load(&mut self) -> *mut Proto {
    let bytecode = self.bcb.get_bytecode();
    let l = self.lua_state();

    unsafe {
      let res = luau_load(
        l,
        c"=FeedbackVectorTest".as_ptr(),
        bytecode.as_ptr() as *const c_void as *const c_char,
        bytecode.len(),
        0,
      );
      assert!(res == 0 && lua_isfunction!(l, -1));

      let top = clvalue!((*l).top.sub(1));
      (*top).inner.l.p
    }
  }

  /// cpp `run`：挂 inline 回调后 resume
  fn run(&mut self) {
    let l = self.lua_state();

    unsafe {
      (*(*l).global).ecb.inlinefunction = self.on_inline;
      assert_eq!(lua_resume(l, null_mut(), 0), 0);
    }
  }
}

/// cpp `AssertInlinerData`
#[repr(C)]
struct AssertInlinerData {
  proto: *mut Proto,
  target: *mut Proto,
  pc: u32,
  called: bool,
}

/// cpp `idInliner`：恒返回 caller 自身
unsafe extern "C-unwind" fn id_inliner(
  _l: *mut lua_State,
  caller: *mut Closure,
  _target: *mut Closure,
  _pc: u32,
) -> *mut Proto {
  unsafe { (*caller).inner.l.p }
}

/// cpp `sealingInliner`：恒返回 null，密封槽位
unsafe extern "C-unwind" fn sealing_inliner(
  _l: *mut lua_State,
  _caller: *mut Closure,
  _target: *mut Closure,
  _pc: u32,
) -> *mut Proto {
  null_mut()
}

/// cpp `idInlinerWithAssert`：校验回调入参后返回 caller 自身
unsafe extern "C-unwind" fn id_inliner_with_assert(
  l: *mut lua_State,
  caller: *mut Closure,
  target: *mut Closure,
  pc: u32,
) -> *mut Proto {
  unsafe {
    let data = (*(*l).global).ecbdata.as_mut_ptr() as *mut AssertInlinerData;
    let caller_proto = (*caller).inner.l.p;
    let target_proto = (*target).inner.l.p;

    assert_eq!((*data).proto, caller_proto);
    assert_eq!((*data).target, target_proto);
    assert_eq!((*data).pc, pc);

    (*data).called = true;
    caller_proto
  }
}

/// 在 ecbdata 中写入断言用数据，返回其指针
///
/// # Safety
/// `l` 的 ecbdata 生命周期覆盖整个用例，且同一 state 内只写一次。
unsafe fn install_assert_data(
  l: *mut lua_State,
  proto: *mut Proto,
  target: *mut Proto,
  pc: u32,
) -> *mut AssertInlinerData {
  unsafe {
    let data = (*(*l).global).ecbdata.as_mut_ptr() as *mut AssertInlinerData;
    data.write(AssertInlinerData {
      proto,
      target,
      pc,
      called: false,
    });
    data
  }
}

mod simple_call {
  //! Source: `tests/FeedbackVector.test.cpp:101-148`

  use ulua_common::{enums::luau_proto_flag::LuauProtoFlag, fflag, fint};
  use ulua_unit_test::type_aliases::{
    scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt,
  };
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use super::*;

  #[test]
  fn simple_call() {
    let _emit_call_fb = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2);

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
      alloc::format!("\n{}", fixture.bcb.dump_function(1)),
      r#"
GETUPVAL R1 0
CALLFB R1 0 1 [0]
LOADK R2 K0 [1]
ADD R0 R1 R2
RETURN R0 1
"#
    );

    let top = fixture.load();

    unsafe {
      let g = *(*top).p.add(0);
      assert_ne!((*g).flags & LuauProtoFlag::LPF_INLINABLE as u8, 0);

      let f = *(*top).p.add(1);
      assert_eq!((*f).feedbackvecsize, 1);

      let fbslot = (*f).feedbackvec.add(0);
      assert_eq!((*fbslot).kind, FeedbackVectorSlotKind::CallTarget);
      assert_eq!((*fbslot).data.call_target.pc, 1);
      assert_eq!((*fbslot).data.call_target.proto, 0);
      assert_eq!((*fbslot).data.call_target.hits, 0);
      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        0
      );

      let data = install_assert_data(fixture.lua_state(), f, g, (*fbslot).data.call_target.pc);
      fixture.on_inline = Some(id_inliner_with_assert);

      fixture.run();

      assert_eq!((*fbslot).data.call_target.pc, 1);
      assert_eq!((*fbslot).data.call_target.proto, (*g).funid);
      assert_eq!((*fbslot).data.call_target.hits, 2);
      assert!((*data).called);
    }
  }
}

mod simple_call_sealed {
  //! Source: `tests/FeedbackVector.test.cpp:150-177`

  use ulua_common::{fflag, fint, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};
  use ulua_unit_test::type_aliases::{
    scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt,
  };

  use super::*;

  #[test]
  fn simple_call_sealed() {
    let _emit_call_fb = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2);

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

    unsafe {
      let f = *(*top).p.add(1);
      let fbslot = (*f).feedbackvec.add(0);

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        0
      );
      // 密封槽位
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1) = LUAU_INSN_FBSLOT_SEALED;

      fixture.on_inline = Some(id_inliner);

      fixture.run();

      assert_eq!((*fbslot).data.call_target.proto, 0);
      assert_eq!((*fbslot).data.call_target.hits, 0);
    }
  }
}

mod simple_call_sealed_on_inline {
  //! Source: `tests/FeedbackVector.test.cpp:179-203`

  use ulua_common::{fflag, fint, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};
  use ulua_unit_test::type_aliases::{
    scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt,
  };

  use super::*;

  #[test]
  fn simple_call_sealed_on_inline() {
    let _emit_call_fb = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2);

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

    unsafe {
      let f = *(*top).p.add(1);
      let fbslot = (*f).feedbackvec.add(0);

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        0
      );

      fixture.on_inline = Some(sealing_inliner);

      fixture.run();

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        LUAU_INSN_FBSLOT_SEALED
      );
    }
  }
}

mod high_order_call {
  //! Source: `tests/FeedbackVector.test.cpp:205-252`

  use ulua_common::{enums::luau_proto_flag::LuauProtoFlag, fflag, fint};
  use ulua_unit_test::type_aliases::{
    scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt,
  };
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use super::*;

  #[test]
  fn high_order_call() {
    let _emit_call_fb = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2);

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
      alloc::format!("\n{}", fixture.bcb.dump_function(1)),
      r#"
MOVE R2 R0
CALLFB R2 0 1 [0]
LOADK R3 K0 [1]
ADD R1 R2 R3
RETURN R1 1
"#
    );

    let top = fixture.load();

    unsafe {
      let g = *(*top).p.add(0);
      assert_ne!((*g).flags & LuauProtoFlag::LPF_INLINABLE as u8, 0);

      let f = *(*top).p.add(1);
      assert_eq!((*f).feedbackvecsize, 1);

      let fbslot = (*f).feedbackvec.add(0);
      assert_eq!((*fbslot).kind, FeedbackVectorSlotKind::CallTarget);
      assert_eq!((*fbslot).data.call_target.pc, 1);
      assert_eq!((*fbslot).data.call_target.proto, 0);
      assert_eq!((*fbslot).data.call_target.hits, 0);
      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        0
      );

      let data = install_assert_data(fixture.lua_state(), f, g, (*fbslot).data.call_target.pc);
      fixture.on_inline = Some(id_inliner_with_assert);

      fixture.run();

      assert_eq!((*fbslot).data.call_target.pc, 1);
      assert_eq!((*fbslot).data.call_target.proto, (*g).funid);
      assert_eq!((*fbslot).data.call_target.hits, 2);
      assert!((*data).called);
    }
  }
}

mod polymorphic_call_sealed {
  //! Source: `tests/FeedbackVector.test.cpp:254-279`

  use ulua_common::{fflag, fint, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};
  use ulua_unit_test::type_aliases::{
    scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt,
  };

  use super::*;

  #[test]
  fn polymorphic_call_sealed() {
    let _emit_call_fb = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2);

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

    unsafe {
      let f = *(*top).p.add(2);
      let fbslot = (*f).feedbackvec.add(0);

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        0
      );

      fixture.on_inline = Some(id_inliner);

      fixture.run();

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        LUAU_INSN_FBSLOT_SEALED
      );
    }
  }
}

mod c_call_sealed {
  //! Source: `tests/FeedbackVector.test.cpp:281-306`

  use ulua_common::{fflag, fint, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};
  use ulua_unit_test::type_aliases::{
    scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt,
  };
  use ulua_vm::{functions::luaopen_base::luaopen_base, macros::lua_pop::lua_pop};

  use super::*;

  #[test]
  fn c_call_sealed() {
    let _emit_call_fb = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2);

    let mut fixture = FeedbackVectorFixture::new();
    fixture.compile(
      r#"
        local function f(h) return h(1) + 1 end
        f(tostring)
    "#,
    );

    let top = fixture.load();

    unsafe {
      let f = *(*top).p.add(0);
      let fbslot = (*f).feedbackvec.add(0);

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        0
      );

      fixture.on_inline = Some(id_inliner);

      // tostring 来自 base 库
      let l = fixture.lua_state();
      lua_pop(l, luaopen_base(l));

      fixture.run();

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        LUAU_INSN_FBSLOT_SEALED
      );
    }
  }
}

mod metamethod_call_sealed {
  //! Source: `tests/FeedbackVector.test.cpp:308-338`

  use ulua_common::{fflag, fint, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};
  use ulua_unit_test::type_aliases::{
    scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt,
  };
  use ulua_vm::{functions::luaopen_base::luaopen_base, macros::lua_pop::lua_pop};

  use super::*;

  #[test]
  fn metamethod_call_sealed() {
    let _emit_call_fb = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2);

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

    unsafe {
      let f = *(*top).p.add(0);
      let fbslot = (*f).feedbackvec.add(0);

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        0
      );

      fixture.on_inline = Some(id_inliner);

      // setmetatable 来自 base 库
      let l = fixture.lua_state();
      lua_pop(l, luaopen_base(l));

      fixture.run();

      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        LUAU_INSN_FBSLOT_SEALED
      );
    }
  }
}

mod namecall {
  //! Source: `tests/FeedbackVector.test.cpp:340-387`

  use ulua_common::{enums::luau_proto_flag::LuauProtoFlag, fflag, fint};
  use ulua_unit_test::type_aliases::{
    scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt,
  };
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use super::*;

  #[test]
  fn namecall() {
    let _emit_call_fb = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2);

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
      alloc::format!("\n{}", fixture.bcb.dump_function(1)),
      r#"
NAMECALL R2 R0 K0 ['g']
CALLFB R2 1 1 [0]
LOADK R3 K1 [1]
ADD R1 R2 R3
RETURN R1 1
"#
    );

    let top = fixture.load();

    unsafe {
      let g = *(*top).p.add(0);
      assert_ne!((*g).flags & LuauProtoFlag::LPF_INLINABLE as u8, 0);

      let f = *(*top).p.add(1);
      assert_eq!((*f).feedbackvecsize, 1);

      let fbslot = (*f).feedbackvec.add(0);
      assert_eq!((*fbslot).kind, FeedbackVectorSlotKind::CallTarget);
      assert_eq!((*fbslot).data.call_target.pc, 2);
      assert_eq!((*fbslot).data.call_target.proto, 0);
      assert_eq!((*fbslot).data.call_target.hits, 0);
      assert_eq!(
        *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
        0
      );

      let data = install_assert_data(fixture.lua_state(), f, g, (*fbslot).data.call_target.pc);
      fixture.on_inline = Some(id_inliner_with_assert);

      fixture.run();

      assert_eq!((*fbslot).data.call_target.proto, (*g).funid);
      assert_eq!((*fbslot).data.call_target.hits, 2);
      assert!((*data).called);
    }
  }
}
