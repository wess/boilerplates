//
// main.dart
// ancilla
// 
// Author: Wess Cope (me@wess.io)
// Created: 09/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

import 'dart:io';

import 'package:{{APP}}/components/nav/provider.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:{{APP}}/components/app/component.dart';
import 'package:{{APP}}/components/app/provider.dart';
import 'package:window_size/window_size.dart';
import 'package:foundation/theme/provider.dart';
import 'package:foundation/theme/colors.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
    setWindowTitle("{{APP}}");
    setWindowMinSize(const Size(720, 600));
    setWindowVisibility(visible: true);
  }

  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (context) => ThemeProvider()),
        ChangeNotifierProvider(create: (context) => AppProvider()),
        ChangeNotifierProvider(create: (context) => NavProvider()),
      ],
      child: Main(),
    )
  );
}

class Main extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: '{{APP}}',
      themeMode: context.watch<ThemeProvider>().mode,
      theme: Nord.theme,
      darkTheme: Nord.darkTheme,
      home: AppComponent(),
    );
  }
}

