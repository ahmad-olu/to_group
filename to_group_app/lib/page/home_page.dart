import 'package:expandable/expandable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:to_group_app/api/local.dart';
import 'package:to_group_app/provider/todos_provider.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Consumer(
          builder: (context, ref, child) {
            return ElevatedButton(
                onPressed: () async {
                  final res = await ref.read(localProvider).deleteToken();
                  if (res == true) {
                    if (!context.mounted) return;
                    Navigator.of(context).push(
                      MaterialPageRoute(
                        builder: (context) => const HomePage(),
                      ),
                    );
                  }
                },
                child: const Text('Exit'));
          },
        ),
      ),
      body: Consumer(
        builder: (context, ref, child) {
          return ref.watch(allTodoProvider).map(
            data: (data) {
              return Padding(
                padding: const EdgeInsets.all(18.0),
                child: ListView.builder(
                  itemCount: data.value.length,
                  itemBuilder: (context, index) {
                    final todo = data.value[index];
                    return ExpandablePanel(
                      header: ListTile(
                        leading: SizedBox(
                          width: 12,
                          child: Column(
                            children: [
                              Text(todo.timeFrom, textScaleFactor: 0.2),
                              Text(todo.timeTo, textScaleFactor: 0.2),
                            ],
                          ),
                        ),
                        title: Text(todo.title),
                        subtitle: Text(todo.description),
                      ),
                      collapsed: const Text(''),
                      expanded: todo.subTodo == null || todo.subTodo!.isEmpty
                          ? const Center(child: Text('Nothing else'))
                          : Container(
                              color: Colors.red,
                              height: 150,
                              child: Padding(
                                padding: const EdgeInsets.all(15.0),
                                child: Container(
                                  color:
                                      Theme.of(context).scaffoldBackgroundColor,
                                  child: ListView.builder(
                                    itemCount: todo.subTodo!.length,
                                    itemBuilder: (context, index) {
                                      final subTodo = todo.subTodo![index];
                                      return ListTile(
                                        title: Text(subTodo.title),
                                        subtitle: Text(
                                            '${subTodo.description} - ${subTodo.timeFrom} to ${subTodo.timeTo}'),
                                      );
                                    },
                                  ),
                                ),
                              ),
                            ),
                    );
                  },
                ),
              );
            },
            error: (error) {
              return Center(
                child: Text(error.error.toString()),
              );
            },
            loading: (loading) {
              return const Center(
                child: CircularProgressIndicator(),
              );
            },
          );
        },
      ),
    );
  }
}
