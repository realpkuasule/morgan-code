#!/usr/bin/env python3
"""
知乎登录 - 修复版发送验证码
"""

import asyncio
import json
from playwright.async_api import async_playwright

async def send_verification_code_fixed():
    phone_number = "18611209750"
    
    print("知乎登录修复版 - 发送验证码")
    print(f"手机号: {phone_number}")
    
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        # 使用更真实的浏览器上下文
        context = await browser.new_context(
            user_agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            viewport={'width': 1920, 'height': 1080},
            locale='zh-CN',
            timezone_id='Asia/Shanghai',
            geolocation={'latitude': 39.9042, 'longitude': 116.4074},  # 北京
            permissions=['geolocation']
        )
        
        page = await context.new_page()
        
        try:
            print("1. 访问知乎登录页面...")
            await page.goto('https://www.zhihu.com/signin', wait_until='networkidle', timeout=30000)
            
            # 等待页面完全加载
            await asyncio.sleep(2)
            
            print("2. 处理可能的弹窗和协议...")
            # 尝试关闭各种弹窗
            close_selectors = [
                '.Modal-closeButton',
                '.Dialog-close',
                '.ant-modal-close',
                'button:has-text("我知道了")',
                'button:has-text("同意")',
                'button:has-text("接受")',
                'button:has-text("好的")'
            ]
            
            for selector in close_selectors:
                try:
                    close_btn = await page.wait_for_selector(selector, timeout=1000)
                    if close_btn:
                        print(f"  关闭弹窗: {selector}")
                        await close_btn.click()
                        await asyncio.sleep(0.5)
                except:
                    pass
            
            print("3. 确保在手机登录标签页...")
            # 尝试点击手机登录标签（如果存在）
            phone_tab_selectors = [
                'div[data-za-detail-view-path-module="手机登录"]',
                '.SignFlow-tab[data-za-detail-view-path-module="手机登录"]',
                'button:has-text("手机登录")',
                '.SignFlow-tab:has-text("手机")'
            ]
            
            for selector in phone_tab_selectors:
                try:
                    phone_tab = await page.wait_for_selector(selector, timeout=1000)
                    if phone_tab:
                        print(f"  切换到手机登录: {selector}")
                        await phone_tab.click()
                        await asyncio.sleep(1)
                        break
                except:
                    pass
            
            print("4. 输入手机号...")
            # 尝试多个选择器找手机输入框
            phone_input_selectors = [
                'input[type="tel"]',
                'input[name="phone"]',
                'input[placeholder*="手机号"]',
                'input[placeholder*="请输入手机号"]',
                '.SignFlow-accountInput input'
            ]
            
            phone_input = None
            for selector in phone_input_selectors:
                try:
                    phone_input = await page.wait_for_selector(selector, timeout=2000)
                    if phone_input:
                        print(f"  找到手机输入框: {selector}")
                        break
                except:
                    continue
            
            if not phone_input:
                # 截图看看页面状态
                await page.screenshot(path='zhihu_no_phone_input.png')
                print("❌ 未找到手机输入框，截图已保存")
                await browser.close()
                return {'status': 'no_phone_input'}
            
            # 清空并输入手机号
            await phone_input.fill('')
            await asyncio.sleep(0.3)
            await phone_input.type(phone_number, delay=100)  # 模拟真人输入
            
            print("5. 查找并点击获取验证码按钮...")
            # 多个选择器尝试
            code_button_selectors = [
                'button:has-text("获取验证码")',
                '.CountingDownButton',
                '.SignFlow-smsSendCode',
                '.SignFlow-smsInputContainer button',
                'button.SignFlow-submitButton'
            ]
            
            get_code_btn = None
            for selector in code_button_selectors:
                try:
                    get_code_btn = await page.wait_for_selector(selector, timeout=2000)
                    if get_code_btn:
                        print(f"  找到按钮: {selector}")
                        break
                except:
                    continue
            
            if not get_code_btn:
                await page.screenshot(path='zhihu_no_code_button.png')
                print("❌ 未找到获取验证码按钮")
                await browser.close()
                return {'status': 'no_code_button'}
            
            # 检查按钮是否可用
            is_disabled = await get_code_btn.get_attribute('disabled')
            if is_disabled:
                print("⚠️ 按钮被禁用，检查可能的原因...")
                
                # 检查是否有图形验证码
                captcha_selectors = [
                    'img[src*="captcha"]',
                    '.Captcha-image',
                    '.SignFlow-captchaContainer img'
                ]
                
                for selector in captcha_selectors:
                    try:
                        captcha = await page.wait_for_selector(selector, timeout=1000)
                        if captcha:
                            await page.screenshot(path='zhihu_captcha_needed.png')
                            print("⚠️ 需要图形验证码，截图已保存")
                            await browser.close()
                            return {'status': 'captcha_needed', 'screenshot': 'zhihu_captcha_needed.png'}
                    except:
                        pass
                
                # 检查错误提示
                error_selectors = [
                    '.SignFlow-error',
                    '.ErrorPage-title',
                    '.SignFlow-captchaError'
                ]
                
                for selector in error_selectors:
                    try:
                        error = await page.wait_for_selector(selector, timeout=1000)
                        if error:
                            error_text = await error.text_content()
                            print(f"⚠️ 错误提示: {error_text}")
                    except:
                        pass
            
            print("6. 点击按钮发送验证码...")
            # 使用多种点击方式尝试
            try:
                # 方式1：直接点击
                await get_code_btn.click()
                print("  已点击按钮")
            except Exception as e:
                print(f"  直接点击失败: {e}")
                
                # 方式2：使用JavaScript点击
                try:
                    await page.evaluate('''(selector) => {
                        const btn = document.querySelector(selector);
                        if (btn) btn.click();
                    }''', code_button_selectors[0])
                    print("  JavaScript点击成功")
                except:
                    print("  JavaScript点击也失败")
            
            # 等待响应
            print("7. 等待验证码发送响应...")
            await asyncio.sleep(3)
            
            # 检查按钮状态变化
            new_button_text = await get_code_btn.text_content()
            print(f"  按钮当前文本: {new_button_text}")
            
            if '秒后重新发送' in new_button_text or 's' in new_button_text or '重新发送' in new_button_text:
                print("✅ 验证码已发送！按钮开始倒计时")
                
                # 查找验证码输入框
                print("8. 查找验证码输入框...")
                code_input_selectors = [
                    'input[type="text"][placeholder*="验证码"]',
                    'input[name="digits"]',
                    '.VerificationCodeInput-input',
                    'input[placeholder*="请输入验证码"]'
                ]
                
                code_input = None
                for selector in code_input_selectors:
                    try:
                        code_input = await page.wait_for_selector(selector, timeout=3000)
                        if code_input:
                            print(f"  找到验证码输入框: {selector}")
                            break
                    except:
                        continue
                
                if code_input:
                    # 截图保存
                    await page.screenshot(path='zhihu_waiting_for_code_fixed.png')
                    print("✅ 截图已保存: zhihu_waiting_for_code_fixed.png")
                    
                    # 保存cookies
                    cookies = await context.cookies()
                    print(f"✅ 当前Cookies: {len(cookies)} 个")
                    
                    await browser.close()
                    return {
                        'status': 'success',
                        'message': '验证码已发送成功！请查收短信',
                        'phone': phone_number,
                        'screenshot': 'zhihu_waiting_for_code_fixed.png'
                    }
                else:
                    print("⚠️ 未找到验证码输入框，但验证码可能已发送")
                    await page.screenshot(path='zhihu_after_send_fixed.png')
                    await browser.close()
                    return {
                        'status': 'code_sent_no_input',
                        'message': '验证码可能已发送，但未找到输入框',
                        'screenshot': 'zhihu_after_send_fixed.png'
                    }
            else:
                print("⚠️ 按钮状态未变化，可能发送失败")
                await page.screenshot(path='zhihu_send_failed_fixed.png')
                
                # 检查页面错误
                page_text = await page.content()
                if '频繁' in page_text:
                    print("⚠️ 可能操作过于频繁")
                if '验证' in page_text:
                    print("⚠️ 可能需要额外验证")
                
                await browser.close()
                return {
                    'status': 'send_failed',
                    'screenshot': 'zhihu_send_failed_fixed.png'
                }
                
        except Exception as e:
            print(f"❌ 过程出错: {e}")
            import traceback
            traceback.print_exc()
            
            try:
                await page.screenshot(path='zhihu_error_fixed.png')
                print("✅ 错误截图已保存")
            except:
                pass
            
            try:
                await browser.close()
            except:
                pass
            
            return {'status': 'error', 'error': str(e)}

async def main():
    print("开始修复版登录流程...")
    result = await send_verification_code_fixed()
    
    print("\n" + "="*60)
    print("执行结果:")
    print("="*60)
    
    status = result['status']
    
    if status == 'success':
        print("✅ " + result['message'])
        print(f"📱 手机号: {result['phone']}")
        print("📷 截图: " + result['screenshot'])
        print("\n请将收到的6位短信验证码发给我")
    elif status == 'captcha_needed':
        print("⚠️ 需要图形验证码")
        print("📷 截图: " + result['screenshot'])
        print("请查看图片并告诉我验证码中的文字")
    elif status == 'code_sent_no_input':
        print("⚠️ " + result['message'])
        print("📷 截图: " + result['screenshot'])
        print("请检查是否收到短信验证码")
    else:
        print(f"❌ 状态: {status}")
        if 'screenshot' in result:
            print(f"📷 截图: {result['screenshot']}")
        if 'error' in result:
            print(f"错误: {result['error']}")
        
        print("\n可能的原因:")
        print("1. 知乎反爬虫机制阻止")
        print("2. 需要处理验证码")
        print("3. 操作频率限制")
        print("4. 网络或页面加载问题")

if __name__ == "__main__":
    asyncio.run(main())