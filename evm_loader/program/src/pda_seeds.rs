use crate::types::Address;
use solana_program::pubkey::Pubkey;

pub const AUTHORITY_SEEDS: &[&[u8]] = &[b"Deposit"];

const ACCOUNT_SEED_VERSION_SLICE: &[u8] = &[crate::config::ACCOUNT_SEED_VERSION];

#[must_use]
pub fn balance_account_seeds<'a>(address: &'a Address, chain_id: &'a [u8]) -> [&'a [u8]; 3] {
    [ACCOUNT_SEED_VERSION_SLICE, address.as_bytes(), chain_id]
}

#[must_use]
pub fn balance_account_seeds_bump_seed<'a>(
    address: &'a Address,
    chain_id: &'a [u8],
    bump_seed: &'a [u8],
) -> [&'a [u8]; 4] {
    [
        ACCOUNT_SEED_VERSION_SLICE,
        address.as_bytes(),
        chain_id,
        bump_seed,
    ]
}

#[must_use]
pub fn contract_account_seeds(address: &Address) -> [&[u8]; 2] {
    [ACCOUNT_SEED_VERSION_SLICE, address.as_bytes()]
}

#[must_use]
pub fn contract_account_seeds_bump_seed<'a>(
    address: &'a Address,
    bump_seed: &'a [u8],
) -> [&'a [u8]; 3] {
    [ACCOUNT_SEED_VERSION_SLICE, address.as_bytes(), bump_seed]
}

#[must_use]
pub fn contract_account_seeds_bump_seed_vec(address: &Address, bump_seed: u8) -> Vec<Vec<Vec<u8>>> {
    vec![contract_account_seeds_bump_seed(address, &[bump_seed])
        .into_iter()
        .map(<[u8]>::to_vec)
        .collect()]
}

#[must_use]
pub fn spl_token_seeds<'a>(address: &'a Address, seed: &'a [u8]) -> [&'a [u8]; 4] {
    [
        ACCOUNT_SEED_VERSION_SLICE,
        b"ContractData",
        address.as_bytes(),
        seed,
    ]
}

#[must_use]
pub fn transfer_seeds<'a>(address: &'a Address, seed: &'a [u8]) -> [&'a [u8]; 4] {
    [
        ACCOUNT_SEED_VERSION_SLICE,
        b"AUTH",
        address.as_bytes(),
        seed,
    ]
}

#[must_use]
pub fn treasury_seeds(index: &[u8]) -> [&[u8]; 2] {
    [crate::config::TREASURY_POOL_SEED.as_bytes(), index]
}

#[must_use]
pub fn treasury_seeds_bump_seed<'a>(index: &'a [u8], bump_seed: &'a [u8]) -> [&'a [u8]; 3] {
    [
        crate::config::TREASURY_POOL_SEED.as_bytes(),
        index,
        bump_seed,
    ]
}

#[must_use]
pub fn main_treasury_seeds() -> [&'static [u8]; 1] {
    [crate::config::TREASURY_POOL_SEED.as_bytes()]
}

#[must_use]
pub fn main_treasury_seeds_bump_seed(bump_seed: &[u8]) -> [&[u8]; 2] {
    [crate::config::TREASURY_POOL_SEED.as_bytes(), bump_seed]
}

pub trait PubkeyExt {
    fn find_program_address_with_seeds(
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> (Pubkey, Vec<Vec<u8>>);
}

impl PubkeyExt for Pubkey {
    fn find_program_address_with_seeds(
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> (Pubkey, Vec<Vec<u8>>) {
        let (pubkey, bump_seed) = Pubkey::find_program_address(seeds, program_id);

        let mut seeds: Vec<_> = seeds.iter().map(|v| v.to_vec()).collect();
        seeds.push(vec![bump_seed]);

        (pubkey, seeds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Address;
    use hex::FromHex;
    use std::str::FromStr;

    #[test]
    fn test_authority_pubkey_mainnet() {
        let neon_evm = Pubkey::from_str("NeonVMyRX5GbCrsAHnUwx1nYYoJAtskU1bWUo6JGNyG").unwrap();

        let (pubkey, _) = Pubkey::find_program_address(AUTHORITY_SEEDS, &neon_evm);

        assert_eq!(
            pubkey.to_string(),
            "CUU8HLwbSc2zFEDenmauiEJbCGCNy4eAHAmznZcjB6Nn"
        );
    }

    #[test]
    fn test_usdt_pubkey_mainnet() {
        let neon_evm = Pubkey::from_str("NeonVMyRX5GbCrsAHnUwx1nYYoJAtskU1bWUo6JGNyG").unwrap();

        // Neon USDT token: https://neonscan.org/token/0x5f0155d08ef4aae2b500aefb64a3419da8bb611a
        let usdt_address = Address::from_hex("0x5f0155d08eF4aaE2B500AefB64A3419dA8bB611a").unwrap();

        let (usdt_pubkey, _) =
            Pubkey::find_program_address(&contract_account_seeds(&usdt_address), &neon_evm);

        assert_eq!(
            usdt_pubkey.to_string(),
            "GHuABgXXF37MqV9WyqJXwvzA2eLkcxKf2t8WbiVzBLnU"
        );
    }

    // Neon tx: https://neonscan.org/tx/0x0729687b2f56398652a6593b87b9932f3fe2f2e0c778eb4841a4e17d961a2a11
    #[test]
    fn test_token_account_pubkey_mainnet() {
        let neon_evm = Pubkey::from_str("NeonVMyRX5GbCrsAHnUwx1nYYoJAtskU1bWUo6JGNyG").unwrap();

        // Neon USDT token: https://neonscan.org/token/0x5f0155d08ef4aae2b500aefb64a3419da8bb611a
        let usdt_address = Address::from_hex("0x5f0155d08eF4aaE2B500AefB64A3419dA8bB611a").unwrap();

        // Neon tx.from address: https://neonscan.org/address/0x35b6c40e3873f361c43c073154bf8b37c1f34cd7
        let address = <[u8; 32]>::from_hex(
            "00000000000000000000000035B6C40e3873F361c43c073154BF8b37C1f34Cd7",
        )
        .unwrap();

        let (pubkey, _) =
            Pubkey::find_program_address(&spl_token_seeds(&usdt_address, &address), &neon_evm);

        assert_eq!(
            pubkey.to_string(),
            "12HWB2U31J5AMgDTaaXBdNGN8jAeJNiwpgkewCNVNKyU"
        );
    }
}
