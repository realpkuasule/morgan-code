#!/usr/bin/env python3
"""
知乎登录 - 发送验证码（第一部分）
手机号: 18611209750
"""

import asyncio
import json
from playwright.async_api import async_playwright

async def send_verification_code():
    """发送验证码到指定手机号"""
    
    phone_number = "18611209750"
    
    print("="*60)
    print(f"知乎登录流程 - 发送验证码")
    print(f"手机号: {phone_number}")
    print("="*60)
    
    async with async_playwright() as p:
        # 启动浏览器，清除现有cookies
        browser = await p.chromium.launch(headless=True)
        context = await browser.new_context(
            user_agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            viewport={'width': 1920, 'height': 1080},
            # 清除cookies
            storage_state=None
        )
        
        page = await context.new_page()
        
        try:
            print("1. 访问知乎登录页面...")
            await page.goto('https://www.zhihu.com/signin', timeout=30000)
            await page.wait_for_load_state('networkidle')
            
            # 检查是否有隐私协议等弹窗
            print("2. 检查页面状态...")
            
            # 查找手机输入框
            print("3. 查找手机输入框...")
            phone_input = await page.wait_for_selector(
                'input[type="tel"], input[name="phone"], input[placeholder*="手机号"], input[placeholder*="请输入手机号"]',
                timeout=10000
            )
            
            print("4. 输入手机号...")
            await phone_input.fill(phone_number)
            
            # 等待一下，让页面可能更新
            await asyncio.sleep(1)
            
            # 检查是否需要其他操作（如密码登录切换）
            print("5. 检查登录方式...")
            
            # 查找获取验证码按钮
            print("6. 查找获取验证码按钮...")
            get_code_btn = await page.wait_for_selector(
                'button:has-text("获取验证码"), .CountingDownButton, .SignFlow-smsInputContainer button, button.SignFlow-smsSendCode',
                timeout=10000
            )
            
            # 检查按钮状态
            is_disabled = await get_code_btn.get_attribute('disabled')
            if is_disabled:
                print("⚠️ 获取验证码按钮被禁用，可能需要先处理其他验证")
                
                # 检查是否有验证码图片
                captcha_img = await page.query_selector('img[src*="captcha"], .Captcha-image')
                if captcha_img:
                    print("⚠️ 需要图形验证码，正在截图...")
                    await page.screenshot(path='zhihu_captcha.png')
                    print("✅ 验证码截图已保存: zhihu_captcha.png")
                    print("请查看图片并告诉我验证码文字")
                    return {'status': 'need_captcha', 'page': page, 'browser': browser, 'context': context}
            
            print("7. 点击获取验证码按钮...")
            await get_code_btn.click()
            
            # 等待响应
            print("8. 等待验证码发送...")
            await asyncio.sleep(3)
            
            # 检查是否有错误提示
            error_elements = await page.query_selector_all('.SignFlow-error, .ErrorPage-title, .SignFlow-captchaError')
            if error_elements:
                for error in error_elements[:3]:
                    error_text = await error.text_content()
                    print(f"⚠️ 错误提示: {error_text}")
            
            # 检查是否开始倒计时
            counting_text = await get_code_btn.text_content()
            if '秒后重新发送' in counting_text or 's' in counting_text:
                print("✅ 验证码已发送！按钮开始倒计时")
                
                # 查找验证码输入框
                print("9. 检查验证码输入框...")
                try:
                    code_input = await page.wait_for_selector(
                        'input[type="text"][placeholder*="验证码"], input[name="digits"], .VerificationCodeInput-input, input[placeholder*="请输入验证码"]',
                        timeout=5000
                    )
                    print("✅ 验证码输入框已出现")
                    
                    # 截图保存当前状态
                    await page.screenshot(path='zhihu_waiting_for_code.png')
                    print("✅ 当前页面截图已保存: zhihu_waiting_for_code.png")
                    
                    # 保存cookies和页面状态
                    cookies = await context.cookies()
                    print(f"✅ 当前Cookies数量: {len(cookies)}")
                    
                    # 返回成功状态，保持浏览器打开（实际中应该关闭，但这里为了演示）
                    await browser.close()
                    
                    return {
                        'status': 'success',
                        'message': '验证码已发送，请查收短信并回复6位验证码',
                        'phone': phone_number,
                        'screenshots': ['zhihu_waiting_for_code.png']
                    }
                    
                except Exception as e:
                    print(f"⚠️ 未找到验证码输入框: {e}")
                    await page.screenshot(path='zhihu_after_send_code.png')
                    print("✅ 截图已保存: zhihu_after_send_code.png")
                    
                    # 检查页面内容
                    page_text = await page.text_content('body')
                    if '验证码' in page_text:
                        print("ℹ️ 页面包含'验证码'文字，可能输入框选择器需要调整")
                    
                    await browser.close()
                    return {'status': 'input_box_not_found', 'screenshot': 'zhihu_after_send_code.png'}
            
            else:
                print(f"⚠️ 按钮状态异常: {counting_text}")
                await page.screenshot(path='zhihu_button_state.png')
                print("✅ 截图已保存: zhihu_button_state.png")
                
                # 检查页面是否有其他提示
                alerts = await page.query_selector_all('.Modal, .Dialog, .ant-modal')
                if alerts:
                    print(f"⚠️ 发现 {len(alerts)} 个弹窗")
                
                await browser.close()
                return {'status': 'button_not_working', 'screenshot': 'zhihu_button_state.png'}
                
        except Exception as e:
            print(f"❌ 发送验证码过程出错: {e}")
            import traceback
            traceback.print_exc()
            
            # 截图保存错误状态
            try:
                await page.screenshot(path='zhihu_error.png')
                print("✅ 错误截图已保存: zhihu_error.png")
            except:
                pass
            
            try:
                await browser.close()
            except:
                pass
            
            return {'status': 'error', 'error': str(e)}

async def main():
    print("开始知乎登录流程 - 发送验证码")
    result = await send_verification_code()
    
    print("\n" + "="*60)
    print("执行结果:")
    print("="*60)
    
    if result['status'] == 'success':
        print("✅ " + result['message'])
        print(f"📱 手机号: {result['phone']}")
        print("📷 截图已保存")
    elif result['status'] == 'need_captcha':
        print("⚠️ 需要图形验证码")
        print("请查看 zhihu_captcha.png 并告诉我验证码文字")
    else:
        print(f"❌ 状态: {result['status']}")
        if 'screenshot' in result:
            print(f"📷 截图: {result['screenshot']}")
        if 'error' in result:
            print(f"错误: {result['error']}")
    
    print("\n下一步：请将收到的短信验证码发给我")

if __name__ == "__main__":
    asyncio.run(main())