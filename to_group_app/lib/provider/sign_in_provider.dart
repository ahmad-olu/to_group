import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:to_group_app/api/auth.dart';

part 'sign_in_provider.g.dart';

class SignInState {
  final String email;
  final String password;
  final bool isSubmitting;
  final bool hasError;
  final String? errorMessage;

  SignInState({
    required this.email,
    required this.password,
    required this.isSubmitting,
    required this.hasError,
    this.errorMessage,
  });

  factory SignInState.empty() => SignInState(
        email: "",
        password: "",
        hasError: false,
        isSubmitting: false,
      );

  SignInState copyWith({
    String? email,
    String? password,
    bool? isSubmitting,
    bool? hasError,
    String? errorMessage,
  }) {
    return SignInState(
      email: email ?? this.email,
      password: password ?? this.password,
      isSubmitting: isSubmitting ?? this.isSubmitting,
      hasError: hasError ?? this.hasError,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }
}

@Riverpod(keepAlive: true)
class SignInNotifier extends _$SignInNotifier {
  @override
  SignInState build() => SignInState.empty();

  void emailStr(String val) {
    state = state.copyWith(email: val);
  }

  void passwordStr(String val) {
    state = state.copyWith(password: val);
  }

  Future<void> signUp() async {
    state = state.copyWith(isSubmitting: true);
    await ref.read(authProvider).login(
          state.email.trim(),
          state.password.trim(),
        );
    try {
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
