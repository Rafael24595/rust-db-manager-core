use super::terminal_cursor::TerminalCursor;

pub trait IManager: Clone {
    fn manage(&self, option: String) -> TerminalCursor<Self> where Self: Sized;
}