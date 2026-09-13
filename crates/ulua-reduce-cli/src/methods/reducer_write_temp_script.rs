use std::{fs::File, io::Write, process};

use ulua_ast::functions::pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block_cst_node_map;

use crate::records::reducer::Reducer;

impl Reducer {
  pub fn write_temp_script(&mut self, minify: bool) {
    let mut source = pretty_print_with_types_ast_stat_block_cst_node_map(
      unsafe { &mut *self.root },
      self.cst_node_map.clone(),
    );

    if minify {
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
