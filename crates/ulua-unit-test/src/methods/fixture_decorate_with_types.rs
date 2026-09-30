use alloc::string::String;

use ulua_analysis::{
  functions::attach_type_data::attach_type_data, type_aliases::module_name_type::ModuleName,
};
use ulua_ast::functions::pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block;

use crate::{functions::raw_handle::raw_handle, records::fixture::Fixture};

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  pub fn decorate_with_types(&mut self, code: &str) -> String {
    let module_name = ModuleName::from(MAIN_MODULE_NAME);
    self
      .file_resolver
      .source
      .insert(module_name.clone(), code.to_owned());

    let frontend = self.get_frontend();
    frontend.mark_dirty(&module_name, None);
    let _type_info = frontend.check_module_name_optional_frontend_options(&module_name, None);

    let source_module = frontend
      .get_source_module_mut(&module_name)
      .expect("decorateWithTypes: 检查后主模块的 SourceModule 应在场");
    let module = frontend.module_resolver.get_module(&module_name);

    // Safety: source_module 句柄指向 frontend.source_modules 表内存活的 SourceModule
    // （行 18 的 get_frontend 借用覆盖至此）；raw_handle(&module) 为 resolver
    // Arc<Module> 堆块地址（强引用存活至语句末）；两 &mut 对应 cpp
    // attachTypeData(SourceModule&, Module&) 顺序变更，借用止于调用；
    // source_module.root 为 arena 存活块，pretty_print 只读。
    unsafe {
      let source_module = source_module.get_mut();
      attach_type_data(source_module, &mut *(raw_handle(&module)));
      pretty_print_with_types_ast_stat_block(&mut *source_module.root)
    }
  }
}
