import 'package:flutter/material.dart';
import 'transaction_service.dart';
import 'nft_screen.dart';

class WalletScreen extends StatefulWidget {
  @override
  _WalletScreenState createState() => _WalletScreenState();
}

class _WalletScreenState extends State<WalletScreen> {
  double balance = 0.0;

  @override
  void initState() {
    super.initState();
    fetchBalance();
  }

  void fetchBalance() async {
    double fetchedBalance = await TransactionService.getBalance("Alice");
    setState(() {
      balance = fetchedBalance;
    });
  }

  void sendTransaction() async {
    await TransactionService.sendTransaction("Alice", "Bob", 10);
    fetchBalance();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text("Hyper-X Wallet")),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text("💰 Balance: $balance HYPX", style: TextStyle(fontSize: 24)),
          SizedBox(height: 20),
          ElevatedButton(
            onPressed: sendTransaction,
            child: Text("Send 10 HYPX to Bob"),
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.push(context, MaterialPageRoute(builder: (context) => NFTScreen()));
            },
            child: Text("View NFTs"),
          ),
        ],
      ),
    );
  }
}
