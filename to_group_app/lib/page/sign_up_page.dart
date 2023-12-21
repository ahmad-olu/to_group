import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:to_group_app/page/login_page.dart';
import 'package:to_group_app/provider/sign_up_provider.dart';

class SignUpPage extends StatefulWidget {
  const SignUpPage({super.key});

  @override
  State<SignUpPage> createState() => _SignUpPageState();
}

class _SignUpPageState extends State<SignUpPage> {
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
                    const Text("Sign Up", textScaleFactor: 2.0),
                    const SizedBox(height: 100),
                    TextFormField(
                      decoration: const InputDecoration(hintText: "Full Name"),
                      onChanged: (value) => ref
                          .read(signUpNotifierProvider.notifier)
                          .fullNameStr(value),
                      validator: (value) {
                        if (value!.isEmpty) {
                          return "Full Name is required";
                        }
                        return null;
                      },
                    ),
                    const SizedBox(height: 20),
                    TextFormField(
                      decoration: const InputDecoration(hintText: "Email"),
                      onChanged: (value) => ref
                          .read(signUpNotifierProvider.notifier)
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
                          .read(signUpNotifierProvider.notifier)
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
                            ref.read(signUpNotifierProvider.notifier).signUp();
                          }
                        },
                        child: const Text('Sign Up')),
                    const SizedBox(height: 20),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        const Text('Already have an account? '),
                        TextButton(
                          onPressed: () {
                            Navigator.of(context).push(
                              MaterialPageRoute(
                                builder: (context) => const LoginPage(),
                              ),
                            );
                          },
                          child: const Text('Sign in'),
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
