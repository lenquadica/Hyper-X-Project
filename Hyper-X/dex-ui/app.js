async function fetchOrders() {
  const response = await fetch("http://localhost:8100/orders");
  const orders = await response.json();
  document.getElementById("order-book").innerHTML = orders.map(order =>
      `<li>${order.is_buy ? '🟢 Buy' : '🔴 Sell'} ${order.amount} HYPX at ${order.price} USDT - ${order.trader}</li>`
  ).join("");
}

async function placeBuyOrder() {
  const user = document.getElementById("user").value;
  const amount = document.getElementById("amount").value;
  const price = document.getElementById("price").value;

  await fetch("http://localhost:8100/order", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ trader: user, amount, price, is_buy: true })
  });

  alert("Buy Order Placed!");
  fetchOrders();
}

async function placeSellOrder() {
  const user = document.getElementById("user").value;
  const amount = document.getElementById("amount").value;
  const price = document.getElementById("price").value;

  await fetch("http://localhost:8100/order", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ trader: user, amount, price, is_buy: false })
  });

  alert("Sell Order Placed!");
  fetchOrders();
}

window.onload = fetchOrders;
