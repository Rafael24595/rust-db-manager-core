use std::vec;

use async_trait::async_trait;

use crate::{domain::filter::data_base_query::DataBaseQuery, infrastructure::repository::i_db_repository::IDBRepository, service::service::Service};

use super::{i_manager::IManager, terminal_cursor::TerminalCursor, terminal_manager::{self, TerminalManager}, terminal_option::TerminalOption};

const HOME: &'static str = "HOME";
const STATUS: &'static str = "STATUS";

const SHOW_DATABASES: &'static str = "SHOW_DATABASES";
const SELECT_DATABASE_PANEL: &'static str = "SELECT_DATABASE_PANEL";
const SELECT_DATABASE: &'static str = "SELECT_DATABASE";

const SHOW_COLLECTIONS: &'static str = "SHOW_COLLECTIONS";
const SELECT_COLLECTION_PANEL: &'static str = "SELECT_COLLECTION_PANEL";
const SELECT_COLLECTION: &'static str = "SELECT_COLLECTION";


#[derive(Clone)]
pub struct TerminalDatabase<T: IDBRepository> {
    service: Service<T>,
    data_base: Option<String>,
    collection: Option<String>
}

#[async_trait]
impl <T: IDBRepository> IManager for TerminalDatabase<T> {

    async fn manage(&self, option: TerminalOption<Self>) -> TerminalCursor<Self> where Self: Sized {
        match option.option().as_str() {
            HOME => self.clone().home(&self.default_header()),
            STATUS => self.clone().status().await,

            SHOW_DATABASES => self.clone().show_databases().await,
            SELECT_DATABASE_PANEL => self.clone().select_database_panel().await,
            SELECT_DATABASE => self.clone().select_database(option),

            SHOW_COLLECTIONS => self.clone().show_collections().await,
            SELECT_COLLECTION_PANEL => self.clone().select_collection_panel().await,
            SELECT_COLLECTION => self.clone().select_collection(option),
            _ => todo!(),
        }
    }

}

impl <T: IDBRepository> TerminalDatabase<T> {

    pub fn new(service: Service<T>) -> TerminalDatabase<T> {
        TerminalDatabase { 
            service: service,
            data_base: None,
            collection: None
        }
    }

    pub async fn launch(&mut self) -> &Self {
        let header = self.default_header();
        let cursor = self.home(&header);
        let _ = TerminalManager::new(cursor).launch().await;
        return self;
    }

    pub fn default_header(&self) -> String {
        return self.info_headers("Select any option: ");
    }

    pub fn info_headers(&self, header: &str) -> String {
        let mut headers = Vec::<String>::new();

        if self.data_base.is_some() {
            headers.push(format!("* Selected data base '{}'.", self.data_base.as_ref().unwrap()));
        }

        if self.collection.is_some() {
            headers.push(format!("* Selected collection '{}'.", self.collection.as_ref().unwrap()));
        }

        if headers.is_empty() {
            return String::from(header);
        }

        return format!("{}\n\n{}", header, headers.join("\n"));
    }

    fn home(&self, header: &str) -> TerminalCursor<Self> {
        let mut cursor: TerminalCursor<Self> = TerminalCursor::new(header);
        cursor.push(TerminalOption::from(String::from("Show databases"), SHOW_DATABASES, self.clone()));
        cursor.push(TerminalOption::from(String::from("Select database"), SELECT_DATABASE_PANEL, self.clone()));
        if self.data_base.is_some() {
            cursor.push(TerminalOption::from(String::from("Show collections"), SHOW_COLLECTIONS, self.clone()));
            cursor.push(TerminalOption::from(String::from("Select collection"), SELECT_COLLECTION_PANEL, self.clone()));
        }
        cursor
    }

    async fn status(self) -> TerminalCursor<Self> {
        let cursor = TerminalCursor::new("//TODO:");
        cursor
    }

    async fn show_databases(&self) -> TerminalCursor<Self> {
        let result = self.service.list_data_bases().await;

        let mut header = self.info_headers("The repository contains the following data bases:");
        if let Err(err) = &result {
            header = err.to_string();
        }
    
        let mut vector = Vec::<String>::new();
        if result.is_ok() {
            vector = result.ok().unwrap();
        }

        let mut elements = Vec::<String>::new();
        for element in vector {
            elements.push(format!(" - {}{}{}", terminal_manager::ANSI_BOLD, element, terminal_manager::ANSI_COLOR_RESET));
        }

        if !elements.is_empty() {
            header = format!("{}\n", header);
        }

        self.home(&format!("{}\n{}", header, elements.join("\n")))
    }

    async fn select_database_panel(&self) -> TerminalCursor<Self> {
        let result = self.service.list_data_bases().await;

        let mut header = self.info_headers("Select one of the following data bases:");
        if let Err(err) = &result {
            header = err.to_string();
        }
    
        let mut vector = Vec::<String>::new();
        if result.is_ok() {
            vector = result.ok().unwrap();
        }

        let mut cursor: TerminalCursor<Self> = TerminalCursor::new(&header);

        for element in vector {
            let args = Vec::from(vec![element.clone()]);
            cursor.push(TerminalOption::from_args(element, SELECT_DATABASE, args, self.clone()));
        }

        cursor.push(TerminalOption::from(String::from("[None]"), SELECT_DATABASE, self.clone()));

        cursor
    }

    fn select_database(&mut self, option: TerminalOption<Self>) -> TerminalCursor<Self> {
        let args = option.args();
        if args.len() > 0 {
            let data_base = args.get(0).unwrap().to_string();
            self.data_base = Some(data_base);
        } else {
            self.data_base = None;
            self.collection = None;
        }

        self.home(&self.default_header())
    }


    async fn show_collections(&self) -> TerminalCursor<Self> {
        if self.data_base.is_none() {
            let header = self.info_headers("No data base selected:");
            return self.home(&header);
        }

        let query = DataBaseQuery::from_data_base(self.data_base.clone().unwrap());

        let result = self.service.list_collections(query).await;

        let mut header = self.info_headers("The repository contains the following collections:");
        if let Err(err) = &result {
            header = err.to_string();
        }
    
        let mut vector = Vec::<String>::new();
        if result.is_ok() {
            vector = result.ok().unwrap();
        }

        let mut elements = Vec::<String>::new();
        for element in vector {
            elements.push(format!(" - {}{}{}", terminal_manager::ANSI_BOLD, element, terminal_manager::ANSI_COLOR_RESET));
        }

        if !elements.is_empty() {
            header = format!("{}\n", header);
        }

        self.home(&format!("{}\n{}", header, elements.join("\n")))
    }

    async fn select_collection_panel(&self) -> TerminalCursor<Self> {
        if self.data_base.is_none() {
            let header = self.info_headers("No data base selected:");
            return self.home(&header);
        }

        let query = DataBaseQuery::from_data_base(self.data_base.clone().unwrap());

        let result = self.service.list_collections(query).await;

        let mut header = self.info_headers("Select one of the following collections:");
        if let Err(err) = &result {
            header = err.to_string();
        }
    
        let mut vector = Vec::<String>::new();
        if result.is_ok() {
            vector = result.ok().unwrap();
        }

        let mut cursor: TerminalCursor<Self> = TerminalCursor::new(&header);

        for element in vector {
            let args = Vec::from(vec![element.clone()]);
            cursor.push(TerminalOption::from_args(element, SELECT_COLLECTION, args, self.clone()));
        }

        cursor.push(TerminalOption::from(String::from("[None]"), SELECT_COLLECTION, self.clone()));

        cursor
    }

    fn select_collection(&mut self, option: TerminalOption<Self>) -> TerminalCursor<Self> {
        let args = option.args();
        if args.len() > 0 {
            let collection = args.get(0).unwrap().to_string();
            self.collection = Some(collection);
        } else {
            self.collection = None;
        }

        self.home(&self.default_header())
    }

}