#!/usr/bin/env python3
"""
知乎热榜直接获取 - 使用Playwright模拟浏览器
需要先登录知乎
"""

import asyncio
from playwright.async_api import async_playwright
import json
from datetime import datetime

class ZhihuDirectBrowser:
    def __init__(self):
        self.timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    
    async def get_hot_list(self):
        """使用浏览器获取热榜"""
        print("=" * 80)
        print("知乎热榜直接获取 - 浏览器模拟方式")
        print(f"时间: {self.timestamp}")
        print("=" * 80)
        
        print("\n⚠️ 注意：知乎需要登录才能查看完整热榜")
        print("请在浏览器中登录知乎账号，或者使用以下方法之一：")
        print()
        print("方法1：手动登录后使用Cookies")
        print("方法2：使用无头浏览器自动化登录")
        print("方法3：使用已登录的浏览器配置文件")
        print()
        
        print("正在尝试无头浏览器访问...")
        
        async with async_playwright() as p:
            # 启动浏览器
            browser = await p.chromium.launch(headless=True)
            context = await browser.new_context(
                user_agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
                viewport={'width': 1920, 'height': 1080}
            )
            
            page = await context.new_page()
            
            try:
                # 访问知乎热榜
                print("访问 https://www.zhihu.com/hot ...")
                await page.goto('https://www.zhihu.com/hot', timeout=30000)
                
                # 等待页面加载
                await page.wait_for_load_state('networkidle')
                
                # 检查是否需要登录
                current_url = page.url
                if 'login' in current_url or 'signin' in current_url:
                    print(f"❌ 需要登录！当前URL: {current_url}")
                    
                    # 尝试获取页面内容看看
                    content = await page.content()
                    
                    if '热门内容' in content or '热榜' in content:
                        print("✅ 页面包含热榜内容，尝试解析...")
                    else:
                        print("❌ 页面不包含热榜内容，需要登录")
                        await browser.close()
                        return None
                else:
                    print(f"✅ 访问成功，当前URL: {current_url}")
                
                # 截屏保存
                await page.screenshot(path='zhihu_hot_screenshot.png')
                print("✅ 页面截图已保存: zhihu_hot_screenshot.png")
                
                # 获取页面HTML
                html = await page.content()
                
                # 分析页面结构
                print("\n分析页面结构...")
                
                # 查找热榜元素
                hot_items = []
                
                # 尝试不同的选择器（知乎的热榜HTML结构）
                selectors = [
                    '.HotList-list .HotItem',
                    '.Topstory-isActive .TopstoryItem',
                    '.HotList .HotItem',
                    '[data-za-detail-view-path-module="HotList"] .HotItem',
                    '.HotList li',
                    '.TopstoryItem'
                ]
                
                for selector in selectors:
                    elements = await page.query_selector_all(selector)
                    if len(elements) > 5:
                        print(f"使用选择器 '{selector}' 找到 {len(elements)} 个项目")
                        
                        for i, element in enumerate(elements[:20], 1):
                            try:
                                # 提取标题
                                title_elem = await element.query_selector('.HotItem-title, .ContentItem-title, .HotItem-content a')
                                title = ""
                                if title_elem:
                                    title = await title_elem.text_content()
                                    title = title.strip()
                                
                                # 提取热度
                                hot_elem = await element.query_selector('.HotItem-metrics, .HotItem-meta, .ContentItem-extra')
                                hot_score = 0
                                if hot_elem:
                                    hot_text = await hot_elem.text_content()
                                    # 解析热度
                                    import re
                                    match = re.search(r'(\d+(?:\.\d+)?)', hot_text.replace(',', ''))
                                    if match:
                                        num = float(match.group(1))
                                        if '万' in hot_text:
                                            hot_score = int(num * 10000)
                                        else:
                                            hot_score = int(num)
                                
                                if title:
                                    hot_items.append({
                                        'rank': i,
                                        'title': title,
                                        'hot_score': hot_score
                                    })
                                
                            except Exception as e:
                                print(f"解析第{i}个项目失败: {e}")
                                continue
                        
                        break
                
                if not hot_items:
                    # 尝试更通用的方法：查找所有可能是热榜项目的元素
                    print("使用通用选择器未找到热榜，尝试搜索关键词...")
                    
                    # 获取页面文本
                    page_text = await page.inner_text('body')
                    
                    # 查找包含"热度"或"回答"的数字
                    import re
                    
                    # 查找类似热榜的结构
                    lines = page_text.split('\n')
                    for line in lines:
                        line = line.strip()
                        if line and len(line) > 10 and not line.startswith('http'):
                            # 简单的标题识别
                            hot_items.append({
                                'rank': len(hot_items) + 1,
                                'title': line[:100],
                                'hot_score': 0
                            })
                            if len(hot_items) >= 20:
                                break
                
                await browser.close()
                
                if hot_items:
                    return hot_items
                else:
                    print("❌ 未找到热榜数据")
                    return None
                
            except Exception as e:
                print(f"❌ 浏览器访问失败: {e}")
                await browser.close()
                return None
    
    def save_results(self, data):
        """保存结果"""
        if not data:
            print("❌ 没有数据可保存")
            return None
        
        output = {
            'timestamp': self.timestamp,
            'source': '知乎直接访问（浏览器模拟）',
            'total_items': len(data),
            'hot_list': data
        }
        
        filename = f'zhihu_hot_direct_{datetime.now().strftime("%Y%m%d_%H%M%S")}.json'
        
        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(output, f, ensure_ascii=False, indent=2)
        
        print(f"✅ 数据已保存到: {filename}")
        return filename
    
    def display_results(self, data):
        """显示结果"""
        if not data:
            print("❌ 没有获取到数据")
            return
        
        print("\n" + "=" * 80)
        print("📊 知乎热榜数据（直接获取）")
        print("=" * 80)
        
        print(f"\n共获取到 {len(data)} 个热榜话题\n")
        
        for item in data[:15]:
            rank = item['rank']
            title = item['title']
            hot_score = item['hot_score']
            
            hot_display = f"{hot_score/10000:.1f}万" if hot_score >= 10000 else f"{hot_score:,}"
            
            print(f"{rank:2d}. {title}")
            if hot_score > 0:
                print(f"    热度: {hot_display}")
            print()

async def main():
    zhihu = ZhihuDirectBrowser()
    data = await zhihu.get_hot_list()
    
    if data:
        zhihu.display_results(data)
        zhihu.save_results(data)
        
        print("\n🎯 总结:")
        print(f"• 使用浏览器模拟直接访问知乎")
        print(f"• 获取时间: {zhihu.timestamp}")
        print(f"• 数据量: {len(data)} 条")
    else:
        print("\n❌ 获取失败，可能需要登录")

if __name__ == "__main__":
    print("开始获取知乎热榜（浏览器方式）...")
    asyncio.run(main())