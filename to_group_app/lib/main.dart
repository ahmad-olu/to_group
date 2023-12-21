import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:to_group_app/provider/auth_provider.dart';
import 'package:to_group_app/provider/sign_in_provider.dart';
import 'package:to_group_app/provider/sign_up_provider.dart';
import 'package:to_group_app/themes.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'page/login_page.dart';
import 'page/splash_page.dart';

void main() {
  runApp(const ProviderScope(child: MainApp()));
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: lightTheme,
      darkTheme: darkTheme,
      themeMode: ThemeMode.light,
      home: const SplashPage(),
    );
  }
}
