import 'package:flutter/material.dart';

class NFTScreen extends StatelessWidget {
  final List<String> nftImages = [
    "https://example.com/nft1.png",
    "https://example.com/nft2.png"
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text("Hyper-X NFT Marketplace")),
      body: ListView.builder(
        itemCount: nftImages.length,
        itemBuilder: (context, index) {
          return Card(
            child: Column(
              children: [
                Image.network(nftImages[index]),
                ElevatedButton(
                  onPressed: () {},
                  child: Text("Buy NFT"),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}
