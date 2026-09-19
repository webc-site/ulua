#[macro_export]
macro_rules! luaL_error {
    ($l:expr, $fmt:expr $(, $($arg:expr),+ )? $(,)? ) => {{
        $crate::functions::lua_l_error_l::lua_l_error_l(
            $l,
            core::ptr::null(),
            core::format_args!($fmt $(, $($arg),* )?),
        );
    }};
}

pub use luaL_error;
