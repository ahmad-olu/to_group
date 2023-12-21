import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:to_group_app/api/local.dart';
part 'auth_provider.g.dart';

@riverpod
Future<String?> authCheck(AuthCheckRef ref) async {
  final a = await ref.watch(localProvider).getToken();
  return a;
}
