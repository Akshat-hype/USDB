const { execSync, exec } = require("child_process");

let lastBlockHash = "";

// Run bitcoin-cli commands
function runCommand(cmd) {
  try {
    const output = execSync(cmd);
    return JSON.parse(output.toString().trim());
  } catch (err) {
    console.error(`❌ Error running "${cmd}":`, err.message);
    return null;
  }
}

// Get latest block info
function getLatestBlock() {
  const hash = execSync("bitcoin-cli -regtest getbestblockhash").toString().trim();
  const block = runCommand(`bitcoin-cli -regtest getblock ${hash}`);
  return block;
}

// Notify the canister with txid, address, sats, usdb_amount, and receiver
function notifyCanister(txid, btcAddress, amountSats, usdbAmount, receiver) {
  const canisterId = "avqkn-guaaa-aaaaa-qaaea-cai";

  const cmd = `didc call ${canisterId} notify_btc_received '(record {
    btc_address = "${btcAddress}";
    txid = "${txid}";
    amount_sats = ${amountSats};
    usdb_amount = ${usdbAmount};
    receiver = principal "${receiver}";
  })'`;

  console.log("🚀 Executing:", cmd);

  exec(cmd, (err, stdout, stderr) => {
    if (err) {
      console.error("❌ Error calling canister:", err.message);
      console.error("stderr:", stderr);
      return;
    }
    console.log("✅ Canister notified successfully!");
    console.log("stdout:", stdout);
  });
}

// Watch Bitcoin blocks
function watchBlocks() {
  console.log("👀 Watching for new blocks on regtest...");
  setInterval(() => {
    const latestBlock = getLatestBlock();
    if (!latestBlock) return;

    if (latestBlock.hash !== lastBlockHash) {
      if (lastBlockHash && latestBlock.previousblockhash !== lastBlockHash) {
        console.warn("⚠️ Possible chain reorganization detected!");
      }

      console.log(`🧱 New block detected:
      • Hash: ${latestBlock.hash}
      • Height: ${latestBlock.height}
      • Time: ${new Date(latestBlock.time * 1000).toLocaleString()}
      `);

      lastBlockHash = latestBlock.hash;

      // 🔥 Example: just take first tx in block
      if (latestBlock.tx && latestBlock.tx.length > 0) {
        const txid = latestBlock.tx[0];  // first tx id
        const btcAddress = "bcrt1qexampleaddress"; // TODO: replace with real
        const sats = 50000; // TODO: parse actual amount from tx
        const usdbAmount = 50; // Example: 1k sats = 1 USDB
        const receiver = "aaaaa-aa"; // TODO: replace with real principal

        notifyCanister(txid, btcAddress, sats, usdbAmount, receiver);
      }
    }
  }, 5000); // every 5 sec
}

watchBlocks();
