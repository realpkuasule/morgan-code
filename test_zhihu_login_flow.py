#!/usr/bin/env python3
"""
测试知乎登录页面结构
"""

import asyncio
from playwright.async_api import async_playwright

async def test_login_page():
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        context = await browser.new_context(
            user_agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            viewport={'width': 1920, 'height': 1080}
        )
        page = await context.new_page()
        
        print("访问知乎登录页面...")
        await page.goto('https://www.zhihu.com/signin', timeout=30000)
        
        await page.wait_for_load_state('networkidle')
        
        # 截图保存
        await page.screenshot(path='zhihu_login_page.png')
        print("截图已保存: zhihu_login_page.png")
        
        # 分析页面内容
        html = await page.content()
        
        # 检查登录方式
        login_methods = []
        
        # 查找手机登录选项
        phone_login = await page.query_selector('div[data-za-detail-view-path-module="手机登录"], button:has-text("手机登录"), .SignFlow-tab[data-za-detail-view-path-module="手机登录"]')
        if phone_login:
            login_methods.append("手机登录")
        
        # 查找密码登录选项  
        password_login = await page.query_selector('div[data-za-detail-view-path-module="密码登录"], button:has-text("密码登录")')
        if password_login:
            login_methods.append("密码登录")
        
        # 查找第三方登录
        wechat_login = await page.query_selector('button:has-text("微信"), .SocialLogin-wechat')
        if wechat_login:
            login_methods.append("微信登录")
            
        qq_login = await page.query_selector('button:has-text("QQ"), .SocialLogin-qq')
        if qq_login:
            login_methods.append("QQ登录")
        
        print(f"\n发现的登录方式: {login_methods}")
        
        # 检查是否需要验证码
        captcha_elements = await page.query_selector_all('img[src*="captcha"], .Captcha-image, .SignFlow-captchaContainer')
        print(f"验证码元素数量: {len(captcha_elements)}")
        
        # 检查手机输入框
        phone_input = await page.query_selector('input[type="tel"], input[name="phone"], input[placeholder*="手机号"]')
        print(f"手机输入框: {'存在' if phone_input else '不存在'}")
        
        # 检查验证码输入框
        code_input = await page.query_selector('input[type="text"][placeholder*="验证码"], input[name="digits"], .VerificationCodeInput-input')
        print(f"验证码输入框: {'存在' if code_input else '不存在'}")
        
        # 检查获取验证码按钮
        get_code_btn = await page.query_selector('button:has-text("获取验证码"), .CountingDownButton, .SignFlow-smsInputContainer button')
        print(f"获取验证码按钮: {'存在' if get_code_btn else '不存在'}")
        
        # 获取页面标题和提示信息
        title = await page.title()
        print(f"页面标题: {title}")
        
        # 查找可能的错误或提示信息
        error_elements = await page.query_selector_all('.SignFlow-error, .ErrorPage-title, .Captcha-errorMessage')
        if error_elements:
            print(f"错误/提示元素数量: {len(error_elements)}")
        
        await browser.close()
        
        return {
            'login_methods': login_methods,
            'has_phone_input': bool(phone_input),
            'has_code_input': bool(code_input),
            'has_get_code_btn': bool(get_code_btn),
            'captcha_count': len(captcha_elements)
        }

async def main():
    print("测试知乎登录页面结构...")
    result = await test_login_page()
    
    print("\n" + "="*60)
    print("测试结果:")
    print("="*60)
    for key, value in result.items():
        print(f"{key}: {value}")
    
    print("\n建议的登录流程:")
    if "手机登录" in result['login_methods']:
        print("1. 选择手机登录方式")
        print("2. 输入手机号码")
        print("3. 点击获取验证码")
        print("4. 输入收到的验证码")
        print("5. 点击登录按钮")
    elif "密码登录" in result['login_methods']:
        print("1. 选择密码登录方式")
        print("2. 输入手机号/用户名")
        print("3. 输入密码")
        print("4. 可能需要验证码")
        print("5. 点击登录按钮")

if __name__ == "__main__":
    asyncio.run(main())