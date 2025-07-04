import 'package:client/core/network/http_service.dart' show HttpService;
import 'package:client/views/widgets/clickable_text.dart' show ClickableText;
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final _formKey = GlobalKey<FormState>();
  final TextEditingController _emailCtrl = TextEditingController();
  final TextEditingController _passwordCtrl = TextEditingController();

  bool _isLoading = false;
  bool _rememberMe = false;
  bool _autoLogin = false;

  void _login() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() => _isLoading = true);

    try {
      final response = await HttpService.post(
        '/note/api/login',
        data: {'email': _emailCtrl.text, 'password': _passwordCtrl.text},
      );

      final data = response.data;
      debugPrint(data.toString());
      if (data['code'] == 0) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('登录成功')));
        }
        // TODO: 保存 token，跳转主页等
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('登录失败: ${data['message']}')));
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('登录失败')));
      }
      debugPrint('Login error: $e');
    }

    setState(() => _isLoading = false);
  }

  @override
  Widget build(BuildContext context) {
    double screenWidth = MediaQuery.of(context).size.width;
    double contentWidth = screenWidth < 600 ? screenWidth * 0.9 : 600;
    return Scaffold(
      // backgroundColor: Color,
      body: SafeArea(
        child: Center(
          child: SingleChildScrollView(
            padding: const EdgeInsets.symmetric(horizontal: 24),
            child: SizedBox(
              width: contentWidth,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Container(
                    width: 100,
                    height: 100,
                    decoration: BoxDecoration(
                      image: DecorationImage(image: AssetImage('assets/images/logo.png'), fit: BoxFit.cover),
                    ),
                  ).animate().fadeIn().scale(delay: 300.ms),
                  const SizedBox(height: 32),
                  Form(
                    key: _formKey,
                    child: Column(
                      children: [
                        TextFormField(
                          controller: _emailCtrl,
                          decoration: const InputDecoration(
                            labelText: '邮箱',
                            border: OutlineInputBorder(),
                            prefixIcon: Icon(Icons.email),
                          ),
                          keyboardType: TextInputType.emailAddress,
                          validator: (val) {
                            if (kDebugMode) {
                              return null;
                            }
                            if (val == null || val.isEmpty) return '请输入邮箱';
                            if (!val.contains('@')) return '邮箱格式错误';
                            return null;
                          },
                        ).animate().fadeIn().slideX(begin: -0.3),
                        const SizedBox(height: 16),
                        TextFormField(
                          controller: _passwordCtrl,
                          decoration: const InputDecoration(
                            labelText: '密码',
                            border: OutlineInputBorder(),
                            prefixIcon: Icon(Icons.lock),
                          ),
                          obscureText: true,
                          validator: (val) {
                            if (kDebugMode) {
                              return null;
                            }
                            if (val == null || val.isEmpty) return '请输入密码';
                            if (val.length < 6) return '密码长度至少6位';
                            return null;
                          },
                        ).animate().fadeIn().slideX(begin: 0.3),
                      ],
                    ),
                  ),

                  const SizedBox(height: 12),

                  Row(
                    children: [
                      Checkbox(
                        value: _rememberMe,
                        onChanged: (value) {
                          setState(() => _rememberMe = value!);
                        },
                      ),
                      const Text('记住密码'),
                      const Spacer(),
                      Checkbox(
                        value: _autoLogin,
                        onChanged: (value) {
                          setState(() => _autoLogin = value!);
                        },
                      ),
                      const Text('自动登录'),
                    ],
                  ),

                  const SizedBox(height: 20),

                  SizedBox(
                    width: double.infinity,
                    height: 48,
                    child: ElevatedButton.icon(
                      onPressed: _isLoading ? null : _login,
                      icon: _isLoading
                          ? const SizedBox(
                              width: 20,
                              height: 20,
                              child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                            )
                          : const Icon(Icons.login),
                      label: Text(_isLoading ? '登录中...' : '登录'),
                    ),
                  ).animate().fadeIn().scale(delay: 300.ms),
                  const SizedBox(height: 10),
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      ClickableText('注册新用户'),
                      const SizedBox(height: 8), // 控制垂直间距
                      ClickableText('忘记密码'),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
