import { exec } from "child_process";

/**
 * Notify the canister about a new BTC transaction.
 * @param {string} txid - The Bitcoin transaction ID
 * @param {string} address - The receiving BTC address
 * @param {number} amount - The amount in satoshis
 */
export function notifyCanister(txid, address, amount) {
  // Replace with your actual canister ID
  const canisterId = "avqkn-guaaa-aaaaa-qaaea-cai";

  // Construct the didc call
  const cmd = `didc call ${canisterId} deposit_btc '(record { txid = "${txid}"; btc_address = "${address}"; amount_sats = ${amount} })'`;

  console.log("🚀 Executing command:", cmd);

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

// Example direct call (for testing)
if (process.argv.length === 5) {
  const [,, txid, address, amount] = process.argv;
  notifyCanister(txid, address, parseInt(amount, 10));
}