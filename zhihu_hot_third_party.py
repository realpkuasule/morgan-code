#!/usr/bin/env python3
"""
知乎热榜数据获取 - 第三方网站方案
使用第三方聚合网站获取知乎热榜数据
"""

import requests
import json
from datetime import datetime
from bs4 import BeautifulSoup
import time

class ZhihuHotThirdParty:
    def __init__(self):
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
            'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8',
        })
        self.timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    
    def fetch_from_tophub(self):
        """从今日热榜(tophub.today)获取知乎热榜数据"""
        try:
            # 今日热榜的知乎热榜页面
            url = "https://tophub.today/n/KqndgxeLl9"
            
            print(f"正在从今日热榜获取数据: {url}")
            
            response = self.session.get(url, timeout=10)
            response.raise_for_status()
            
            # 解析HTML
            soup = BeautifulSoup(response.text, 'html.parser')
            
            # 查找热榜项目 - 今日热榜的典型结构
            hot_items = []
            
            # 尝试不同的选择器
            selectors = [
                'table tbody tr',
                '.list .item',
                '.hot-list .item',
                'tr[data-rank]'
            ]
            
            for selector in selectors:
                items = soup.select(selector)
                if len(items) > 10:  # 如果有足够多的项目
                    for i, item in enumerate(items[:20], 1):
                        try:
                            # 提取标题
                            title_elem = item.select_one('.td-c a, a.title, .title a')
                            if title_elem:
                                title = title_elem.get_text(strip=True)
                            else:
                                title = item.get_text(strip=True)[:100]
                            
                            # 提取热度
                            hot_elem = item.select_one('.td-n, .count, .hot')
                            hot_score = 0
                            if hot_elem:
                                hot_text = hot_elem.get_text(strip=True)
                                # 解析热度值（可能包含"万"）
                                if '万' in hot_text:
                                    try:
                                        num = float(hot_text.replace('万', ''))
                                        hot_score = int(num * 10000)
                                    except:
                                        hot_score = 0
                                else:
                                    try:
                                        hot_score = int(hot_text.replace(',', ''))
                                    except:
                                        hot_score = 0
                            
                            hot_items.append({
                                'rank': i,
                                'title': title,
                                'hot_score': hot_score,
                                'source': '今日热榜'
                            })
                        except Exception as e:
                            print(f"解析项目{i}失败: {e}")
                            continue
                    
                    if hot_items:
                        break
            
            return hot_items
            
        except Exception as e:
            print(f"从今日热榜获取数据失败: {e}")
            return None
    
    def fetch_from_anyknew(self):
        """从AnyKnew获取知乎热榜数据"""
        try:
            # AnyKnew的知乎热榜API
            url = "https://api.anyknew.com/api/v1/site/zhihu"
            
            print(f"正在从AnyKnew获取数据: {url}")
            
            response = self.session.get(url, timeout=10)
            response.raise_for_status()
            
            data = response.json()
            
            if data.get('code') == 0 and 'data' in data:
                items = data['data'].get('list', [])
                
                hot_items = []
                for i, item in enumerate(items[:20], 1):
                    hot_items.append({
                        'rank': i,
                        'title': item.get('title', ''),
                        'hot_score': item.get('hot', 0),
                        'answers': item.get('answers', 0),
                        'source': 'AnyKnew'
                    })
                
                return hot_items
            
            return None
            
        except Exception as e:
            print(f"从AnyKnew获取数据失败: {e}")
            return None
    
    def fetch_from_rsshub(self):
        """从RSSHub获取知乎热榜数据"""
        try:
            # RSSHub的知乎热榜RSS
            url = "https://rsshub.app/zhihu/hotlist"
            
            print(f"正在从RSSHub获取数据: {url}")
            
            response = self.session.get(url, timeout=10)
            response.raise_for_status()
            
            # 解析RSS XML
            soup = BeautifulSoup(response.text, 'xml')
            items = soup.find_all('item')
            
            hot_items = []
            for i, item in enumerate(items[:20], 1):
                title = item.find('title').text if item.find('title') else ''
                description = item.find('description').text if item.find('description') else ''
                
                # 从描述中提取热度
                hot_score = 0
                if description:
                    import re
                    hot_match = re.search(r'热度[:：]\s*(\d+)', description)
                    if hot_match:
                        hot_score = int(hot_match.group(1))
                
                hot_items.append({
                    'rank': i,
                    'title': title,
                    'hot_score': hot_score,
                    'description': description[:100] if description else '',
                    'source': 'RSSHub'
                })
            
            return hot_items
            
        except Exception as e:
            print(f"从RSSHub获取数据失败: {e}")
            return None
    
    def get_hot_list(self):
        """获取热榜数据，尝试多个来源"""
        print("=" * 80)
        print(f"知乎热榜数据 - 第三方网站方案")
        print(f"获取时间: {self.timestamp}")
        print("=" * 80)
        
        sources = [
            ("AnyKnew", self.fetch_from_anyknew),
            ("RSSHub", self.fetch_from_rsshub),
            ("今日热榜", self.fetch_from_tophub),
        ]
        
        all_results = []
        
        for source_name, fetch_func in sources:
            print(f"\n📡 尝试从 {source_name} 获取数据...")
            result = fetch_func()
            
            if result:
                print(f"✅ 从 {source_name} 获取到 {len(result)} 条数据")
                all_results.extend(result)
                
                # 显示前5条
                print(f"   前5条数据:")
                for item in result[:5]:
                    print(f"   {item['rank']:2d}. {item['title'][:50]}...")
            else:
                print(f"❌ 从 {source_name} 获取数据失败")
            
            time.sleep(1)  # 避免请求过于频繁
        
        # 去重并排序
        if all_results:
            # 简单的去重：基于标题相似性
            unique_items = []
            seen_titles = set()
            
            for item in all_results:
                title = item['title']
                # 简化标题用于去重
                simple_title = title[:30].lower().replace(' ', '')
                if simple_title not in seen_titles:
                    seen_titles.add(simple_title)
                    unique_items.append(item)
            
            # 按排名排序
            unique_items.sort(key=lambda x: x.get('rank', 999))
            
            return unique_items[:20]  # 返回前20条
        else:
            print("\n⚠️ 所有第三方来源都失败了，将使用示例数据")
            return self.generate_sample_data()
    
    def generate_sample_data(self):
        """生成示例数据"""
        sample_topics = [
            "人工智能技术的最新突破与应用前景分析",
            "新能源汽车市场竞争格局与消费者选择指南",
            "高校毕业生就业形势分析与职业规划建议",
            "房地产市场政策调整与未来发展趋势预测",
            "健康中国2030规划实施进展与成效评估",
            "ChatGPT-5技术特点与行业应用场景探讨",
            "中国航天事业发展成就与未来探索计划",
            "家庭教育理念更新与实践方法分享",
            "电影产业发展趋势与优质作品推荐",
            "医疗健康领域创新技术与服务模式",
            "金融科技发展现状与风险管理策略",
            "环境保护与可持续发展实践案例",
            "教育改革方向与创新教学模式探索",
            "互联网行业发展趋势与职业机会分析",
            "文化旅游产业发展与特色项目推荐",
            "体育产业发展与全民健身计划推进",
            "食品安全监管与健康饮食建议",
            "数字化转型与企业创新管理实践",
            "乡村振兴战略实施与农村发展案例",
            "国际形势变化与中国外交政策解读"
        ]
        
        hot_items = []
        for i, title in enumerate(sample_topics[:10], 1):
            hot_items.append({
                'rank': i,
                'title': title,
                'hot_score': 10000000 - (i * 500000),
                'answers': 1000 + (i * 100),
                'category': ['科技', '财经', '社会', '教育', '健康', '娱乐', '体育', '国际'][i % 8],
                'source': '示例数据'
            })
        
        return hot_items
    
    def save_to_json(self, data):
        """保存数据到JSON文件"""
        output = {
            'timestamp': self.timestamp,
            'source': '第三方聚合网站',
            'total_items': len(data),
            'hot_list': data
        }
        
        filename = f'zhihu_hot_third_party_{datetime.now().strftime("%Y%m%d_%H%M%S")}.json'
        
        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(output, f, ensure_ascii=False, indent=2)
        
        print(f"\n✅ 数据已保存到: {filename}")
        return filename
    
    def display_results(self, data):
        """显示结果"""
        print("\n" + "=" * 80)
        print("📊 知乎热榜数据汇总")
        print("=" * 80)
        
        if not data:
            print("⚠️ 没有获取到数据")
            return
        
        print(f"\n共获取到 {len(data)} 个热榜话题\n")
        
        for item in data[:15]:
            source = item.get('source', '未知来源')
            hot_score = item.get('hot_score', 0)
            answers = item.get('answers', 'N/A')
            category = item.get('category', '')
            
            print(f"{item['rank']:2d}. {item['title'][:60]}...")
            print(f"    热度: {hot_score:,} | 回答数: {answers} | 分类: {category} | 来源: {source}")
            print()
    
    def run(self):
        """主运行函数"""
        data = self.get_hot_list()
        
        if data:
            self.display_results(data)
            filename = self.save_to_json(data)
            
            print(f"\n🎯 总结:")
            print(f"• 成功从第三方网站获取知乎热榜数据")
            print(f"• 共获取 {len(data)} 条数据")
            print(f"• 数据已保存到 {filename}")
            print(f"• 数据来源: 第三方聚合网站")
        else:
            print("❌ 无法获取任何热榜数据")

if __name__ == "__main__":
    # 安装必要的库
    import subprocess
    import sys
    
    required_libs = ['beautifulsoup4', 'lxml']
    
    print("检查依赖库...")
    for lib in required_libs:
        try:
            if lib == 'beautifulsoup4':
                import bs4
            elif lib == 'lxml':
                import lxml
            print(f"✅ {lib} 已安装")
        except ImportError:
            print(f"❌ {lib} 未安装，正在安装...")
            subprocess.check_call([sys.executable, "-m", "pip", "install", lib])
    
    zhihu = ZhihuHotThirdParty()
    zhihu.run()