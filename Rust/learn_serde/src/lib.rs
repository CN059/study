pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// ## 使用派生宏可以方便的实现序列化和反序列化
/// 需要在 Cargo.toml 中添加 serde 和 serde_json 依赖，并且启用 serde 的 derive 功能。
/// ```toml
/// [dependencies]
/// serde = { version = "1.0", features = ["derive"] }
/// serde_json = "1.0"
/// ```
/// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    mod serde_json;
}
