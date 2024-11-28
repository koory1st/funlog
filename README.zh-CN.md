# funlog

简体中文 | [English](README.md)

一个用于函数日志记录的 Rust 属性宏，具有可配置选项。

## 特性

- 记录函数的进入和退出
- 参数值日志记录
- 返回值日志记录
- 泛型类型支持
- 可配置的日志级别
- 输出长度限制
- 支持嵌套函数调用

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
funlog = "0.1.0"
```

## 使用方法

### 基础日志记录

```rust
use funlog::funlog;

#[funlog]
fn hello() {
    println!("Hello!");
}
```

### 参数日志记录

```rust
#[funlog(param)]
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

### 返回值日志记录

```rust
#[funlog(ret)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 泛型函数日志记录

```rust
#[funlog(param, gener)]
fn print_item<T: std::fmt::Display + std::fmt::Debug>(item: T) {
    println!("{}", item);
}
```

### 长度限制

限制记录的参数和返回值的长度：

```rust
#[funlog(param="10", ret="20")]
fn process_data(data: String) -> String {
    data.repeat(2)
}
```

### 日志级别

指定不同的日志级别（debug、info、warn、error、trace）：

```rust
#[funlog(debug)]
fn debug_function() {
    println!("Debug function called");
}
```

### 嵌套函数调用

跟踪函数调用链：

```rust
#[funlog(param, ret)]
fn add1(a: i32, b: i32) -> i32 {
    a + b
}

#[funlog(param, ret)]
fn add2(a: i32, b: i32) -> i32 {
    add1(a, b)
}
```

## 配置选项

- `param` - 启用参数日志记录
- `ret` - 启用返回值日志记录
- `gener` - 启用泛型类型信息日志记录
- `param="n"` - 将参数日志长度限制为 n 个字符
- `ret="n"` - 将返回值日志长度限制为 n 个字符
- 日志级别：
  - `debug`
  - `info`（默认）
  - `warn`
  - `error`
  - `trace`

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m '添加一些很棒的特性'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个 Pull Request
