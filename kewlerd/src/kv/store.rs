use super::database::Database;

pub struct KvStore<V> {
    pub database: Vec<Database<V>>
}