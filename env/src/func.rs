use anyhow::{Context, Result};

pub fn env_string(env: String) -> Result<String> {
    let env = &env;
    std::env::var(env).context(format!("environment '{env}': must be specified"))
}

pub fn env_string_default(env: String, default: String) -> Result<String> {
    Ok(std::env::var(env).unwrap_or(default))
}

pub fn env_i64(env: String) -> Result<i64> {
    let env = &env;
    std::env::var(env)
        .context(format!("environment '{env}': must be specified"))?
        .parse::<i64>()
        .context(format!("environment '{env}': parse i64"))
}

pub fn env_i64_default(env: String, default: String) -> Result<i64> {
    let env = &env;
    std::env::var(env)
        .unwrap_or(default)
        .parse::<i64>()
        .context(format!("default environment '{env}': parse i64"))
}

pub fn env_u16(env: String) -> Result<u16> {
    let env = &env;
    std::env::var(env)
        .context(format!("environment '{env}': must be specified"))?
        .parse::<u16>()
        .context(format!("environment '{env}': parse u16"))
}

pub fn env_u16_default(env: String, default: String) -> Result<u16> {
    let env = &env;
    std::env::var(env)
        .unwrap_or(default)
        .parse::<u16>()
        .context(format!("default environment '{env}': parse u16"))
}
