#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, vec, Address, Env, Map, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum MilestoneStatus {
    Pending,
    Approved,
    Released,
}

#[contracttype]
#[derive(Clone)]
pub struct Milestone {
    pub index: u32,
    pub amount: i128,
    pub status: MilestoneStatus,
    pub approvals: u32,
    pub required_approvals: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub proposal_id: u64,
    pub researcher: Address,
    pub token: Address,         // XLM (native) or USDC contract
    pub total_amount: i128,
    pub released_amount: i128,
    pub milestones: Vec<Milestone>,
    pub governance: Address,    // only governance contract can approve
}

const ESCROWS: Symbol = symbol_short!("ESCROWS");
const APPROVALS: Symbol = symbol_short!("APPROVS");
const ADMIN: Symbol = symbol_short!("ADMIN");

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
    }

    /// Called after governance passes a proposal — locks funds in escrow.
    pub fn create_escrow(
        env: Env,
        caller: Address,
        proposal_id: u64,
        researcher: Address,
        token_address: Address,
        total_amount: i128,
        milestone_amounts: Vec<i128>,
        required_approvals: u32,
        governance: Address,
    ) {
        caller.require_auth();
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if caller != admin {
            panic!("unauthorized");
        }

        // Transfer funds from caller into this contract
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&caller, &env.current_contract_address(), &total_amount);

        let mut milestones: Vec<Milestone> = Vec::new(&env);
        for (i, amount) in milestone_amounts.iter().enumerate() {
            milestones.push_back(Milestone {
                index: i as u32,
                amount,
                status: MilestoneStatus::Pending,
                approvals: 0,
                required_approvals,
            });
        }

        let escrow = Escrow {
            proposal_id,
            researcher,
            token: token_address,
            total_amount,
            released_amount: 0,
            milestones,
            governance,
        };

        let mut escrows: Map<u64, Escrow> = env
            .storage()
            .instance()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));
        escrows.set(proposal_id, escrow);
        env.storage().instance().set(&ESCROWS, &escrows);

        env.events()
            .publish((symbol_short!("escrowed"), proposal_id), total_amount);
    }

    /// DAO member approves a milestone.
    pub fn approve_milestone(
        env: Env,
        approver: Address,
        proposal_id: u64,
        milestone_index: u32,
    ) {
        approver.require_auth();

        let mut escrows: Map<u64, Escrow> = env.storage().instance().get(&ESCROWS).unwrap();
        let mut escrow = escrows.get(proposal_id).expect("escrow not found");

        if approver != escrow.governance {
            panic!("only governance contract may approve");
        }

        let mut milestone = escrow.milestones.get(milestone_index).expect("bad index");
        if milestone.status != MilestoneStatus::Pending {
            panic!("milestone not pending");
        }

        milestone.approvals += 1;
        if milestone.approvals >= milestone.required_approvals {
            milestone.status = MilestoneStatus::Approved;
        }

        escrow.milestones.set(milestone_index, milestone);
        escrows.set(proposal_id, escrow);
        env.storage().instance().set(&ESCROWS, &escrows);
    }

    /// Release funds for an approved milestone to the researcher.
    pub fn release_milestone(env: Env, proposal_id: u64, milestone_index: u32) {
        let mut escrows: Map<u64, Escrow> = env.storage().instance().get(&ESCROWS).unwrap();
        let mut escrow = escrows.get(proposal_id).expect("escrow not found");

        let mut milestone = escrow.milestones.get(milestone_index).expect("bad index");
        if milestone.status != MilestoneStatus::Approved {
            panic!("milestone not approved");
        }

        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(
            &env.current_contract_address(),
            &escrow.researcher,
            &milestone.amount,
        );

        escrow.released_amount += milestone.amount;
        milestone.status = MilestoneStatus::Released;
        escrow.milestones.set(milestone_index, milestone.clone());
        escrows.set(proposal_id, escrow.clone());
        env.storage().instance().set(&ESCROWS, &escrows);

        env.events().publish(
            (symbol_short!("released"), proposal_id),
            (milestone_index, milestone.amount),
        );
    }

    /// Pay a trial participant directly from escrow (final milestone or dedicated allocation).
    pub fn pay_participant(
        env: Env,
        caller: Address,
        proposal_id: u64,
        participant: Address,
        amount: i128,
    ) {
        caller.require_auth();
        let escrows: Map<u64, Escrow> = env.storage().instance().get(&ESCROWS).unwrap();
        let escrow = escrows.get(proposal_id).expect("escrow not found");

        if caller != escrow.governance {
            panic!("only governance may trigger participant payouts");
        }

        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(&env.current_contract_address(), &participant, &amount);

        env.events()
            .publish((symbol_short!("paid"), proposal_id), (participant, amount));
    }

    pub fn get_escrow(env: Env, proposal_id: u64) -> Escrow {
        let escrows: Map<u64, Escrow> = env.storage().instance().get(&ESCROWS).unwrap();
        escrows.get(proposal_id).expect("not found")
    }
}
