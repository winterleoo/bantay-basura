#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, log};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    BagOwner(Symbol),    // Maps unique Bag QR string to Resident Address
    Escrow(Address),     // Maps Resident to their locked "Environmental Deposit"
}

#[contract]
pub struct BantayBasuraContract;

#[contractimpl]
impl BantayBasuraContract {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Links a range of Bag IDs to a specific resident
    pub fn register_bag(env: Env, admin: Address, bag_id: Symbol, resident: Address) {
        admin.require_auth(); // Only LGU/Barangay can register labels
        env.storage().instance().set(&DataKey::BagOwner(bag_id), &resident);
    }

    /// Resident deposits an "Environmental Bond" (security deposit)
    pub fn deposit_bond(env: Env, resident: Address, amount: i128) {
        resident.require_auth();
        let mut balance: i128 = env.storage().instance().get(&DataKey::Escrow(resident.clone())).unwrap_or(0);
        balance += amount;
        env.storage().instance().set(&DataKey::Escrow(resident), &balance);
    }

    /// Inspector scans a mismanaged bag and issues a penalty
    pub fn issue_penalty(env: Env, inspector: Address, bag_id: Symbol, fine_amount: i128) {
        inspector.require_auth();
        
        // Find who owns the bag
        let resident: Address = env.storage().instance().get(&DataKey::BagOwner(bag_id)).expect("Bag not registered");
        
        let mut balance: i128 = env.storage().instance().get(&DataKey::Escrow(resident.clone())).unwrap_or(0);
        
        assert!(balance >= fine_amount, "Insufficient bond for fine");

        balance -= fine_amount;
        env.storage().instance().set(&DataKey::Escrow(resident.clone()), &balance);
        
        log!(&env, "Fine issued to resident for mismanaged bag", resident, fine_amount);
    }

    pub fn get_bond_balance(env: Env, resident: Address) -> i128 {
        env.storage().instance().get(&DataKey::Escrow(resident)).unwrap_or(0)
    }
}