use super::{i_manager::IManager, terminal_cursor::TerminalCursor};

#[derive(Clone)]
pub struct TerminalOption<T: IManager> {
    option: String,
    focus: bool,
    title: String,
    manager: T,
}

impl <T: IManager> TerminalOption<T> {
    
    pub fn from(title: String, option: &str, manager: T) -> TerminalOption<T> {
        TerminalOption {
            option: String::from(option),
            focus: false,
            title: title,
            manager: manager
        }
    }

    pub fn is_focused(&self) -> bool {
        return self.focus;
    }

    pub fn title(&self) -> String {
        return self.title.clone();
    }

    pub fn focused(&mut self) -> &Self {
        self.focus = true;
        return self;
    }

    pub fn unfocused(&mut self) -> &Self {
        self.focus = false;
        return self;
    }

    pub fn execute(&mut self) -> TerminalCursor<T> {
        return self.manager.manage(self.option.clone());
    }

}