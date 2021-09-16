//
// colors.dart
// Ancilla
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/22/2021
// 
// Copywrite (c) 2021 Wess.io
//


import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

import '../extensions/color.dart';

class Nord {
  static List<Color> polar = [
    "#2E3440".toColor(),
    "#3B4252".toColor(),
    "#434C5E".toColor(),
    "#4C566A".toColor(),
  ];

  static List<Color> snow = [
    "#D8DEE9".toColor(),
    "#E5E9F0".toColor(),
    "#ECEFF4".toColor(),
  ];

  static List<Color> frost = [
    "#8FBCBB".toColor(),
    "#88C0D0".toColor(),
    "#81A1C1".toColor(),
    "#5E81AC".toColor(),
  ];

  static List<Color> aurora = [
    "#BF616A".toColor(),
    "#D08770".toColor(),
    "#EBCB8B".toColor(),
    "#A3BE8C".toColor(),
    "#B48EAD".toColor(),
  ];

  static MaterialColor _convert(Color color) {
    return MaterialColor(
      color.value, {
        50: color.withAlpha(45),
        100: color.withAlpha(65),
        200: color.withAlpha(85),
        300: color.withAlpha(105),
        400: color.withAlpha(125),
        500: color.withAlpha(145),
        600: color.withAlpha(165),
        700: color.withAlpha(185),
        800: color.withAlpha(205),
        900: color.withAlpha(225),
      }
    );
  }

  static ThemeData get theme => ThemeData(
    brightness: Brightness.light,
    primarySwatch: _convert(Nord.snow[0])
  );

  static ThemeData get darkTheme => ThemeData(
    brightness: Brightness.dark,
    primarySwatch: _convert(Nord.polar[3]),
    scaffoldBackgroundColor: Nord.polar[1]
  );
}
