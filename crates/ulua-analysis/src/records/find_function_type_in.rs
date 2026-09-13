use crate::records::{function_type::FunctionType, iterative_type_visitor::IterativeTypeVisitor};

#[derive(Debug, Clone)]
pub struct FindFunctionTypeIn {
  pub base: IterativeTypeVisitor,
  pub number_of_lambda_parameters: i32,
  pub candidate: *const FunctionType,
}
