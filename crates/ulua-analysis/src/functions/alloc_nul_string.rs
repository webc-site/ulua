use core::ptr::copy_nonoverlapping;

use ulua_ast::records::{allocator::Allocator, ast_name::AstName};

/// 在 AST bump arena 里复制一份 `contents` 并补 NUL 结尾，返回其首地址。
///
/// `AstName` 的存储形态是 NUL 结尾字节串指针（批 2 后 `*const u8`），故把 Rust
/// `String`/`&str` 交回 AST 节点前必须做一次 arena 内的字节复制（cpp
/// `allocateStringView`，Ast.h/Lexer.cpp 以 `char*` 存、同一字节域）。本函数是该
/// 复制的唯一收口点。
pub fn alloc_nul_string(allocator: &mut Allocator, contents: &str) -> *mut u8 {
  let size = contents.len();
  let result = allocator.allocate(size + 1);

  // Safety: Allocator::allocate 为 bump 页分配，OOM 时 handle_alloc_error 中止而非返回 null，
  // 否则返回至少 size+1 字节、按 ALIGN 对齐且此后地址永不移动的块；contents.as_ptr()/len 由
  // &str 保证有效可读；copy 长度 size 与写入的 result.add(size) 均在块内，末字节补 NUL。
  unsafe {
    copy_nonoverlapping(contents.as_ptr(), result, size);
    *result.add(size) = 0;
  }

  result
}

/// arena 内复制 `contents` 并包成 `AstName`。
///
/// cpp 里 `AstName(allocateStringView(alloc, name))` 是成对出现的两步（全 crate 14
/// 处），这里合一，避免 NUL 结尾指针这一中间形态外泄到调用点。
pub fn alloc_name(allocator: &mut Allocator, contents: &str) -> AstName {
  AstName::ast_name_u8(alloc_nul_string(allocator, contents))
}
