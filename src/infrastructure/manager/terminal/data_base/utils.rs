use crate::{commons::exception::connect_exception::ConnectException, domain::filter::{data_base_query::DataBaseQuery, filter_element::FilterElement}, infrastructure::{manager::terminal::{terminal_cursor::TerminalCursor, terminal_option::TerminalOption}, repository::i_db_repository::IDBRepository}};

use super::manager_database::{self, ManagerDatabase};

impl <T: IDBRepository> ManagerDatabase<T> {

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

    pub fn home(&self, header: &str) -> TerminalCursor<Self> {
        let mut cursor: TerminalCursor<Self> = TerminalCursor::new(self.clone(), header);

        cursor.push(TerminalOption::from(String::from("Show databases"), manager_database::SHOW_DATABASES, self.clone()));
        cursor.push(TerminalOption::from(String::from("Select database"), manager_database::SELECT_DATABASE_PANEL, self.clone()));

        if self.data_base.is_some() {
            cursor.push(TerminalOption::from(String::from("Show collections"), manager_database::SHOW_COLLECTIONS, self.clone()));
            cursor.push(TerminalOption::from(String::from("Select collection"), manager_database::SELECT_COLLECTION_PANEL, self.clone()));
        }

        if self.collection.is_some() {
            cursor.push(TerminalOption::from(String::from("Show elements"), manager_database::SHOW_ELEMENTS, self.clone()));
            cursor.push(TerminalOption::from(String::from("Select element"), manager_database::SELECT_ELEMENTS_PANEL, self.clone()));
        }

        if self.element.is_some() {
            cursor.push(TerminalOption::from(String::from("Show selected"), manager_database::SHOW_SELECTED, self.clone()));
        }

        cursor
    }

    pub fn home_headers(&self) -> TerminalCursor<Self> {
        self.home(&self.default_header())
    }

    pub async fn valide_data_base_connection(&mut self) -> Result<(), ConnectException>  {        
        self.verify_database()?;

        let query = DataBaseQuery::from_data_base(self.data_base.clone().unwrap());
        if !self.service.data_base_exists(query).await? {
            let exception = ConnectException::new(String::from("Data base does not exists."));
            return Err(exception);
        }

        Ok(())
    }

    pub async fn valide_collection_connection(&mut self) -> Result<(), ConnectException> {        
        self.verify_collection()?;

        let query = DataBaseQuery::from(self.data_base.clone().unwrap(), self.collection.clone().unwrap());
        if !self.service.collection_exists(query).await? {
            let exception = ConnectException::new(String::from("Collection does not exists."));
            return Err(exception);
        }
        
        Ok(())
    }

    pub async fn valide_element_connection(&mut self) -> Result<(), ConnectException> {
        self.verify_element()?;

        let filter = FilterElement::from_id_chain_collection(self.element.clone().unwrap());
        let query = DataBaseQuery::from_filter(self.data_base.clone().unwrap(), self.collection.clone().unwrap(), filter);
        
        let _ = self.service.find(query).await?;

        Ok(())
    }
    
    pub fn verify_element(&self) -> Result<(), ConnectException> {
        if self.element.is_none() {
            let exception = ConnectException::new(String::from("No element selected."));
            return Err(exception);
        }
        self.verify_collection()
    }

    pub fn verify_collection(&self) -> Result<(), ConnectException> {
        if self.collection.is_none() {
            let exception = ConnectException::new(String::from("No collection selected."));
            return Err(exception);
        }
        self.verify_database()
    }

    pub fn verify_database(&self) -> Result<(), ConnectException> {
        if self.data_base.is_none() {
            let exception = ConnectException::new(String::from("No data base selected."));
            return Err(exception);
        }
        Ok(())
    }

    pub fn reset_database(&mut self) {
        self.data_base = None;
        self.reset_collection();
    }

    pub fn reset_collection(&mut self) {
        self.collection = None;
        self.reset_element();
    }

    pub fn reset_element(&mut self) {
        self.element = None
    }

}