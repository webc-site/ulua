use alloc::vec::Vec;

use ulua_ast::{enums::ast_stat_ref::AstStatRef, records::ast_stat_block::AstStatBlock};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{node::Stat, reducer::Reducer};

/// 把一个块 body 的语句坐标并入 `result`（cpp `append(AstStatBlock*)` 判空
/// 语义的引用形态：判空由调用方的 `Option`/句柄类型折叠完成）。
fn extend_body(result: &mut Vec<Stat>, block: &AstStatBlock) {
  // `Nodes` 元素由 parser 写入、句柄恒非空（`Node::get` 给出存活引用）
  result.extend(block.body.iter().map(Stat::from_ref));
}

impl Reducer {
  /// cpp `Reducer::getNestedStats` (`CLI/src/Reduce.cpp:185-230`)：
  /// 按语句的动态类型取出其嵌套语句列表（if/else 的 else 链递归展开）。
  ///
  /// 入参/返回均为 arena 句柄：cpp 的「空指针进→解引用 UB」在句柄模型下于
  /// 构造点（`Node::new`）即折叠为 `Option`，本函数不再有 null 分支。
  pub fn get_nested_stats(&self, stat: Stat) -> Vec<Stat> {
    let mut result: Vec<Stat> = Vec::new();

    match stat.as_stat_ref() {
      AstStatRef::Block(block) => {
        extend_body(&mut result, block);
      }
      AstStatRef::If(ifs) => {
        extend_body(&mut result, ifs.thenbody.get());

        if let Some(else_stat) = ifs.elsebody.get() {
          match else_stat.as_stat_ref() {
            AstStatRef::Block(else_block) => {
              extend_body(&mut result, else_block);
            }
            AstStatRef::If(_) => {
              let elsebody = Stat::from_ref(else_stat);
              result.extend(self.get_nested_stats(elsebody));
            }
            _ => {
              eprintln!("AstStatIf's else clause can have more statement types than I thought");
              LUAU_ASSERT!(false);
            }
          }
        }
      }
      AstStatRef::While(stat) => extend_body(&mut result, stat.body.get()),
      AstStatRef::Repeat(stat) => extend_body(&mut result, stat.body.get()),
      AstStatRef::For(stat) => extend_body(&mut result, stat.body.get()),
      AstStatRef::ForIn(stat) => extend_body(&mut result, stat.body.get()),
      AstStatRef::Function(stat) => extend_body(&mut result, stat.func.get().body.get()),
      AstStatRef::LocalFunction(stat) => extend_body(&mut result, stat.func.get().body.get()),
      _ => {}
    }

    result
  }
}
