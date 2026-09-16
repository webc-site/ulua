#[macro_export]
macro_rules! LUAU_REQUIRE_ERROR {
    ($result:expr, $type:ty) => {{
        using T = $type;
        const res = ($result);
        if !$crate::functions::find_error::find_error::<T>(&res) {
            $crate::functions::dump_errors::dump_errors(&res);
            $crate::REQUIRE_MESSAGE!(false, "Expected to find {} error", stringify!($type));
        }
    }};
}

pub use LUAU_REQUIRE_ERROR;
