**USDB WHITEPAPER**

---

## Abstract

USDB is a decentralized, Bitcoin-backed stablecoin protocol designed to bridge the gap between the world’s most secure digital asset (Bitcoin) and the growing need for programmable, stable assets in Web3 ecosystems. Built on top of REE, Omnity, and the Internet Computer (ICP), USDB offers a trustless, censorship-resistant, and composable solution for issuing USD-pegged tokens backed by BTC.

Unlike traditional stablecoins, USDB does not rely on centralized custodians or opaque reserves. Instead, it adopts a decentralized custody and minting framework leveraging advanced technologies such as Chain Key Bitcoin (CKBTC), DPS (Decentralized PSBT Signing), and programmable runes to offer a seamless, cross-chain experience.

---

## Introduction

The need for a stable and decentralized unit of account is becoming increasingly critical as DeFi, DAOs, and Web3 applications scale globally. Bitcoin, while dominant as a store of value, lacks native programmability and is often excluded from DeFi due to its lack of smart contract support.

Meanwhile, USD-backed stablecoins such as USDT and USDC, while liquid and accessible, depend on centralized custodians, posing risks to transparency, censorship resistance, and decentralization.

USDB introduces a new design: a USD-pegged token backed by Bitcoin, issued through decentralized mechanisms, interoperable with multiple chains, and built on one of the most secure and scalable blockchain stacks — the Internet Computer.

---

## Motivation

The current stablecoin ecosystem is plagued by centralization, limited composability, and insufficient transparency. USDT and USDC dominate markets but come with trust assumptions. Other alternatives offer decentralization but are often over-reliant on ETH or RWAs.

Bitcoin is underutilized in the stablecoin domain due to lack of native smart contract functionality. USDB solves this by combining:

- Bitcoin’s unmatched security and liquidity
- ICP’s scalability and chain integration via CKBTC and HTTPS Outcalls
- REE’s Bitcoin-native smart contract layer
- Omnity’s cross-chain bridge to EVM, Cosmos, and Polkadot

---

## Architecture Overview

USDB operates through a layered architecture to facilitate secure issuance and cross-chain compatibility.

First, users lock Bitcoin through a decentralized custody model. This system uses DPS and threshold signatures to securely store BTC without depending on a single entity. Once the BTC is locked and verified, users can mint USDB tokens according to defined overcollateralization ratios.

The minting and burning mechanisms are managed by canisters on the Internet Computer. These canisters ensure all protocol rules are enforced automatically — including maintaining sufficient collateral, verifying user actions, and interacting with oracles for real-time BTC pricing.

Finally, USDB is made available across other ecosystems using Omnity bridges, which convert USDB into ERC-20, CW20, or other compatible token formats for usage in various DeFi protocols.

---

## Core Components

### Bitcoin Custody

Bitcoin custody in USDB is designed to be decentralized and verifiable. Through Decentralized PSBT Signing (DPS), BTC is stored using threshold cryptography and multisig wallets. No single entity holds control over the funds. Users initiate a PSBT (Partially Signed Bitcoin Transaction) which locks BTC into a protocol-managed address. This ensures that the protocol can verify ownership and lock status without centralized dependence.

The protocol uses Bitcoin light clients or SPV proofs within ICP canisters to confirm deposits and interact with the Bitcoin network securely and autonomously.

### Minting Logic

Minting USDB requires users to lock BTC in a vault with an overcollateralization requirement (typically 150%). For example, locking \$150 worth of BTC allows minting up to 100 USDB tokens. This helps the protocol absorb volatility in BTC prices and maintain the 1 USD peg.

Minting is permissionless and transparent. The system evaluates BTC's current price via oracle feeds and enforces collateral requirements before issuing new USDB tokens.

### Redemption Process

To redeem BTC, users must burn their USDB. Once burned, the system validates the transaction and releases BTC back to the original depositor via DPS threshold signing. This release process ensures that only the original vault owner can retrieve their BTC, maintaining user-level custody.

Redemptions may be subject to small delays or fees based on network congestion and vault health to maintain fairness and system stability.

---

## Metadata Structure

Each minted USDB token includes on-chain metadata, improving transparency and auditability:

- **runeID**: Unique token identifier
- **BTC amount**: Underlying BTC value locked
- **mint time**: Timestamp of minting
- **owner**: Wallet address or ICP principal ID
- **oracle price**: BTC/USD price at mint
- **collateralization**: Vault ratio at time of mint

This data is stored in a decentralized manner using JSON structures accessible via transparency dashboards.

---

## Stability Mechanism

USDB ensures price stability through several integrated mechanisms:

- **Minting and Redemption Fees**: Dynamic fees based on system demand and volatility. These discourage excessive minting or redemption during high-risk periods.
- **Stability Fee**: An annualized fee charged on outstanding debt (minted USDB) to incentivize users to maintain healthy vaults and reduce risk buildup. Collected fees are added to the protocol’s reserve.
- **Liquidation Mechanism**: Vaults falling below the collateral threshold are subject to automatic liquidation. Third parties can trigger liquidation in exchange for discounted collateral, thereby maintaining solvency.
- **Insurance Reserves**: A portion of collected fees is allocated to a treasury buffer to cover shortfalls or unexpected losses.
- **Dynamic Collateral Ratios**: Protocol governance can adjust ratios based on BTC volatility and market health.

---

## Governance

USDB governance is handled via a DAO built on Internet Computer’s SNS framework or a dedicated governance canister. Token holders can propose and vote on protocol changes, such as:

- Collateral ratios
- Fee structures
- Stability parameters
- Oracle integrations
- Treasury operations

Governance is designed to be transparent and open, allowing for stakeholder participation in evolving the protocol. Voting power may be proportional to stake or delegated through representative models.

---

## Cross-Chain Integration

USDB can be bridged to multiple ecosystems including Ethereum, Cosmos, and Polkadot using Omnity. Bridging allows users to leverage USDB in DeFi protocols, liquidity pools, and exchanges across various networks.

Wrapped tokens on these chains are backed 1:1 by native USDB on ICP. Burning a wrapped token triggers the release of native USDB which can then be redeemed for BTC if desired.

---

## Transparency & Proof-of-Reserves

Transparency is a cornerstone of USDB. A real-time portal displays:

- Total BTC collateral locked
- Total USDB in circulation
- Vault statuses
- Liquidations and protocol health
- Treasury balances and fee collections

Data is pulled directly from on-chain sources and Bitcoin nodes to ensure trust and integrity.

---

## Use Cases

USDB opens up a wide range of use cases for individuals, developers, and institutions:

- **DeFi Collateral**: Borrowing and lending with a BTC-backed stablecoin
- **Remittances**: Send USD value globally with Bitcoin security
- **DAO Treasuries**: Store value in a decentralized, non-custodial asset
- **Payments**: Enable stable crypto payments with wallet integrations
- **Yield Generation**: Provide USDB liquidity and earn fees

---

## Conclusion

USDB represents a major step forward in decentralized finance by enabling a truly Bitcoin-backed stablecoin with full programmability, cross-chain functionality, and transparency. With robust collateral management, dynamic stability mechanisms, and community-driven governance, USDB offers a secure and scalable solution for a decentralized economy.

Its architecture, built on the strengths of ICP, REE, and Omnity, allows users to unlock the power of Bitcoin while participating in a stable, transparent, and composable financial ecosystem.

---

_Developed by Team G | In collaboration with @icphubs, @icphub_IN, and @blockhacks. #BuiltonICP #100daysofcode_
