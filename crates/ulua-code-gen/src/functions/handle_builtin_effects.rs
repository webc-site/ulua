use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::records::const_prop_state::ConstPropState;

/// fastcall 副作用分类（cpp OptimizeConstProp.cpp `handleBuiltinEffects`）：
/// 绝大多数 fastcall 无堆副作用，走空臂；仅表写、元表写使整个堆失效，
/// buffer 写使 heap buffer 数据失效。其余 builtin 与原逐项枚举臂同效
/// （原 match 无 `_` 臂，空集即全部剩余变体的补）。
pub fn handle_builtin_effects(
  state: &mut ConstPropState,
  bfid: LuauBuiltinFunction,
  first_return_reg: u32,
  _nresults: i32,
) {
  match bfid {
    LuauBuiltinFunction::LBF_TABLE_INSERT => {
      state.invalidate_heap();
      return; // table.insert does not modify result registers.
    }
    LuauBuiltinFunction::LBF_RAWSET | LuauBuiltinFunction::LBF_SETMETATABLE => {
      state.invalidate_heap();
    }
    LuauBuiltinFunction::LBF_BUFFER_WRITEU8
    | LuauBuiltinFunction::LBF_BUFFER_WRITEU16
    | LuauBuiltinFunction::LBF_BUFFER_WRITEU32
    | LuauBuiltinFunction::LBF_BUFFER_WRITEF32
    | LuauBuiltinFunction::LBF_BUFFER_WRITEF64
    | LuauBuiltinFunction::LBF_BUFFER_WRITEINTEGER => {
      state.invalidate_heap_buffer_data();
    }
    _ => {}
  }

  // TODO: 有些 fastcall 只改 value 不改 tag
  // TODO: fastcall 与普通 call 不同，也许无需从返回值起失效所有寄存器
  state.invalidate_registers_from(first_return_reg as i32);
}
