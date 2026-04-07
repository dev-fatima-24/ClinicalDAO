#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Env, Map, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u64,
    pub researcher: Address,
    pub title: String,
    pub ipfs_cid: String,       // off-chain metadata
    pub funding_amount: i128,
    pub milestone_count: u32,
    pub votes_for: i128,
    pub votes_against: i128,
    pub status: ProposalStatus,
    pub deadline: u64,          // ledger timestamp
    pub threshold_numerator: u32,   // M
    pub threshold_denominator: u32, // N
}

const PROPOSALS: Symbol = symbol_short!("PROPOSALS");
const PROP_COUNT: Symbol = symbol_short!("PROPCOUNT");
const VOTES: Symbol = symbol_short!("VOTES");       // Map<(proposal_id, voter) -> bool>
const TOKEN: Symbol = symbol_short!("TOKEN");
const ADMIN: Symbol = symbol_short!("ADMIN");

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    pub fn initialize(env: Env, admin: Address, token_contract: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token_contract);
        env.storage().instance().set(&PROP_COUNT, &0u64);
    }

    /// Researcher submits a proposal (must be SEP-10 verified caller).
    pub fn submit_proposal(
        env: Env,
        researcher: Address,
        title: String,
        ipfs_cid: String,
        funding_amount: i128,
        milestone_count: u32,
        voting_period_secs: u64,
        threshold_numerator: u32,
        threshold_denominator: u32,
    ) -> u64 {
        researcher.require_auth();

        let count: u64 = env.storage().instance().get(&PROP_COUNT).unwrap_or(0);
        let id = count + 1;

        let proposal = Proposal {
            id,
            researcher,
            title,
            ipfs_cid,
            funding_amount,
            milestone_count,
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Active,
            deadline: env.ledger().timestamp() + voting_period_secs,
            threshold_numerator,
            threshold_denominator,
        };

        let mut proposals: Map<u64, Proposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Map::new(&env));
        proposals.set(id, proposal);

        env.storage().instance().set(&PROPOSALS, &proposals);
        env.storage().instance().set(&PROP_COUNT, &id);

        env.events().publish((symbol_short!("proposed"), id), id);
        id
    }

    /// Token holder casts a vote.
    pub fn cast_vote(env: Env, voter: Address, proposal_id: u64, support: bool) {
        voter.require_auth();

        let mut proposals: Map<u64, Proposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap();

        let mut proposal = proposals.get(proposal_id).expect("proposal not found");

        if env.ledger().timestamp() > proposal.deadline {
            panic!("voting period ended");
        }
        if !matches!(proposal.status, ProposalStatus::Active) {
            panic!("proposal not active");
        }

        // Prevent double voting
        let vote_key = (proposal_id, voter.clone());
        let mut vote_map: Map<(u64, Address), bool> = env
            .storage()
            .instance()
            .get(&VOTES)
            .unwrap_or(Map::new(&env));
        if vote_map.contains_key(vote_key.clone()) {
            panic!("already voted");
        }

        // Get voter's token balance as vote weight
        let token: Address = env.storage().instance().get(&TOKEN).unwrap();
        let weight: i128 = env
            .invoke_contract(&token, &symbol_short!("balance"), vec![&env, voter.to_val()]);

        if support {
            proposal.votes_for += weight;
        } else {
            proposal.votes_against += weight;
        }

        vote_map.set(vote_key, support);
        proposals.set(proposal_id, proposal);

        env.storage().instance().set(&VOTES, &vote_map);
        env.storage().instance().set(&PROPOSALS, &proposals);

        env.events()
            .publish((symbol_short!("voted"), proposal_id), (voter, support));
    }

    /// Finalize a proposal after voting deadline.
    pub fn finalize(env: Env, proposal_id: u64) -> ProposalStatus {
        let mut proposals: Map<u64, Proposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap();

        let mut proposal = proposals.get(proposal_id).expect("proposal not found");

        if env.ledger().timestamp() <= proposal.deadline {
            panic!("voting still active");
        }
        if !matches!(proposal.status, ProposalStatus::Active) {
            panic!("already finalized");
        }

        let total = proposal.votes_for + proposal.votes_against;
        let passed = total > 0
            && proposal.votes_for * (proposal.threshold_denominator as i128)
                >= total * (proposal.threshold_numerator as i128);

        proposal.status = if passed {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Rejected
        };

        proposals.set(proposal_id, proposal.clone());
        env.storage().instance().set(&PROPOSALS, &proposals);

        env.events()
            .publish((symbol_short!("finalized"), proposal_id), passed);

        proposal.status
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        let proposals: Map<u64, Proposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap();
        proposals.get(proposal_id).expect("not found")
    }

    pub fn proposal_count(env: Env) -> u64 {
        env.storage().instance().get(&PROP_COUNT).unwrap_or(0)
    }
}
