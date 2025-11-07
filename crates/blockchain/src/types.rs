// Simplified blockchain types for prototype
// In production, these would be full Substrate pallets with on-chain storage

use parity_scale_codec::{Encode, Decode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

/// User role types
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Serialize, Deserialize)]
pub enum UserRole {
    Professional,
    Institution,
    Employer,
}

/// User information (would be stored on-chain in production)
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Serialize, Deserialize)]
pub struct BlockchainUser {
    pub wallet_address: Vec<u8>,
    pub role: UserRole,
    pub is_verified: bool,
}

/// Institution information (would be stored on-chain in production)
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Serialize, Deserialize)]
pub struct BlockchainInstitution {
    pub institution_name: Vec<u8>,
    pub country: Vec<u8>,
    pub accreditation_number: Option<Vec<u8>>,
    pub is_accredited: bool,
}

/// Credential status
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Serialize, Deserialize)]
pub enum CredentialStatus {
    Pending,
    Issued,
    Revoked,
    Expired,
}

/// Credential information (would be stored on-chain in production)
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Serialize, Deserialize)]
pub struct BlockchainCredential {
    pub credential_id: Vec<u8>,
    pub holder_address: Vec<u8>,
    pub issuer_address: Vec<u8>,
    pub ipfs_hash: Vec<u8>,
    pub chain_hash: Vec<u8>,
    pub status: CredentialStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_encoding() {
        let role = UserRole::Professional;
        let encoded = role.encode();
        let decoded = UserRole::decode(&mut &encoded[..]).unwrap();
        assert_eq!(role, decoded);
    }

    #[test]
    fn test_blockchain_user_encoding() {
        let user = BlockchainUser {
            wallet_address: b"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_vec(),
            role: UserRole::Institution,
            is_verified: true,
        };
        let encoded = user.encode();
        let decoded = BlockchainUser::decode(&mut &encoded[..]).unwrap();
        assert_eq!(user, decoded);
    }

    #[test]
    fn test_credential_status_encoding() {
        let status = CredentialStatus::Issued;
        let encoded = status.encode();
        let decoded = CredentialStatus::decode(&mut &encoded[..]).unwrap();
        assert_eq!(status, decoded);
    }

    #[test]
    fn test_blockchain_credential_encoding() {
        let credential = BlockchainCredential {
            credential_id: b"SSP-123".to_vec(),
            holder_address: b"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_vec(),
            issuer_address: b"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_vec(),
            ipfs_hash: b"QmXxx...".to_vec(),
            chain_hash: vec![1, 2, 3, 4],
            status: CredentialStatus::Issued,
        };
        let encoded = credential.encode();
        let decoded = BlockchainCredential::decode(&mut &encoded[..]).unwrap();
        assert_eq!(credential, decoded);
    }
}
