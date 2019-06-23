#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_core::{
    env::println,
    memory::format,
    storage,
};
use ink_lang::contract;

struct SystemAccount {
    /// The account id of the user. This entry is created the 
    /// first time a user appears in the system and will be 
    /// persisted for as long as the user is active.
    /// It is 
    user: storage::Value<AccountId>,

    /// Tokens owned by the user
    tokens: storage::Value<u64>,

    /// Tokens locked as a sign of trust from other members
    /// of the community to this user. Neither this user nor
    /// the sponsors can use them as long as they are locked.
    sponsors: storage::HashMap<AccountId, u64>,
}

/// Struct holding all data of a proposal to be voted on.
/// It could be a standard enhancement proposal or it could be
/// a complaint against another user who has violated the rules.
struct Proposal {
    /// Unique id of this proposal.
    id: storage::Value<u64>,

    /// User who created the proposal.
    user: storage::Value<AccountId>,

    /// Text of the proposal.
    text: storage::Value<AccountId>,

    /// Account ids voted for or against this proposal.
    votes: storage::HashMap<u64, bool>

    /// Bool showing whether or not this is a complaint.
    is_complaint: storage::Value<bool>,

    /// If the storage is a complaint set the account id against
    /// which the complaint is being raised.
    against: storage::Value<AccountId>,

    /// Is the proposal active?
    closed: storage::Value<bool>,

    /// Unix timestamp when it was created
    created_on: storage::Value<u64>,

    /// Deadline for all the users to cast their votes
    closes_on: storage::Value<u64>,
}

contract! {
    /// Contract holding all the active proposals to be voted on
    /// and all the proposals that have already been voted on.
    struct Proposals {
        /// Set of active proposals
        active_proposals: storage::HashMap<Proposal, bool>

        /// Set of closed proposals. Any proposal whose deadline has
        /// passed or some consensus has been reached is added to this
        /// set
        archived_proposals: storage::HashMap<Proposal, bool>
    }

    /// Contract implementing the Augemented Bonding Token Curve.
    struct ABTCurve {
        tokens_minted: storage::Value<u64>,
    }

    /// Pool of tokens available to be distributed for the project.
    struct FundingPool {
        tokens: storage::Value<u64>,
    }

    /// Pool of tokens exchanged by users with Bonfire tokens.
    struct ReservePool {
        tokens: storage::Value<u64>,
    }

    /// Contract of the parent DAO.
    struct Bonfire.dao.global {
        /// The account that initializes the global bonfire. Once
        /// the DAO has been initialized, the owner is set to null
        /// to prevent one person from having unfettered control to
        owner: storage::Value<AccountId>,

        /// Hash map used as a set to hold the administrators of this
        /// DAO. Administrators are able to perform some operations
        /// that require "superuser" credentials, but they are not able
        /// to perform arbitrary changes to the state of the contract.
        admins: storage::HashMap<u64, bool>
    }

    /// Contract of the local DAOs
    struct Bonfire.dao.local {
        /// Unique name of the Bonfire instance
        name: storage::Value<str>,
    }

    impl Deploy for Bonfire.dao.global {
        /// Initializes the global Bonfire DAO. The caller is set as
        /// the owner of the DAO, the first member and the first admin.
        /// In a subsequent call the owner should downgrade herself.
        fn deploy(&mut self) {}
    }

    impl Deploy for Bonfire.dao.local {
        /// Deploys an instance of a local Bonfire DAO. Checks to ensure
        /// that the name of the DAO is unique, sets the owner to be the
        /// caller account id.
        fn deploy(&mut self) {}
    }

    impl Bonfire.dao.global {
        /// Once the owner has ensured that the initialization process
        /// has completed successfully, she will convert herself to a
        /// simple admin of the project. Will fail if the user is not an
        /// owner.
        pub(external) fn drop_credentials(&mut self) -> bool {}

        /// Adds a registered user to the set of administrators.
        /// Will fail if the user is not registered, if she is already
        /// an administrator or if the caller is not an administrator.
        pub(external) fn add_administrator(&mut self) -> bool {}

        pub(external) fn remove_administrator(&mut self) -> bool {}

        /// Function to be called by any user who wishes to participate
        /// in the community. Successful registration does not mean that
        /// the user is able to do anything. In order to become eligible
        /// to vote, the user must be sponsored by at least two other
        /// users. Will fail if the user is already registered.
        pub(external) fn register_user(&mut self) -> bool {}

        /// Returns whether or not the user has at least two sponsors.
        fn is_user_active(&self) -> bool { false }

        /// Lock tokens in favor of a user.
        /// Fails if the user has not enough money or the other account does not exist
        pub(external) fn sponsor_user(&mut self, u64 accountToSponsor, u64 tokens) -> bool {}

        /// Withdraw tokens from a sponsorship to show dissaproval.
        pub(external) fn withdraw_sponsorship(& mut self, u64 accountToWithdrawFrom, u64 tokens) -> bool {}
    }

    impl Bonfire.dao.local {
        pub(external) fn flip(&mut self) {}
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut contract = Bonfire.dao::deploy_mock();
        assert_eq!(contract.get(), false);
        contract.flip();
        assert_eq!(contract.get(), true);
    }
}
