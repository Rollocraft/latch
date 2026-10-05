//! Runtime services used by trusted enforcement adapters.
//!
//! Budget accounting is process-local until a durable runtime store is wired in.
//! The execution gate controls cooperative trusted adapters; OS isolation and
//! durable audit storage are still required for hostile agent processes.

pub mod budget;
pub mod gate;
