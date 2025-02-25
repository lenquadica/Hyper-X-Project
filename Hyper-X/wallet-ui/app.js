async function fetchBalance() {
  const user = document.getElementById("user").value;
  const response = await fetch(`http://localhost:8090/balance/${user}`);
  const balance = await response.text();
  document.getElementById("balance").innerText = balance;
}

async function sendTransaction() {
  const user = document.getElementById("user").value;
  await fetch("http://localhost:8090/send", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ sender: user, receiver: "Bob", amount: 10 })
  });
  alert("Transaction Sent!");
}
