#!/usr/bin/env python3
import requests
import json
import re
import time
from datetime import datetime

class ZhihuHotSimple:
    def __init__(self):
        self.session = requests.Session()
        # 使用更完整的浏览器头部
        self.session.headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0',
            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7',
            'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6',
            'Accept-Encoding': 'gzip, deflate, br',
            'Connection': 'keep-alive',
            'Upgrade-Insecure-Requests': '1',
            'Sec-Fetch-Dest': 'document',
            'Sec-Fetch-Mode': 'navigate',
            'Sec-Fetch-Site': 'none',
            'Sec-Fetch-User': '?1',
            'Cache-Control': 'max-age=0',
            'sec-ch-ua': '"Not_A Brand";v="8", "Chromium";v="120", "Microsoft Edge";v="120"',
            'sec-ch-ua-mobile': '?0',
            'sec-ch-ua-platform': '"Windows"',
        }
    
    def try_different_approaches(self):
        """尝试不同的方法获取热榜数据"""
        approaches = [
            self.try_direct_api,
            self.try_mobile_api,
            self.try_search_api,
            self.try_alternative_domains
        ]
        
        for approach in approaches:
            print(f"\n尝试方法: {approach.__name__}")
            result = approach()
            if result:
                return result
        
        return None
    
    def try_direct_api(self):
        """尝试直接API"""
        try:
            url = "https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total"
            params = {'limit': 50}
            response = self.session.get(url, params=params, timeout=10)
            if response.status_code == 200:
                return response.json()
        except:
            pass
        return None
    
    def try_mobile_api(self):
        """尝试移动端API"""
        try:
            # 移动端热榜
            url = "https://api.zhihu.com/topstory/hot-list"
            headers = {
                'x-api-version': '3.0.76',
                'x-app-version': '10.10.0',
                'x-app-za': 'OS=Android',
            }
            response = self.session.get(url, headers=headers, timeout=10)
            if response.status_code == 200:
                return response.json()
        except:
            pass
        return None
    
    def try_search_api(self):
        """尝试搜索相关API"""
        try:
            # 知乎热榜搜索
            url = "https://www.zhihu.com/api/v4/search/top_search"
            response = self.session.get(url, timeout=10)
            if response.status_code == 200:
                return response.json()
        except:
            pass
        return None
    
    def try_alternative_domains(self):
        """尝试替代域名"""
        domains = [
            "https://zhihu.com",
            "https://www.zhihu.com",
            "https://api.zhihu.com"
        ]
        
        for domain in domains:
            try:
                url = f"{domain}/hot"
                response = self.session.get(url, timeout=10)
                if response.status_code == 200:
                    # 尝试从HTML中提取数据
                    return self.extract_from_html(response.text)
            except:
                continue
        
        return None
    
    def extract_from_html(self, html):
        """从HTML中提取数据（简化版）"""
        try:
            # 简单的正则匹配
            pattern = r'"title":"([^"]+)".*?"score":(\d+)'
            matches = re.findall(pattern, html)
            
            if matches:
                data = {"data": []}
                for title, score in matches[:50]:
                    item = {
                        "target": {
                            "title": title,
                            "id": "0",
                            "answer_count": 0
                        },
                        "detail_text": f"{score} 万热度"
                    }
                    data["data"].append(item)
                return data
        except:
            pass
        return None
    
    def run(self):
        print("=" * 80)
        print("知乎热榜获取工具")
        print(f"当前时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print("=" * 80)
        
        print("\n正在尝试获取热榜数据...")
        
        data = self.try_different_approaches()
        
        if not data:
            print("\n⚠️ 所有方法都失败了，可能是以下原因：")
            print("1. 知乎加强了反爬虫机制")
            print("2. 需要登录或验证")
            print("3. IP地址被限制")
            print("4. API接口已变更")
            
            print("\n🎯 建议的替代方案：")
            print("1. 使用官方API（需要申请API key）")
            print("2. 使用Selenium等浏览器自动化工具")
            print("3. 使用第三方热榜聚合网站")
            print("4. 使用知乎App截图")
            
            # 提供模拟数据示例
            print("\n📊 以下是模拟的知乎热榜数据（示例）：")
            self.show_sample_data()
            return
        
        print(f"\n✅ 成功获取热榜数据")
        self.display_results(data)
    
    def show_sample_data(self):
        """显示示例数据"""
        sample_hot = [
            {"title": "2026年两会热议话题：人工智能发展与应用", "hot": 12500000, "answers": 3450},
            {"title": "新能源汽车价格战持续，消费者该如何选择？", "hot": 9800000, "answers": 2890},
            {"title": "ChatGPT-5正式发布，有哪些新功能值得关注？", "hot": 8700000, "answers": 4560},
            {"title": "房地产市场新政策解读", "hot": 7600000, "answers": 3120},
            {"title": "大学生就业形势分析与建议", "hot": 6500000, "answers": 2780},
            {"title": "健康饮食新趋势：植物基食品受追捧", "hot": 5400000, "answers": 1890},
            {"title": "电影《流浪地球3》预告片发布引发热议", "hot": 4300000, "answers": 2450},
            {"title": "中国航天最新进展：月球基地建设计划", "hot": 3800000, "answers": 1670},
            {"title": "职场中如何有效提升沟通能力", "hot": 3200000, "answers": 1340},
            {"title": "家庭教育新观念：尊重孩子个性发展", "hot": 2800000, "answers": 980},
        ]
        
        for i, item in enumerate(sample_hot, 1):
            print(f"{i:2d}. {item['title'][:50]}...")
            print(f"    热度: {item['hot']:,} | 回答数: {item['answers']:,}")
            print()
    
    def display_results(self, data):
        """显示结果"""
        print("\n📈 知乎热榜 TOP 20")
        print("-" * 80)
        
        if 'data' in data:
            items = data['data']
        elif 'top_search' in data:
            items = data['top_search']['words']
        else:
            items = []
        
        for i, item in enumerate(items[:20], 1):
            if isinstance(item, dict):
                if 'target' in item:
                    # API v3格式
                    title = item['target'].get('title', '未知标题')
                    hot_text = item.get('detail_text', '0')
                    answer_count = item['target'].get('answer_count', 0)
                elif 'query' in item:
                    # 搜索格式
                    title = item.get('query', '未知标题')
                    hot_text = item.get('display_query', '0')
                    answer_count = item.get('answer_count', 0)
                else:
                    title = item.get('title', '未知标题')
                    hot_text = item.get('hot_text', '0')
                    answer_count = item.get('answer_count', 0)
            else:
                title = str(item)
                hot_text = "0"
                answer_count = 0
            
            # 提取热度数字
            hot_score = 0
            if isinstance(hot_text, str):
                hot_match = re.search(r'(\d+(?:\.\d+)?)', hot_text.replace(',', ''))
                if hot_match:
                    hot_score = float(hot_match.group(1))
                    if '万' in hot_text:
                        hot_score *= 10000
            
            print(f"{i:2d}. {title[:60]}...")
            print(f"    热度: {int(hot_score):,} ({hot_text}) | 回答数: {answer_count}")
            print()

if __name__ == "__main__":
    zhihu = ZhihuHotSimple()
    zhihu.run()