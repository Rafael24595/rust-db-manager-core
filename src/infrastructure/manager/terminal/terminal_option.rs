use super::{i_manager::IManager, terminal_cursor::TerminalCursor};

#[derive(Clone)]
pub struct TerminalOption<T: IManager> {
    option: String,
    focus: bool,
    title: String,
    args: Vec<String>,
    manager: T,
}

impl <T: IManager> TerminalOption<T> {
    
    pub fn from(title: String, option: &str, manager: T) -> TerminalOption<T> {
        TerminalOption::from_args(title, option, Vec::new(), manager)
    }

    pub fn from_args(title: String, option: &str, args: Vec<String>, manager: T) -> TerminalOption<T> {
        TerminalOption {
            option: String::from(option),
            focus: false,
            title: title,
            args: args,
            manager: manager
        }
    }

    pub fn is_focused(&self) -> bool {
        return self.focus;
    }

    pub fn option(&self) -> String {
        return self.option.clone();
    }

    pub fn title(&self) -> String {
        return self.title.clone();
    }

    pub fn args(&self) -> Vec<String> {
        return self.args.clone();
    }

    pub fn focused(&mut self) -> &Self {
        self.focus = true;
        return self;
    }

    pub fn unfocused(&mut self) -> &Self {
        self.focus = false;
        return self;
    }

    pub async fn execute(&mut self) -> TerminalCursor<T> {
        return self.manager.manage(self.clone()).await;
    }

}