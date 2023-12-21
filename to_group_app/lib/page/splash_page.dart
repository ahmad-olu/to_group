import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:to_group_app/provider/auth_provider.dart';

import 'home_page.dart';
import 'login_page.dart';

class SplashPage extends ConsumerWidget {
  const SplashPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return ref.watch(authCheckProvider).maybeMap(
          data: (data) {
            if (data.value != null) {
              return const HomePage();
            } else {
              return const LoginPage();
            }
          },
          orElse: () => const LoginPage(),
        );
  }
}
