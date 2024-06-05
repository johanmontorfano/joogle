/// Conditionally includes code if a specific feature is enabled.
#[macro_export]
macro_rules! ifcfg {
    ($feature:expr, $($item:tt)*) => {
        #[cfg(feature = $feature)]
        {
            $($item)*
        }
    };
}

/// Does a compilation error when trying to compile the source for a release
/// with specific flags enabled.
#[macro_export]
macro_rules! forbid_prod_flag {
    ($feature:expr) => {
        #[cfg(all(feature = $feature, not(debug_assertions)))]
        compile_error!(concat!($feature, " can't be used in prod."));
    };
}
