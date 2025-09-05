# Funlog 错误提示测试示例

这个目录包含了用于测试 funlog 宏错误提示改进的示例文件。每个示例都故意包含错误的用法，用来验证错误信息的质量和有用性。

## 测试文件说明

### 1. `error_test_duplicate_log_levels.rs`
**测试内容**: 重复日志级别配置
**错误类型**: `ConflictingOptions`
**预期错误信息**:
```
funlog 配置冲突: 'debug' 和 'info' 不能同时使用
💡 提示: 请选择其中一个选项
```

**测试用例**:
- 同时使用两个日志级别: `#[funlog(debug, info)]`
- 同时使用三个日志级别: `#[funlog(trace, debug, warn)]`
- print 和其他级别冲突: `#[funlog(print, error)]`

### 2. `error_test_invalid_parameters.rs`
**测试内容**: 无效参数名
**错误类型**: `InvalidParameter`
**预期错误信息**:
```
funlog 参数错误: 参数 'nonexistent_param' 不存在
💡 提示: 可用的参数有: valid_param, another_param
   正确用法: #[funlog(params(valid_param, another_param))]
```

**测试用例**:
- 参数名不存在: `#[funlog(params(nonexistent_param))]`
- 部分参数名错误: `#[funlog(params(valid_param, wrong_param))]`
- 在无参数函数中使用 params: `#[funlog(params(some_param))]`

### 3. `error_test_spelling_mistakes.rs`
**测试内容**: 拼写错误和智能建议
**错误类型**: `InvalidAttribute`
**预期错误信息**:
```
funlog 配置错误: 未知的配置选项 'debgu'
💡 提示: 你是想使用 'debug' 吗？
📖 可用的配置选项:
   日志级别: print, trace, debug, info, warn, error
   参数控制: all, none, params(参数名...)
   位置控制: onStart, onEnd, onStartEnd
   返回值: retVal
```

**测试用例**:
- 各种属性的拼写错误: `debgu`, `prnit`, `tarce`, `ifno`, `wran`, `eror`
- 位置控制拼写错误: `onStrat`, `onEdn`
- 其他选项拼写错误: `retVla`, `al`, `non`

### 4. `error_test_conflicting_options.rs`
**测试内容**: 配置选项冲突
**错误类型**: `ConflictingOptions`
**预期错误信息**:
```
funlog 配置冲突: 'all' 和 'none' 不能同时使用
💡 提示: 请选择其中一个选项
```

**测试用例**:
- 参数配置冲突: `all` vs `none`, `all` vs `params()`, `none` vs `params()`
- 位置配置冲突: `onStart` vs `onEnd`, `onStart` vs `onStartEnd`
- 复杂的多重冲突

### 5. `error_test_syntax_errors.rs`
**测试内容**: 语法错误和格式问题
**错误类型**: `InvalidAttribute`, `ParseError`
**预期错误信息**:
```
funlog 配置错误: 未知的配置选项 'param(...)'
💡 提示: 你是想使用 'params' 吗？
```

**测试用例**:
- `params` 写成 `param`: `#[funlog(param(a))]`
- 不支持的列表格式: `#[funlog(level(debug))]`
- 空的参数列表: `#[funlog(params())]`
- 嵌套结构: `#[funlog(config(debug, params(a)))]`

### 6. `error_test_edge_cases.rs`
**测试内容**: 边缘情况和特殊错误
**错误类型**: `InvalidAttribute`
**预期错误信息**: 根据具体情况提供建议或显示可用选项

**测试用例**:
- 完全未知的属性: `completely_unknown_attribute`
- 大小写错误: `DEBUG`
- 其他语言关键字: `console`
- 相似但错误的属性: `retValue`, `debugLevel`
- 复数形式: `errors`
- 缺少/多余字母: `deb`, `debugg`

## 如何运行测试

### 方法 1: 使用测试脚本（推荐）
```bash
# 运行所有错误测试
./test_error_messages.sh
```

### 方法 2: 单独测试每个示例
```bash
# 测试重复日志级别
cargo check --example error_test_duplicate_log_levels

# 测试无效参数
cargo check --example error_test_invalid_parameters

# 测试拼写错误
cargo check --example error_test_spelling_mistakes

# 测试配置冲突
cargo check --example error_test_conflicting_options

# 测试语法错误
cargo check --example error_test_syntax_errors

# 测试边缘情况
cargo check --example error_test_edge_cases
```

### 方法 3: 批量测试
```bash
# 测试所有错误示例（会显示所有编译错误）
for example in error_test_*; do
    echo "Testing $example..."
    cargo check --example "${example%.rs}"
    echo "---"
done
```

## 预期结果

所有这些示例都**应该**产生编译错误，这是预期的行为。重点是验证：

1. **错误信息质量**: 错误信息是否清晰易懂
2. **建议有用性**: 是否提供了有用的修复建议
3. **中文本地化**: 错误信息是否使用了友好的中文
4. **智能建议**: 拼写错误是否得到了正确的建议
5. **上下文感知**: 错误信息是否包含了相关的上下文信息

## 评估标准

好的错误信息应该包含：
- ✅ 清晰的问题描述
- ✅ 具体的错误位置
- ✅ 可行的修复建议
- ✅ 相关的帮助信息
- ✅ 友好的语言表达

## 注意事项

1. 这些示例文件**不应该**被正常编译
2. 错误信息的具体格式可能因 Rust 编译器版本而略有不同
3. 某些复杂的语法错误可能在更早的解析阶段被捕获
4. 测试时请关注错误信息的实用性而不是完美的格式

## 贡献

如果你发现了新的错误场景或有改进建议，请：
1. 添加新的测试用例到相应的文件中
2. 更新这个 README 文档
3. 确保错误信息提供了有用的建议