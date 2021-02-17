#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::needless_return)]
#![allow(clippy::cargo_common_metadata)]

pub mod upd {
    pub trait Updatable<StateUpdate> {
        type StateUpdateError;

        /// # Errors
        /// Will return `Err` when update precondition does not fit the State
        fn apply(&mut self, update: StateUpdate) -> Result<(), Self::StateUpdateError>;
    }
}
