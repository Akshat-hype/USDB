**BITHOLD WHITEPAPER**

---

## Abstract

Bithold is a decentralized, Bitcoin-backed stablecoin protocol designed to bridge the gap between the world’s most secure digital asset (Bitcoin) and the growing need for programmable, stable assets in Web3 ecosystems. Built on top of REE, Omnity, and the Internet Computer (ICP), Bithold offers a trustless, censorship-resistant, and composable solution for issuing USD-pegged tokens backed by BTC.

Unlike traditional stablecoins, Bithold does not rely on centralized custodians or opaque reserves. Instead, it adopts a decentralized custody and minting framework leveraging advanced technologies such as Chain Key Bitcoin (CKBTC), DPS (Decentralized PSBT Signing), and programmable runes to offer a seamless, cross-chain experience.

---

## Introduction

The need for a stable and decentralized unit of account is becoming increasingly critical as DeFi, DAOs, and Web3 applications scale globally. Bitcoin, while dominant as a store of value, lacks native programmability and is often excluded from DeFi due to its lack of smart contract support.

Meanwhile, USD-backed stablecoins such as USDT and USDC, while liquid and accessible, depend on centralized custodians, posing risks to transparency, censorship resistance, and decentralization.

Bithold introduces a new design: a USD-pegged token backed by Bitcoin, issued through decentralized mechanisms, interoperable with multiple chains, and built on one of the most secure and scalable blockchain stacks — the Internet Computer.

---

## Motivation

The current stablecoin ecosystem is plagued by centralization, limited composability, and insufficient transparency. USDT and USDC dominate markets but come with trust assumptions. Other alternatives offer decentralization but are often over-reliant on ETH or RWAs.

Bitcoin is underutilized in the stablecoin domain due to lack of native smart contract functionality. Bithold solves this by combining:

* Bitcoin’s unmatched security and liquidity
* ICP’s scalability and chain integration via CKBTC and HTTPS Outcalls
* REE’s Bitcoin-native smart contract layer
* Omnity’s cross-chain bridge to EVM, Cosmos, and Polkadot

---

## Architecture Overview

The Bithold protocol consists of three major layers:

1. **Collateral Management Layer** – Users lock BTC using a decentralized custody system (DPS with multisig/threshold wallets) to mint Bithold.
2. **Stablecoin Logic Layer** – A canister on ICP mints/burns Bithold runes based on BTC collateralization, using vaults.
3. **Cross-Chain Layer** – Bithold is wrapped via Omnity into ERC-20/CW20 tokens for use in other chains like Ethereum, Cosmos, or Polkadot.

The minting and burning process follows a trustless approach, where users deposit BTC, generate a PSBT, and on confirmation, mint equivalent Bithold tokens (runes). When Bithold is returned and burned, BTC is released via threshold signing.

---

## Core Components

### Bitcoin Custody

Bithold leverages Decentralized PSBT Signing (DPS) to enable secure BTC custody. Instead of relying on a single keyholder, a threshold-based system (e.g., 3-of-5 multisig or FROST) ensures decentralized governance over BTC held in protocol-controlled vaults.

Each minting of Bithold requires the user to initiate a PSBT locking BTC into a known multisig address. The transaction is verified using SPV proofs or Bitcoin light clients integrated into ICP canisters.

### Minting Logic

The protocol maintains an overcollateralization ratio (e.g., 150%) to prevent insolvency. A user locking \$150 worth of BTC can mint up to 100 Bithold tokens. The ratio dynamically adjusts based on BTC volatility and protocol risk parameters.

Canisters on ICP handle minting logic, keeping track of vaults, collateralization ratios, timestamps, and oracle-fed BTC price data (via Chain Key Bitcoin or HTTPS Outcalls).

### Redemption Process

Burning Bithold initiates the redemption of BTC. The user submits proof of burn, and the canister verifies eligibility and triggers a DPS threshold sign to release BTC. Time delays, withdrawal fees, and vault health status influence redemption conditions.

---

## Metadata Structure

Each minted Bithold rune includes the following metadata:

* **runeID**: Unique identifier
* **BTC amount**: Locked value
* **mint time**: Timestamp
* **owner**: Principal ID or wallet address
* **oracle price**: BTC/USD at mint time
* **collateralization**: Ratio at mint

This data is stored on-chain in JSON format, verifiable and auditable via transparency dashboards.

---

## Stability Mechanism

Bithold incorporates autonomous stabilizers:

* **Mint fees**: Adjust dynamically to discourage over-minting
* **Liquidation logic**: Under-collateralized vaults are liquidated
* **Treasury buffers**: Protocol collects fees for insurance reserves
* **Governance votes**: DAO determines monetary parameters

---

## Governance

Bithold is governed by a DAO built using Internet Computer’s SNS (Service Nervous System) or a custom governance canister. Token holders can propose changes to minting logic, collateral ratios, fee curves, and treasury operations.

Governance ensures decentralization of protocol upgrades and financial policies. Community participation is encouraged via voting incentives and staking rewards.

---

## Cross-Chain Integration

Through Omnity, Bithold can be bridged to EVM (ERC-20), Cosmos (CW20), and Polkadot (Substrate) ecosystems. Users can use Bithold in DeFi protocols, liquidity pools, lending markets, and payment rails across chains.

Wrapped tokens are backed 1:1 with Bithold on ICP, held in audited vaults. Redemption requires burning wrapped token and retrieving Bithold from the main chain.

---

## Transparency & Proof-of-Reserves

A transparency portal displays:

* BTC locked on-chain with explorer links
* Bithold minted & outstanding
* Vault statuses
* Liquidations
* Treasury reserves

All data is live-fed from ICP canisters and Bitcoin nodes.

---

## Use Cases

* **DeFi Collateral**: Use Bithold in lending protocols and AMMs
* **Remittances**: Send USD-pegged crypto backed by BTC
* **DAO Treasuries**: Diversify holdings with decentralized stables
* **Payments**: Stable merchant payments via Plug/MetaMask
* **BTC Yield**: Earn fees as a liquidity provider

---

## Roadmap

**Month 1: Protocol Architecture**

* Finalize specs for minting, burning, metadata
* Build BTC price oracle integration
* Start development of REE-compatible canisters

**Month 2: Collateral & Custody**

* Implement DPS custody and mint/redeem logic
* Test SPV verification and vault management

**Month 3: Interoperability Layer**

* Omnity integration for ERC-20/CW20 formats
* Frontend dashboard for minting/redeeming

**Month 4: Governance + Stabilization**

* Launch SNS or DAO governance
* Deploy stabilizers and transparency tools

**Month 5–6: Debug & Final Audit**

* Internal testing
* External audit
* Launch marketing + education

**Month 7: Community Launch**

* Bithold public minting goes live
* Initial liquidity programs for DeFi
* Cross-chain support via Omnity bridges

**Month 8–9: Ecosystem Growth**

* Listings on major DEXes & wallets
* Developer grants & integrations
* Begin DAO treasury operations

**Month 10–12: Expansion**

* Add support for other BTC-based assets
* Support for institutional custody options
* Launch mobile-friendly minting app

---

## Conclusion

Bithold represents a new frontier in stablecoin design: secure, decentralized, and truly cross-chain. By leveraging Bitcoin’s trustless value, ICP’s programmability, and Omnity’s reach, Bithold creates a bridge between BTC and DeFi, without sacrificing decentralization.

With proper risk parameters, transparent governance, and robust architecture, Bithold has the potential to become a leading Bitcoin-backed stable asset in the Web3 economy.

---

*Developed by Team G | In collaboration with @icphubs, @icphub\_IN, and @blockhacks. #BuiltonICP #100daysofcode*
