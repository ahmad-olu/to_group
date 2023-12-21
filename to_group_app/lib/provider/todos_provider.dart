import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:to_group_app/api/todo.dart';
import 'package:to_group_app/model/todo.dart';
part 'todos_provider.g.dart';

@riverpod
Future<List<TodoModel>> allTodo(AllTodoRef ref) async {
  final todo = ref.watch(todoProvider).getAllTodo();
  return todo;
}
