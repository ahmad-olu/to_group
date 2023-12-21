import 'package:flutter/material.dart';

ThemeData lightTheme = ThemeData(
  appBarTheme: AppBarTheme(
      color: Colors.brown.shade500,
      iconTheme: const IconThemeData(color: Colors.white)),
  colorScheme: ColorScheme.fromSeed(
    seedColor: Colors.brown,
    brightness: Brightness.light,
  ),
);

ThemeData darkTheme = ThemeData(
  appBarTheme: AppBarTheme(
      color: Colors.lime.shade300,
      iconTheme: const IconThemeData(color: Colors.black)),
  colorScheme: ColorScheme.fromSeed(
    seedColor: Colors.lime,
    brightness: Brightness.dark,
  ),
);
