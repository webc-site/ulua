#[macro_export]
macro_rules! CASE {
    ($lhs:expr, $rhs:expr, $t:ty, $le:ident, $re:ident, $block:block) => {
        else if let (Some($le), Some($re)) = (
            unsafe { $crate::rtti::ast_node_as::<$t>($lhs as *mut $crate::records::ast_node::AstNode) }.as_ref(),
            unsafe { $crate::rtti::ast_node_as::<$t>($rhs as *mut $crate::records::ast_node::AstNode) }.as_ref(),
        ) $block
    };
}
