import 'dart:convert';
import 'dart:developer';

import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:to_group_app/api/local.dart';
import 'package:http/http.dart' as http;
import 'package:to_group_app/model/login_model.dart';
part 'auth.g.dart';

class Auth {
  final Local localStorage;

  static String baseUrl = "http://localhost:3000";

  Auth({required this.localStorage});
  Future<void> signUp(
    String fullName,
    String email,
    String password,
  ) async {
    try {
      final response = await http.post(
        Uri.parse("$baseUrl/users"),
        body: jsonEncode(
            {'full_name': fullName, 'email': email, 'password': password}),
        headers: {
          "Content-Type": "application/json",
        },
      );

      if (response.statusCode == 200) {
        final data = LoginResponse.fromJson(response.body);
        localStorage.saveToken(data.token);
      } else if (response.statusCode >= 400) {
        final Map<String, dynamic> err = json.decode(response.body)["error"];
        log('error message: $err');
      }
      //"error": "not authenticated"
    } catch (e) {
      log("error::::$e");
    }
  }

  Future<void> login(
    String email,
    String password,
  ) async {
    try {
      final response = await http.post(
        Uri.parse("$baseUrl/login"),
        body: jsonEncode({'email': email, 'password': password}),
        headers: {
          "Content-Type": "application/json",
        },
      );

      if (response.statusCode == 200) {
        final data = LoginResponse.fromJson(response.body);
        log(":::::: $data");
        localStorage.saveToken(data.token);
      } else if (response.statusCode >= 400) {
        final Map<String, dynamic> err = json.decode(response.body)["error"];
        log('error message: $err');
      }
    } catch (e) {
      log("error::::$e");
    }
  }
}

@riverpod
Auth auth(AuthRef ref) {
  final local = ref.watch(localProvider);
  return Auth(localStorage: local);
}
