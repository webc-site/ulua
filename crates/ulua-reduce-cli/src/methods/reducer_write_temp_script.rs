use std::{fs::File, io::Write, process};

use ulua_ast::functions::pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block_cst_node_map;

use crate::records::reducer::Reducer;

impl Reducer {
  pub fn write_temp_script(&mut self, minify: bool) {
    // `root` 仅在 `run_from_source` 解析成功后落位，且任何试跑都发生在那之后
    // （见 `run_from_source` 的调用序）；句柄借出 `&mut` 供 printer 走 cpp 的
    // 非 const `prettyPrintWithTypes(*root, ...)` 语义，借用止于本次打印。
    let mut root = self
      .root
      .expect("write_temp_script 只在解析成功、root 落位后调用");
    let mut source =
      pretty_print_with_types_ast_stat_block_cst_node_map(root.get_mut(), &self.cst_node_map);

    if minify {
      // cpp 原地 erase 单个 '\n' 折叠连续空行：从命中处重扫，语义一致
      let mut pos = 0;
      while let Some(found_pos) = source[pos..].find("\n\n") {
        source.remove(pos + found_pos);
        pos += found_pos;
      }
    }

    let file = File::create(&self.script_name);
    let mut f = match file {
      Ok(f) => f,
      Err(_) => {
        println!("Unable to open temp script to {}", self.script_name);
        process::exit(2);
      }
    };

    for comment in &self.parse_result.hotcomments {
      if writeln!(f, "--!{}", comment.content).is_err() {
        println!("Unable to write to temp script {}", self.script_name);
        process::exit(3);
      }
    }

    if f.write_all(source.as_bytes()).is_err() {
      println!("Unable to write to temp script {}", self.script_name);
      process::exit(3);
    }
  }
}
