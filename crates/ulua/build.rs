use std::{env, fs, path::Path};

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
  let cases_dir = Path::new(&manifest_dir).join("../../benchmarks/cases");
  println!("cargo:rerun-if-changed={}", cases_dir.display());

  let out_dir = env::var("OUT_DIR").unwrap();
  fs::create_dir_all(&out_dir).ok();
  let dest_path = Path::new(&out_dir).join("benchmarks_cases.rs");

  let mut cases: Vec<String> = Vec::new();
  if let Ok(entries) = fs::read_dir(&cases_dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      if path.extension().and_then(|s| s.to_str()) == Some("lua")
        && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
      {
        cases.push(stem.to_string());
      }
    }
  }

  cases.sort();

  let mut code = String::from(
    "/// 自动扫描 benchmarks/cases/ 目录生成的基准程序分发宏\nmacro_rules! for_each_benchmark {\n  ($macro:ident) => {\n",
  );
  for name in &cases {
    code.push_str(&format!(
      "    $macro!({name}, include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/../../benchmarks/cases/{name}.lua\")));\n"
    ));
  }
  code.push_str("  };\n}\n");

  fs::write(&dest_path, code).expect("写入 benchmarks_cases.rs 失败");
}
