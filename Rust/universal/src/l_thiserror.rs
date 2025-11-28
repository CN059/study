/// 该库为标准库提供了一个方便的派生宏 std::error::Error 特性。
use thiserror::Error;// 这个必须引入，我们实现的派生宏是thiserror的Error
use std::io;// 这个也要引入，下面的错误类型中用到了
#[derive(Error, Debug)]
pub enum DataStoreError {
    /// 这里的error是属性宏（procedural macro），用于为结构体、枚举、函数添加自定义的元数据
    /// 派生宏：一般只能用于为类型整体生成代码，而不能为单个字段或变体生成代码
    /// 类函数宏：通常用于生成代码片段，而不是为类型或字段添加元数据（比如println!宏）
    /// 
    /// 错误可以是枚举、带命名字段的结构体、元组结构体或单元结构体。
    /// 提供 #[error("...")] 可以为错误生成一个 Display trait 的 impl
    /// 
    /// 这些消息支持一种从错误中插值的简写形式。
    /// ```rust
    /// #[error("{var}")] ⟶ write!("{}", self.var)
    /// #[error("{0}")] ⟶ write!("{}", self.0)
    /// #[error("{var:?}")] ⟶ write!("{:?}", self.var)
    /// #[error("{0:?}")] ⟶ write!("{:?}", self.0)
    /// ```
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),

    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },

    #[error("unknown data store error")]
    Unknown,
}

#[test]
fn test_datastore_error(){
    let err1 = DataStoreError::Disconnect(io::Error::new(io::ErrorKind::Other, "connection lost"));
    let err2 = DataStoreError::Redaction("user_password".to_string());
    let err3 = DataStoreError::InvalidHeader {
        expected: "v1".to_string(),
        found: "v2".to_string(),
    };
    let err4 = DataStoreError::Unknown;
    print!("err1: {:?}", err1);
    print!("err2: {:?}", err2);
    print!("err3: {:?}", err3);
    print!("err4: {:?}", err4);
}
