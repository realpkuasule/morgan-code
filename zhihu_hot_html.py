#!/usr/bin/env python3
import requests
import json
import re
from typing import List, Dict, Optional
import time
from bs4 import BeautifulSoup

class ZhihuHotHTML:
    def __init__(self):
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
            'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8',
            'Accept-Encoding': 'gzip, deflate, br',
            'Connection': 'keep-alive',
            'Upgrade-Insecure-Requests': '1',
        })
    
    def fetch_hot_page(self) -> Optional[str]:
        """获取知乎热榜HTML页面"""
        try:
            url = "https://www.zhihu.com/hot"
            response = self.session.get(url, timeout=10)
            response.raise_for_status()
            
            # 检查是否是验证页面
            if "验证" in response.text or "verification" in response.text.lower():
                print("遇到验证页面，尝试模拟浏览器...")
                return None
            
            return response.text
        except Exception as e:
            print(f"获取热榜页面失败: {e}")
            return None
    
    def parse_hot_items(self, html: str) -> List[Dict]:
        """从HTML解析热榜项目"""
        items = []
        
        soup = BeautifulSoup(html, 'html.parser')
        
        # 尝试不同的选择器
        selectors = [
            '.HotList-list .HotItem',  # 常见的选择器
            '[data-za-detail-view-path-module="HotItem"]',
            '.css-1nd7dqm',  # 可能的类名
            'div[class*="HotItem"]'
        ]
        
        hot_items = None
        for selector in selectors:
            hot_items = soup.select(selector)
            if hot_items:
                break
        
        if not hot_items:
            print("未找到热榜项目，尝试其他方法...")
            # 尝试直接查找包含热度数字的元素
            hot_elements = soup.find_all(text=re.compile(r'\d+\.?\d*万?'))
            for element in hot_elements[:50]:
                if element.parent and '热度' in str(element.parent):
                    print(f"找到热度元素: {element}")
        
        for item in hot_items[:50]:  # 限制前50个
            try:
                # 提取标题
                title_elem = item.select_one('.HotItem-title')
                if not title_elem:
                    title_elem = item.select_one('h2, .title, [class*="title"]')
                
                title = title_elem.get_text(strip=True) if title_elem else "未知标题"
                
                # 提取热度
                hot_elem = item.select_one('.HotItem-metrics')
                if not hot_elem:
                    hot_elem = item.select_one('[class*="metrics"], [class*="hot"]')
                
                hot_text = hot_elem.get_text(strip=True) if hot_elem else "0"
                
                # 解析热度数字
                hot_score = 0
                hot_match = re.search(r'(\d+(?:\.\d+)?)', hot_text.replace(',', ''))
                if hot_match:
                    hot_score = float(hot_match.group(1))
                    if '万' in hot_text:
                        hot_score *= 10000
                
                # 提取链接
                link_elem = item.select_one('a')
                url = link_elem.get('href', '') if link_elem else ''
                if url and not url.startswith('http'):
                    url = 'https://www.zhihu.com' + url
                
                # 提取问题ID
                question_id = ''
                if url:
                    q_match = re.search(r'/question/(\d+)', url)
                    if q_match:
                        question_id = q_match.group(1)
                
                items.append({
                    'title': title,
                    'hot_score': int(hot_score),
                    'hot_text': hot_text,
                    'url': url,
                    'question_id': question_id
                })
                
            except Exception as e:
                continue
        
        return items
    
    def get_answer_count(self, question_id: str) -> int:
        """获取问题回答数"""
        try:
            if not question_id:
                return 0
            
            # 使用知乎API v4
            url = f"https://www.zhihu.com/api/v4/questions/{question_id}"
            headers = {
                'authorization': 'Bearer Mi4xaGJXQUFBQUFBQUFBY0lMN2JDeGJDeGNBQUFCaEFsVk5QN1hEV2dEQUxqN3FGVmVQT0VEeHNWS2prZkllYk40WVFB',
                'x-requested-with': 'fetch',
                'x-zse-93': '101_3_3.0',
            }
            
            response = self.session.get(url, headers=headers, timeout=5)
            if response.status_code == 200:
                data = response.json()
                return data.get('answer_count', 0)
        except Exception as e:
            print(f"获取回答数失败({question_id}): {e}")
        
        return 0
    
    def run(self):
        print("正在获取知乎热榜数据(HTML方式)...")
        print("=" * 80)
        
        html = self.fetch_hot_page()
        if not html:
            print("无法获取热榜HTML")
            return
        
        items = self.parse_hot_items(html)
        
        if not items:
            print("解析热榜失败，显示原始HTML片段...")
            # 保存HTML用于调试
            with open('zhihu_hot_debug.html', 'w', encoding='utf-8') as f:
                f.write(html[:5000])
            print("已保存HTML片段到 zhihu_hot_debug.html")
            return
        
        print(f"共解析到 {len(items)} 个热榜话题\n")
        
        # 显示所有项目
        for i, item in enumerate(items, 1):
            print(f"{i:2d}. {item['title'][:60]}...")
            print(f"    热度: {item['hot_score']:,} ({item['hot_text']})")
            
            # 获取回答数
            if item['question_id']:
                answer_count = self.get_answer_count(item['question_id'])
                if answer_count > 0:
                    print(f"    回答数: {answer_count}")
            
            if item['url']:
                print(f"    链接: {item['url'][:80]}...")
            print()

if __name__ == "__main__":
    zhihu = ZhihuHotHTML()
    zhihu.run()