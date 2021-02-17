#[allow(clippy::clippy::cargo_common_metadata)]

pub mod upd {
    pub trait Updatable<StateUpdate> {
        type StateUpdateError;
        fn apply(&mut self, update: StateUpdate) -> Result<(), Self::StateUpdateError>;
    }
}
