use crate::{
  records::type_function_serializer::TypeFunctionSerializer,
  type_aliases::{type_function_kind::TypeFunctionKind, type_or_pack::TypeOrPack},
};

impl TypeFunctionSerializer {
  pub fn serialize_children_type_or_pack_type_function_kind(
    &mut self,
    kind: TypeOrPack,
    tfkind: TypeFunctionKind,
  ) {
    match (kind, tfkind) {
      (TypeOrPack::V0(ty), TypeFunctionKind::V0(tfti)) => {
        self.serialize_children_type_id_type_function_type_id(ty, tfti);
      }
      (TypeOrPack::V1(tp), TypeFunctionKind::V1(tftp)) => {
        self.serialize_children_type_pack_id_type_function_type_pack_id(tp, tftp);
      }
      _ => {}
    }
  }
}
