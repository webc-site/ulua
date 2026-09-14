use crate::{
  enums::solver_mode::SolverMode,
  records::{
    frontend::Frontend, frontend_module_resolver::FrontendModuleResolver,
    frontend_options::FrontendOptions,
  },
};

pub fn get_module_resolver(
  frontend: &mut Frontend,
  options: Option<FrontendOptions>,
) -> &mut FrontendModuleResolver {
  if (frontend.get_luau_solver_mode() == SolverMode::New) || options.is_none() {
    return &mut frontend.module_resolver;
  }

  if options.as_ref().unwrap().for_autocomplete {
    &mut frontend.module_resolver_for_autocomplete
  } else {
    &mut frontend.module_resolver
  }
}
