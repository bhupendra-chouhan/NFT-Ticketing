#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// Tracks overall ticket statistics across the platform
#[contracttype]
#[derive(Clone)]
pub struct TicketStats {
    pub total_issued: u64,   // Total NFT tickets ever issued
    pub total_used: u64,     // Total tickets that have been redeemed/used
    pub total_revoked: u64,  // Total tickets that have been revoked by the issuer
}

// Symbol key for global ticket statistics
const TICKET_STATS: Symbol = symbol_short!("TKT_STATS");

// Counter key for generating unique ticket IDs
const TICKET_COUNT: Symbol = symbol_short!("TKT_CNT");

// Mapping a unique ticket ID to its NFTTicket data
#[contracttype]
pub enum TicketBook {
    Ticket(u64),
}

// Core NFT Ticket structure — represents one gated-access ticket
#[contracttype]
#[derive(Clone)]
pub struct NFTTicket {
    pub ticket_id: u64,       // Unique identifier for this ticket
    pub event_name: String,   // Name of the event or gated resource
    pub owner: Address,       // Wallet address of the ticket holder
    pub issued_at: u64,       // Ledger timestamp when the ticket was minted
    pub used_at: u64,         // Ledger timestamp when the ticket was redeemed (0 if unused)
    pub is_used: bool,        // Whether the ticket has been redeemed
    pub is_revoked: bool,     // Whether the ticket has been revoked by the issuer
}

#[contract]
pub struct NFTTicketGatingContract;

#[contractimpl]
impl NFTTicketGatingContract {

    /// Mint a new NFT ticket for a specific event and assign it to `owner`.
    /// Returns the unique ticket_id of the newly minted ticket.
    pub fn mint_ticket(env: Env, event_name: String, owner: Address) -> u64 {
        // Increment the global ticket counter to derive a new unique ID
        let mut count: u64 = env.storage().instance().get(&TICKET_COUNT).unwrap_or(0);
        count += 1;

        let issued_at = env.ledger().timestamp();

        // Build the new NFT ticket
        let ticket = NFTTicket {
            ticket_id: count,
            event_name,
            owner: owner.clone(),
            issued_at,
            used_at: 0,
            is_used: false,
            is_revoked: false,
        };

        // Update platform-wide stats
        let mut stats = Self::view_ticket_stats(env.clone());
        stats.total_issued += 1;

        // Persist ticket, updated stats, and new counter
        env.storage().instance().set(&TicketBook::Ticket(count), &ticket);
        env.storage().instance().set(&TICKET_STATS, &stats);
        env.storage().instance().set(&TICKET_COUNT, &count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "NFT Ticket minted. Ticket-ID: {}, Owner: {}", count, owner);
        count
    }

    /// Verify and redeem (use) a ticket at the gate.
    /// Marks the ticket as used so it cannot be reused.
    /// Panics if the ticket is already used, revoked, or does not exist.
    pub fn use_ticket(env: Env, ticket_id: u64) {
        let mut ticket = Self::view_ticket(env.clone(), ticket_id);

        if ticket.ticket_id == 0 {
            log!(&env, "Ticket-ID {} not found.", ticket_id);
            panic!("Ticket not found.");
        }
        if ticket.is_revoked {
            log!(&env, "Ticket-ID {} has been revoked.", ticket_id);
            panic!("Ticket has been revoked and is no longer valid.");
        }
        if ticket.is_used {
            log!(&env, "Ticket-ID {} is already used.", ticket_id);
            panic!("Ticket has already been redeemed.");
        }

        // Mark ticket as used
        ticket.is_used = true;
        ticket.used_at = env.ledger().timestamp();

        // Update stats
        let mut stats = Self::view_ticket_stats(env.clone());
        stats.total_used += 1;

        env.storage().instance().set(&TicketBook::Ticket(ticket_id), &ticket);
        env.storage().instance().set(&TICKET_STATS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Ticket-ID {} successfully redeemed.", ticket_id);
    }

    /// Revoke a ticket (e.g., due to fraud, refund, or cancellation).
    /// A revoked ticket cannot be used at the gate.
    /// Panics if the ticket is already used, already revoked, or does not exist.
    pub fn revoke_ticket(env: Env, ticket_id: u64) {
        let mut ticket = Self::view_ticket(env.clone(), ticket_id);

        if ticket.ticket_id == 0 {
            log!(&env, "Ticket-ID {} not found.", ticket_id);
            panic!("Ticket not found.");
        }
        if ticket.is_used {
            log!(&env, "Cannot revoke Ticket-ID {} — already redeemed.", ticket_id);
            panic!("Cannot revoke a ticket that has already been used.");
        }
        if ticket.is_revoked {
            log!(&env, "Ticket-ID {} is already revoked.", ticket_id);
            panic!("Ticket is already revoked.");
        }

        ticket.is_revoked = true;

        let mut stats = Self::view_ticket_stats(env.clone());
        stats.total_revoked += 1;

        env.storage().instance().set(&TicketBook::Ticket(ticket_id), &ticket);
        env.storage().instance().set(&TICKET_STATS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Ticket-ID {} has been revoked.", ticket_id);
    }

    /// Retrieve the full details of a specific ticket by its ID.
    /// Returns a default (zeroed) NFTTicket if the ID does not exist.
    pub fn view_ticket(env: Env, ticket_id: u64) -> NFTTicket {
        env.storage().instance().get(&TicketBook::Ticket(ticket_id)).unwrap_or(NFTTicket {
            ticket_id: 0,
            event_name: String::from_str(&env, "Not_Found"),
            owner: Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"),
            issued_at: 0,
            used_at: 0,
            is_used: false,
            is_revoked: false,
        })
    }

    /// Retrieve aggregated platform-wide ticket statistics.
    pub fn view_ticket_stats(env: Env) -> TicketStats {
        env.storage().instance().get(&TICKET_STATS).unwrap_or(TicketStats {
            total_issued: 0,
            total_used: 0,
            total_revoked: 0,
        })
    }
}