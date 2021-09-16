//
// theme.dart
// Ancilla
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/22/2021
// 
// Copywrite (c) 2021 Wess.io
//

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class ThemeProvider extends ChangeNotifier {
  ThemeMode _mode = ThemeMode.system;
  
  ThemeMode get mode => _mode;
  set mode(ThemeMode mode) {
    _mode = mode;
  
    notifyListeners();
  }
  
}