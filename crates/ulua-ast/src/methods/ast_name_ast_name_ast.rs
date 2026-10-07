use ulua_common::functions::c_str::cstr_bytes;

use crate::records::ast_name::AstName;

impl Default for AstName {
  fn default() -> Self {
    Self::new()
  }
}

impl AstName {
  /// 从存活指针直接包名（cpp `AstName(const char* value)`，Ast.h:30）：
  /// 通过 cstr_bytes 计算长度并记录在 len 字段，使得后续 as_bytes 均为 O(1)。
  ///
  /// 边界登记：本函数解引用**调用方传入**的 `value`，其真实前提（null，或指向
  /// NUL 结尾且在本 `AstName` 存活期内可读的驻留串）由调用点构造序兑现——现存
  /// 消费面散布 `ulua-analysis` 重水合路径与各 `tests/`（本批边界禁止连改），
  /// 升格为 `unsafe fn` + 逐参 `# Safety` 须与消费者同批推进，故此处先以
  /// `// Safety:` 钉住前提、不改签名谎报 safe。
  pub fn ast_name_u8(value: *const u8) -> Self {
    if value.is_null() {
      Self::new()
    } else {
      // Safety: 上段契约——非空即指向 NUL 结尾的存活驻留串（alloc_nul_string /
      // 名表 intern 缓冲产出），strlen 扫描止步于终止 NUL，界内只读。
      let len = unsafe { cstr_bytes(value.cast()) }.len() as u32;
      Self::from_raw_parts(value, len)
    }
  }
}
