use ulua_ast::records::ast_name::AstName;

use crate::{
  functions::{follow_type, get_type::get},
  records::{
    infinite_type_finder::InfiniteTypeFinder, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::{error_type::ErrorType, name_type::Name, type_id::TypeId},
};

impl InfiniteTypeFinder {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found_infinite_type
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    if self.found_infinite_type {
      return false;
    }

    let name = ast_name_to_name(petv.name);
    let tf = unsafe {
      let scope = self.scope.as_ref();
      if let Some(prefix) = petv.prefix {
        scope.lookup_imported_type(&ast_name_to_name(prefix), &name)
      } else {
        scope.lookup_type(&name)
      }
    };

    let Some(tf) = tf else {
      return true;
    };

    if follow_type::follow(tf.r#type()) != follow_type::follow(self.signature.fn_sig.r#type()) {
      return true;
    }

    // 对齐 C++：比较前 follow 双方，任一侧为 ErrorType 时跳过该对
    // （error 不能作为「类型别名参数不同」的证据）。
    for (argument, parameter) in petv.type_arguments.iter().zip(tf.type_params()) {
      let pending_type_arg = follow_type::follow(*argument);
      let tf_type_param = follow_type::follow(parameter.ty);
      if get::<ErrorType>(pending_type_arg).is_some() || get::<ErrorType>(tf_type_param).is_some() {
        continue;
      }
      if pending_type_arg != tf_type_param {
        self.found_infinite_type = true;
        return false;
      }
    }

    for (argument, parameter) in petv.pack_arguments.iter().zip(tf.type_pack_params()) {
      if *argument != parameter.tp {
        self.found_infinite_type = true;
        return false;
      }
    }

    false
  }
}

fn ast_name_to_name(name: AstName) -> Name {
  // as_str_or_empty 已对 null 返回 ""，无需再判空
  name.as_str_or_empty().to_string()
}
