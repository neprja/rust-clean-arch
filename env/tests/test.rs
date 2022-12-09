extern crate env;
use env::{env, *};

#[test]
fn test_env_string() {
    std::env::set_var("ENV_NAME", "key");

    let expected = match env!(ENV_NAME: String) {
        Ok(ok) => ok,
        _ => "".to_string(),
    };

    assert_eq!(expected, "key")
}
