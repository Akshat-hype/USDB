import fetch from 'node-fetch';
import { Buffer } from 'node:buffer';

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
  const auth = Buffer.from(`${RPC_USER}:${RPC_PASS}`).toString('base64');

  const res = await fetch(rpcURL, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Basic ${auth}`
    },
    body: JSON.stringify({
      jsonrpc: '1.0',
      id: 'watcher',
      method,
      params,
    }),
  });

  const text = await res.text();
  console.log("📦 Raw RPC Response:", text);

  let json;
  try {
    json = JSON.parse(text);
  } catch (err) {
    console.error("❌ Failed to parse JSON response:", err);
    return null;
  }

  if (json.error) {
    console.error(`❌ RPC Error from ${method}:`, json.error);
    return null;
  }

  return json.result;
}

async function pollMempool() {
  const txids = await callRPC('getrawmempool');
  if (!txids || !Array.isArray(txids)) return;

  for (const txid of txids) {
    const rawTx = await callRPC('getrawtransaction', [txid, true]);
    if (!rawTx || !rawTx.vout) continue;

    for (const output of rawTx.vout) {
      const addresses = output.scriptPubKey?.addresses || [];
      const matched = addresses.find(addr => DEPOSIT_ADDRESSES.includes(addr));
      if (matched) {
        const amount = Math.floor(output.value * 100000000); // in sats
        console.log(`📥 UTXO detected for ${matched}: ${amount} sats`);
        notifyCanister(txid, matched, amount);
      }
    }
  }
}

// 🌀 Start polling
setInterval(pollMempool, POLL_INTERVAL);
console.log('👁️ Watcher started...');
