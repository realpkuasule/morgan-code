#!/bin/bash

echo "开始测试所有知乎热榜获取脚本..."
echo "========================================"
echo

# 创建结果目录
mkdir -p test_results

echo "1. 测试 zhihu_hot.py (API方式)..."
echo "----------------------------------------"
python3 zhihu_hot.py 2>&1 | tee test_results/test1_api.txt
echo

echo "2. 测试 zhihu_hot_html.py (HTML解析)..."
echo "----------------------------------------"
python3 zhihu_hot_html.py 2>&1 | tee test_results/test2_html.txt
echo

echo "3. 测试 zhihu_hot_simple.py (多种方法尝试)..."
echo "----------------------------------------"
python3 zhihu_hot_simple.py 2>&1 | tee test_results/test3_simple.txt
echo

echo "4. 测试 zhihu_hot_final.py (最终方案)..."
echo "----------------------------------------"
python3 zhihu_hot_final.py 2>&1 | tee test_results/test4_final.txt
echo

echo "5. 检查网络连接..."
echo "----------------------------------------"
curl -s -I https://www.zhihu.com 2>&1 | tee test_results/test5_network.txt
echo

echo "6. 尝试获取知乎首页..."
echo "----------------------------------------"
timeout 5 curl -s https://www.zhihu.com | head -100 2>&1 | tee test_results/test6_homepage.txt
echo

echo "========================================"
echo "测试完成！结果已保存到 test_results/ 目录"
echo

# 生成总结报告
echo "生成测试总结报告..."
echo "========================================" > test_results/summary.txt
echo "知乎热榜获取脚本测试总结" >> test_results/summary.txt
echo "测试时间: $(date)" >> test_results/summary.txt
echo "========================================" >> test_results/summary.txt
echo >> test_results/summary.txt

echo "各脚本测试结果:" >> test_results/summary.txt
echo "----------------" >> test_results/summary.txt

# 检查每个测试的结果
check_result() {
    local test_file=$1
    local test_name=$2
    
    echo -n "$test_name: " >> test_results/summary.txt
    if grep -q "成功\|成功获取\|✅" "$test_file"; then
        echo "✓ 成功" >> test_results/summary.txt
    elif grep -q "失败\|无法获取\|⚠️\|403\|401\|302" "$test_file"; then
        echo "✗ 失败" >> test_results/summary.txt
    else
        echo "? 未知" >> test_results/summary.txt
    fi
}

check_result "test_results/test1_api.txt" "API方式"
check_result "test_results/test2_html.txt" "HTML解析"
check_result "test_results/test3_simple.txt" "多种方法"
check_result "test_results/test4_final.txt" "最终方案"

echo >> test_results/summary.txt
echo "详细错误分析:" >> test_results/summary.txt
echo "--------------" >> test_results/summary.txt

# 提取错误信息
for file in test_results/test*.txt; do
    echo "$(basename $file):" >> test_results/summary.txt
    grep -E "失败|错误|Error|403|401|302|Forbidden|Authorization" "$file" | head -3 >> test_results/summary.txt
    echo >> test_results/summary.txt
done

echo "建议方案:" >> test_results/summary.txt
echo "----------" >> test_results/summary.txt
cat << 'EOF' >> test_results/summary.txt
1. 浏览器自动化 (Selenium) - 最可靠但最复杂
2. 第三方API服务 - 最稳定但可能需要付费
3. 移动端API模拟 - 技术门槛高
4. 聚合网站爬取 - 最简单但数据可能不及时
EOF

cat test_results/summary.txt