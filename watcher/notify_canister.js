// watcher.js
import { Actor, HttpAgent } from "@dfinity/agent";
import fetch from "node-fetch";

// Replace with your actual canister ID
const CANISTER_ID = "your-canister-id-here";

// Import your canister interface (IDL)
import { idlFactory as notify_idl } from "./notify_canister.did.js";

async function main() {
  try {
    // Setup Agent
    const agent = new HttpAgent({ host: "http://127.0.0.1:4943", fetch });

    // Only required in local dev
    await agent.fetchRootKey();

    // Create Actor for your notify canister
    const notifyActor = Actor.createActor(notify_idl, {
      agent,
      canisterId: CANISTER_ID,
    });

    console.log("Watcher started...");

    // Example: simulate watcher detecting something
    setInterval(async () => {
      console.log("Watcher detected an event, notifying canister...");

      try {
        // Call your canister's notify function
        const response = await notifyActor.notify_canister("Watcher triggered");
        console.log("Canister response:", response);
      } catch (err) {
        console.error("Error notifying canister:", err);
      }
    }, 10000); // every 10 sec
  } catch (e) {
    console.error("Watcher error:", e);
  }
}

main();
