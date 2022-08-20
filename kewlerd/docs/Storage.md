# Storage

**Kewler** needs storage to save configurations. The idea is to implement a very simple persistent key-value storage.

## Version 0.1.0

Features:

- simple interface to CRUD data and create a database

### Specification

Interface
```rust
/// Database management
fn create_database(name: &str ) -> Result<()>;
fn load_database(name: &str ) -> Result<Database>;
fn delete_database(name: &str ) -> Result<()>;

/// Data operations implemented for a Database 
fn insert(key: T, value: Z) -> Result<()>;
fn get(key: &T) -> Option<&Z>
fn update(key: &T, value: Z) -> Result<()>;
fn delete(key: &T) -> Result<()>;

```