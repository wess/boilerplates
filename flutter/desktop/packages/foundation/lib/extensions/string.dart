//
// string.dart
// foundation
// 
// Author: Wess Cope (me@wess.io)
// Created: 09/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

import 'dart:io';

extension StringExt on String {
  String toFirstUpperCase() {
    final first = this.substring(0, 1);
    final rest = this.substring(1);

    return first.toUpperCase() + rest;
  }

  Future<bool> get isDirectory async => (await FileSystemEntity.type(this)) == FileSystemEntityType.directory;
  Future<bool> get isFile async => (await FileSystemEntity.type(this)) == FileSystemEntityType.file; 
}