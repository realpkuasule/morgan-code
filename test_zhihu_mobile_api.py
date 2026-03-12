import requests
import json

# 尝试移动端API
url = "https://api.zhihu.com/topstory/hot-list?limit=50"

headers = {
    'User-Agent': 'Mozilla/5.0 (Linux; Android 10; SM-G975F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36',
    'Accept': 'application/json, text/plain, */*',
    'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8',
    'Accept-Encoding': 'gzip, deflate, br',
    'Connection': 'keep-alive',
    'x-api-version': '3.0',
    'x-app-version': '8.40.0',
    'x-app-za': 'OS=Android&Release=10',
    'x-app-build': 'release',
    'x-app-versioncode': '100840',
    'x-network-type': 'WiFi',
    'x-udid': 'AFAKEUDID1234567890',
}

try:
    response = requests.get(url, headers=headers, timeout=10)
    print(f"状态码: {response.status_code}")
    print(f"内容类型: {response.headers.get('content-type')}")
    print(f"内容长度: {len(response.text)}")
    
    if response.status_code == 200:
        try:
            data = response.json()
            print("\nAPI响应成功")
            print(f"数据键: {data.keys()}")
            if 'data' in data:
                print(f"热榜数量: {len(data.get('data', []))}")
                if data.get('data'):
                    item = data['data'][0]
                    print(f"\n第一条热榜示例:")
                    print(f"标题: {item.get('target', {}).get('title', '无标题')}")
                    print(f"热度: {item.get('detail_text', '无热度')}")
        except json.JSONDecodeError:
            print("\n响应不是有效的JSON")
            print("响应内容:", response.text[:500])
    else:
        print(f"\n错误响应: {response.text[:500]}")
        
except Exception as e:
    print(f"错误: {e}")