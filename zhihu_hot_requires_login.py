#!/usr/bin/env python3
"""
知乎热榜获取 - 需要登录的版本
使用方法：
1. 先在浏览器中登录知乎
2. 获取Cookies
3. 将Cookies粘贴到下面的COOKIES变量中
"""

import requests
import json
import re
from datetime import datetime
from typing import List, Dict, Optional

class ZhihuHotWithLogin:
    def __init__(self, cookies: str = None):
        """
        初始化，需要知乎的Cookies
        
        Args:
            cookies: 知乎的Cookies字符串，格式如 "_zap=xxx; _xsrf=xxx; d_c0=xxx; ..."
        """
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            'Accept': 'application/json, text/plain, */*',
            'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8',
            'Accept-Encoding': 'gzip, deflate, br',
            'Connection': 'keep-alive',
            'Referer': 'https://www.zhihu.com/hot',
            'Sec-Fetch-Dest': 'empty',
            'Sec-Fetch-Mode': 'cors',
            'Sec-Fetch-Site': 'same-origin',
            'x-requested-with': 'fetch',
        })
        
        # 设置Cookies
        if cookies:
            self.set_cookies(cookies)
        
        self.timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    
    def set_cookies(self, cookies_str: str):
        """设置Cookies"""
        cookies_dict = {}
        for cookie in cookies_str.split(';'):
            cookie = cookie.strip()
            if '=' in cookie:
                key, value = cookie.split('=', 1)
                cookies_dict[key] = value
        
        # 设置到session
        requests.utils.add_dict_to_cookiejar(self.session.cookies, cookies_dict)
        print(f"已设置 {len(cookies_dict)} 个Cookies")
    
    def test_login(self) -> bool:
        """测试是否已登录"""
        try:
            url = "https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total?limit=1"
            response = self.session.get(url, timeout=10)
            
            if response.status_code == 200:
                print("✅ 登录状态: 已登录")
                return True
            elif response.status_code == 401:
                print("❌ 登录状态: 未登录或Cookies无效")
                print(f"错误信息: {response.text}")
                return False
            else:
                print(f"⚠️ 登录状态: 未知 (状态码: {response.status_code})")
                return False
                
        except Exception as e:
            print(f"❌ 登录测试失败: {e}")
            return False
    
    def fetch_hot_list_api(self) -> Optional[Dict]:
        """使用API获取热榜数据"""
        try:
            url = "https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total"
            params = {
                'limit': 50,
                'desktop': 'true'
            }
            
            print("正在通过API获取热榜数据...")
            response = self.session.get(url, params=params, timeout=10)
            
            if response.status_code == 200:
                data = response.json()
                print(f"✅ API获取成功，获取到 {len(data.get('data', []))} 条数据")
                return data
            else:
                print(f"❌ API请求失败: 状态码 {response.status_code}")
                print(f"响应: {response.text[:200]}")
                return None
                
        except Exception as e:
            print(f"❌ API获取失败: {e}")
            return None
    
    def fetch_hot_list_html(self) -> Optional[str]:
        """获取HTML页面"""
        try:
            url = "https://www.zhihu.com/hot"
            
            print("正在获取HTML页面...")
            response = self.session.get(url, timeout=10)
            
            if response.status_code == 200:
                print(f"✅ HTML获取成功，内容长度: {len(response.text)}")
                return response.text
            else:
                print(f"❌ HTML获取失败: 状态码 {response.status_code}")
                return None
                
        except Exception as e:
            print(f"❌ HTML获取失败: {e}")
            return None
    
    def parse_api_data(self, data: Dict) -> List[Dict]:
        """解析API数据"""
        hot_items = []
        
        if not data or 'data' not in data:
            return hot_items
        
        for i, item in enumerate(data['data'], 1):
            try:
                target = item.get('target', {})
                
                # 提取热度值
                detail_text = item.get('detail_text', '')
                hot_score = 0
                if detail_text:
                    match = re.search(r'(\d+(?:\.\d+)?)', detail_text.replace(',', ''))
                    if match:
                        num = float(match.group(1))
                        if '万' in detail_text:
                            hot_score = int(num * 10000)
                        else:
                            hot_score = int(num)
                
                # 基本信息
                hot_item = {
                    'rank': i,
                    'title': target.get('title', ''),
                    'hot_score': hot_score,
                    'answer_count': target.get('answer_count', 0),
                    'question_id': str(target.get('id', '')),
                    'url': f"https://www.zhihu.com/question/{target.get('id', '')}",
                    'excerpt': target.get('excerpt', ''),
                    'category': target.get('relationship', {}).get('is_author', False) and '专栏' or '问题'
                }
                
                hot_items.append(hot_item)
                
            except Exception as e:
                print(f"解析第{i}条数据失败: {e}")
                continue
        
        return hot_items
    
    def parse_html_data(self, html: str) -> List[Dict]:
        """解析HTML数据（备用方法）"""
        hot_items = []
        
        try:
            # 简单的HTML解析
            import re
            
            # 查找热榜项目
            # 知乎热榜的HTML结构可能会变化，这里使用通用匹配
            pattern = r'<[^>]*class="[^"]*HotItem[^"]*"[^>]*>.*?</div>'
            items = re.findall(pattern, html, re.DOTALL)
            
            if not items:
                # 尝试其他模式
                pattern = r'<[^>]*data-za-detail-view-path-module="HotList"[^>]*>.*?</div>'
                items = re.findall(pattern, html, re.DOTALL)
            
            for i, item_html in enumerate(items[:50], 1):
                try:
                    # 提取标题
                    title_match = re.search(r'<[^>]*class="[^"]*HotItem-title[^"]*"[^>]*>(.*?)</', item_html, re.DOTALL)
                    title = title_match.group(1).strip() if title_match else ""
                    
                    # 清理HTML标签
                    title = re.sub(r'<[^>]*>', '', title)
                    
                    # 提取热度
                    hot_match = re.search(r'<[^>]*class="[^"]*HotItem-metrics[^"]*"[^>]*>(.*?)</', item_html, re.DOTALL)
                    hot_text = hot_match.group(1).strip() if hot_match else ""
                    
                    hot_score = 0
                    if hot_text:
                        num_match = re.search(r'(\d+(?:\.\d+)?)', hot_text.replace(',', ''))
                        if num_match:
                            num = float(num_match.group(1))
                            if '万' in hot_text:
                                hot_score = int(num * 10000)
                            else:
                                hot_score = int(num)
                    
                    if title:
                        hot_items.append({
                            'rank': i,
                            'title': title[:200],
                            'hot_score': hot_score,
                            'source': 'HTML解析'
                        })
                        
                except Exception as e:
                    print(f"解析HTML项目{i}失败: {e}")
                    continue
            
            return hot_items
            
        except Exception as e:
            print(f"HTML解析失败: {e}")
            return []
    
    def get_hot_list(self, method: str = 'api') -> List[Dict]:
        """获取热榜数据"""
        print("=" * 80)
        print(f"知乎热榜数据获取 - 需要登录版本")
        print(f"时间: {self.timestamp}")
        print("=" * 80)
        
        # 先测试登录状态
        if not self.test_login():
            print("\n❌ 请先设置有效的Cookies！")
            print("=" * 80)
            return []
        
        hot_items = []
        
        if method == 'api':
            data = self.fetch_hot_list_api()
            if data:
                hot_items = self.parse_api_data(data)
        elif method == 'html':
            html = self.fetch_hot_list_html()
            if html:
                hot_items = self.parse_html_data(html)
        else:
            print(f"❌ 未知的方法: {method}")
        
        return hot_items
    
    def display_results(self, data: List[Dict]):
        """显示结果"""
        if not data:
            print("\n❌ 没有获取到数据")
            return
        
        print("\n" + "=" * 80)
        print("📊 知乎热榜数据")
        print("=" * 80)
        
        print(f"\n共获取到 {len(data)} 个热榜话题\n")
        
        # 显示前20个
        for item in data[:20]:
            rank = item['rank']
            title = item['title']
            hot_score = item.get('hot_score', 0)
            answer_count = item.get('answer_count', 0)
            
            # 格式化显示
            if hot_score >= 10000:
                hot_display = f"{hot_score/10000:.1f}万"
            else:
                hot_display = f"{hot_score:,}"
            
            print(f"{rank:2d}. {title}")
            if hot_score > 0:
                print(f"    热度: {hot_display}", end="")
            if answer_count > 0:
                print(f" | 回答: {answer_count:,}", end="")
            if hot_score > 0 or answer_count > 0:
                print()
            print()
    
    def save_to_json(self, data: List[Dict]):
        """保存数据到JSON文件"""
        if not data:
            print("❌ 没有数据可保存")
            return None
        
        output = {
            'timestamp': self.timestamp,
            'source': '知乎直接获取（需要登录）',
            'total_items': len(data),
            'average_hot_score': sum(item.get('hot_score', 0) for item in data) // max(1, len(data)),
            'hot_list': data
        }
        
        filename = f'zhihu_hot_with_login_{datetime.now().strftime("%Y%m%d_%H%M%S")}.json'
        
        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(output, f, ensure_ascii=False, indent=2)
        
        print(f"✅ 数据已保存到: {filename}")
        return filename

def main():
    print("知乎热榜获取工具 - 需要登录版本")
    print("-" * 60)
    
    # 在这里粘贴你的知乎Cookies
    # 获取方法：在已登录知乎的浏览器中打开控制台，输入 document.cookie
    COOKIES = """
    将你的Cookies粘贴在这里
    格式示例：_zap=xxx; _xsrf=xxx; d_c0=xxx; ...
    """
    
    if COOKIES.strip() == "将你的Cookies粘贴在这里":
        print("❌ 请先设置Cookies！")
        print("\n📝 如何获取Cookies：")
        print("1. 在浏览器中登录知乎（https://www.zhihu.com）")
        print("2. 打开开发者工具（F12）")
        print("3. 在Console标签页输入：document.cookie")
        print("4. 复制输出的Cookies字符串")
        print("5. 粘贴到脚本的COOKIES变量中")
        print("\n示例Cookies格式：")
        print('_zap=xxxxxx; _xsrf=xxxxxx; d_c0="xxxxxx"; ...')
        return
    
    # 创建实例
    zhihu = ZhihuHotWithLogin(COOKIES.strip())
    
    # 获取数据（优先使用API，失败时使用HTML）
    data = zhihu.get_hot_list(method='api')
    
    if not data:
        print("\n尝试使用HTML方式...")
        data = zhihu.get_hot_list(method='html')
    
    if data:
        zhihu.display_results(data)
        filename = zhihu.save_to_json(data)
        
        print("\n🎯 执行总结:")
        print(f"• 状态: ✅ 成功")
        print(f"• 时间: {zhihu.timestamp}")
        print(f"• 数据量: {len(data)} 条")
        print(f"• 文件: {filename}")
    else:
        print("\n❌ 获取失败，请检查：")
        print("1. Cookies是否有效")
        print("2. 是否已登录知乎")
        print("3. 网络连接是否正常")

if __name__ == "__main__":
    main()