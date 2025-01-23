//! EIP-7706 types, constants, and utilities.

use crate::ExcessGasAndPrice;
use core::ops::{Add, Mul, Sub};

/// The [ResourceVector] type represents a vector of fee parameters for the [EIP-7706] fee schedule.
///
/// [EIP-7706]: https://eips.ethereum.org/EIPS/eip-7706
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourceVector {
    /// The execution resource fee parameter.
    pub execution: u64,
    /// The calldata resource fee parameter.
    pub calldata: u64,
    /// The blob resource fee parameter.
    pub blob: u64,
}

impl ResourceVector {
    /// Creates a new [ResourceVector] with the given fees.
    pub fn new(execution: u64, calldata: u64, blob: u64) -> Self {
        Self {
            execution,
            calldata,
            blob,
        }
    }

    /// Returns `true` if all parameters are less than or equal to the corresponding fees in `other`.`
    pub fn all_less_or_equal(&self, other: &Self) -> bool {
        self.execution <= other.execution
            && self.calldata <= other.calldata
            && self.blob <= other.blob
    }

    /// Subtracts the paramters in `rhs` from the parameters in `self`, saturating at zero.
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self {
            execution: self.execution.saturating_sub(rhs.execution),
            calldata: self.calldata.saturating_sub(rhs.calldata),
            blob: self.blob.saturating_sub(rhs.blob),
        }
    }

    /// Returns the sum of the paramters.
    pub fn sum(&self) -> u64 {
        self.blob + self.calldata + self.execution
    }
}

impl Add for ResourceVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            execution: self.execution + rhs.execution,
            calldata: self.calldata + rhs.calldata,
            blob: self.blob + rhs.blob,
        }
    }
}

impl Sub for ResourceVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            execution: self.execution - rhs.execution,
            calldata: self.calldata - rhs.calldata,
            blob: self.blob - rhs.blob,
        }
    }
}

impl Mul for ResourceVector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            execution: self.execution * rhs.execution,
            calldata: self.calldata * rhs.calldata,
            blob: self.blob * rhs.blob,
        }
    }
}

/// The [ResourcePrices] type represents the prices of each resource on the [EIP-7706] fee schedule
/// for a given block.
///
/// [EIP-7706]: https://eips.ethereum.org/EIPS/eip-7706
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourcePrices {
    /// The execution resource price.
    pub execution: ExcessGasAndPrice,
    /// The calldata resource price.
    pub calldata: ExcessGasAndPrice,
    /// The blob resource price.
    pub blob: ExcessGasAndPrice,
}

impl ResourcePrices {
    /// Creates a new [ResourcePrices] with the given prices.
    pub fn new(
        execution: ExcessGasAndPrice,
        calldata: ExcessGasAndPrice,
        blob: ExcessGasAndPrice,
    ) -> Self {
        Self {
            execution,
            calldata,
            blob,
        }
    }

    /// Return the gas prices for each resource as a [ResourceVector].
    pub fn gas_prices(&self) -> ResourceVector {
        ResourceVector {
            execution: self.execution.gasprice as u64,
            calldata: self.calldata.gasprice as u64,
            blob: self.blob.gasprice as u64,
        }
    }

    /// Return the excess gas for each resource as a [ResourceVector].
    pub fn excess_gas(&self) -> ResourceVector {
        ResourceVector {
            execution: self.execution.excess_gas,
            calldata: self.calldata.excess_gas,
            blob: self.blob.excess_gas,
        }
    }
}
