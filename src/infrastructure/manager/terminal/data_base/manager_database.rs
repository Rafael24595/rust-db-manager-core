use std::vec;

use async_trait::async_trait;

use crate::{
    domain::filter::{data_base_query::DataBaseQuery, filter_element::FilterElement}, 
    infrastructure::{manager::terminal::{
        i_manager::IManager, terminal_cursor::TerminalCursor, terminal_manager::{self, TerminalManager}, terminal_option::TerminalOption}, 
        repository::i_db_repository::IDBRepository
    }, 
    service::service::Service};

const HOME: &'static str = "HOME";
const STATUS: &'static str = "STATUS";

const TEXT_INPUT: &'static str = "TEXT_INPUT";

pub const SHOW_DATABASES: &'static str = "SHOW_DATABASES";
pub const SELECT_DATABASE_PANEL: &'static str = "SELECT_DATABASE_PANEL";
pub const SELECT_DATABASE: &'static str = "SELECT_DATABASE";

pub const SHOW_COLLECTIONS: &'static str = "SHOW_COLLECTIONS";
pub const SELECT_COLLECTION_PANEL: &'static str = "SELECT_COLLECTION_PANEL";
pub const SELECT_COLLECTION: &'static str = "SELECT_COLLECTION";

pub const SHOW_ELEMENTS: &'static str = "SHOW_ELEMENTS";
pub const SELECT_ELEMENTS_PANEL: &'static str = "SELECT_ELEMENTS_PANEL";
pub const SELECT_ELEMENT: &'static str = "SELECT_ELEMENT";

pub const SHOW_SELECTED: &'static str = "SHOW_SELECTED";

#[derive(Clone)]
pub struct ManagerDatabase<T: IDBRepository> {
    pub service: Service<T>,
    pub data_base: Option<String>,
    pub collection: Option<String>,
    pub element: Option<Vec<String>>
}

#[async_trait]
impl <T: IDBRepository> IManager for ManagerDatabase<T> {

    fn text_input_option(&self) -> &str {
        return TEXT_INPUT;
    }

    async fn manage(&self, option: TerminalOption<Self>) -> TerminalCursor<Self> where Self: Sized {
        match option.option().as_str() {
            HOME => self.clone().home(&self.default_header()),
            STATUS => self.clone().status().await,

            TEXT_INPUT => self.clone().translate_query(option).await,

            SHOW_DATABASES => self.clone().show_databases().await,
            SELECT_DATABASE_PANEL => self.clone().select_database_panel().await,
            SELECT_DATABASE => self.clone().select_database(option),

            SHOW_COLLECTIONS => self.clone().show_collections().await,
            SELECT_COLLECTION_PANEL => self.clone().select_collection_panel().await,
            SELECT_COLLECTION => self.clone().select_collection(option),

            SHOW_ELEMENTS => self.clone().show_elements().await,
            SELECT_ELEMENTS_PANEL => self.clone().select_element_panel().await,
            SELECT_ELEMENT => self.clone().select_element(option),

            SHOW_SELECTED => self.clone().show_selected().await,
            _ => todo!(),
        }
    }

}

impl <T: IDBRepository> ManagerDatabase<T> {

    pub fn new(service: Service<T>) -> ManagerDatabase<T> {
        ManagerDatabase { 
            service: service,
            data_base: None,
            collection: None,
            element: None
        }
    }

    pub async fn launch(&mut self) -> &Self {
        let header = self.default_header();
        let cursor = self.home(&header);
        let _ = TerminalManager::new(cursor).launch().await;
        return self;
    }

    async fn status(self) -> TerminalCursor<Self> {
        let cursor = TerminalCursor::new(self.clone(), "//TODO:");
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

        let mut cursor: TerminalCursor<Self> = TerminalCursor::new(self.clone(), &header);

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
            self.reset_database();
        }

        self.home_headers()
    }


    async fn show_collections(&self) -> TerminalCursor<Self> {
        if let Err(error) = self.verify_database() {
            let header = self.info_headers(&error.message());
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
        if let Err(error) = self.verify_database() {
            let header = self.info_headers(&error.message());
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

        let mut cursor: TerminalCursor<Self> = TerminalCursor::new(self.clone(), &header);

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
            self.reset_collection();
        }

        self.home_headers()
    }


    async fn show_elements(&self) -> TerminalCursor<Self> {
        if let Err(error) = self.verify_collection() {
            let header = self.info_headers(&error.message());
            return self.home(&header);
        }

        let query = DataBaseQuery::from(self.data_base.clone().unwrap(), self.collection.clone().unwrap());

        let result = self.service.find_all_lite(query).await;

        let mut header = self.info_headers("The repository contains the following items:");
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

    async fn select_element_panel(&self) -> TerminalCursor<Self> {
        if let Err(error) = self.verify_collection() {
            let header = self.info_headers(&error.message());
            return self.home(&header);
        }

        let query = DataBaseQuery::from(self.data_base.clone().unwrap(), self.collection.clone().unwrap());

        let result = self.service.find_all_lite(query).await;

        let mut header = self.info_headers("Select one of the following elements:");
        if let Err(err) = &result {
            header = err.to_string();
        }
    
        let mut vector = Vec::<String>::new();
        if result.is_ok() {
            vector = result.ok().unwrap();
        }

        let mut cursor: TerminalCursor<Self> = TerminalCursor::new(self.clone(), &header);

        for element in vector {
            let args = Vec::from(vec![element.clone()]);
            cursor.push(TerminalOption::from_args(element, SELECT_ELEMENT, args, self.clone()));
        }

        cursor.push(TerminalOption::from(String::from("[None]"), SELECT_ELEMENT, self.clone()));

        cursor
    }

    fn select_element(&mut self, option: TerminalOption<Self>) -> TerminalCursor<Self> {
        let args = option.args();
        if args.len() > 0 {
            let element = args.get(0).unwrap().to_string();
            self.element = Some(Vec::from(vec![element]));
        } else {
            self.reset_element();
        }

        self.home_headers()
    }

    async fn show_selected(&self) -> TerminalCursor<Self> {
        if let Err(error) = self.verify_element() {
            let header = self.info_headers(&error.message());
            return self.home(&header);
        }

        let filter = FilterElement::from_id_chain_collection(self.element.clone().unwrap());
        let query = DataBaseQuery::from_filter(self.data_base.clone().unwrap(), self.collection.clone().unwrap(), filter);

        let r_elements = self.service.find_query(query).await;
        if r_elements.is_err() {
            let header = self.info_headers(&format!("Cannot find enlement: {}", r_elements.unwrap_err().to_string()));
            return self.home(&header);
        }

        let mut elements = r_elements.unwrap();

        if elements.len() == 1 {
            let header = self.info_headers("Item:");
            return self.home(&format!("{}\n\n{}", header, elements.remove(0)));
        }

        elements = elements.iter()
            .map(|e| format!(" {}{}{}", terminal_manager::ANSI_BOLD, e, terminal_manager::ANSI_COLOR_RESET))
            .collect::<Vec<String>>();

        let header = self.info_headers("Items:");
        self.home(&format!("{}\n\n{}", header, elements.join("\n\n")))
    }
    

    async fn translate_query(&mut self, option: TerminalOption<Self>) -> TerminalCursor<Self> {
        let args = option.args();
        if args.len() == 0 {
            return self.home_headers();
        }

        let mut fragments = args.get(0).unwrap().split(">").map(|f| String::from(f)).collect::<Vec<String>>();
        let first = String::from(fragments.remove(0).trim());

        if first.is_empty() || first == "*" {
            return self.translate_path(first, fragments).await;
        }

        return self.home_headers();
    }

}