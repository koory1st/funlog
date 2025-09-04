# Funlog

[English](README_EN.md) | 中文

一个用于跟踪 Rust 函数调用的过程宏库。

## 简介

Funlog 是一个轻量级的 Rust 过程宏，用于自动记录函数的调用信息。它可以记录函数的参数、返回值，并支持多种日志级别和灵活的配置选项。

## 特性

- 🚀 **零运行时开销** - 仅在 debug 构建中生效，release 构建中完全移除
- 📝 **多种日志级别** - 支持 trace、debug、info、warn、error 和 print
- 🎯 **灵活的参数记录** - 可选择记录所有参数、指定参数或不记录参数
- ⏰ **位置控制** - 可在函数开始、结束或两者都记录
- 🔄 **返回值记录** - 可选择记录函数返回值
- 🛠️ **易于使用** - 简单的属性宏语法

## 安装

将以下内容添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
funlog = "0.1.0"

# 如果使用日志级别（非 print），还需要添加日志库
log = "0.4"
env_logger = "0.10"
```

## 快速开始

### 基本用法

```rust
use funlog::funlog;

#[funlog(debug)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    env_logger::init();
    let result = add(3, 5);
    println!("Result: {}", result);
}
```

### 使用 print（无需日志库）

```rust
use funlog::funlog;

#[funlog(print)]
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("World");
}
```

## 配置选项

### 日志级别

- `print` - 使用 `println!` 宏（无需日志库设置）
- `trace` - 使用 `log::trace!`
- `debug` - 使用 `log::debug!`
- `info` - 使用 `log::info!`
- `warn` - 使用 `log::warn!`
- `error` - 使用 `log::error!`

### 参数记录选项

- `all` - 记录所有函数参数（默认）
- `none` - 不记录参数
- `params(param1, param2, ...)` - 记录指定参数

### 位置控制选项

- `onStart` - 仅在函数开始时记录
- `onEnd` - 仅在函数结束时记录
- `onStartEnd` - 在开始和结束时都记录（默认）

### 返回值记录

- `retVal` - 在日志中包含返回值

## 使用示例

### 记录所有参数和返回值

```rust
#[funlog(info, all, retVal)]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

### 记录指定参数

```rust
#[funlog(debug, params(name, age))]
fn create_user(name: &str, age: u32, email: &str) -> String {
    format!("User: {} ({})", name, age)
}
```

### 仅在函数结束时记录

```rust
#[funlog(warn, onEnd, retVal)]
fn expensive_calculation() -> f64 {
    // 复杂计算
    std::thread::sleep(std::time::Duration::from_millis(100));
    42.0
}
```

### 组合多个选项

```rust
#[funlog(info, onStartEnd, params(input), retVal)]
fn process_data(input: &str, config: &Config) -> Result<String, Error> {
    // 处理逻辑
    Ok(input.to_uppercase())
}
```

## 示例项目

项目包含了丰富的示例，展示了所有功能组合：

```bash
# 运行基本示例
cargo run --example raw_debug

# 运行参数记录示例
cargo run --example raw_all
cargo run --example raw_params

# 运行位置控制示例
cargo run --example raw_position_start
cargo run --example raw_position_end

# 运行返回值记录示例
cargo run --example raw_return_value

# 查看所有示例
ls examples/
```

详细的示例说明请参考 [examples/README.md](examples/README.md)。

## 测试

运行所有测试：

```bash
# 运行单元测试
cargo test

# 运行示例测试脚本
./run_tests.sh
```

## 工作原理

Funlog 是一个过程宏，在编译时分析函数并生成相应的日志代码。它只在 debug 构建中生效，在 release 构建中会完全移除，确保零运行时开销。

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 作者

- Levy Gu <32436334@qq.com>

## 更新日志

### v0.1.0
- 初始版本发布
- 支持多种日志级别
- 支持灵活的参数和返回值记录
- 支持位置控制
- 完整的测试覆盖和示例