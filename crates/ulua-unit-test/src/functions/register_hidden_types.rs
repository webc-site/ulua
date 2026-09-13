use ulua_analysis::records::frontend::Frontend;

pub fn register_hidden_types(frontend: &mut Frontend) {
  frontend.globals.register_hidden_test_types();
}
