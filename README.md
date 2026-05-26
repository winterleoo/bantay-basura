# BantayBasura
QR-based waste accountability and enforcement for Philippine LGUs.

## Problem & Solution
Anonymous trash dumping clogs drainage systems. BantayBasura links QR-coded bags to households via Soroban smart contracts, allowing LGUs to issue on-chain fines for mismanaged waste.

## Timeline
- **Week 1:** Bag-to-Address registry and penalty logic.
- **Week 2:** Inspector mobile app for scanning and photo-upload verification.

## Stellar Features
- Soroban Smart Contracts (Registry & Fines)
- Clawback (LGU enforcement of bond)

## How to Build
`soroban contract build`

## How to Test
`cargo test`

## How to Deploy
`soroban contract deploy --network testnet --source my-account --wasm /app/target/wasm32v1-none/release/bantay_basura.wasm`

## Sample CLI Invocation
`soroban contract invoke --id [CID] --source [INSPECTOR] --network testnet -- issue_penalty --inspector [INSPECTOR_ADDR] --bag_id BAG_001_TONDO --fine_amount 100`

## License
MIT