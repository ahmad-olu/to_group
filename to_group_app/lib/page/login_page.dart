import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:to_group_app/main.dart';
import 'package:to_group_app/page/sign_up_page.dart';
import 'package:to_group_app/provider/sign_in_provider.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final _formKey = GlobalKey<FormState>();
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: Form(
          key: _formKey,
          child: Padding(
            padding: const EdgeInsets.all(30.0),
            child: Consumer(
              builder: (context, ref, child) {
                return Column(
                  children: [
                    const Text("Sign In", textScaleFactor: 2.0),
                    const SizedBox(height: 100),
                    TextFormField(
                      decoration: const InputDecoration(hintText: "Email"),
                      onChanged: (value) => ref
                          .read(signInNotifierProvider.notifier)
                          .emailStr(value),
                      validator: (value) {
                        if (value!.isEmpty) {
                          return "Full Name is required";
                        }
                        return null;
                      },
                    ),
                    const SizedBox(height: 20),
                    TextFormField(
                      decoration: const InputDecoration(hintText: "Password"),
                      obscureText: true,
                      onChanged: (value) => ref
                          .read(signInNotifierProvider.notifier)
                          .passwordStr(value),
                      validator: (value) {
                        if (value!.isEmpty) {
                          return "Full Name is required";
                        }
                        return null;
                      },
                    ),
                    const SizedBox(height: 40),
                    ElevatedButton(
                        onPressed: () {
                          if (_formKey.currentState!.validate()) {
                            ref.read(signInNotifierProvider.notifier).signUp();
                          }
                        },
                        child: const Text('Sign In')),
                    const SizedBox(height: 20),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        const Text('Already have an account? '),
                        TextButton(
                          onPressed: () {
                            Navigator.of(context).push(
                              MaterialPageRoute(
                                builder: (context) => const SignUpPage(),
                              ),
                            );
                          },
                          child: const Text('Sign Up'),
                        )
                      ],
                    ),
                  ],
                );
              },
            ),
          ),
        ),
      ),
    );
  }
}
