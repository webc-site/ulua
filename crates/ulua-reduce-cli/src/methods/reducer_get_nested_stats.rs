use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_stat_block::AstStatBlock, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_while::AstStatWhile,
  },
  rtti::{AstNodeClass, ast_node_as_unchecked},
};
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
    let node = stat.get();

    // 单 body 语句族（while/repeat/for/for-in）的同形臂：下转后取 body 句柄的
    // 只读视图入列。body 为非空 `Node`（类型层已折叠 cpp 判空），`get()` 即
    // `&AstStatBlock`，与旧 `Block::new(as_ptr)` 桥（恒 Some）等价。
    macro_rules! body_stat {
      ($ty:ty) => {{
        let stat = unsafe { ast_node_as_unchecked::<$ty>(&node.base) };
        extend_body(&mut result, stat.body.get());
      }};
    }
    // 函数语句族（function/local function）的同形臂：func 句柄两级 `get()`
    // 落到函数 body 块（func/body 均为非空 Node，无判空可省）。
    macro_rules! func_stat {
      ($ty:ty) => {{
        let stat = unsafe { ast_node_as_unchecked::<$ty>(&node.base) };
        extend_body(&mut result, stat.func.get().body.get());
      }};
    }

    match node.base.class_index {
      AstStatBlock::CLASS_INDEX => {
        let block = unsafe { ast_node_as_unchecked::<AstStatBlock>(&node.base) };
        extend_body(&mut result, block);
      }
      AstStatIf::CLASS_INDEX => {
        let ifs = unsafe { ast_node_as_unchecked::<AstStatIf>(&node.base) };
        extend_body(&mut result, ifs.thenbody.get());

        if let Some(else_stat) = ifs.elsebody.get() {
          match else_stat.base.class_index {
            AstStatBlock::CLASS_INDEX => {
              let else_block = unsafe { ast_node_as_unchecked::<AstStatBlock>(else_stat) };
              extend_body(&mut result, else_block);
            }
            AstStatIf::CLASS_INDEX => {
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
      AstStatWhile::CLASS_INDEX => body_stat!(AstStatWhile),
      AstStatRepeat::CLASS_INDEX => body_stat!(AstStatRepeat),
      AstStatFor::CLASS_INDEX => body_stat!(AstStatFor),
      AstStatForIn::CLASS_INDEX => body_stat!(AstStatForIn),
      AstStatFunction::CLASS_INDEX => func_stat!(AstStatFunction),
      AstStatLocalFunction::CLASS_INDEX => func_stat!(AstStatLocalFunction),
      _ => {}
    }

    result
  }
}
