// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::{account_format, AccountError, ComputeKey, Parameters};
use snarkvm_algorithms::traits::{SignatureScheme, PRF};
use snarkvm_fields::{One, Zero};
use snarkvm_utilities::{FromBytes, ToBytes, UniformRand};

use base58::{FromBase58, ToBase58};
use rand::{CryptoRng, Rng};
use std::{fmt, str::FromStr};

#[derive(Derivative)]
#[derivative(
    Clone(bound = "C: Parameters"),
    PartialEq(bound = "C: Parameters"),
    Eq(bound = "C: Parameters")
)]
pub struct PrivateKey<C: Parameters> {
    seed: C::AccountSeed,
    pub(super) sk_sig: C::ProgramScalarField,
    pub(super) r_sig: C::ProgramScalarField,
}

impl<C: Parameters> PrivateKey<C> {
    /// Creates a new account private key.
    pub fn new<R: Rng + CryptoRng>(rng: &mut R) -> Self {
        // Sample a random account seed.
        Self::from(&C::AccountSeed::rand(rng))
    }

    /// Returns `true` if the private key is well-formed. Otherwise, returns `false`.
    pub fn is_valid(&self) -> bool {
        match self.to_compute_key() {
            Ok(compute_key) => compute_key.is_valid(),
            Err(error) => {
                eprintln!("Failed to validate private key: {}", error);
                false
            }
        }
    }

    /// Signs a message using the account private key.
    pub fn sign<R: Rng + CryptoRng>(&self, message: &[u8], rng: &mut R) -> Result<C::AccountSignature, AccountError> {
        Ok(C::account_signature_scheme().sign(&(self.sk_sig, self.r_sig), message, rng)?)
    }

    /// Returns a reference to the account compute key.
    pub fn to_compute_key(&self) -> Result<ComputeKey<C>, AccountError> {
        Ok(ComputeKey::from_private_key(self)?)
    }

    /// Returns the decryption key.
    pub fn to_decryption_key(&self) -> Result<C::ProgramScalarField, AccountError> {
        Ok(self.sk_sig + self.r_sig + self.to_compute_key()?.sk_prf())
    }
}

impl<C: Parameters> From<&C::AccountSeed> for PrivateKey<C> {
    /// Returns the account private key from an account seed.
    fn from(seed: &C::AccountSeed) -> Self {
        Self {
            seed: seed.clone(),
            sk_sig: C::AccountPRF::evaluate(seed, &C::ProgramScalarField::zero())
                .expect("Failed to derive private key component for PRF(seed, 0)"),
            r_sig: C::AccountPRF::evaluate(seed, &C::ProgramScalarField::one())
                .expect("Failed to derive private key component for PRF(seed, 1)"),
        }
    }
}

impl<C: Parameters> FromStr for PrivateKey<C> {
    type Err = AccountError;

    /// Reads in an account private key string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.from_base58()?;
        if data.len() != 43 {
            return Err(AccountError::InvalidByteLength(data.len()));
        }

        if data[0..11] != account_format::PRIVATE_KEY_PREFIX {
            return Err(AccountError::InvalidPrefixBytes(data[0..11].to_vec()));
        }

        Ok(Self::from(&FromBytes::read_le(&data[11..43])?))
    }
}

impl<C: Parameters> fmt::Display for PrivateKey<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut private_key = [0u8; 43];
        private_key[0..11].copy_from_slice(&account_format::PRIVATE_KEY_PREFIX);
        self.seed
            .write_le(&mut private_key[11..43])
            .expect("seed formatting failed");

        write!(f, "{}", private_key.to_base58())
    }
}

impl<C: Parameters> fmt::Debug for PrivateKey<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PrivateKey {{ seed: {:?} }}", self.seed)
    }
}
