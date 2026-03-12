import requests
import feedparser
from io import BytesIO

# 测试知乎RSS
urls = [
    "https://www.zhihu.com/rss",
    "https://rss.zhihu.com/",
    "https://www.zhihu.com/feed",
    "https://zhihu.com/rss/hot",
]

for url in urls:
    print(f"\n测试URL: {url}")
    try:
        response = requests.get(url, timeout=10, headers={
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        })
        print(f"状态码: {response.status_code}")
        print(f"内容类型: {response.headers.get('content-type')}")
        
        if response.status_code == 200:
            # 尝试解析RSS
            try:
                feed = feedparser.parse(BytesIO(response.content))
                print(f"RSS标题: {feed.feed.get('title', '无标题')}")
                print(f"条目数: {len(feed.entries)}")
                
                if feed.entries:
                    print("前3个条目:")
                    for i, entry in enumerate(feed.entries[:3], 1):
                        print(f"  {i}. {entry.get('title', '无标题')[:60]}...")
            except:
                print("不是有效的RSS格式")
                # 查看内容开头
                print(f"内容开头: {response.text[:200]}")
        else:
            print(f"响应内容: {response.text[:200]}")
            
    except Exception as e:
        print(f"错误: {e}")