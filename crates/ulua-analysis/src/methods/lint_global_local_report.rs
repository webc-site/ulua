use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_global_local::LintGlobalLocal};

impl LintGlobalLocal {
  pub fn report(&mut self) {
    let context = self.context;
    let placeholder = unsafe { (*context).placeholder };

    for &gv in &self.global_refs {
      let g = unsafe { self.globals.find(&(*gv).name) };

      match g {
        None => unsafe {
          emit_warning(
            &mut *context,
            Code::UnknownGlobal,
            (*gv).base.base.location,
            format_args!(
              "Unknown global '{}'; consider assigning to it first",
              (*gv).name
            ),
          );
        },
        Some(g) if !g.assigned && !g.builtin => unsafe {
          emit_warning(
            &mut *context,
            Code::UnknownGlobal,
            (*gv).base.base.location,
            format_args!(
              "Unknown global '{}'; consider assigning to it first",
              (*gv).name
            ),
          );
        },
        Some(g) => {
          if let Some(ref replacement) = g.deprecated {
            unsafe {
              if !replacement.is_empty() {
                emit_warning(
                  &mut *context,
                  Code::DeprecatedGlobal,
                  (*gv).base.base.location,
                  format_args!(
                    "Global '{}' is deprecated, use '{}' instead",
                    (*gv).name,
                    replacement
                  ),
                );
              } else {
                emit_warning(
                  &mut *context,
                  Code::DeprecatedGlobal,
                  (*gv).base.base.location,
                  format_args!("Global '{}' is deprecated", (*gv).name),
                );
              }
            }
          }
        }
      }
    }

    for (_name, g) in self.globals.iter() {
      if !g.function_ref.is_empty() && g.assigned && unsafe { (*g.first_ref).name } != placeholder {
        let top = *g.function_ref.last().unwrap();

        unsafe {
          if !(*top).debugname.value.is_null() {
            emit_warning(
              &mut *context,
              Code::GlobalUsedAsLocal,
              (*g.first_ref).base.base.location,
              format_args!(
                "Global '{}' is only used in the enclosing function '{}'; consider changing it to local",
                (*g.first_ref).name,
                (*top).debugname
              ),
            );
          } else {
            emit_warning(
              &mut *context,
              Code::GlobalUsedAsLocal,
              (*g.first_ref).base.base.location,
              format_args!(
                "Global '{}' is only used in the enclosing function defined at line {}; consider changing it to local",
                (*g.first_ref).name,
                (*top).base.base.location.begin.line + 1
              ),
            );
          }
        }
      } else if g.assigned
        && !g.read_before_written
        && !g.defined_in_module_scope
        && unsafe { (*g.first_ref).name } != placeholder
      {
        unsafe {
          emit_warning(
            &mut *context,
            Code::GlobalUsedAsLocal,
            (*g.first_ref).base.base.location,
            format_args!(
              "Global '{}' is never read before being written. Consider changing it to local",
              (*g.first_ref).name
            ),
          );
        }
      }
    }
  }
}
