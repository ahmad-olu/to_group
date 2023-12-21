import 'dart:convert';

class ObjectId {
  final String oid;

  ObjectId({required this.oid});

  Map<String, dynamic> toMap() {
    return {
      r'$oid': oid,
    };
  }

  factory ObjectId.fromMap(Map<String, dynamic> map) {
    return ObjectId(
      oid: map[r'$oid'] ?? '',
    );
  }

  String toJson() => json.encode(toMap());

  factory ObjectId.fromJson(String source) =>
      ObjectId.fromMap(json.decode(source));
}
