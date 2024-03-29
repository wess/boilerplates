//
// color.dart
// foundation
// 
// Author: Wess Cope (me@wess.io)
// Created: 09/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

import 'package:flutter/material.dart';


Color _colorFromHex(String hex) {
  hex = hex.replaceAll('#', '');

  switch(hex.length) {
    case 3:
      hex = "FF" + hex + hex;
      break;
    case 6:
      hex = "FF" + hex;
      break;
    case 8:
      break;
    default:
      throw Exception("Color hex code can only be 3, 6 or 8 characters long (not counting #)");
  }


  return Color(
    int.parse("0x$hex")
  );
}

extension ColorExt on Color {
  static Color from(String hexString) {
    return _colorFromHex(hexString);
  }
}

extension StringColorExt on String {
  toColor() {
    return _colorFromHex(this);
  }
}
