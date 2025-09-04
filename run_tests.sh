#!/bin/bash

# Funlog 测试运行脚本

set -e

echo "🧪 Funlog 测试套件"
echo "=================="

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 函数：打印带颜色的消息
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 检查参数
case "${1:-all}" in
    "all")
        echo "运行所有测试（不包括忽略的测试）..."
        cargo test
        print_status "所有活跃测试完成"
        ;;
    "ignored")
        echo "运行被忽略的测试..."
        cargo test -- --ignored
        print_warning "忽略的测试完成（可能需要特殊环境）"
        ;;
    "full")
        echo "运行所有测试（包括忽略的测试）..."
        cargo test -- --include-ignored
        print_status "完整测试套件完成"
        ;;
    "params")
        echo "运行参数配置相关测试..."
        cargo test --test raw_test_params_all
        cargo test --test raw_test_params_none  
        cargo test --test raw_test_params_specific
        print_status "参数配置测试完成"
        ;;
    "positions")
        echo "运行位置控制相关测试..."
        cargo test --test raw_test_position_start
        cargo test --test raw_test_position_end
        print_status "位置控制测试完成"
        ;;
    "levels")
        echo "运行日志级别相关测试..."
        cargo test --test raw_test_debug
        cargo test --test raw_test_info
        cargo test --test raw_test_warn
        cargo test --test raw_test_error
        print_status "日志级别测试完成"
        ;;
    "return")
        echo "运行返回值相关测试..."
        cargo test --test raw_test_return_value
        print_status "返回值测试完成"
        ;;
    "combined")
        echo "运行组合配置测试..."
        cargo test --test raw_test_combined_options
        print_status "组合配置测试完成"
        ;;
    "void")
        echo "运行无返回值函数测试..."
        cargo test --test raw_test_void_functions
        print_status "无返回值函数测试完成"
        ;;
    "complex")
        echo "运行复杂类型测试..."
        cargo test --test raw_test_complex_types
        print_status "复杂类型测试完成"
        ;;
    "edge")
        echo "运行边界情况测试..."
        cargo test --test raw_test_edge_cases
        print_status "边界情况测试完成"
        ;;
    "print")
        echo "运行 Print 模式测试..."
        cargo test --test raw_test_print_modes -- --include-ignored
        print_warning "Print 模式测试完成（需要 stdout 捕获）"
        ;;
    "coverage")
        echo "生成测试覆盖率报告..."
        echo ""
        echo "📊 测试统计："
        echo "=============="
        
        # 统计测试文件数量
        test_files=$(find tests -name "*.rs" | wc -l)
        echo "测试文件总数: $test_files"
        
        # 运行测试并统计
        echo ""
        echo "运行测试统计..."
        cargo test 2>&1 | grep -E "(test result:|running [0-9]+ test)" | tail -20
        
        print_status "测试覆盖率统计完成"
        ;;
    "help"|"-h"|"--help")
        echo ""
        echo "用法: $0 [选项]"
        echo ""
        echo "选项:"
        echo "  all        运行所有活跃测试（默认）"
        echo "  ignored    仅运行被忽略的测试"
        echo "  full       运行所有测试（包括忽略的）"
        echo "  params     运行参数配置测试"
        echo "  positions  运行位置控制测试"
        echo "  levels     运行日志级别测试"
        echo "  return     运行返回值测试"
        echo "  combined   运行组合配置测试"
        echo "  void       运行无返回值函数测试"
        echo "  complex    运行复杂类型测试"
        echo "  edge       运行边界情况测试"
        echo "  print      运行 Print 模式测试"
        echo "  coverage   显示测试覆盖率统计"
        echo "  help       显示此帮助信息"
        echo ""
        echo "示例:"
        echo "  $0 all      # 运行所有活跃测试"
        echo "  $0 params   # 仅运行参数相关测试"
        echo "  $0 full     # 运行完整测试套件"
        ;;
    *)
        print_error "未知选项: $1"
        echo "使用 '$0 help' 查看可用选项"
        exit 1
        ;;
esac

echo ""
echo "🎉 测试运行完成！"