pub struct ConnectionData {
    connection: String
}

impl ConnectionData {

    pub fn new(connection: String) -> ConnectionData {
        ConnectionData {
            connection
        }
    }

    pub fn connection(self) -> String {
        return self.connection;
    }

}