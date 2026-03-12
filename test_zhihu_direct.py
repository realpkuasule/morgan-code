import requests
import sys

url = "https://www.zhihu.com/hot"
headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
}

try:
    response = requests.get(url, headers=headers, timeout=10)
    print(f"状态码: {response.status_code}")
    print(f"内容长度: {len(response.text)}")
    print(f"重定向历史: {response.history}")
    
    # 检查是否重定向到登录页面
    if 'login' in response.url or 'signin' in response.url:
        print("被重定向到登录页面")
        print(f"最终URL: {response.url}")
    
    # 检查页面标题
    if '<title>' in response.text:
        start = response.text.find('<title>') + 7
        end = response.text.find('</title>', start)
        if start > 7 and end > start:
            print(f"页面标题: {response.text[start:end]}")
    
except Exception as e:
    print(f"错误: {e}")