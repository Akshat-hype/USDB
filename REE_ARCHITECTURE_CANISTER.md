# REE-Compatible Canister for BTC-Collateralized Runes
---

Architecture Overview
What are BTC-Collateralized Runes?
BTC-collateralized runes are blockchain-native tokens created through a system that ensures each token is backed by Bitcoin. Traditional Bitcoin lacks programmability for NFTs or DeFi applications due to protocol limitations. Runes overcome this by using Bitcoin's UTXO model and leveraging the OP_RETURN opcode to store metadata efficiently.

A BTC-collateralized rune is a fungible or semi-fungible token that is only minted when Bitcoin has been securely deposited and verified. These tokens can later be redeemed or burned when the corresponding BTC is withdrawn. This design maintains a trustless and verifiable 1:1 value parity between the rune and the Bitcoin backing it.

---

Minting BTC-Collateralized Runes
To initiate minting, users first lock their BTC into a predefined address. This triggers our system to fetch real-time BTC/USD pricing using a dedicated price-oracle canister — which pulls reliable data (e.g., from Binance’s API) through HTTPS outcalls. This real-time price feed is critical to determining the accurate value of BTC locked.

Once the system receives a valid price, it proceeds to verify whether the user’s BTC has been securely locked. This verification can happen through an off-chain bridge service or an ICP bridge canister that monitors incoming BTC transactions.

Based on the current BTC price and the total BTC deposited, the protocol calculates how many rune tokens should be issued to the user. The minting mechanism is governed by a 1:1 value model, ensuring that every token minted is fully backed by the equivalent BTC value.

After calculations, the canister invokes the mint function to issue rune tokens to the user’s principal. The protocol also records the associated metadata — mapping the amount of BTC locked, the number of runes issued, and collateralization data — all necessary for future audits, redemptions, and preventing any form of over-minting.

---

Burning BTC-Collateralized Runes
The rune burning process allows users to reclaim their BTC by surrendering their rune tokens. However, the system enforces several verification checks before proceeding:

The user submits a request to burn a specific number of runes.

The protocol verifies the user’s balance to ensure they hold enough runes to complete the request.

Upon validation, the system burns the tokens from the user's balance using the rune ledger.

The canister then prepares a Bitcoin transaction to return the correct BTC amount to the user's provided BTC address.

Since canisters on ICP cannot natively sign Bitcoin transactions, there are two potential solutions:

Threshold ECDSA: Use Internet Computer's support for threshold ECDSA signing to generate signatures for BTC transactions on-chain.

Off-Chain Relay Service: Emit an event from the canister which is monitored by an off-chain service. This service listens, signs the transaction, and broadcasts it to the Bitcoin network.

This system ensures secure minting and redemption of BTC-backed tokens in a trust-minimized and scalable way — paving the path for truly decentralized stable assets on Bitcoin.