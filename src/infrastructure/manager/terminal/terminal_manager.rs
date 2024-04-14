use std::io::{self, Write};

use crossterm::event::{read, Event, KeyCode, KeyEventKind};

use super::{i_manager::IManager, terminal_cursor::TerminalCursor};

const ANSI_COLOR_RESET: &'static str = "\x1b[0m";
const ANSI_BACKGROUND_WHITE: &'static str = "\x1b[47m";

#[derive(Clone)]
pub struct TerminalManager<T: IManager> {
    cursor: TerminalCursor<T>,
}

impl <T: IManager> TerminalManager<T> {
    
    pub fn new(cursor: TerminalCursor<T>) -> TerminalManager<T> {
        return TerminalManager {cursor};
    }

    pub fn launch(&mut self) -> io::Result<()> {

        self.hide_cursor();

        loop {

            self.clear_screen();

            self.print();

            let key_event = match read()? {
                Event::Key(event) => event,
                _ => continue, // Skip non-key events
            };
            
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Up => {self.cursor.decrease();},
                    KeyCode::Down => {self.cursor.increase();},
                    KeyCode::Enter => {
                        println!("Enter");

                        let update = self.manage();
                        if update.is_none() {
                            println!("Something goes wrong!");
                            break;
                        }

                        self.cursor = update.unwrap();

                    },
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

    fn clear_screen(&self) {
        print!("\x1b[2J\x1b[1;1H");
    }
    
    fn hide_cursor(&self) {
        print!("\x1b[?25l");
    }
    
    fn show_cursor(&self) {
        print!("\x1b[?25h");
    }

    fn print(&mut self) {

        print!("{}\n\n", self.cursor.header());

        for cursor in self.cursor.options().iter_mut().enumerate() {
            let index = cursor.0;
            let position = cursor.1;

            let mut title = position.title();
            if position.is_focused() {
                title = format!("{}{}{}", ANSI_BACKGROUND_WHITE, title, ANSI_COLOR_RESET);
            }
            print!("{}.- {}.\n", index + 1, title);
        }

        let _ = io::stdout().flush();

    }

    fn manage(&mut self) -> Option<TerminalCursor<T>> {
        let o_option = self.cursor.option();
        if o_option.is_some() {
            let option = o_option.unwrap();
            return Some(option.execute());
        }
        None
    }

}