use std::string::String;

pub fn make_huge_function_source() -> String {
  let mut source = String::new();
  source.push_str("if ... then\n");
  source.push_str("local _ = {\n");
  for i in 0..40000 {
    source.push_str("0.");
    source.push_str(&format!("{}", i));
    source.push(',');
  }
  source.push_str("}\n");
  source.push_str("end\n");
  source.push_str("return bit32.lshift('84', -1)");
  source
}
