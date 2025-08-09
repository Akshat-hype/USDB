const { execSync } = require('child_process');
const axios = require('axios');

let lastBlockHash = '';

function runCommand(cmd) {
  try {
    const output = execSync(cmd);
    return JSON.parse(output.toString().trim());
  } catch (err) {
    console.error(`❌ Error running "${cmd}":`, err.message);
    return null;
  }
}

function getLatestBlock() {
  const hash = execSync('bitcoin-cli -regtest getbestblockhash').toString().trim();
  const block = runCommand(`bitcoin-cli -regtest getblock ${hash}`);
  return block;
}

function notifyCanister(blockData) {
  axios.post('https://<your-canister-endpoint>', {
    blockHash: blockData.hash,
    blockHeight: blockData.height,
    previousBlockHash: blockData.previousblockhash,
    timestamp: new Date().toISOString(),
  })
  .then(response => {
    console.log('✅ Canister notified successfully:', response.data);
  })
  .catch(error => {
    console.error('❌ Failed to notify canister:', error.message);
    // Retry once after 2 seconds
    setTimeout(() => notifyCanister(blockData), 2000);
  });
}

function watchBlocks() {
  console.log('👀 Watching for new blocks on regtest...');
  setInterval(() => {
    const latestBlock = getLatestBlock();
    if (!latestBlock) return;

    if (latestBlock.hash !== lastBlockHash) {
      if (lastBlockHash && latestBlock.previousblockhash !== lastBlockHash) {
        console.warn('⚠️ Possible chain reorganization detected!');
      }

      console.log(`🧱 New block detected:
      • Hash: ${latestBlock.hash}
      • Height: ${latestBlock.height}
      • Time: ${new Date(latestBlock.time * 1000).toLocaleString()}
      `);

      lastBlockHash = latestBlock.hash;
      notifyCanister(latestBlock);
    }
  }, 5000); // every 5 seconds
}

watchBlocks();
