//! `DemoFileResolver::readSource` (`CLI/src/Web.cpp:18-25`).
//!
//! ```cpp
//! std::optional<Luau::SourceCode> readSource(const Luau::ModuleName& name) override
//! {
//!     auto it = source.find(name);
//!     if (it == source.end())
//!         return std::nullopt;
//!
//!     return Luau::SourceCode{it->second, Luau::SourceCode::MODULE};
//! }
//! ```

use ulua_analysis::{records::source_code::SourceCode, type_aliases::module_name_type::ModuleName};

use crate::records::demo_file_resolver::DemoFileResolver;

impl DemoFileResolver {
  pub fn read_source(&self, name: &ModuleName) -> Option<SourceCode> {
    let it = self.source.get(name)?;
    Some(SourceCode {
      source: it.clone(),
      r#type: SourceCode::MODULE,
    })
  }
}
