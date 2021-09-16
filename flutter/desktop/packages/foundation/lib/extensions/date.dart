//
// date.dart
// foundation
// 
// Author: Wess Cope (me@wess.io)
// Created: 09/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

extension DateTimeExt on DateTime {
  int get unixTimestamp => this.toUtc().millisecondsSinceEpoch;
}
