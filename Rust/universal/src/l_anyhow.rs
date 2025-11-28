use anyhow::{Result, anyhow};

// 使用anyhow创建错误
// 这里的Result是anyhow::Result的别名，实现了anyhow自己的错误处理机制
pub fn divide(dividend: f64, divisor: f64) -> Result<f64> {
    if divisor == 0.0 {
        Err(anyhow!("Division by zero is not allowed"))
    } else {
        Ok(dividend / divisor)
    }
}

#[test]
fn divide_zero(){
    let result = divide(10.0, 0.0);
    // print!("result: {:?}", result);
    assert!(result.is_err());
}