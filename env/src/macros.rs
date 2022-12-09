#[macro_export]
macro_rules! str {
    ($env:expr) => {{
        env_string($env.into())
    }};

    ($env:expr, default: $default:expr) => {
        env_string_default($env.into(), $default.into())
    };
}

#[macro_export]
macro_rules! i64 {
    ($env:expr) => {
        env_i64($env.into())
    };

    ($env:expr, default: $default:expr) => {
        env_i64_default($env.into(), $default.into())
    };
}

#[macro_export]
macro_rules! u16 {
    ($env:expr) => {
        env_u16($env.into())
    };

    ($env:expr, default: $default:expr) => {
        env_u16_default($env.into(), $default.into())
    };
}
