use ulua_ast::records::ast_expr_unary::AstExprUnaryOp;

use crate::{enums::type_constant_folding::Type, records::constant::Constant};

pub fn fold_unary(result: &mut Constant, op: AstExprUnaryOp, arg: &Constant) {
  match op {
    AstExprUnaryOp::Not => {
      if arg.r#type != Type::Unknown {
        result.r#type = Type::Boolean;
        result.data.value_boolean = !arg.is_truthful();
      }
    }
    AstExprUnaryOp::Minus => {
      if arg.r#type == Type::Number {
        result.r#type = Type::Number;
        unsafe {
          result.data.value_number = -arg.data.value_number;
        }
      } else if arg.r#type == Type::Vector {
        result.r#type = Type::Vector;
        unsafe {
          result.data.value_vector[0] = -arg.data.value_vector[0];
          result.data.value_vector[1] = -arg.data.value_vector[1];
          result.data.value_vector[2] = -arg.data.value_vector[2];
          result.data.value_vector[3] = -arg.data.value_vector[3];
        }
      }
    }
    AstExprUnaryOp::Len => {
      if arg.r#type == Type::String {
        result.r#type = Type::Number;
        result.data.value_number = arg.string_length as f64;
      }
    }
  }
}
