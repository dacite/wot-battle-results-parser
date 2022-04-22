/// A macro that tries to destructure `PickleValue` to the given variant,
/// wrapped in a `Result`. Used to avoid using if let everywhere and have the
/// entire code shift right. Once if let chains stablize, this is probably not
/// needed.
macro_rules! try_variant {
    ($target: expr, $pattern: path) => {{
        if let $pattern(value) = $target {
            Ok(value)
        } else {
            Err(anyhow::anyhow!("Wrong variant. Expected: {}", stringify!($pattern)))
        }
    }};
}

pub(crate) use try_variant; 