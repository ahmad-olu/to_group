import 'dart:convert';
import 'dart:developer';

import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:to_group_app/api/local.dart';
import 'package:http/http.dart' as http;
import 'package:to_group_app/model/login_model.dart';
import 'package:to_group_app/model/todo.dart';
part 'todo.g.dart';

class Todo {
  final Local localStorage;

  static String baseUrl = "http://localhost:3000";

  Todo({required this.localStorage});
  Future<List<TodoModel>> getAllTodo() async {
    final headerToken = await localStorage.getToken();
    try {
      final response = await http.get(
        Uri.parse("$baseUrl/todo"),
        headers: {
          "x-auth-token": headerToken ?? '',
        },
      );
      if (response.statusCode == 200) {
        final List<dynamic> data = json.decode(response.body)["result"];
        log("::::::::::::$data");
        return data.map((todo) => TodoModel.fromMap(todo)).toList();
      } else if (response.statusCode >= 400) {
        final Map<String, dynamic> err = json.decode(response.body)["error"];
        throw Exception('${response.statusCode}:error message: $err');
      }
      //"error": "not authenticated"
      throw Exception('error message: error occurred');
    } catch (e) {
      log("error message: $e");
      throw Exception('error message: $e');
    }
  }
  // Future<void> getAllTodo(
  //   String fullName,
  //   String email,
  //   String password,
  // ) async {
  //   try {
  //     final response = await http.post(
  //       Uri.parse("$baseUrl/users"),
  //       body: jsonEncode(
  //           {'full_name': fullName, 'email': email, 'password': password}),
  //       headers: {
  //         "Content-Type": "application/json",
  //       },
  //     );

  //     if (response.statusCode == 200) {
  //       final data = LoginResponse.fromJson(response.body);
  //       localStorage.saveToken(data.token);
  //     } else if (response.statusCode >= 400) {
  //       final Map<String, dynamic> err = json.decode(response.body)["error"];
  //       log('error message: $err');
  //     }
  //     //"error": "not authenticated"
  //   } catch (e) {
  //     log("error::::$e");
  //   }
  // }
}

@riverpod
Todo todo(TodoRef ref) {
  final local = ref.watch(localProvider);
  return Todo(localStorage: local);
}
