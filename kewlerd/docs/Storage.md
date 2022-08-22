# Storage

**Kewler** needs storage to save configurations. The idea is to implement a very simple persistent key-value storage.

## Version 0.1.0

Features:

- simple interface to CRUD data and create a database

### Specification

Use `KvStore` to manage `Database`s which essentially are `HashMap`.

```rust
/// KvStore 
fn save() -> Result<()>; // serialize the KvStore and save it to file
fn load_or_init(path: &PathBuf ) -> Result<KvStore>; // attempts to load the KvStore from file (prior serialize to BSON) or will create an empty
fn add_database(name: &str) -> Option<()>; // attempts to add a database by name 
fn drop_database(name: &str) -> Option<()>; // attempts to drop a database by name

/// Database
fn insert(key: T, value: Z) -> Result<()>;
fn get(key: &T) -> Option<&Z>
fn update(key: &T, value: Z) -> Result<()>;
fn delete(key: &T) -> Result<()>;
```
