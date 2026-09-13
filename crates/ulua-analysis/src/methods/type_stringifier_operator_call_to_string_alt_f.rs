//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:585:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:585-623` (hand-ported)

use ulua_common::FFlag;

use crate::{
  records::{
    primitive_type::{PrimitiveType, Type},
    type_stringifier::TypeStringifier,
  },
  type_aliases::type_id::TypeId,
};
impl TypeStringifier {
  /// C++ `void operator()(TypeId, const PrimitiveType& ptv)`.
  pub fn operator_call_17(&mut self, _ty: TypeId, ptv: &PrimitiveType) {
    let state = unsafe { &mut *self.state };
    match ptv.r#type {
      Type::NilType => state.emit("nil"),
      Type::Boolean => state.emit("boolean"),
      Type::Number => state.emit("number"),
      Type::String => state.emit("string"),
      Type::Thread => state.emit("thread"),
      Type::Buffer => state.emit("buffer"),
      Type::Function => state.emit("function"),
      Type::Table => state.emit("table"),
      Type::Integer => {
        if FFlag::LuauIntegerType2.get() {
          state.emit("integer");
        } else {
          // C++ [[fallthrough]] to: throw InternalCompilerError("Unknown primitive type")
          panic!("Unknown primitive type {:?}", ptv.r#type);
        }
      }
    }
  }
}
