use crate::infrastructure::{manager::terminal::terminal_cursor::TerminalCursor, repository::i_db_repository::IDBRepository};

use super::manager_database::ManagerDatabase;

impl <T: IDBRepository> ManagerDatabase<T> {
    
    pub async fn translate_path(&mut self, first: String, fragments: Vec<String>) -> TerminalCursor<Self> {
        if first == "*" {
            return self.translate_query_path(fragments, false).await;
        }

        return self.translate_query_path(fragments, true).await;
    }

    async fn translate_query_path(&mut self, fragments: Vec<String>, sw_relative: bool) -> TerminalCursor<Self> {
        let (fragment, fragments) = self.first_fragment(fragments);
        let result = self.translate_query_path_database(fragment, fragments.clone(), sw_relative).await;
        if result.is_some() {
            return result.unwrap();
        }

        let (fragment, fragments) = self.first_fragment(fragments);
        let result = self.translate_query_path_collection(fragment, fragments.clone(), sw_relative).await;
        if result.is_some() {
            return result.unwrap();
        }

        let (fragment, fragments) = self.first_fragment(fragments);
        let result = self.translate_query_path_elements(fragment, fragments.clone(), sw_relative).await;
        if result.is_some() {
            return result.unwrap();
        }

        let (fragment, _) = self.first_fragment(fragments);
        if fragment.is_some() {
            return self.home(&self.info_headers("Cannot undertand query extra parameters detected."));
        }

        return self.home_headers();
    }

    fn first_fragment(&self, mut fragments: Vec<String>) -> (Option<String>, Vec<String>) {
        if fragments.len() != 0 {
            return (Some(String::from(fragments.remove(0).trim())), fragments);
        }
        return (None, fragments);
    }

    async fn translate_query_path_database(&mut self, o_fragment: Option<String>, fragments: Vec<String>, sw_relative: bool) -> Option<TerminalCursor<Self>> {
        if o_fragment.is_none() {
            self.reset_database();
            return Some(self.home_headers());
        }

        let fragment = o_fragment.clone().unwrap();

        if let Ok(result) = self.translate_query_path_back(fragment.clone(), fragments.clone()) {
            return result; 
        }

        if !sw_relative || (sw_relative && self.data_base.is_none()) {
            self.data_base = Some(fragment);
            let result = self.valide_data_base_connection().await;
            if result.is_err() {
                self.reset_database();
                return Some(self.home(&self.info_headers(&result.unwrap_err().message())));
            }
            return None;
        }
        
        self.translate_query_path_collection(o_fragment, fragments, sw_relative).await
    }

    async fn translate_query_path_collection(&mut self, o_fragment: Option<String>, fragments: Vec<String>, sw_relative: bool) -> Option<TerminalCursor<Self>> {        
        if o_fragment.is_none() {
            return Some(self.home_headers());
        }
        
        let fragment = o_fragment.clone().unwrap();

        if let Ok(result) = self.translate_query_path_back(fragment.clone(), fragments.clone()) {
            return result; 
        }
        
        if !sw_relative || (sw_relative && self.collection.is_none()) {
            self.collection = Some(fragment);
            let result = self.valide_collection_connection().await;
            if result.is_err() {
                self.reset_collection();
                return Some(self.home(&self.info_headers(&result.unwrap_err().message())));
            }
            return None;
        }

        self.translate_query_path_elements(o_fragment, fragments, sw_relative).await
    }

    async fn translate_query_path_elements(&mut self, o_fragment: Option<String>, fragments: Vec<String>, sw_relative: bool) -> Option<TerminalCursor<Self>> {
        if o_fragment.is_none() {
            return Some(self.home_headers());
        }

        let fragment = o_fragment.unwrap();

        if let Ok(result) = self.translate_query_path_back(fragment.clone(), fragments) {
            return result; 
        }

        if !sw_relative || (sw_relative && self.element.is_none()) {
            let step = String::from(fragment)
                .split(",")
                .map(|id| String::from(id.trim()))
                .collect::<Vec<String>>();
            self.element = Some(step);
            let result = self.valide_element_connection().await;
            if result.is_err() {
                self.reset_collection();
                return Some(self.home(&self.info_headers(&result.unwrap_err().message())));
            }
        }

        None
    }

    fn translate_query_path_back(&mut self, fragment: String, fragments: Vec<String>) -> Result<Option<TerminalCursor<Self>>, ()> {
        if fragment != ".." {
            return Err(()); 
        }

        let mut result = Ok(None);

        if self.element.is_some() {
            self.reset_element();
            if fragments.len() == 0 {
                result = Ok(Some(self.home_headers()));
            }
            return result;    
        }

        if self.collection.is_some() {
            self.reset_collection();
            if fragments.len() == 0 {
                result = Ok(Some(self.home_headers()));
            }
            return result;    
        }

        if self.data_base.is_some() {
            self.reset_database();
            if fragments.len() == 0 {
                result = Ok(Some(self.home_headers()));
            }
            return result;    
        }
        
        return result; 
    }

}