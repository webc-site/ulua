use alloc::string::String;

use crate::type_aliases::documentation_symbol::DocumentationSymbol;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct FunctionParameterDocumentation {
  pub(crate) name: String,
  pub(crate) documentation: DocumentationSymbol,
}
