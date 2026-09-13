use alloc::string::String;

use crate::records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef};

impl BytecodeBuilder {
  pub fn set_debug_function_name(&mut self, name: StringRef) {
    let index = self.add_string_table_entry(name);

    self.functions[self.current_function as usize].debugname = index;

    // C++: `functions[currentFunction].dumpname = std::string(name.data, name.length);`
    // dumpname is the function's debug NAME (e.g. 'foo'), shown in DUPCLOSURE
    // constant dumps — NOT the function's bytecode dump. The port erroneously
    // invoked the dump function pointer here.
    if self.dump_function_ptr.is_some() {
      self.functions[self.current_function as usize].dumpname =
        String::from_utf8_lossy(name.as_bytes()).into_owned();
    }
  }
}
