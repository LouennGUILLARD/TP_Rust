pub mod memory;

use async_trait::async_trait;

use crate::domain::VotingMachine;

#[async_trait]
pub trait storage {
    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>;
    async fn put_voting_machine(&self, voting_machine: VotingMachine) -> anyhow::Result<()>;
}