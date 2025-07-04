import 'package:dio/dio.dart';

class HttpService {
  static final Dio _dio = Dio(
    BaseOptions(
      baseUrl: 'http://127.0.0.1:3000', // 统一配置 baseUrl
      connectTimeout: const Duration(seconds: 5),
      receiveTimeout: const Duration(seconds: 5),
      headers: {'Content-Type': 'application/json'},
    ),
  );

  static Future<Response> post(String path, {Map<String, dynamic>? data}) async {
    try {
      final response = await _dio.post(path, data: data);
      return response;
    } on DioException catch (e) {
      throw Exception('POST 请求错误: ${e.response?.data ?? e.message}');
    }
  }

  static Future<Response> get(String path, {Map<String, dynamic>? params}) async {
    try {
      final response = await _dio.get(path, queryParameters: params);
      return response;
    } on DioException catch (e) {
      throw Exception('GET 请求错误: ${e.response?.data ?? e.message}');
    }
  }
}
