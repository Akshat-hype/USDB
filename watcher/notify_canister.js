import { exec } from 'child_process';

export function notifyCanister(txid, address, amount) {
  const cmd = `didc call <canister_id> deposit_btc '(record { txid = "${txid}"; address = "${address}"; amount = ${amount}; })'`;

  exec(cmd, (err, stdout, stderr) => {
    if (err) {
      console.error(`❌ Error calling canister:`, err);
      return;
    }
    console.log(`✅ Canister notified:`, stdout);
  });
}
notify_canister.js
// export function notifyCanister(txid, address, amount) {
//   console.log(`📢 Notify: TXID=${txid}, Address=${address}, Amount=${amount} sats`);
// }
