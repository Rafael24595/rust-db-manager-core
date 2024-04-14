use crate::{infrastructure::repository::i_db_repository::IDBRepository, service::service::Service};

use super::{i_manager::IManager, terminal_cursor::TerminalCursor, terminal_manager::TerminalManager, terminal_option::TerminalOption};

const STATUS: &'static str = "STATUS";
const SHOW_DATABASES: &'static str = "SHOW_DATABASES";

#[derive(Clone)]
pub struct TerminalDatabase<T: IDBRepository> {
    service: Service<T>,
}

impl <T: IDBRepository> IManager for TerminalDatabase<T> {

    fn manage(&self, option: String) -> TerminalCursor<Self> where Self: Sized {
        match option.as_str() {
            STATUS => self.clone().status(),
            SHOW_DATABASES => self.clone().show_databases(),
            _ => todo!(),
        }
    }

}

impl <T: IDBRepository> TerminalDatabase<T> {

    pub fn new(service: Service<T>) -> TerminalDatabase<T> {
        TerminalDatabase { service: service }
    }

    pub fn launch(&mut self) -> &Self {
        let cursor = self.base_cursor();
        let _ = TerminalManager::new(cursor).launch();
        return self;
    }

    fn base_cursor(&self) -> TerminalCursor<Self> {
        let mut cursor: TerminalCursor<Self> = TerminalCursor::new("Select any option: ");
        cursor.push(TerminalOption::from(String::from("Status"), STATUS, self.clone()));
        cursor.push(TerminalOption::from(String::from("Show databases"), SHOW_DATABASES, self.clone()));
        cursor
    }

    fn status(self) -> TerminalCursor<Self> {
        let cursor = TerminalCursor::new("//TODO:");
        cursor
    }

    fn show_databases(self) -> TerminalCursor<Self> {
        let cursor = TerminalCursor::new("//TODO:");
        cursor
    }

}