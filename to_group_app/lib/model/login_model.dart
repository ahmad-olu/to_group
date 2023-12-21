import 'dart:convert';

class LoginResponse {
  final String id;
  final String fullName;
  final String email;
  final String token;

  LoginResponse({
    required this.id,
    required this.fullName,
    required this.email,
    required this.token,
  });

  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'full_name': fullName,
      'email': email,
      'token': token,
    };
  }

  factory LoginResponse.fromMap(Map<String, dynamic> map) {
    return LoginResponse(
      id: map['id'] ?? '',
      fullName: map['full_name'] ?? '',
      email: map['email'] ?? '',
      token: map['token'] ?? '',
    );
  }

  String toJson() => json.encode(toMap());

  factory LoginResponse.fromJson(String source) =>
      LoginResponse.fromMap(json.decode(source));

  @override
  String toString() {
    return 'LoginResponse(id: $id, fullName: $fullName, email: $email, token: $token)';
  }
}
