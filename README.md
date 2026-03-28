# NFT Ticket Gating

## Table of Contents

- [Project Title](#nft-ticket-gating)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)

---

## Project Description

**NFT Ticket Gating** is a smart contract built on the [Stellar](https://stellar.org/) blockchain using the [Soroban SDK](https://soroban.stellar.org/). It enables event organizers and platform operators to mint, verify, and manage NFT-based access tickets entirely on-chain.

Each ticket is a unique, tamper-proof digital asset tied to a specific event and a wallet address. At the venue gate (physical or digital), the ticket's validity is verified directly against the blockchain — no third-party ticketing backend required. Once redeemed, a ticket is permanently marked as used, eliminating double-entry fraud. Organizers also retain the ability to revoke tickets before they are redeemed, handling refunds or cancellations transparently.

The contract exposes four clean functions:

| Function | Actor | Description |
|---|---|---|
| `mint_ticket` | Organizer / Issuer | Mints a new NFT ticket for an event and assigns it to an owner address |
| `use_ticket` | Gate Verifier | Redeems a ticket at the gate; marks it as permanently used |
| `revoke_ticket` | Organizer / Issuer | Revokes a ticket before it is used (refund / cancellation) |
| `view_ticket` | Anyone | Returns full details of a ticket by its unique ID |
| `view_ticket_stats` | Anyone | Returns platform-wide aggregated ticket statistics |

---

## Project Vision

The vision behind **NFT Ticket Gating** is to bring trustless, verifiable access control to events and gated communities — removing intermediaries, eliminating counterfeit tickets, and giving both organizers and attendees full transparency over the ticket lifecycle.

By anchoring every ticket to the Stellar blockchain via Soroban smart contracts, the platform aims to:

- Make ticket ownership **self-sovereign** — stored in the holder's own wallet, not a centralized database.
- Make ticket validity **publicly verifiable** — anyone can query the contract to confirm authenticity.
- Reduce fraud through **immutable redemption records** — once a ticket is used, the ledger permanently reflects that state.
- Lower costs by **removing third-party ticketing platforms** and their associated fees.

---

## Key Features

- **NFT Ticket Minting** — Organizers can issue unique tickets linked to an event name and an owner wallet address. Each ticket receives a sequentially generated unique ID stored on-chain.

- **On-Chain Gate Verification & Redemption** — Gate verifiers call `use_ticket` to redeem a ticket. The contract enforces that a ticket cannot be reused or used if revoked, preventing double-entry.

- **Ticket Revocation** — Organizers can revoke any unredeemed ticket (e.g., upon a refund or event cancellation). Revoked tickets are permanently blocked from gate entry.

- **Transparent Ticket Inspection** — Any party can call `view_ticket` with a ticket ID to retrieve full details: owner address, event name, issuance timestamp, redemption timestamp, and current status.

- **Aggregate Platform Statistics** — `view_ticket_stats` provides a real-time snapshot of total tickets issued, total redeemed, and total revoked across the entire platform.

- **Tamper-Proof Ledger Records** — All ticket state changes (mint, use, revoke) are persisted to Soroban instance storage with TTL extension, ensuring data longevity and immutability.

---

## Future Scope

- **Transferable Tickets** — Add a `transfer_ticket` function so holders can securely transfer ticket ownership to another wallet address, enabling a trustless secondary market.

- **Role-Based Access Control** — Introduce an `admin` address at contract initialization to restrict sensitive operations (`mint_ticket`, `revoke_ticket`) to authorized organizers only, preventing unauthorized minting.

- **Ticket Tiers & Metadata** — Support multiple ticket tiers (e.g., VIP, General Admission, Backstage) with rich on-chain metadata such as seat numbers, zone identifiers, and perks.

- **Expiry Enforcement** — Add an `expires_at` timestamp field. The `use_ticket` function would automatically reject tickets presented after the event end time.

- **Batch Minting** — Enable organizers to mint multiple tickets in a single transaction to reduce gas costs for large-scale events.

- **Cross-Contract Composability** — Expose the `view_ticket` function as a gating primitive for other Soroban contracts — e.g., a DAO contract that grants voting rights only to verified ticket holders.

- **Frontend dApp Integration** — Build a companion web application that connects to a Freighter/Lobstr wallet, allowing attendees to view their tickets and organizers to manage events through a user-friendly dashboard.

- **Event Analytics Dashboard** — Leverage `view_ticket_stats` and per-ticket timestamps to generate attendance curves, peak entry times, and redemption rate analytics.

## Contract Details:
Contract ID: CBMNTWHYF2OP3JZPYTIPJJKJWQHGM7ZUHMFCQUBBYWRJDE6U6R7O5VF5
<img width="1855" height="893" alt="image" src="https://github.com/user-attachments/assets/3e87050d-8dc7-4ee4-9750-014a70968599" />

