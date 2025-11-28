use anyhow::Context;

#[derive(Debug)]
pub struct Limits {
    #[allow(dead_code)]
    lo: usize,
    #[allow(dead_code)]
    hi: usize,
}
pub fn first_char(s: &str) -> char {
    s.chars().next().unwrap_or_default()
}
#[derive(thiserror::Error, std::fmt::Debug)]
pub enum ExampleErrors{
    #[error("invalid rdo_lookahead_frames {0} (expected < {max})", max = i32::MAX)]
    InvalidLookahead(u32),

    #[error("first letter must be lowercase but was {:?}", first_char(.0))]
    WrongCase(String),

    #[error("invalid index {idx}, expected at least {} and at most {}", .limits.lo, limits.hi)]
    OutOfBounds { idx: usize, limits: Limits },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("this is a custom error,no actual use")]
    Aerror,
}

// 这里我们定义一个函数，返回一个anyhow::Result类型的错误，用来模拟程序执行中产生的错误
pub fn make_error()->anyhow::Result<()>{
    Err(ExampleErrors::Aerror.into())
}

// 这个函数调用make_error函数，接收到了错误，并使用？运算符传播错误
pub fn invoke_and_appear_error()->anyhow::Result<()>{
    make_error()?;
    Ok(())
}

#[test]
fn test_anyhow_spread_error(){
    // 这里我们获取的是invoke_and_appear_error函数返回的Result,它有可能是Ok,不过我们的逻辑让它必定是Err了
    let _ =invoke_and_appear_error().unwrap();
    // 按照这样的调用，实际输出的时候程序会panic，因为unwrap在遇到Err时会触发panic
    // panic里面的message就是我们使用thiserror定义的自定义错误类型，实际执行也可以看到输出中包含了this is a custom error,no actual use

    /* 
        thread 'l_anyhow::test_anyhow_error' (4178930) panicked at src/l_anyhow.rs:43:38:
        called `Result::unwrap()` on an `Err` value: this is a custom error,no actual use
    */
}

// 这个函数调用make_error函数，接收到了错误，并使用with_context方法为错误添加更多的上下文信息
// 实际的上下文信息，可以通过闭包传递真实的文件，句柄等等的详细信息
pub fn invoke_and_take_more_meaasge()->anyhow::Result<()>{
    make_error().with_context(|| "invoke_and_take_more_meaasge failed")?;
    Ok(())
}
#[test]
fn test_anyhow_take_more_message_error(){
    // 这个例子测试anyhow携带更多的错误上下文，通过给with_context传递一个闭包来实现
    let _ = invoke_and_take_more_meaasge().unwrap();

    /* 
        thread 'l_anyhow::test_anyhow_take_more_message_error' (4182971) panicked at src/l_anyhow.rs:61:44:
        called `Result::unwrap()` on an `Err` value: invoke_and_take_more_meaasge failed
    */
}

#[test]
fn make_error_by_anyhow(){
    // 直接使用anyhow创建一个错误
    Err(anyhow::anyhow!("this is an anyhow error")).unwrap()
}