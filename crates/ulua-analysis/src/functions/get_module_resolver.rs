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

  // 上方已对 `options.is_none()` 早退，此处为 Some；`is_some_and` 合并判定与取值，
  // 与 `as_ref().unwrap().for_autocomplete` 行为等价且无判空分支。
  if options.is_some_and(|o| o.for_autocomplete) {
    &mut frontend.module_resolver_for_autocomplete
  } else {
    &mut frontend.module_resolver
  }
}
