use anyhow::Result;
use key_value::{key_value::Error, log_error, Impl, ImplStore};
use rusqlite::Connection;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::task;
use wit_bindgen_wasmtime::async_trait;

#[derive(Clone)]
pub enum DatabaseLocation {
    InMemory,
    Path(PathBuf),
}

#[derive(Clone)]
pub struct KeyValueSqlite {
    location: DatabaseLocation,
    connection: Option<Arc<Mutex<Connection>>>,
}

impl KeyValueSqlite {
    pub fn new(location: DatabaseLocation) -> Self {
        Self {
            location,
            connection: None,
        }
    }
}

#[async_trait]
impl Impl for KeyValueSqlite {
    async fn open(&mut self, name: &str) -> Result<Box<dyn ImplStore>, Error> {
        if self.connection.is_none() {
            task::block_in_place(|| {
                let connection = match &self.location {
                    DatabaseLocation::InMemory => {
                        println!("Using in-memory key-value store");
                        Connection::open_in_memory()
                    }
                    DatabaseLocation::Path(path) => {
                        println!("Using {} for key-value store", path.display());
                        Connection::open(path)
                    }
                }
                .map_err(log_error)?;

                connection
                    .execute(
                        "CREATE TABLE IF NOT EXISTS spin_key_value (
                           store TEXT NOT NULL,
                           key   TEXT NOT NULL,
                           value BLOB NOT NULL,

                           PRIMARY KEY (store, key)
                        )",
                        (),
                    )
                    .map_err(log_error)?;

                self.connection = Some(Arc::new(Mutex::new(connection)));

                Ok(())
            })?;
        }

        Ok(Box::new(SqliteStore {
            name: name.to_owned(),
            connection: self.connection.as_ref().unwrap().clone(),
        }))
    }

    fn clone(&self) -> Box<dyn Impl> {
        Box::new(Clone::clone(self))
    }
}

struct SqliteStore {
    name: String,
    connection: Arc<Mutex<Connection>>,
}

#[async_trait]
impl ImplStore for SqliteStore {
    async fn get(&self, key: &str) -> Result<Vec<u8>, Error> {
        task::block_in_place(|| {
            self.connection
                .lock()
                .unwrap()
                .prepare_cached("SELECT value FROM spin_key_value WHERE store=$1 AND key=$2")
                .map_err(log_error)?
                .query_map([&self.name, key], |row| row.get(0))
                .map_err(log_error)?
                .next()
                .ok_or(Error::NoSuchKey)?
                .map_err(log_error)
        })
    }

    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        task::block_in_place(|| {
            self.connection
                .lock()
                .unwrap()
                .prepare_cached(
                    "INSERT INTO spin_key_value (store, key, value) VALUES ($1, $2, $3)
                     ON CONFLICT(store, key) DO UPDATE SET value=$3",
                )
                .map_err(log_error)?
                .execute(rusqlite::params![&self.name, key, value])
                .map_err(log_error)
                .map(drop)
        })
    }

    async fn delete(&self, key: &str) -> Result<(), Error> {
        task::block_in_place(|| {
            self.connection
                .lock()
                .unwrap()
                .prepare_cached("DELETE FROM spin_key_value WHERE store=$1 AND key=$2")
                .map_err(log_error)?
                .execute([&self.name, key])
                .map_err(log_error)
                .map(drop)
        })
    }

    async fn exists(&self, key: &str) -> Result<bool, Error> {
        match self.get(key).await {
            Ok(_) => Ok(true),
            Err(Error::NoSuchKey) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get_keys(&self) -> Result<Vec<String>, Error> {
        task::block_in_place(|| {
            self.connection
                .lock()
                .unwrap()
                .prepare_cached("SELECT key FROM spin_key_value WHERE store=$1")
                .map_err(log_error)?
                .query_map([&self.name], |row| row.get(0))
                .map_err(log_error)?
                .map(|r| r.map_err(log_error))
                .collect()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use key_value::{KeyValue, KeyValueDispatch};

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn all() -> Result<()> {
        let mut kv = KeyValueDispatch::new(
            [(
                "default".to_owned(),
                Box::new(KeyValueSqlite::new(DatabaseLocation::InMemory)) as Box<dyn Impl>,
            )]
            .into_iter()
            .collect(),
        );

        kv.allowed_stores = ["default", "foo"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();

        assert!(matches!(
            kv.exists(42, "bar").await,
            Err(Error::InvalidStore)
        ));

        assert!(matches!(kv.open("foo").await, Err(Error::NoSuchStore)));
        assert!(matches!(
            kv.open("forbidden").await,
            Err(Error::AccessDenied)
        ));

        let store = kv.open("default").await?;

        assert!(!kv.exists(store, "bar").await?);

        assert!(matches!(kv.get(store, "bar").await, Err(Error::NoSuchKey)));

        kv.set(store, "bar", b"baz").await?;

        assert!(kv.exists(store, "bar").await?);

        assert_eq!(b"baz" as &[_], &kv.get(store, "bar").await?);

        kv.set(store, "bar", b"wow").await?;

        assert_eq!(b"wow" as &[_], &kv.get(store, "bar").await?);

        assert_eq!(&["bar".to_owned()] as &[_], &kv.get_keys(store).await?);

        kv.delete(store, "bar").await?;

        assert!(!kv.exists(store, "bar").await?);

        assert_eq!(&[] as &[String], &kv.get_keys(store).await?);

        assert!(matches!(kv.get(store, "bar").await, Err(Error::NoSuchKey)));

        kv.close(store).await;

        assert!(matches!(
            kv.exists(store, "bar").await,
            Err(Error::InvalidStore)
        ));

        Ok(())
    }
}
