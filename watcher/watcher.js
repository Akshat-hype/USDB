import fetch from 'node-fetch';
import {
  BTC_NODE_URL,
  RPC_USER,
  RPC_PASS,
  DEPOSIT_ADDRESSES,
  POLL_INTERVAL,
  rpcURL
} from './config.js';

import { notifyCanister } from './notify_canister.js';

async function callRPC(method, params = []) {
  const res = await fetch(rpcURL, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      jsonrpc: '1.0',
      id: 'watcher',
      method,
      params,
    }),
  });

  const text = await res.text(); // get raw response
  console.log("📦 Raw RPC Response:", text);

  return JSON.parse(text); // then try parsing
}



async function pollMempool() {
  const txids = await callRPC('getrawmempool');
  if (!txids) return;

  for (const txid of txids) {
    const rawTx = await callRPC('getrawtransaction', [txid, true]);
    if (!rawTx || !rawTx.vout) continue;

    for (const output of rawTx.vout) {
      const addresses = output.scriptPubKey.addresses || [];
      const matched = addresses.find(addr => DEPOSIT_ADDRESSES.includes(addr));
      if (matched) {
        const amount = Math.floor(output.value * 100000000); // satoshis
        console.log(`📥 UTXO detected for ${matched}: ${amount} sats`);
        notifyCanister(txid, matched, amount);
      }
    }
  }
}

// 🌀 Start polling loop
setInterval(pollMempool, POLL_INTERVAL);
console.log('👁️ Watcher started...');
