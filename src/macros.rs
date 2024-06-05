#[macro_export]
macro_rules! ifcfg {
    ($feature:expr, $($item:tt)*) => {
        #[cfg(feature = $feature)]
        {
            $($item)*
        }
    };
}
