// C++ `luaG_runerror(l, fmt, ...)` — per the project varargs convention the
// format string is a Rust format literal and the C fmt parameter is unused
#[macro_export]
macro_rules! lua_g_runerror {
    ($l:expr, $fmt:expr $(, $($arg:expr),+ )? $(,)? ) => {{
        $crate::functions::lua_g_runerror_l::lua_g_runerror_l(
            $l,
            core::ptr::null(),
            format_args!($fmt $(, $($arg),* )?),
        )
    }};
}

pub use lua_g_runerror;
pub use lua_g_runerror as luaG_runerror;
