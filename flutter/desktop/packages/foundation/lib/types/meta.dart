//
// meta.dart
// founation
// 
// Author: Wess Cope (me@wess.io)
// Created: 09/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

import 'dart:convert' as dart_convert;

class Meta {
  Map<String, dynamic> _backing = {};

  dynamic get(String key) => _backing[key];

  Meta insert(Map<String, dynamic> data) {
    _backing = {
      ..._backing,
      ...data
    };

    return this;
  }

  void clear() => _backing = {};

  Map<String, dynamic> get json => _backing;

  String get jsonString => dart_convert.json.encode(json);

  operator [](String key) => _backing[key];
  operator []=(String key, dynamic value) => _backing[key] = value;

  Meta({Map<String, dynamic> data = const {}}) : _backing = data;

  factory Meta.fromJsonString(String data) => Meta(data: dart_convert.json.decode(data));
}