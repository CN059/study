// 常用的验证函数（电子邮件、URL、长度等）和特性 - 可与 `validator_derive` 一起使用
// 字段验证库，如果要为结构体等使用，需要开启 derive 特性
// 这个库提供了很多已有的验证器函数可以直接调用

// A trait that the Validate derive will impl
#[derive(Debug, validator::Validate, serde::Deserialize)]
pub struct SignupData {
    // 预定义的验证器
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,

    // 两个验证器，第一个先校验长度是不是大于一，第二个再校验唯一用户名（使用自定义校验函数）
    // 自定义校验函数的返回类型必须是 Result<(), validator::ValidationError>

    // #[validate(length(min = 5, max = 20))] 这里可以使用min=?,max=?来指定长度范围
    #[validate(length(min = 1), custom(function = "validate_unique_username"))]
    // 使用serde的rename属性来指定序列化和反序列化时的字段名称
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,

    // exclusive_min表示不包含最小值，min表示包含最小值
    // 同理还有exclusive_max和max
    #[validate(range(exclusive_min = 0.0, max = 100.0))]
    height: f32,
}

fn validate_unique_username(username: &str) -> Result<(), validator::ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(validator::ValidationError::new("terrible_username123"));
    }
    Ok(())
}

// 这个函数使用了validate接受一个实现了Validate trait的类型作为参数，以对任意的实现了Validate trait的类型进行校验
// 返回一个anyhow::Result类型的错误，用来表示校验过程中发生的错误，anyhow就可以捕获任意的错误并返回给调用者
pub fn check_data<T>(data: &T) -> anyhow::Result<&T>
// 我们需要使用 where T: validator::Validate, 来指定T必须实现Validate trait
where T: validator::Validate,
{
    validator::Validate::validate(&data)?;
    Ok(data)
}

#[test]
fn test_validator_mail_error(){
    let data = SignupData {
        mail: "invalid_email".to_string(),
        site: "https://example.com".to_string(),
        first_name: "CN059".to_string(),
        age: 19,
        height: 50.0,
    };
    // 如果不引入validater::Validate trait，那么这里会报错
    // 使用完全限定语法即可解决问题
    match validator::Validate::validate(&data) {
        Ok(_) => println!("Validation passed"),
        Err(e) => println!("Validation errors: {:?}", e),
    }
}

#[test]
fn test_validator_check_data_function(){
    // 这里我们使用自定义的返回anyhow::Result类型的check_data函数来进行校验
    // 但是它的输出结果相比直接调用validator来说，有很多东西都没有输出，以下是对比
    // anyhow的：check_data: Validation errors: mail: Validation error: email [{"value": String("")}]
    // validator的：Validation errors: ValidationErrors({"mail": Field([ValidationError { code: "email", message: None, params: {"value": String("")} }])})
    let data = SignupData {
        mail: "".to_string(),
        site: "https://example.com".to_string(),
        first_name: "CN059".to_string(),
        age: 19,
        height: 50.0,
    };
    // 经过我的分析，我发现，ValidationError这个结构体，实现了Debug和Display
    // 上面提到的输出不一致就是这个原因
    // 但是Display并没有少输出关键的东西，只不过以一种另外的格式输出了
    // Debug则是直接输出序列化后的json格式
    print!("{}", colored::Colorize::blue("Using validator::Validate::validate"));
    match validator::Validate::validate(&data) {
        Ok(_) => println!("Validation passed"),
        Err(e) => println!("Validation errors: {:?}", e),
    }
    // 在研究这里的时候，我用到了colored这个库，可以给终端输出添加颜色
    // 这个库也有一些问题，它只为实现了Display的类型添加了颜色支持
    // 也就是说格式化字符串左侧的占位符必须是{}，而不能是{:?}，否则就不会有颜色输出
    // 同时右边也必须是字符串类型，&str也可以
    match check_data(&data) {
        Ok(_) => println!("check_data: Validation passed"),
        Err(e) => 
            println!("check_data: Validation errors: {}", colored::Colorize::red(e.to_string().as_str())),
    }
    println!("{}", colored::Colorize::red("hello"));
}
