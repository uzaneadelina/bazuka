use super::hash::Hash;
use crate::crypto::{SignatureScheme, VerifiableRandomFunction};

// A proof that you are the validator for this block
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
pub enum ValidatorProof<V: VerifiableRandomFunction> {
    Unproven,
    Proof {
        vrf_output: V::Out,
        vrf_proof: V::Proof,
    },
}

impl<V: VerifiableRandomFunction> ValidatorProof<V> {
    pub fn is_unproven(&self) -> bool {
        match self {
            Self::Proof { .. } => false,
            Self::Unproven => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
pub struct ProofOfStake<S: SignatureScheme, V: VerifiableRandomFunction> {
    /// public-key of validator
    pub validator: S::Pub,
    /// when the validator started validating this block
    pub timestamp: u32,
    /// vrf proof for this validator
    pub proof: ValidatorProof<V>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
pub struct Header<H: Hash, S: SignatureScheme, V: VerifiableRandomFunction> {
    /// the parent hash
    pub parent_hash: H::Output,
    /// block number or block height
    pub number: u64,
    /// the merkle root of current block
    pub block_root: H::Output,
    /// aux data for Proof-of-Stake consensus
    pub proof_of_stake: ProofOfStake<S, V>,
}

impl<H: Hash, S: SignatureScheme, V: VerifiableRandomFunction> Header<H, S, V> {
    pub fn hash(&self) -> H::Output {
        H::hash(&bincode::serialize(&self).expect("convert header to bincode format"))
    }
}
