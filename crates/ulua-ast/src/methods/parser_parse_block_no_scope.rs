use crate::{
  enums::type_lexer::Type,
  functions::is_stat_last::is_stat_last,
  records::{
    ast_stat_block::AstStatBlock, location::Location, parser::Parser, temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_block_no_scope(&mut self) -> *mut AstStatBlock {
    let mut body = TempVector::new(&mut self.scratch_stat);

    let prev_position = self.lexer.previous_location().end;

    while !self.block_follow(self.lexer.current()) {
      let old_recursion_count = self.recursion_counter;

      self.increment_recursion_counter("block");

      let stat = self.parse_stat();

      self.recursion_counter = old_recursion_count;

      if self.lexer.current().r#type == Type::SEMICOLON {
        self.next_lexeme();
        // Safety: stat 为 parse_stat 刚 arena 分配的存活语句节点（alloc_stat 恒非空，失败中止，错误分支返回 AstStatError），此刻尚未 push 进 body、无其他持有者，写 has_semicolon 字段单线程独占、无别名冲突。
        unsafe {
          (*stat).has_semicolon = true;
        }
        // Safety: 同上：stat 仍是刚分配的独占存活 arena 节点，把 base.location.end 更新为分号词素末坐标；写基类前缀字段在单线程串行下无并发读。
        unsafe {
          (*stat).base.location.end = self.lexer.previous_location().end;
        }
      }

      body.push_back(stat);

      // Safety: stat 为 parse_stat 分配的非空存活语句节点，只读判定其是否为最后一条语句。
      if is_stat_last(unsafe { &*stat }) {
        break;
      }
    }

    let location = Location::new(prev_position, self.lexer.current().location.begin);

    let body_array = self.copy_temp_vector_nodes(&body);
    let node = self.alloc(AstStatBlock::new(location, body_array, true));

    // cpp Parser.cpp:391 parseBlockNoScope 不创建 CstNode（Luau 无 CstStatBlock），
    // 模块根块的 CST 归属其语句解析器，勿在此补建。
    node
  }
}
