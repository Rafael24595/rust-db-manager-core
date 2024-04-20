use super::{i_manager::IManager, terminal_option::TerminalOption};

#[derive(Clone)]
pub struct TerminalCursor<T: IManager> {
    manager: T,
    header: String,
    options: Vec<TerminalOption<T>>,
    cursor: usize,
}

impl <T: IManager> TerminalCursor<T> {
    
    pub fn new(manager: T, header: &str) -> Self {
        TerminalCursor {
            manager: manager,
            header: String::from(header),
            options: Vec::new(),
            cursor: 0
        }
    }

    pub fn manager(&self) -> T {
        self.manager.clone()
    }

    pub fn header(&self) -> String {
        self.header.clone()
    }

    pub fn options(&mut self) -> Vec<TerminalOption<T>> {
        if self.cursor > self.options.len() {
            self.cursor = 0;
        }

        for option in self.options.iter_mut().enumerate() {
            option.1.unfocused();
            if option.0 == self.cursor {
                option.1.focused();
            }
        }

        self.options.clone()
    }

    pub fn option(&mut self) -> Option<&mut TerminalOption<T>> {
        for option in self.options.iter_mut().enumerate() {
            if option.0 == self.cursor {
                return Some(option.1);
            }
        }
        
        None
    }

    pub fn increase(&mut self) -> &Self {
        if self.options.len() > 0 && self.cursor < self.options.len() - 1 {
            self.cursor = self.cursor + 1;
        }
        self
    }

    pub fn decrease(&mut self) -> &Self {
        if self.cursor > 0 {
            self.cursor = self.cursor - 1;
        }
        self
    }

    pub fn push(&mut self, option: TerminalOption<T>) -> &Self {
        self.options.push(option);
        self
    }

}