import 'package:flutter/material.dart';
import 'wallet_screen.dart';
import 'nft_screen.dart';

void main() {
  runApp(HyperXWallet());
}

class HyperXWallet extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Hyper-X Wallet',
      theme: ThemeData.dark(),
      home: WalletScreen(),
    );
  }
}
