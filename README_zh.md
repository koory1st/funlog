# funlog

funlog 是一个强大而简洁的 Rust 函数日志记录宏。它通过简单的属性标注，帮助你追踪函数的执行过程，自动记录参数、返回值和执行位置等信息。

## 特性

- 🔍 自动记录函数调用信息
  - 函数进入和退出时刻
  - 参数值和类型
  - 返回值和类型
  - 代码位置（文件名、行号）

- 💡 智能类型支持
  - Result 和 Option 类型
  - 泛型类型（需实现 Debug trait）
  - 自定义结构体
  - 复杂类型组合

- ⚙️ 灵活配置
  - 多级日志（debug、info、warn、error、trace）
  - 可控输出长度
  - 详细的上下文信息

## 快速开始

1. 添加依赖
```toml
[dependencies]
funlog = "0.1.0"
log = "0.4"
env_logger = "0.10"
```

2. 初始化日志
```rust
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default())
        .filter_level(log::LevelFilter::Info)
        .init();
}
```

3. 使用宏
```rust
use funlog::funlog;

// 记录参数和返回值
#[funlog(param, ret)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 使用示例

### 基础函数日志
```rust
#[funlog(param, ret)]
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

let result = multiply(6, 7);
```
输出：
```
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:4 my_app::multiply(x: 6, y: 7) begin
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:4 my_app::multiply(x: 6, y: 7)->42 end
```

### Result 类型
```rust
#[funlog(param, ret)]
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(x / y)
    }
}
```
输出：
```
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:10 my_app::divide(x: 10, y: 2) begin
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:10 my_app::divide(x: 10, y: 2)->Ok(5) end
```

### Option 类型
```rust
#[funlog(param, ret)]
fn find_first(list: &[i32]) -> Option<i32> {
    list.first().copied()
}
```
输出：
```
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:15 my_app::find_first(list: [1, 2, 3]) begin
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:15 my_app::find_first(list: [1, 2, 3])->Some(1) end
```

## 配置选项

### 宏属性
- `param` - 记录参数
- `ret` - 记录返回值
- `debug/info/warn/error/trace` - 日志级别

### 环境变量
```bash
# 设置日志级别
RUST_LOG=debug cargo run
```

## 注意事项

1. 类型要求
   - 需要记录的类型必须实现 `Debug` trait
   - 可通过 `#[derive(Debug)]` 自动实现

2. 日志格式
   - 时间戳（UTC）
   - 日志级别
   - 文件位置
   - 函数信息
   - 参数和返回值

3. 性能
   - 编译期生成代码，运行时开销小
   - 可通过日志级别控制输出

## 许可证

MIT License - 详见 LICENSE 文件
