import 'package:http/http.dart' as http;
import 'dart:convert';

class TransactionService {
  static Future<double> getBalance(String user) async {
    final response = await http.get(Uri.parse('http://localhost:8090/balance/$user'));
    return double.parse(response.body.split(": ")[1]);
  }

  static Future<void> sendTransaction(String sender, String receiver, int amount) async {
    await http.post(
      Uri.parse('http://localhost:8090/send'),
      headers: {"Content-Type": "application/json"},
      body: jsonEncode({"sender": sender, "receiver": receiver, "amount": amount}),
    );
  }
}
