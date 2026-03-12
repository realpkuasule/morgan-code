import requests
import json

# 知乎热榜API（可能需要特定的headers）
url = "https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total?limit=50&desktop=true"

headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    'Accept': 'application/json, text/plain, */*',
    'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8',
    'Accept-Encoding': 'gzip, deflate, br',
    'Connection': 'keep-alive',
    'Sec-Fetch-Dest': 'empty',
    'Sec-Fetch-Mode': 'cors',
    'Sec-Fetch-Site': 'same-origin',
    'Referer': 'https://www.zhihu.com/hot',
    'x-requested-with': 'fetch',
    'x-zse-93': '101_3_3.0',
    'x-zse-96': '2.0_aLDU8Xmwxg67PRWhGm1TQkEIpS49OzvKLKZvBw8KdF4=',
}

try:
    response = requests.get(url, headers=headers, timeout=10)
    print(f"状态码: {response.status_code}")
    print(f"内容类型: {response.headers.get('content-type')}")
    print(f"内容长度: {len(response.text)}")
    
    if response.status_code == 200:
        try:
            data = response.json()
            print("\nAPI响应成功，数据结构:")
            print(f"是否有data字段: {'data' in data}")
            if 'data' in data:
                print(f"data数量: {len(data['data'])}")
                if data['data']:
                    print(f"第一条数据示例: {json.dumps(data['data'][0], ensure_ascii=False)[:200]}...")
        except json.JSONDecodeError:
            print("\n响应不是有效的JSON")
            print("响应内容:", response.text[:500])
    else:
        print(f"\n错误响应: {response.text[:500]}")
        
except Exception as e:
    print(f"错误: {e}")