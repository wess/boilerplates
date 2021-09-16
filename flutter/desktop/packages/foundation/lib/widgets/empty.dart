//
// empty.dart
// foundation
// 
// Author: Wess Cope (me@wess.io)
// Created: 09/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

import 'package:flutter/material.dart';

class EmptyAction {
  final String label;
  final VoidCallback action;

  EmptyAction(this.label, this.action);
}

class EmptyView extends StatelessWidget {
  final String? label;
  final IconData? icon;
  final EmptyAction? action;

  EmptyView({
    Key? key,
    this.label,
    this.icon,
    this.action
  }) : super(key: key);


  @override
  Widget build(BuildContext context) {
    return Center(
      child: Container(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            ...(this.icon == null ? [] : [
                Icon(
                  icon,
                  size: 68,
                  color: Colors.white.withAlpha(20)
                ),
            ]),
            SizedBox(height: 12,),
            ...(
              this.label == null ? [] : [
                Text(
                  label!,
                  style: TextStyle(
                    color: Colors.white.withAlpha(20),
                    fontSize: 28                
                  )
                ),
              ]
            ),
            SizedBox(height: 60,),
            ...(
              this.action == null ? [] : [
                TextButton(
                  child: Container(
                    padding: EdgeInsets.fromLTRB(20, 10, 20, 10),
                    child: Text(this.action!.label)
                  ),
                  onPressed: this.action!.action, 
                )
              ]
            )
          ],
        )
      )
    );  
  }
}
