import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:to_group_app/api/auth.dart';

part 'sign_up_provider.g.dart';

class SignUpState {
  final String fullName;
  final String email;
  final String password;
  final bool isSubmitting;
  final bool hasError;
  final String? errorMessage;

  SignUpState({
    required this.fullName,
    required this.email,
    required this.password,
    required this.isSubmitting,
    required this.hasError,
    this.errorMessage,
  });

  factory SignUpState.empty() => SignUpState(
        fullName: "",
        email: "",
        password: "",
        hasError: false,
        isSubmitting: false,
      );

  SignUpState copyWith({
    String? fullName,
    String? email,
    String? password,
    bool? isSubmitting,
    bool? hasError,
    String? errorMessage,
  }) {
    return SignUpState(
      fullName: fullName ?? this.fullName,
      email: email ?? this.email,
      password: password ?? this.password,
      isSubmitting: isSubmitting ?? this.isSubmitting,
      hasError: hasError ?? this.hasError,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }
}

@Riverpod(keepAlive: true)
class SignUpNotifier extends _$SignUpNotifier {
  @override
  SignUpState build() => SignUpState.empty();

  void emailStr(String val) {
    state = state.copyWith(email: val);
  }

  void fullNameStr(String val) {
    state = state.copyWith(fullName: val);
  }

  void passwordStr(String val) {
    state = state.copyWith(password: val);
  }

  Future<void> signUp() async {
    state = state.copyWith(isSubmitting: true);
    try {
      await ref.read(authProvider).signUp(
            state.fullName.trim(),
            state.email.trim(),
            state.password.trim(),
          );
      state = state.copyWith(isSubmitting: false);
    } catch (e) {
      state = state.copyWith(
        isSubmitting: false,
        hasError: true,
        errorMessage: e.toString(),
      );
    }
  }
}
