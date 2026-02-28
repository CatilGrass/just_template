# just_template

> 只是通过模板生成代码的工具

## 模板文件编写规则

`just_template` 用于生成重复性代码，包含三个核心概念：`实现标记`、`实现块` 和 `参数替换` 

1. 实现标记
   用一行以 10 个连续 `>` 开头的行来标记一个“实现点”，格式为 `>>>>>>>>>> NAME`
   该标记用于定位，系统将提取对应名称的“实现块”内容，并在此处进行重复展开。

2. 实现块
   用以下语法声明一个实现块，它可以定义一段可重复生成的代码模板：
   `@@@ NAME >>>`
   [模板内容]
   `@@@ <<<`
   在实现块内部，可以使用形如 `<<<PARAM>>>` 的参数占位符。
   当通过指令（如 `insert_impl!`）为某个实现块添加具体实现时，系统会复制该块的内容，替换其中的参数，并将结果追加到对应的实现标记处

3. 参数
   - 写在实现块之外的参数会被直接进行全局替换
   - 写在实现块之内的参数，会在为该块生成每个具体实现时，被替换为对应的值

使用示例：

原始模板文件内容：
```rust
// Auto generated
use std::collections::HashMap;

pub async fn my_func(
    name: &str,
    data: &[u8],
    params: &HashMap<String, String>,
) -> Option<Result<Vec<u32>, std::io::Error>> {
    match name {
>>>>>>>>>> arms
@@@ >>> arms
        "<<<crate_name>>>" => Some(<<<crate_name>>>::exec(data, params).await),
@@@ <<<
        _ => None,
    }
}
```

执行添加实现的指令：
```rust
tmpl!(tmpl += {
    arms {
        (crate_name = "my"),
        (crate_name = "you")
    }
});
```

系统会为 `arms` 实现块生成两个具体实现，此时实现块内容变为：
```rust
        "my" => Some(my::exec(data, params).await),
        "you" => Some(you::exec(data, params).await),
```

最终，模板展开生成的代码如下：
```rust
// Auto generated
use std::collections::HashMap;

pub async fn my_func(
    name: &str,
    data: &[u8],
    params: &HashMap<String, String>,
) -> Option<Result<Vec<u32>, std::io::Error>> {
    match name {
        "my" => Some(my::exec(data, params).await),
        "you" => Some(you::exec(data, params).await),
        _ => None,
    }
}
```

## 安装

将此添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
just_template = "0.1"
```

## 许可证

本项目采用 MIT 和 Apache 2.0 双重许可。
详见 LICENSE 文件。
