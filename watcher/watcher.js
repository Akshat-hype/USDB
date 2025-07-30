const BitcoinCore = require("bitcoin-core");
const { HttpAgent, Identity, Actor } = require("@dfinity/agent");
const { idlFactory } = require("../.dfx/local/canisters/usdb-rune-backend/usdb-rune-backend.did.js");
const fetch = require("node-fetch");

global.fetch = fetch; // Required for @dfinity/agent

// Setup Bitcoin RPC
const client = new BitcoinCore({
  network: "regtest",
  username: "yourrpcuser",
  password: "yourrpcpassword",
  port: 18443,
  host: "127.0.0.1"
});

// Your canister ID & principal mapping
const USDB_CANISTER_ID = "your-canister-id";
const userMap = {
  "tb1qmc3r3vnjtzfj2slehrklehw5vmqr3je8d8wzc6": {
    principal: "user-principal-here",
    amount: 1000
  }
};

// Setup Internet Computer Agent
const agent = new HttpAgent({ host: "http://127.0.0.1:4943" }); // local replica

const usdb = Actor.createActor(idlFactory, {
  agent,
  canisterId: USDB_CANISTER_ID,
});

// Monitor BTC mempool or latest block
async function checkTransactions() {
  const transactions = await client.listTransactions("*", 100);
  for (let tx of transactions) {
    if (tx.category === "receive" && userMap[tx.address]) {
      const user = userMap[tx.address];
      console.log(`✅ Received ${tx.amount} BTC for ${user.principal}`);

      try {
        const result = await usdb.confirm_and_mint(user.amount, [tx.txid]);
        console.log("🎉 Minted USDB:", result);
        delete userMap[tx.address]; // Prevent re-processing
      } catch (err) {
        console.error("Minting failed:", err);
      }
    }
  }
}

setInterval(checkTransactions, 10_000); // Check every 10 seconds
