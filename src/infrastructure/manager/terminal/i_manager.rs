use async_trait::async_trait;

use super::{terminal_cursor::TerminalCursor, terminal_option::TerminalOption};

#[async_trait]
pub trait IManager: Clone + Send + Sync {
    async fn manage(&self, option: TerminalOption<Self>) -> TerminalCursor<Self> where Self: Sized;
}