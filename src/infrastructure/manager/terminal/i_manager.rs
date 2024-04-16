use async_trait::async_trait;

use super::{terminal_cursor::TerminalCursor, terminal_option::TerminalOption};

#[async_trait]
pub trait IManager: Clone + Send + Sync {
    fn text_input_option(&self) -> &str;
    async fn manage(&self, option: TerminalOption<Self>) -> TerminalCursor<Self> where Self: Sized;
}

#[derive(Clone)]
pub struct VoidManager<T: Clone + Send + Sync> {
    pub void: Option<T>
}

#[async_trait]
impl <T: Clone + Send + Sync> IManager for VoidManager<T> {
    fn text_input_option(&self) -> &str {
        return "";
    }
    async fn manage(&self, _option: TerminalOption<Self>) -> TerminalCursor<Self> {
        TerminalCursor::new(self.clone(),"")
    }
}