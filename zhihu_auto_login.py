import asyncio
from playwright.async_api import async_playwright
import json
import os
from datetime import datetime

async def zhihu_login_and_fetch(phone_number, verification_code=None):
    """登录知乎并获取热榜数据"""
    
    async with async_playwright() as p:
        # 启动浏览器
        browser = await p.chromium.launch(headless=True)
        context = await browser.new_context(
            user_agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            viewport={'width': 1920, 'height': 1080}
        )
        page = await context.new_page()
        
        print("1. 访问知乎登录页面...")
        await page.goto('https://www.zhihu.com/signin', timeout=30000)
        await page.wait_for_load_state('networkidle')
        
        # 检查是否已经在其他标签页登录
        cookies = await context.cookies()
        if any(cookie['name'] == '_zap' for cookie in cookies):
            print("检测到已登录状态")
        else:
            print("需要登录...")
            
            # 查找手机输入框
            phone_input = await page.wait_for_selector('input[type="tel"], input[name="phone"], input[placeholder*="手机号"]', timeout=5000)
            
            # 输入手机号
            print(f"2. 输入手机号: {phone_number}")
            await phone_input.fill(phone_number)
            
            # 查找获取验证码按钮
            get_code_btn = await page.wait_for_selector('button:has-text("获取验证码"), .CountingDownButton, .SignFlow-smsInputContainer button', timeout=5000)
            
            print("3. 点击获取验证码按钮...")
            await get_code_btn.click()
            
            # 等待验证码输入框出现
            code_input = await page.wait_for_selector('input[type="text"][placeholder*="验证码"], input[name="digits"], .VerificationCodeInput-input', timeout=5000)
            
            if verification_code:
                print(f"4. 输入验证码: {verification_code}")
                await code_input.fill(verification_code)
                
                # 查找登录按钮
                login_btn = await page.wait_for_selector('button[type="submit"], button:has-text("登录"), .SignFlow-submitButton', timeout=5000)
                
                print("5. 点击登录按钮...")
                await login_btn.click()
                
                # 等待登录完成
                try:
                    await page.wait_for_url('https://www.zhihu.com/**', timeout=10000)
                    print("✅ 登录成功！")
                except:
                    print("⚠️ 登录状态不确定，继续尝试...")
            else:
                print("⚠️ 未提供验证码，等待用户输入...")
                print("请检查手机短信，然后将验证码发给我")
                return None
        
        # 保存cookies供后续使用
        cookies = await context.cookies()
        with open('zhihu_cookies.json', 'w') as f:
            json.dump(cookies, f)
        print(f"✅ Cookies已保存: {len(cookies)} 个")
        
        # 访问热榜页面
        print("\n6. 访问知乎热榜...")
        await page.goto('https://www.zhihu.com/hot', timeout=30000)
        await page.wait_for_load_state('networkidle')
        
        # 截图保存
        await page.screenshot(path='zhihu_hot_after_login.png')
        print("✅ 热榜页面截图已保存")
        
        # 尝试获取热榜数据
        print("\n7. 尝试获取热榜数据...")
        
        # 方法1：直接解析页面
        hot_items = []
        try:
            # 等待热榜项目加载
            await page.wait_for_selector('.HotList-list .HotItem, .TopstoryItem', timeout=5000)
            
            # 获取热榜项目
            hot_elements = await page.query_selector_all('.HotList-list .HotItem, .TopstoryItem')
            print(f"找到 {len(hot_elements)} 个热榜项目")
            
            for i, element in enumerate(hot_elements[:20], 1):
                try:
                    # 获取标题
                    title_elem = await element.query_selector('.HotItem-title, .ContentItem-title')
                    title = await title_elem.text_content() if title_elem else ""
                    
                    # 获取热度
                    hot_elem = await element.query_selector('.HotItem-metrics, .ContentItem-extra')
                    hot_text = await hot_elem.text_content() if hot_elem else ""
                    
                    if title:
                        hot_items.append({
                            'rank': i,
                            'title': title.strip(),
                            'hot_text': hot_text.strip()
                        })
                        
                except Exception as e:
                    print(f"解析第{i}个项目失败: {e}")
                    continue
                    
        except Exception as e:
            print(f"页面解析失败: {e}")
        
        # 方法2：尝试调用API
        api_data = None
        try:
            print("尝试调用热榜API...")
            # 在页面上下文中执行JavaScript获取API数据
            api_response = await page.evaluate("""
                async () => {
                    try {
                        const response = await fetch('https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total?limit=50&desktop=true', {
                            credentials: 'include',
                            headers: {
                                'Accept': 'application/json',
                                'x-requested-with': 'fetch'
                            }
                        });
                        if (response.ok) {
                            return await response.json();
                        }
                        return null;
                    } catch (e) {
                        return {error: e.toString()};
                    }
                }
            """)
            
            if api_response and 'data' in api_response:
                print(f"API获取成功: {len(api_response['data'])} 条数据")
                api_data = api_response
            elif api_response and 'error' in api_response:
                print(f"API调用失败: {api_response['error']}")
            else:
                print("API调用返回空或无效数据")
                
        except Exception as e:
            print(f"API调用异常: {e}")
        
        await browser.close()
        
        return {
            'cookies_saved': len(cookies),
            'hot_items_from_page': hot_items,
            'api_data': api_data,
            'timestamp': datetime.now().isoformat()
        }

async def main():
    print("知乎自动登录和热榜获取工具")
    print("="*60)
    
    # 这里需要用户提供手机号和验证码
    # 为了测试，我先用占位符
    phone = input("请输入手机号（当前环境无法输入，请通过对话提供）: ") or "13800138000"
    code = input("请输入验证码（通过对话提供）: ") or None
    
    if not code:
        print("\n⚠️ 需要验证码才能继续")
        print("流程：")
        print("1. 我会先获取验证码")
        print("2. 你收到短信后告诉我验证码")
        print("3. 我输入验证码完成登录")
        return
    
    result = await zhihu_login_and_fetch(phone, code)
    
    if result:
        print("\n" + "="*60)
        print("执行结果:")
        print("="*60)
        print(f"Cookies保存: {result['cookies_saved']} 个")
        print(f"页面解析热榜: {len(result.get('hot_items_from_page', []))} 条")
        
        if result.get('hot_items_from_page'):
            print("\n前5条热榜:")
            for item in result['hot_items_from_page'][:5]:
                print(f"{item['rank']}. {item['title'][:50]}...")
                if item.get('hot_text'):
                    print(f"   热度: {item['hot_text']}")
        
        if result.get('api_data'):
            print(f"\nAPI数据: {len(result['api_data'].get('data', []))} 条")
            
        print(f"\n完成时间: {result['timestamp']}")

if __name__ == "__main__":
    # 由于当前环境无法交互输入，先测试流程
    print("注意：当前为服务器环境，无法交互输入")
    print("请通过对话提供手机号和验证码")
    print("\n测试基本流程...")
    
    # 测试无验证码情况
    asyncio.run(zhihu_login_and_fetch("test", None))