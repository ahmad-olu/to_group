import 'dart:convert';

import 'package:to_group_app/model/object_id.dart';

class TodoModel {
  final ObjectId id;
  final ObjectId uid;
  final String title;
  final String description;
  final String timeFrom;
  final String timeTo;
  final String date;
  final bool isCompleted;
  final List<SubTodo>? subTodo;

  TodoModel({
    required this.id,
    required this.uid,
    required this.title,
    required this.description,
    required this.timeFrom,
    required this.timeTo,
    required this.date,
    required this.isCompleted,
    required this.subTodo,
  });

  Map<String, dynamic> toMap() {
    return {
      '_id': id.toMap(),
      'uid': uid.toMap(),
      'title': title,
      'description': description,
      'time_from': timeFrom,
      'time_to': timeTo,
      'date': date,
      'is_completed': isCompleted,
      'sub_todo': subTodo?.map((x) => x.toMap()).toList(),
    };
  }

  factory TodoModel.fromMap(Map<String, dynamic> map) {
    return TodoModel(
      id: ObjectId.fromMap(map['_id']),
      uid: ObjectId.fromMap(map['uid']),
      title: map['title'] ?? '',
      description: map['description'] ?? '',
      timeFrom: map['time_from'] ?? '',
      timeTo: map['time_to'] ?? '',
      date: map['date'] ?? '',
      isCompleted: map['is_completed'] ?? false,
      subTodo: map['sub_todo'] != null
          ? List<SubTodo>.from(map['sub_todo']?.map((x) => SubTodo.fromMap(x)))
          : null,
    );
  }

  String toJson() => json.encode(toMap());

  factory TodoModel.fromJson(String source) =>
      TodoModel.fromMap(json.decode(source));

  @override
  String toString() {
    return 'TodoModel(id: $id, uid: $uid, title: $title, description: $description, time_from: $timeFrom, time_to: $timeTo, date: $date, is_completed: $isCompleted, sub_todo: $subTodo)';
  }
}

class SubTodo {
  final String id;
  final String title;
  final String description;
  final String timeFrom;
  final String timeTo;
  final String icon;
  final String color;
  final String taskType;

  SubTodo({
    required this.id,
    required this.title,
    required this.description,
    required this.timeFrom,
    required this.timeTo,
    required this.icon,
    required this.color,
    required this.taskType,
  });

  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'title': title,
      'description': description,
      'time_from': timeFrom,
      'time_to': timeTo,
      'icon': icon,
      'color': color,
      'task_type': taskType,
    };
  }

  factory SubTodo.fromMap(Map<String, dynamic> map) {
    return SubTodo(
      id: map['id'] ?? '',
      title: map['title'] ?? '',
      description: map['description'] ?? '',
      timeFrom: map['time_from'] ?? '',
      timeTo: map['time_to'] ?? '',
      icon: map['icon'] ?? '',
      color: map['color'] ?? '',
      taskType: map['task_type'] ?? '',
    );
  }

  String toJson() => json.encode(toMap());

  factory SubTodo.fromJson(String source) =>
      SubTodo.fromMap(json.decode(source));

  @override
  String toString() {
    return 'SubTodo(id: $id, title: $title, description: $description, time_from: $timeFrom, time_to: $timeTo, icon: $icon, color: $color, task_type: $taskType)';
  }
}
