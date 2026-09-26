use crate::functions::bytecode_cursor::Cursor;

/// cpp `readString`（`Bytecode/src/BytecodeGraph.cpp:17`）：`string_id == 0` 是空串，
/// 否则查表。索引越界说明字节码损坏，返回 `None` 而非 cpp 的 release 下越界读。
pub(crate) fn read_string<'a>(strings: &[&'a [u8]], c: &mut Cursor<'_>) -> Option<&'a [u8]> {
  let string_id = c.var_int()?;
  if string_id == 0 {
    return Some(&[]);
  }
  strings.get((string_id - 1) as usize).copied()
}
