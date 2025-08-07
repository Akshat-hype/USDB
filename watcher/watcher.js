const { execSync } = require('child_process');
const axios = require('axios');

let lastBlockHash = '';

function getLatestBlockHash() {
  try {
    const output = execSync('bitcoin-cli -regtest getbestblockhash');
    return output.toString().trim();
  } catch (err) {
    console.error('Error fetching block hash:', err.message);
    return null;
  }
}

function notifyCanister(blockHash) {
  axios.post('https://<your-canister-endpoint>', {
    blockHash: blockHash,
    timestamp: new Date().toISOString(),
  })
  .then(response => {
    console.log('✅ Canister notified:', response.data);
  })
  .catch(error => {
    console.error('❌ Failed to notify canister:', error.message);
  });
}

function watchBlocks() {
  console.log('👀 Watching for new blocks on regtest...');
  setInterval(() => {
    const currentHash = getLatestBlockHash();
    if (currentHash && currentHash !== lastBlockHash) {
      console.log(`🧱 New block detected: ${currentHash}`);
      lastBlockHash = currentHash;
      notifyCanister(currentHash);
    }
  }, 5000); // every 5 seconds
}

watchBlocks();
