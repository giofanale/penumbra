use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use cnidarium_component::ActionHandler;
use penumbra_chain::NoteSource;
use penumbra_shielded_pool::component::NoteManager;

use crate::DaoOutput;

#[async_trait]
impl ActionHandler for DaoOutput {
    type CheckStatelessContext = ();
    async fn check_stateless(&self, _context: ()) -> Result<()> {
        // Any output from the DAO is valid (it's just a transparent output).
        Ok(())
    }

    async fn check_stateful<S: StateRead + 'static>(&self, _state: Arc<S>) -> Result<()> {
        // Any output from the DAO is valid (it's just a transparent output).
        Ok(())
    }

    async fn execute<S: StateWrite>(&self, mut state: S) -> Result<()> {
        // Executing a DAO output is just minting a note to the recipient of the output.
        state
            .mint_note(self.value, &self.address, NoteSource::DaoOutput)
            .await
    }
}
