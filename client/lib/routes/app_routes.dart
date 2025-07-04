import 'package:client/views/login/login_page.dart' show LoginPage;
import 'package:client/views/notes/note_list_page.dart';
import 'package:flutter/material.dart' show Scaffold, Text, Center;
import 'package:go_router/go_router.dart' show GoRoute, GoRouter;

class AppRoutes {
  static final GoRouter router = GoRouter(
    initialLocation: '/user/login',
    routes: [
      GoRoute(path: '/user/login', name: 'user_login', builder: (context, state) => const LoginPage()),
      GoRoute(path: "/note/list", name: "note_list", builder: (context, state) => const NoteListPage()),
      // GoRoute(path: '/notes', name: 'notes', builder: (context, state) => const NoteListPage()),
      // GoRoute(path: '/settings', name: 'settings', builder: (context, state) => const SettingsPage()),
    ],
    errorBuilder: (context, state) => const Scaffold(body: Center(child: Text('页面未找到'))),
  );
}
