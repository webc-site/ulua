const LBC_TYPE_VECTOR: i32 = 8;
const LBC_TYPE_ANY: i32 = 15;

pub fn luau_library_type_lookup(library: &str, member: &str) -> i32 {
  if library == "Vector3" && (member == "xAxis" || member == "yAxis") {
    return LBC_TYPE_VECTOR;
  }

  LBC_TYPE_ANY
}
