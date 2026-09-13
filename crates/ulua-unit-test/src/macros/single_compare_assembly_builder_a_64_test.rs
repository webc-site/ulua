#[macro_export]
macro_rules! SINGLE_COMPARE {
    ($inst:expr, $($args:expr),*) => {
        $crate::CHECK!(
            $crate::check(
                |build: &mut $crate::AssemblyBuilderA64| {
                    build.$inst;
                },
                &[$($args),*]
            )
        );
    };
}

pub use SINGLE_COMPARE;
