use std::vec;

use async_trait::async_trait;

use crate::{domain::filter::{data_base_query::DataBaseQuery, filter_element::FilterElement}, infrastructure::repository::i_db_repository::IDBRepository, service::service::Service};

use super::{i_manager::IManager, terminal_cursor::TerminalCursor, terminal_manager::{self, TerminalManager}, terminal_option::TerminalOption};

const HOME: &'static str = "HOME";
const STATUS: &'static str = "STATUS";

const TEXT_INPUT: &'static str = "TEXT_INPUT";

const SHOW_DATABASES: &'static str = "SHOW_DATABASES";
const SELECT_DATABASE_PANEL: &'static str = "SELECT_DATABASE_PANEL";
const SELECT_DATABASE: &'static str = "SELECT_DATABASE";

const SHOW_COLLECTIONS: &'static str = "SHOW_COLLECTIONS";
const SELECT_COLLECTION_PANEL: &'static str = "SELECT_COLLECTION_PANEL";
const SELECT_COLLECTION: &'static str = "SELECT_COLLECTION";

const SHOW_ELEMENTS: &'static str = "SHOW_ELEMENTS";
const SELECT_ELEMENTS_PANEL: &'static str = "SELECT_ELEMENTS_PANEL";
const SELECT_ELEMENT: &'static str = "SELECT_ELEMENT";

const SHOW_SELECTED: &'static str = "SHOW_SELECTED";

#[derive(Clone)]
pub struct TerminalDatabase<T: IDBRepository> {
    service: Service<T>,
    data_base: Option<String>,
    collection: Option<String>,
    element: Option<Vec<String>>
}

#[async_trait]
impl <T: IDBRepository> IManager for TerminalDatabase<T> {

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

impl <T: IDBRepository> TerminalDatabase<T> {

    pub fn new(service: Service<T>) -> TerminalDatabase<T> {
        TerminalDatabase { 
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

        if self.element.is_some() {
            let collection = self.element.as_ref().unwrap()
                .iter()
                .map(|i| format!("'{}'", i))
                .collect::<Vec<String>>();

            if collection.len() > 1 {
                headers.push(format!("* Selected elements: {}.", collection.join(", ")));
            } else if collection.len() == 1 {
                headers.push(format!("* Selected element {}.", collection.get(0).unwrap()));
            }
        }

        if headers.is_empty() {
            return String::from(header);
        }

        return format!("{}\n\n{}", header, headers.join("\n"));
    }

    fn home(&self, header: &str) -> TerminalCursor<Self> {
        let mut cursor: TerminalCursor<Self> = TerminalCursor::new(self.clone(), header);

        cursor.push(TerminalOption::from(String::from("Show databases"), SHOW_DATABASES, self.clone()));
        cursor.push(TerminalOption::from(String::from("Select database"), SELECT_DATABASE_PANEL, self.clone()));

        if self.data_base.is_some() {
            cursor.push(TerminalOption::from(String::from("Show collections"), SHOW_COLLECTIONS, self.clone()));
            cursor.push(TerminalOption::from(String::from("Select collection"), SELECT_COLLECTION_PANEL, self.clone()));
        }

        if self.collection.is_some() {
            cursor.push(TerminalOption::from(String::from("Show elements"), SHOW_ELEMENTS, self.clone()));
            cursor.push(TerminalOption::from(String::from("Select element"), SELECT_ELEMENTS_PANEL, self.clone()));
        }

        if self.element.is_some() {
            cursor.push(TerminalOption::from(String::from("Show selected"), SHOW_SELECTED, self.clone()));
        }

        cursor
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

        self.home(&self.default_header())
    }


    async fn show_collections(&self) -> TerminalCursor<Self> {
        if let Some(error) = self.verify_database() {
            return error;
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
        if let Some(error) = self.verify_database() {
            return error;
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

        self.home(&self.default_header())
    }


    async fn show_elements(&self) -> TerminalCursor<Self> {
        if let Some(error) = self.verify_collection() {
            return error;
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
        if let Some(error) = self.verify_collection() {
            return error;
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

        self.home(&self.default_header())
    }

    async fn show_selected(&self) -> TerminalCursor<Self> {
        if let Some(error) = self.verify_element() {
            return error;
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
            return self.home(&self.default_header());
        }

        let mut fragments = args.get(0).unwrap().split(">").map(|f| String::from(f)).collect::<Vec<String>>();
        let first = String::from(fragments.remove(0).trim());

        if fragments.len() == 0 {
            return self.home(&self.default_header());
        }

        if first.is_empty() || first == "*" {
            return self.translate_path(first, fragments).await;
        }

        return self.home(&self.default_header());
    }

    async fn translate_path(&mut self, first: String, fragments: Vec<String>) -> TerminalCursor<Self> {
        if first == "*" {
            return self.translate_query_path(fragments, false).await;
        }

        return self.translate_query_path(fragments, true).await;
    }

    async fn translate_query_path(&mut self, fragments: Vec<String>, sw_relative: bool) -> TerminalCursor<Self> {
        let (fragments, result) = self.translate_query_path_database(fragments, sw_relative).await;
        if result.is_some() {
            return result.unwrap();
        }

        let (fragments, result) = self.translate_query_path_collection(fragments, sw_relative).await;
        if result.is_some() {
            return result.unwrap();
        }

        let (fragments, result) = self.translate_query_path_elements(fragments, sw_relative).await;
        if result.is_some() {
            return result.unwrap();
        }

        return self.translate_query_path_all(fragments, sw_relative).await;
    }


    async fn translate_query_path_database(&mut self, mut fragments: Vec<String>, sw_relative: bool) -> (Vec<String>, Option<TerminalCursor<Self>>) {
        let update = !sw_relative || (sw_relative && self.data_base.is_none()) && fragments.len() != 0;
        if update {
            let step = String::from(fragments.remove(0).trim());
            self.data_base = Some(step);
        }
        (fragments, None)
    }

    async fn translate_query_path_collection(&mut self, mut fragments: Vec<String>, sw_relative: bool) -> (Vec<String>, Option<TerminalCursor<Self>>) {
        let update = !sw_relative || (sw_relative && self.collection.is_none()) && fragments.len() != 0;
        let evalue =  !sw_relative || (sw_relative && fragments.len() == 0) && self.data_base.is_some();
        if update {
            let step = String::from(fragments.remove(0).trim());
            self.collection = Some(step);
        } else if evalue {
            let query = DataBaseQuery::from_data_base(self.data_base.clone().unwrap());
            let result: Result<Vec<String>, crate::commons::exception::connect_exception::ConnectException> = self.service.list_collections(query).await;
            if result.is_err() {
                self.reset_database();
                return (fragments, Some(self.home(&self.info_headers("Cannot understand input query."))));
            }
            return (fragments, Some(self.home(&self.default_header())));
        }
        (fragments, None)
    }

    async fn translate_query_path_elements(&mut self, mut fragments: Vec<String>, sw_relative: bool) -> (Vec<String>, Option<TerminalCursor<Self>>) {
        let update = !sw_relative || (sw_relative && self.element.is_none()) && fragments.len() != 0;
        let evalue =  !sw_relative || (sw_relative && fragments.len() == 0) && self.data_base.is_some() && self.collection.is_some();
        if update {
            let result = String::from(fragments.remove(0).trim());
            let step = String::from(result)
                .split(",")
                .map(|id| String::from(id.trim()))
                .collect::<Vec<String>>();
            self.element = Some(step);
        } else if evalue {
            let query = DataBaseQuery::from(self.data_base.clone().unwrap(), self.collection.clone().unwrap());
            let result: Result<Vec<String>, crate::commons::exception::connect_exception::ConnectException> = self.service.find_all_lite(query).await;
            if result.is_err() {
                self.reset_collection();
                return (fragments, Some(self.home(&self.info_headers("Cannot understand input query."))));
            }
            return (fragments, Some(self.home(&self.default_header())));
        }
        (fragments, None)
    }

    async fn translate_query_path_all(&mut self, fragments: Vec<String>, sw_relative: bool) -> TerminalCursor<Self> {
        if sw_relative && fragments.len() != 0 || self.data_base.is_none() || self.collection.is_none() || self.element.is_none() {
            self.reset_database();
            return self.home(&self.info_headers("Cannot understand input query."));
        }

        let filter = FilterElement::from_id_chain_collection(self.element.clone().unwrap());
        let query = DataBaseQuery::from_filter(self.data_base.clone().unwrap(), self.collection.clone().unwrap(), filter);
        
        let result = self.service.find(query).await;
        if result.is_err() {
            self.reset_element();
            return self.home(&self.info_headers("Cannot understand input query."));
        }

        self.home(&self.default_header())
    }
    
    fn verify_element(&self) -> Option<TerminalCursor<Self>> {
        if self.element.is_none() {
            let header = self.info_headers("No element selected:");
            return Some(self.home(&header));
        }

        self.verify_collection()
    }

    fn verify_collection(&self) -> Option<TerminalCursor<Self>> {
        if self.collection.is_none() {
            let header = self.info_headers("No collection selected:");
            return Some(self.home(&header));
        }

        self.verify_database()
    }

    fn verify_database(&self) -> Option<TerminalCursor<Self>> {
        if self.data_base.is_none() {
            let header = self.info_headers("No data base selected:");
            return Some(self.home(&header));
        }

        None
    }

    fn reset_database(&mut self) {
        self.data_base = None;
        self.reset_collection();
    }

    fn reset_collection(&mut self) {
        self.collection = None;
        self.reset_element();
    }

    fn reset_element(&mut self) {
        self.element = None
    }

}