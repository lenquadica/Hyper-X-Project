async function fetchNFTs() {
  const response = await fetch("http://localhost:8081/nfts");
  const nfts = await response.json();
  document.getElementById("nft-list").innerHTML = nfts.map(nft =>
      `<li>${nft.metadata} - Owned by: ${nft.owner} 
      <button onclick="buyNFT(${nft.id})">Buy</button></li>`).join("");
}

async function mintNFT() {
  const metadata = document.getElementById("metadata").value;
  await fetch("http://localhost:8081/mint", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ owner: "Alice", metadata })
  });
  alert("NFT Minted!");
  fetchNFTs();
}

async function buyNFT(nftId) {
  await fetch(`http://localhost:8081/buy/${nftId}`, { method: "POST" });
  alert("NFT Purchased!");
  fetchNFTs();
}

window.onload = fetchNFTs;
