async function fetchBlocks() {
    const response = await fetch("http://localhost:8080/blocks");
    const blocks = await response.json();
    document.getElementById("blocks").innerHTML = blocks.map(b => `<li>${JSON.stringify(b)}</li>`).join("");
}
