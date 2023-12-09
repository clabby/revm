use crate::{Error, Precompile, PrecompileResult, PrecompileWithAddress, StandardPrecompileFn};
use alloc::vec::Vec;
use core::cmp::min;
use revm_primitives::{secp256k1, B256};

pub const ECRECOVER: PrecompileWithAddress = PrecompileWithAddress(
    crate::u64_to_address(1),
    Precompile::Standard(ec_recover_run as StandardPrecompileFn),
);

fn ec_recover_run(i: &[u8], target_gas: u64) -> PrecompileResult {
    const ECRECOVER_BASE: u64 = 3_000;

    if ECRECOVER_BASE > target_gas {
        return Err(Error::OutOfGas);
    }
    let mut input = [0u8; 128];
    input[..min(i.len(), 128)].copy_from_slice(&i[..min(i.len(), 128)]);

    let msg = B256::from_slice(&input[0..32]);

    let mut sig = [0u8; 65];
    sig[0..64].copy_from_slice(&input[64..128]);

    if input[32..63] != [0u8; 31] || !matches!(input[63], 27 | 28) {
        return Ok((ECRECOVER_BASE, Vec::new()));
    }

    sig[64] = input[63] - 27;

    let out = secp256k1::ecrecover(&sig, &msg)
        .map(|o| o.to_vec())
        .unwrap_or_default();

    Ok((ECRECOVER_BASE, out))
}
