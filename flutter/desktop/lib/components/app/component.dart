//
// component.dart
// vanir
// 
// Author: Wess Cope (me@wess.io)
// Created: 09/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

import 'package:flutter/material.dart';
import 'package:foundation/theme/colors.dart';
import 'package:{{APP}}/components/nav/component.dart';

class AppComponent extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Container(
            color: Nord.polar[0],
            child: NavComponent()
          ),
          Expanded(
            child: Container(
              child: Text("there")
            ),
          ),
        ],
      )
    );
  }
}