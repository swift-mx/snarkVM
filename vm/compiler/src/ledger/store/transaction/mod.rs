// Copyright (C) 2019-2022 Aleo Systems Inc.
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

mod deployment;
pub use deployment::*;

use crate::ledger::{
    map::{memory_map::MemoryMap, Map, MapRead},
    Origin,
    Transaction,
    Transition,
};
use console::{
    network::prelude::*,
    program::ProgramID,
    types::{Field, Group},
};

use anyhow::Result;
use std::borrow::Cow;

pub enum TransactionType<N: Network> {
    /// A transaction that is a deployment, contains `program ID`.
    Deploy(ProgramID<N>),
    /// A transaction that is an execution, contains `([transition ID], additional fee ID)`.
    Execute(Vec<N::TransitionID>, N::TransitionID),
}

// /// A trait for transaction storage.
// pub trait TransactionStorage<N: Network>: Clone {
//     /// The mapping of `transaction ID` to `transaction type`.
//     type IDMap: for<'a> Map<'a, N::TransactionID, TransactionType<N>>;
//
//     /// Returns the ID map.
//     fn id_map(&self) -> &Self::IDMap;
//
//     /// Returns the transaction ID that contains the given `program ID`.
//     fn find_transaction_id(&self, program_id: &ProgramID<N>) -> Result<Option<N::TransactionID>> {
//         match self.reverse_id_map().get(program_id)? {
//             Some(Cow::Borrowed(transaction_id)) => Ok(Some(*transaction_id)),
//             Some(Cow::Owned(transaction_id)) => Ok(Some(transaction_id)),
//             None => Ok(None),
//         }
//     }
//
//     /// Returns the program ID for the given `transaction ID`.
//     fn get_program_id(&self, transaction_id: &N::TransactionID) -> Result<ProgramID<N>> {
//         // Retrieve the program ID.
//         match self.id_map().get(transaction_id)? {
//             Some(Cow::Borrowed(program_id)) => Ok(program_id),
//             Some(Cow::Owned(program_id)) => Ok(program_id),
//             None => Ok(vec![]),
//         }
//     }
//
//     /// Returns the deployment for the given `transaction ID`.
//     fn get(&self, transaction_id: &N::TransactionID) -> Result<Transaction<N>> {
//         // Retrieve the program ID.
//         let program_id = self.get_program_id(transaction_id)?;
//
//         // Retrieve the additional fee.
//         let additional_fee = self.additional_fee_map().get(&program_id)?;
//
//         // Return the deployment.
//         Ok(Transaction::new(edition, program, verifying_keys))
//     }
//
//     /// Stores the given `deployment transaction` pair into storage.
//     fn insert(&self, transaction: &Transaction<N>) -> Result<()> {
//         // Ensure the transaction is a deployment.
//         let (transaction_id, deployment, additional_fee) = match transaction {
//             Transaction::Deploy(transaction_id, deployment, additional_fee) => {
//                 (transaction_id, deployment, additional_fee)
//             }
//             Transaction::Execute(..) => {
//                 bail!("Attempted to insert non-deployment transaction into deployment storage.")
//             }
//         };
//
//         // Retrieve the program.
//         let program = deployment.program();
//         // Retrieve the program ID.
//         let program_id = *program.id();
//
//         // Ensure the number of functions matches the number of verifying keys.
//         if program.functions().len() != deployment.verifying_keys().len() {
//             bail!("Transaction has an incorrect number of verifying keys, according to the program.");
//         }
//         // Ensure the deployment contains the correct functions.
//         for function in program.functions() {
//             if !deployment.verifying_keys().contains(function.name()) {
//                 bail!("Transaction is missing a verifying key for function '{}'.", function.name());
//             }
//         }
//
//         // Store the program ID.
//         self.id_map().insert(transaction_id, program_id)?;
//         // Store the reverse program ID.
//         self.reverse_id_map().insert(program_id, transaction_id)?;
//
//         // Store the additional fee.
//         self.additional_fee_map().insert(program_id, additional_fee.clone())?;
//
//         Ok(())
//     }
//
//     /// Removes the deployment for the given `transaction ID`.
//     fn remove(&self, transaction_id: &N::TransactionID) -> Result<()> {
//         // Retrieve the program ID.
//         let program_id = self.get_program_id(transaction_id)?;
//         // Retrieve the program.
//         let program = self.program_map().get(&program_id)?;
//
//         // Remove the program ID.
//         self.id_map().remove(transaction_id)?;
//         // Remove the reverse program ID.
//         self.reverse_id_map().remove(&program_id)?;
//         // Remove the additional fee.
//         self.additional_fee_map().remove(&program_id)?;
//
//         Ok(())
//     }
// }
//
// /// An in-memory transaction storage.
// #[derive(Clone, Default)]
// pub struct TransactionMemory<N: Network> {
//     /// The mapping of `transaction ID` to `transaction type`.
//     id_map: MemoryMap<N::TransactionID, TransactionType<N>>,
//     /// The mapping of `program ID` to `transaction ID`.
//     reverse_id_map: MemoryMap<ProgramID<N>, N::TransactionID>,
//     /// The mapping of `program ID` to `additional fee`.
//     additional_fee_map: MemoryMap<ProgramID<N>, AdditionalFee<N>>,
// }
//
// impl<N: Network> TransactionMemory<N> {
//     /// Creates a new in-memory deployment storage.
//     pub fn new() -> Self {
//         Self {
//             id_map: MemoryMap::default(),
//             reverse_id_map: MemoryMap::default(),
//             additional_fee_map: MemoryMap::default(),
//         }
//     }
// }
//
// #[rustfmt::skip]
// impl<N: Network> TransactionStorage<N> for TransactionMemory<N> {
//     type IDMap = MemoryMap<N::TransactionID, Vec<Field<N>>>;
//     type ReverseIDMap = MemoryMap<Field<N>, N::TransactionID>;
//     type AdditionalFeeMap = MemoryMap<ProgramID<N>, AdditionalFee<N>>;
//
//     /// Returns the ID map.
//     fn id_map(&self) -> &Self::IDMap {
//         &self.id_map
//     }
//
//     /// Returns the reverse ID map.
//     fn reverse_id_map(&self) -> &Self::ReverseIDMap {
//         &self.reverse_id_map
//     }
//
//     /// Returns the additional fee map.
//     fn additional_fee_map(&self) -> &Self::AdditionalFeeMap {
//         &self.additional_fee_map
//     }
// }

// /// A trait for transaction storage.
// pub trait TransactionStorage<N: Network>: Clone {
//     type TransactionMap: for<'a> Map<'a, N::TransactionID, (Transaction<N>, N::TransitionID)>;
//     type ExecutionMap: for<'a> Map<'a, N::TransactionID, (Vec<N::TransitionID>, Option<N::TransitionID>)>;
//     type TransitionPublicKeysMap: for<'a> Map<'a, Group<N>, N::TransitionID>;
// }
//
// /// An in-memory transaction storage.
// #[derive(Clone)]
// pub struct TransactionMemory<N: Network>(core::marker::PhantomData<N>);
//
// #[rustfmt::skip]
// impl<N: Network> TransactionStorage<N> for TransactionMemory<N> {
//     type TransactionMap = MemoryMap<N::TransactionID, (Transaction<N>, N::TransitionID)>;
//     type ExecutionMap = MemoryMap<N::TransactionID, (Vec<N::TransitionID>, Option<N::TransitionID>)>;
//     type TransitionPublicKeysMap = MemoryMap<Group<N>, N::TransitionID>;
// }
//
// /// The transaction state stored in a ledger.
// #[derive(Clone)]
// pub struct TransactionStore<N: Network, T: TransactionStorage<N>> {
//     /// The map of program deployments.
//     deployments: T::TransactionMap,
//     /// The map of program executions.
//     executions: T::ExecutionMap,
//     /// The map of transitions.
//     transitions: T::TransitionsMap,
// }
//
// impl<N: Network> TransactionStore<N, TransactionMemory<N>> {
//     /// Initializes a new instance of `TransactionStore`.
//     pub fn new() -> Self {
//         Self { deployments: Default::default(), executions: Default::default(), transitions: Default::default() }
//     }
// }
//
// impl<N: Network, T: TransactionStorage<N>> TransactionStore<N, T> {
//     /// Initializes a new instance of `TransactionStore` from the given maps.
//     pub fn from_maps(
//         deployments: T::TransactionMap,
//         executions: T::ExecutionMap,
//         transitions: T::TransitionsMap,
//     ) -> Result<Self> {
//         let transaction_store = Self { deployments, executions, transitions };
//
//         // TODO (raychu86): Enforce that all the transaction state is valid.
//
//         Ok(transaction_store)
//     }
//
//     // /// Checks the given transaction is well formed and unique.
//     // pub fn check_transaction(&self, transaction: &Transaction<N>) -> Result<()> {
//     //     let transaction_id = transaction.id();
//     //     if self.contains_transaction_id(&transaction_id)? {
//     //         bail!("Transaction '{transaction_id}' already exists in the ledger")
//     //     }
//     //
//     //     // Ensure the ledger does not already contain a given transition public keys.
//     //     for tpk in transaction.transition_public_keys() {
//     //         if self.contains_transition_public_key(tpk)? {
//     //             bail!("Transition public key '{tpk}' already exists in the ledger")
//     //         }
//     //     }
//     //
//     //     // Ensure that the origin are valid.
//     //     for origin in transaction.origins() {
//     //         if !self.contains_origin(origin)? {
//     //             bail!("The given transaction references a non-existent origin {}", &origin)
//     //         }
//     //
//     //         match origin {
//     //             // Check that the commitment exists in the ledger.
//     //             Origin::Commitment(commitment) => {
//     //                 if !self.contains_commitment(commitment)? {
//     //                     bail!("The given transaction references a non-existent commitment {}", &commitment)
//     //                 }
//     //             }
//     //             // TODO (raychu86): Ensure that the state root exists in the ledger.
//     //             // Check that the state root is an existing state root.
//     //             Origin::StateRoot(_state_root) => {
//     //                 bail!("State roots are currently not supported (yet)")
//     //             }
//     //         }
//     //     }
//     //
//     //     // Ensure the ledger does not already contain a given serial numbers.
//     //     for serial_number in transaction.serial_numbers() {
//     //         if self.contains_serial_number(serial_number)? {
//     //             bail!("Serial number '{serial_number}' already exists in the ledger")
//     //         }
//     //     }
//     //
//     //     // Ensure the ledger does not already contain a given commitments.
//     //     for commitment in transaction.commitments() {
//     //         if self.contains_commitment(commitment)? {
//     //             bail!("Commitment '{commitment}' already exists in the ledger")
//     //         }
//     //     }
//     //
//     //     // Ensure the ledger does not already contain a given nonces.
//     //     for nonce in transaction.nonces() {
//     //         if self.contains_nonce(nonce)? {
//     //             bail!("Nonce '{nonce}' already exists in the ledger")
//     //         }
//     //     }
//     //
//     //     Ok(())
//     // }
//     //
//     // /// Adds the given transaction to the transaction store.
//     // pub fn insert(&mut self, transaction: Transaction<N>) -> Result<()> {
//     //     // Check that there are not collisions with existing transactions.
//     //     self.check_transaction(&transaction)?;
//     //
//     //     /* ATOMIC CODE SECTION */
//     //
//     //     // Insert the transaction. This code section executes atomically.
//     //     {
//     //         let mut transaction_store = self.clone();
//     //
//     //         for transition in transaction.transitions() {
//     //             let transition_id = transition.id();
//     //
//     //             // Insert the transitions.
//     //             transaction_store.transitions.insert(*transition_id, transition.clone())?;
//     //
//     //             // Insert the transition public key.
//     //             transaction_store.transition_public_keys.insert(*transition.tpk(), *transition_id)?;
//     //
//     //             // Insert the serial numbers.
//     //             for serial_number in transition.serial_numbers() {
//     //                 transaction_store.serial_numbers.insert(*serial_number, *transition_id)?;
//     //             }
//     //
//     //             // Insert the commitments.
//     //             for commitment in transition.commitments() {
//     //                 transaction_store.commitments.insert(*commitment, *transition_id)?;
//     //             }
//     //
//     //             // Insert the origins.
//     //             for origin in transition.origins() {
//     //                 transaction_store.origins.insert(*origin, *transition_id)?;
//     //             }
//     //
//     //             // Insert the nonces.
//     //             for nonce in transition.nonces() {
//     //                 transaction_store.nonces.insert(*nonce, *transition_id)?;
//     //             }
//     //         }
//     //
//     //         // Insert the deployment or execution.
//     //         match transaction {
//     //             Transaction::Deploy(transaction_id, deployment, additional_fee) => {
//     //                 transaction_store.deployments.insert(transaction_id, (deployment, *additional_fee.id()))?;
//     //             }
//     //             Transaction::Execute(transaction_id, execution, additional_fee) => {
//     //                 let transition_ids = execution.iter().map(|transition| *transition.id()).collect();
//     //
//     //                 transaction_store
//     //                     .executions
//     //                     .insert(transaction_id, (transition_ids, additional_fee.map(|t| *t.id())))?;
//     //             }
//     //         }
//     //
//     //         *self = Self {
//     //             deployments: transaction_store.deployments,
//     //             executions: transaction_store.executions,
//     //             transitions: transaction_store.transitions,
//     //             transition_public_keys: transaction_store.transition_public_keys,
//     //             origins: transaction_store.origins,
//     //             serial_numbers: transaction_store.serial_numbers,
//     //             commitments: transaction_store.commitments,
//     //             nonces: transaction_store.nonces,
//     //         };
//     //     }
//     //
//     //     Ok(())
//     // }
// }
//
// impl<N: Network, T: TransactionStorage<N>> TransactionStore<N, T> {
//     /// Returns an iterator over the transaction IDs, for all transitions in `self`.
//     pub fn transaction_ids(&self) -> impl '_ + Iterator<Item = Cow<'_, N::TransactionID>> {
//         self.deployments.keys().chain(self.executions.keys())
//     }
// }
//
// impl<N: Network, T: TransactionStorage<N>> TransactionStore<N, T> {
//     /// Returns `true` if the given transaction ID exists.
//     pub fn contains_transaction_id(&self, transaction_id: &N::TransactionID) -> Result<bool> {
//         self.deployments.contains_key(transaction_id).or_else(|_| self.executions.contains_key(transaction_id))
//     }
// }
//
// use crate::Execution;
// use core::borrow::Borrow;
//
// impl<N: Network, T: TransactionStorage<N>> TransactionStore<N, T> {
//     /// Returns the transaction for the given transaction id.
//     pub fn get_transaction(&self, transaction_id: N::TransactionID) -> Result<Transaction<N>> {
//         if let Some(value) = self.deployments.get(&transaction_id)? {
//             // Return the deployment transaction.
//             let (deployment, additional_fee) = value.borrow();
//             Transaction::from_deployment(deployment.clone(), self.get_transition(*additional_fee)?.into_owned())
//         } else if let Some(value) = self.executions.get(&transaction_id)? {
//             // Return the execution transaction.
//             let (execution, additional_fee) = value.borrow();
//
//             let transitions: Result<Vec<_>> = execution
//                 .iter()
//                 .map(|transition_id| self.get_transition(*transition_id).map(|t| t.into_owned()))
//                 .collect();
//             let execution = Execution::from(N::ID, &transitions?)?;
//             let additional_fee = match additional_fee {
//                 Some(transition_id) => Some(self.get_transition(*transition_id)?.into_owned()),
//                 None => None,
//             };
//
//             Transaction::from_execution(execution, additional_fee)
//         } else {
//             bail!("Missing transaction with id {transaction_id}")
//         }
//     }
//
//     /// Returns the transactions for the given transition id.
//     pub fn get_transition(&self, transition_id: N::TransitionID) -> Result<Cow<'_, Transition<N>>> {
//         match self.transitions.get(&transition_id)? {
//             Some(transition) => Ok(transition),
//             None => bail!("Missing transition with id {transition_id}"),
//         }
//     }
// }
