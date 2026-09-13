use alloc::vec::Vec;
use core::{mem::size_of, slice::from_raw_parts};

use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_while::AstStatWhile,
  },
  rtti::ast_node_as,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::reducer::Reducer;

impl Reducer {
  pub fn get_nested_stats(&self, stat: *mut AstStat) -> Vec<*mut AstStat> {
    let mut result: Vec<*mut AstStat> = Vec::new();

    let mut append = |block: *mut AstStatBlock| {
      if !block.is_null() {
        unsafe {
          let data = (*block).body.begin();
          let end = (*block).body.end();
          let size = (end as usize - data as usize) / size_of::<*mut AstStat>();
          if !data.is_null() && size > 0 {
            let slice = from_raw_parts(data, size);
            result.extend_from_slice(slice);
          }
        }
      }
    };

    unsafe {
      if stat.is_null() {
        return result;
      }

      let node_ptr = stat as *mut AstNode;

      let block = ast_node_as::<AstStatBlock>(node_ptr);
      if !block.is_null() {
        append(block);
      } else {
        let ifs = ast_node_as::<AstStatIf>(node_ptr);
        if !ifs.is_null() {
          append((*ifs).thenbody);
          if !(*ifs).elsebody.is_null() {
            let else_ptr = (*ifs).elsebody as *mut AstNode;
            let else_block = ast_node_as::<AstStatBlock>(else_ptr);
            if !else_block.is_null() {
              append(else_block);
            } else {
              let else_if = ast_node_as::<AstStatIf>(else_ptr);
              if !else_if.is_null() {
                let inner_stats = self.get_nested_stats(else_if as *mut AstStat);
                result.extend(inner_stats);
              } else {
                eprintln!("AstStatIf's else clause can have more statement types than I thought");
                LUAU_ASSERT!(false);
              }
            }
          }
        } else {
          let w = ast_node_as::<AstStatWhile>(node_ptr);
          if !w.is_null() {
            append((*w).body);
          } else {
            let r = ast_node_as::<AstStatRepeat>(node_ptr);
            if !r.is_null() {
              append((*r).body);
            } else {
              let f = ast_node_as::<AstStatFor>(node_ptr);
              if !f.is_null() {
                append((*f).body);
              } else {
                let f_in = ast_node_as::<AstStatForIn>(node_ptr);
                if !f_in.is_null() {
                  append((*f_in).body);
                } else {
                  let f_func = ast_node_as::<AstStatFunction>(node_ptr);
                  if !f_func.is_null() {
                    append((*(*f_func).func).body);
                  } else {
                    let f_local = ast_node_as::<AstStatLocalFunction>(node_ptr);
                    if !f_local.is_null() {
                      append((*(*f_local).func).body);
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    result
  }
}
