import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:shared_preferences/shared_preferences.dart';

part 'local.g.dart';

class Local {
  static const String tokenKey = "token";

  Future<void> saveToken(String token) async {
    final res = await SharedPreferences.getInstance();
    await res.setString(tokenKey, token);
  }

  Future<String?> getToken() async {
    final res = await SharedPreferences.getInstance();
    return res.getString(tokenKey);
  }

  Future<bool> deleteToken() async {
    final res = await SharedPreferences.getInstance();
    return res.remove(tokenKey);
  }
}

@riverpod
Local local(LocalRef ref) {
  return Local();
}
