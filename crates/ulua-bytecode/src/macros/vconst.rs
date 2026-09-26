macro_rules! VCONST {
    (@kind Number) => {
        crate::enums::r#type::Type::Number
    };
    (@kind String) => {
        crate::enums::r#type::Type::String
    };
    (@kind Import) => {
        crate::enums::r#type::Type::Import
    };
    (@kind Table) => {
        crate::enums::r#type::Type::Table
    };
    (@kind Closure) => {
        crate::enums::r#type::Type::Closure
    };
    (@kind ClassShape) => {
        crate::enums::r#type::Type::ClassShape
    };
    ($v:expr, $kind:ident, $constants:expr) => {
        LUAU_ASSERT!(
            ($v as usize) < $constants.len()
                && $constants[$v as usize].kind() == VCONST!(@kind $kind)
        )
    };
}

pub(crate) use VCONST;
