macro_rules! VCONST {
    (@kind Nil) => {
        crate::enums::r#type::Type::Nil
    };
    (@kind Boolean) => {
        crate::enums::r#type::Type::Boolean
    };
    (@kind Number) => {
        crate::enums::r#type::Type::Number
    };
    (@kind Integer) => {
        crate::enums::r#type::Type::Integer
    };
    (@kind Vector) => {
        crate::enums::r#type::Type::Vector
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
                && $constants[$v as usize].r#type == VCONST!(@kind $kind)
        )
    };
}

pub(crate) use VCONST;
