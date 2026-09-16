use alloc::string::String;

use ulua_common::{
  enums::luau_bytecode_type::{LBC_TYPE_FUNCTION, LBC_TYPE_OPTIONAL_BIT},
  functions::format_append::formatAppend,
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  functions::get_base_type_string::get_base_type_string, records::bytecode_builder::BytecodeBuilder,
};

impl BytecodeBuilder {
  pub fn dump_type_info(&self) -> String {
    let mut result = String::new();

    for (i, function) in self.functions.iter().enumerate() {
      let typeinfo = function.typeinfo.as_bytes();
      if typeinfo.is_empty() {
        continue;
      }

      let encoded_type = typeinfo[0];

      LUAU_ASSERT!(encoded_type == LBC_TYPE_FUNCTION.0 as u8);

      formatAppend(&mut result, format_args!("{}: function(", i));

      LUAU_ASSERT!(typeinfo.len() >= 2);

      let numparams = typeinfo[1];

      LUAU_ASSERT!((1 + numparams as usize - 1) < typeinfo.len());

      for (j, &et) in typeinfo[2..2 + numparams as usize].iter().enumerate() {
        let optional = if (et & LBC_TYPE_OPTIONAL_BIT.0 as u8) != 0 {
          "?"
        } else {
          ""
        };

        let base_type_str = get_base_type_string(et);
        formatAppend(&mut result, format_args!("{}{}", base_type_str, optional));

        if j + 1 != numparams as usize {
          formatAppend(&mut result, format_args!(", "));
        }
      }

      formatAppend(&mut result, format_args!(")\n"));
    }

    result
  }
}
