//
// list.dart
// foundation
// 
// Author: Wess Cope (me@wess.io)
// Created: 09/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

extension ListExt<T> on List<T> {
  T? find(bool Function(T index) predicate) {
    try {
      return firstWhere(predicate);
    } on StateError {
      return null;
    }
  }

  List<T> slice(int start, int end) {
    var stop = end < 0 ? this.length + end : end;

    return this.sublist(start, stop);
  }
}

