#[macro_export]
macro_rules! NONSTRICT_REQUIRE_FUNC_DEFINITION_ERR {
    ($pos:expr, $argname:expr, $result:expr) => {
        let pos_ = $pos;
        let argname_ = $argname;
        let result_ = &$result;
        let mut err_index = 0;
        $crate::macros::nonstrict_require_err_at_pos::NONSTRICT_REQUIRE_ERR_AT_POS!(
            pos_, result_, err_index
        );
        let err = $crate::functions::get::get::<
            $crate::records::error::NonStrictFunctionDefinitionError,
        >(&result_.errors[err_index]);
        assert!(
            err.is_some(),
            "Expected NonStrictFunctionDefinitionError at {:?}",
            pos_
        );
        let err = err.unwrap();
        assert_eq!(err.argument, argname_);
    };
}

pub use NONSTRICT_REQUIRE_FUNC_DEFINITION_ERR;
