#!/usr/bin/env python3
import requests
import json
import re
from typing import List, Dict, Optional
import time

class ZhihuHot:
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
    
    def fetch_hot_list(self) -> Optional[Dict]:
        """获取知乎热榜数据"""
        try:
            # 知乎热榜API
            url = "https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total"
            params = {
                'limit': 50,
                'desktop': 'true'
            }
            
            response = self.session.get(url, params=params, timeout=10)
            response.raise_for_status()
            
            return response.json()
        except Exception as e:
            print(f"获取热榜失败: {e}")
            return None
    
    def extract_hot_items(self, data: Dict) -> List[Dict]:
        """提取热榜项目信息"""
        items = []
        
        if not data or 'data' not in data:
            return items
        
        for item in data['data']:
            try:
                target = item.get('target', {})
                
                # 提取热度值
                detail_text = item.get('detail_text', '')
                hot_score = 0
                hot_match = re.search(r'(\d+(?:\.\d+)?)', detail_text.replace(',', ''))
                if hot_match:
                    hot_score = float(hot_match.group(1))
                    if '万' in detail_text:
                        hot_score *= 10000
                
                # 提取回答数
                answer_count = target.get('answer_count', 0)
                
                # 获取问题ID用于获取真实回答数
                question_id = str(target.get('id', ''))
                
                hot_item = {
                    'title': target.get('title', ''),
                    'hot_score': int(hot_score),
                    'answer_count': answer_count,
                    'question_id': question_id,
                    'url': f"https://www.zhihu.com/question/{question_id}" if question_id else '',
                    'excerpt': target.get('excerpt', '')
                }
                items.append(hot_item)
            except Exception as e:
                print(f"解析项目失败: {e}")
                continue
        
        # 按热度排序
        items.sort(key=lambda x: x['hot_score'], reverse=True)
        return items
    
    def get_accurate_answer_count(self, question_id: str) -> int:
        """获取准确回答数"""
        try:
            url = f"https://www.zhihu.com/api/v4/questions/{question_id}"
            response = self.session.get(url, timeout=5)
            if response.status_code == 200:
                data = response.json()
                return data.get('answer_count', 0)
        except:
            pass
        return 0
    
    def run(self):
        print("正在获取知乎热榜数据...")
        print("=" * 80)
        
        data = self.fetch_hot_list()
        if not data:
            print("无法获取热榜数据")
            return
        
        items = self.extract_hot_items(data)
        
        print(f"共获取到 {len(items)} 个热榜话题\n")
        
        # 显示前20个
        for i, item in enumerate(items[:20], 1):
            # 获取更准确的回答数
            accurate_count = self.get_accurate_answer_count(item['question_id'])
            if accurate_count > 0:
                answer_count = accurate_count
            else:
                answer_count = item['answer_count']
            
            print(f"{i:2d}. {item['title'][:60]}...")
            print(f"    热度: {item['hot_score']:,} | 回答数: {answer_count}")
            if item['excerpt']:
                print(f"    简介: {item['excerpt'][:80]}...")
            print()

if __name__ == "__main__":
    zhihu = ZhihuHot()
    zhihu.run()