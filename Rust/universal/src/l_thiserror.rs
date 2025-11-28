/* /// 该库为标准库提供了一个方便的派生宏 std::error::Error 特性。
use thiserror::Error;
use std::error;
// 这个必须引入，我们实现的派生宏是thiserror的Error
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
 */
/* #[test]
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
} */

// std::error::Error
// 使用派生宏实现Debug,可以用{:?}打印
// Debug是一个标准库中的 trait，定义在 std::fmt 模块中。它的作用是为类型提供调试信息的格式化输出。
// Debug trait 的输出主要面向开发者作用是为类型提供调试信息的格式化输出
#[derive(Debug)]
pub struct User{
    id:u32,
    name:String,
    age:u8,
}

// 手动实现Display trait.
// 不能使用派生宏实现Display trait，这个Display trait其实就是允许我们println!("{}", user)这种打印方式
// 而Debug trait允许我们println!("{:?}", user)这种打印方式
// 这两种都可以用来打印结构体的详细信息
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f是一个可变引用，指向一个 fmt::Formatter 结构体，用于写入格式化后的字符串
        // fmt::Formatter 结构体是一个复杂的格式化器，后面深入研究一下，它的底层似乎是一个缓冲区
        // 返回值是 fmt::Result 类型，表示格式化操作的结果
        //pub type Result = result::Result<(), Error>;
        // 其实就是fmt这个mod定义的自己的result类型，默认Ok就是()，Err就是fmt::Error
        write!(f, "[impl Display] User {{ id: {}, name: {}, age: {} }}", self.id, self.name, self.age)
        // write是一个类函数宏，类似于println!宏，功能就是把右边格式化后的字符串写入到左边的f中
    }
}
// 这里就是手动实现 std::error::Error trait
impl std::error::Error for User {

    // 返回导致当前错误的下层错误（如果存在）
    // 用于构建错误链，帮助调试和错误追踪
    // 默认实现返回 None ,意思就是，这个错误不是由底层的什么错误导致的
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
        // 这里的返回值可以是None,也可以是Some(),这里是None
        // 如果是Some()，里面放的是一个满足 std::error::Error trait ，并且生命周期是'static 的引用
        // 生命周期为'static 意味着这个引用在程序的整个生命周期内都是有效的，这是Rust关于错误处理的常见要求
    }
}
#[test]
fn test_std_debug_trait(){
    println!();
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        age: 30,
    };
    println!("user: {:?}", user);// 这里会调用Debug trait的fmt方法
}
#[test]
fn test_std_display_trait(){ 
    println!();
    let user = User {
        id: 2,
        name: "Bob".to_string(),
        age: 25,
    };
    println!("user: {}", user);// 这里会调用Display trait的fmt方法
}

// thiserror
// 为标准库提供了一个方便的派生宏 std::error::Error trait。可以用于快速自定义错误类型
#[derive(thiserror::Error, std::fmt::Debug)]
pub enum DataStoreError {
    // 枚举变量内部携带一些其他类型的数据，是代数数据类型（ADTs）
    // 这种特性允许你在枚举的每个变体中存储不同类型的数据，从而使枚举不仅仅是简单的标签，还能携带额外的信息。
    /* 
        在 Rust 中，枚举的每个变体可以是以下三种形式：
        1、单元变体：不携带任何数据。
        2、元组变体：携带一个或多个未命名的数据。
        3、结构体变体：携带一个或多个命名的数据。
    */
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    // 通过 #[from] 属性宏，可以自动实现 From trait，这样就可以直接使用 std::io::Error 转换为 DataStoreError

    // 这里的Redaction变体携带一个 String 类型的数据，{0} 是一个占位符，用于在错误消息中插入该数据。
    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    // 用于表示复杂的错误，字段名称可以提高代码的可读性。
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },

    // #[error("{var}")] ⟶ write!("{}", self.var)
    // #[error("{0}")] ⟶ write!("{}", self.0)
    // #[error("{var:?}")] ⟶ write!("{:?}", self.var)
    // #[error("{0:?}")] ⟶ write!("{:?}", self.0)
    #[error("unknown data store error")]
    Unknown,
}
/* 
```
fn handle_error(error: DataStoreError) {
        match error {
        // 枚举变体里面携带的额外参数在match匹配时就很有用处
            DataStoreError::Disconnect(io_err) => {
                println!("I/O error: {}", io_err);
            }
            DataStoreError::Redaction(key) => {
                println!("Key redacted: {}", key);
            }
            DataStoreError::InvalidHeader { expected, found } => {
                println!("Invalid header: expected {}, found {}", expected, found);
            }
            DataStoreError::Unknown => {
                println!("Unknown error");
            }
        }
    }
``` 
*/


#[test]
fn test_datastore_error(){
    println!("{}",DataStoreError::Disconnect(std::io::Error::new(std::io::ErrorKind::Other, "connection lost")));
    println!("{}",DataStoreError::Redaction("user_password".to_string()));
    println!("{}",DataStoreError::InvalidHeader {
        expected: "v1".to_string(),
        found: "v2".to_string(),
    });
    println!("{}",DataStoreError::Unknown);
}

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
    // 这些简写形式可以与任何其他格式参数一起使用，这些参数可以是任意表达式。
    #[error("invalid rdo_lookahead_frames {0} (expected < {max})", max = i32::MAX)]
    InvalidLookahead(u32),
    #[error("first letter must be lowercase but was {:?}", first_char(.0))]
    WrongCase(String),

    // 一般来讲，如果要使用到结构体字段里面的结构体实例的成员，那么访问这个实例，前面要加点，就像上面的.0一样
    // 不过有一个省略规则，我们可以省略前面的点，就像limits.hi一样
    #[error("invalid index {idx}, expected at least {} and at most {}", .limits.lo, limits.hi)]
    OutOfBounds { idx: usize, limits: Limits },

    // 这里的#[from]属性宏可以自动实现 From trait，这样就可以直接使用 std::io::Error 转换为 ExampleErrors::Io
    // 当我们在某个函数中使用？运算符的时候，如果错误类型是std::io::Error，那么它就会自动转换为Io错误类型
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    // #[from] 属性始终意味着同一个字段也是 #[source] ，因此您永远不需要同时指定这两个属性。
    // 任何实现了 std::error::Error 或对 dyn std::error::Error 进行解引用的错误类型都可以作为错误源。
}
#[test]
fn test_example_errors(){
    let limit=Limits { lo: 0, hi: 5 };
    println!("{}",ExampleErrors::InvalidLookahead(5000));
    println!("{}",ExampleErrors::WrongCase("Hello".to_string()));
    println!("{}",ExampleErrors::OutOfBounds { idx: 10, limits: limit});
    println!("{}的首字母是{}", ExampleErrors::WrongCase("Hello".to_string()), first_char("Hello"));
}
