use sqlite;
use sqlite::{Connection, open, Row};
use std::collections::VecDeque;

static DB_PATH: &str = "C:\\Users\\Jxson\\jutils.ui\\data.db";

pub struct Database
{
    pub(self) batch_trans: bool,
    pub(self) rows_updated: u32,
    pub(self) db_location: String,

    pub(self) command_prio: usize,
    pub(self) command_queue: VecDeque<String>

}

impl Database
{
    pub(crate) fn new(path_to_db: String) -> Self {
        Self {
            batch_trans: false,
            rows_updated: 0,
            db_location: (path_to_db.to_owned()).to_owned(),
            command_queue: VecDeque::new(),
            command_prio: 0
        }
    }

    /**
    * Vulnerable to sql-injections...
    */
    pub fn query(&self, query: &str) -> Option<sqlite::Result<Row>>
    {
        let connection: Connection = open(":memory:").unwrap();
        let mut prepared_statement = connection.prepare(query).unwrap();

        prepared_statement.iter().next()

    }

    pub fn execute(&mut self, query: &str) -> u32
    {
        if self.is_using_batch_trans() == true
        {
            self.command_queue.insert(self.command_prio,query.to_owned());
            self.command_prio+=1;
            return 0
        }
        let connection: Connection = open(DB_PATH).unwrap();
        match connection.execute(query) {
            Ok(_) =>
                {
                    println!("{} rows updated", self.get_rows_updated());
                    1
                }
            Err(_) => 0,
        }
    }

    pub fn use_batch(&mut self, batching: bool)
    {
       self.batch_trans = batching
    }



    pub fn commit(&mut self) -> u32
    {
        let connection: Connection = open(DB_PATH).unwrap();

        self.command_queue.retain(| current| {
            match connection.execute(current) {
                Ok(_) => self.rows_updated+=1,
                Err(_) => ()
            };
            true
        });

        println!("{} overall rows updated", self.get_rows_updated());
        self.use_batch(false);
        self.rows_updated
    }

    pub fn get_rows_updated(&self) -> u32 {
        self.rows_updated
    }
    fn is_using_batch_trans(&self) -> bool {
        self.batch_trans
    }
}