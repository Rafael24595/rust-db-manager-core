use std::io::{self, Write};

use crossterm::event::{read, Event, KeyCode, KeyEventKind};

use super::{i_manager::IManager, terminal_cursor::TerminalCursor, terminal_option::TerminalOption};

pub(crate) const ANSI_RESET: &'static str = "\x1b[0m";
pub(crate) const ANSI_BACKGROUND_WHITE: &'static str = "\x1b[47m";
pub(crate) const ANSI_BOLD: &'static str = "\x1b[1m";

pub(crate) const ANSI_COLOR_RED: &'static str = "\x1b[31m";
pub(crate) const ANSI_COLOR_GREEN: &'static str = "\x1b[32m";
pub(crate) const ANSI_COLOR_YELLOW: &'static str = "\x1b[33m";

#[derive(Clone)]
pub struct TerminalManager<T: IManager> {
    cursor: TerminalCursor<T>,
}

impl <T: IManager> TerminalManager<T> {
    
    pub fn new(cursor: TerminalCursor<T>) -> TerminalManager<T> {
        return TerminalManager {cursor};
    }

    pub async fn launch(&mut self) -> io::Result<()> {

        self.hide_cursor();

        loop {
            self.clear_screen();
            self.print(false);

            let key_event = match read()? {
                Event::Key(event) => event,
                _ => continue, // Skip non-key events
            };
            
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Up => {self.cursor.decrease();},
                    KeyCode::Down => {self.cursor.increase();},
                    KeyCode::Enter => {

                        let update = self.manage().await;
                        if update.is_none() {
                            println!("Something goes wrong!");
                            break;
                        }

                        self.cursor = update.unwrap();
                    },
                    KeyCode::Char('t') => {
                        self.clear_screen();
                        self.print(true);

                        let input = self.keyboard_input();
                        
                        let update: Option<TerminalCursor<T>> = self.manage_query(input).await;
                        if update.is_none() {
                            println!("Something goes wrong!");
                            break;
                        }

                        self.cursor = update.unwrap();
                    }
                    KeyCode::Esc => {
                        println!("Exit");
                        break;
                    }
                    _ => println!("Wrong key!"),
                }
            }
        }
        
        Ok(())

    }

    fn keyboard_input(&self) -> String {
        self.show_cursor();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        self.hide_cursor();

        return input;
    }

    fn clear_screen(&self) {
        print!("\x1b[2J\x1b[1;1H");
        let _ = io::stdout().flush();
    }
    
    fn hide_cursor(&self) {
        print!("\x1b[?25l");
        let _ = io::stdout().flush();
    }
    
    fn show_cursor(&self) {
        print!("\x1b[?25h");
        let _ = io::stdout().flush();
    }

    fn print(&mut self, sw_ignore_focus: bool) {
        print!("{}\n\n", self.cursor.header());

        for cursor in self.cursor.options().iter_mut().enumerate() {
            let index = cursor.0;
            let position = cursor.1;

            let mut title = position.title();
            if !sw_ignore_focus && position.is_focused() {
                title = format!("{}{}{}", ANSI_BACKGROUND_WHITE, title, ANSI_RESET);
            }
            print!("{}.- {}.\n", index + 1, title);
        }

        print!("\n");

        let _ = io::stdout().flush();
    }

    async fn manage(&mut self) -> Option<TerminalCursor<T>> {
        let o_option = self.cursor.option().cloned();
        if o_option.is_some() {
            let mut option = o_option.unwrap();
            if option.input_required() {
                let input = self.keyboard_input();
                option.push_arg(input);
            }

            println!("\n Please stand by...");

            return Some(option.execute().await);
        }
        None
    }

    async fn manage_query(&mut self, query: String) -> Option<TerminalCursor<T>> {
        let manager = self.cursor.manager();
        let args = Vec::from(vec![query.clone()]);
        let mut option = TerminalOption::from_input(args, manager);
        return Some(option.execute().await);
    }

}