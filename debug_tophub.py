#!/usr/bin/env python3
"""
调试脚本：检查今日热榜页面实际内容
"""

import requests
from bs4 import BeautifulSoup

def debug_tophub():
    url = "https://tophub.today/n/KqndgxeLl9"
    
    session = requests.Session()
    session.headers.update({
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    })
    
    print(f"获取页面: {url}")
    response = session.get(url, timeout=15)
    
    print(f"状态码: {response.status_code}")
    print(f"内容长度: {len(response.text)} 字符")
    
    soup = BeautifulSoup(response.text, 'html.parser')
    
    print("\n=== 页面标题 ===")
    print(soup.title.string if soup.title else "无标题")
    
    print("\n=== 查找表格结构 ===")
    tables = soup.find_all('table')
    print(f"找到 {len(tables)} 个表格")
    
    for i, table in enumerate(tables):
        print(f"\n表格 {i+1}:")
        rows = table.find_all('tr')
        print(f"  行数: {len(rows)}")
        
        if rows:
            # 显示第一行的内容
            first_row = rows[0]
            cells = first_row.find_all(['td', 'th'])
            print(f"  第一行列数: {len(cells)}")
            for j, cell in enumerate(cells):
                text = cell.get_text(strip=True)
                print(f"    单元格 {j}: '{text}'")
    
    print("\n=== 查找热榜项目 ===")
    # 尝试不同的选择器
    selectors = [
        'tr',
        '.list tr',
        'table tr',
        '.item',
        '.hot-item'
    ]
    
    for selector in selectors:
        items = soup.select(selector)
        if len(items) > 10:
            print(f"\n选择器 '{selector}' 找到 {len(items)} 个项目")
            print("前3个项目的内容:")
            
            for k in range(min(3, len(items))):
                item = items[k]
                print(f"\n项目 {k+1}:")
                print(f"  HTML: {str(item)[:200]}...")
                print(f"  文本: {item.get_text(strip=True)[:100]}...")
                
                # 查找链接
                links = item.find_all('a')
                if links:
                    print(f"  链接: {len(links)} 个")
                    for link in links[:2]:
                        print(f"    - 文本: '{link.get_text(strip=True)[:50]}'")
                        print(f"      链接: {link.get('href', '无')}")
                
                # 查找数字（热度值）
                import re
                text = item.get_text()
                numbers = re.findall(r'\d+[,\.]?\d*[万亿]?', text)
                if numbers:
                    print(f"  找到数字: {numbers}")
            
            break
    
    print("\n=== 页面关键区域 ===")
    # 查找可能包含热榜的div
    divs_with_many_tr = soup.find_all(lambda tag: tag.name == 'div' and len(tag.find_all('tr')) > 5)
    print(f"包含多个tr的div数量: {len(divs_with_many_tr)}")
    
    if divs_with_many_tr:
        div = divs_with_many_tr[0]
        print(f"第一个div中的tr数量: {len(div.find_all('tr'))}")
        
        # 显示前5行
        rows = div.find_all('tr')[:5]
        for i, row in enumerate(rows):
            print(f"\n行 {i+1}:")
            text = row.get_text(strip=True)
            print(f"  完整文本: {text}")
            
            # 提取各部分
            parts = re.split(r'\s+', text)
            print(f"  分割部分: {parts}")

if __name__ == "__main__":
    debug_tophub()