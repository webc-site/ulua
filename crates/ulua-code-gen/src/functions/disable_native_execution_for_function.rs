use ulua_vm::{
  macros::{clvalue::clvalue, ttisfunction::ttisfunction},
  records::lua_state::lua_State,
  type_aliases::t_value::TValue,
};

use crate::functions::on_destroy_function::on_destroy_function;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn disable_native_execution_for_function(l: *mut lua_State, level: i32) {
  unsafe {
    if l.is_null() {
      return;
    }

    // CODEGEN_ASSERT(unsigned(level) < unsigned(l->ci - l->base_ci));
    let ci = (*l).ci;
    let base_ci = (*l).base_ci;

    let diff = ci.offset_from(base_ci);
    if !(level as u32) < diff as u32 {
      // CODEGEN_ASSERT should abort via handler; keep behavior conservative if it doesn't.
      return;
    }

    // const CallInfo* ci = l->ci - level;
    let ci_ptr = ci.offset(-(level as isize));

    // const TValue* o = ci->func;
    let o = (*ci_ptr).func as *const TValue;

    // CODEGEN_ASSERT(ttisfunction(o));
    if !ttisfunction!(o) {
      return;
    }

    // Proto* proto = clvalue(o)->l.p;
    let cl = clvalue!(o);
    let proto = (*cl).inner.l.p;

    if proto.is_null() {
      return;
    }

    // CODEGEN_ASSERT(proto->codeentry != proto->code);
    if (*proto).codeentry == (*proto).code {
      return;
    }

    on_destroy_function(l, proto);
  }
}
