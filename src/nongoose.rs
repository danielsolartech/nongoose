mod builder;

pub use builder::NongooseBuilder;
use mongodb::{results::InsertOneResult, sync::Database};
#[cfg(feature = "async")]
use tokio::task::spawn_blocking;

use crate::{errors::Result, Schema};

#[derive(Clone)]
pub struct Nongoose {
  builder: NongooseBuilder,
}

impl Nongoose {
  pub fn build(database: Database) -> NongooseBuilder {
    NongooseBuilder {
      database,
      schemas: Vec::new(),
    }
  }

  /// Finds a single document by its `_id` field. `Nongoose.find_by_id(id)` is almost equivalent to `MongoDB.find_one(doc! { "_id": id })`. If
  /// you want to query by a document's `_id`, use `Nongoose.find_by_id()` instead of `Nongoose.find_one()`.
  ///
  /// This function triggers `MongoDB.find_one()`.
  ///
  /// # Example
  /// ```rust,no_run
  /// // Find one `User` document by `_id`
  /// match nongoose.find_by_id::<User>(&ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap())
  /// {
  ///   Ok(user) => {
  ///     println!("User found: {}", user.id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error finding user: {}", error);
  ///   },
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn find_by_id<T>(&self, id: &T::__SchemaId) -> Result<T>
  where
    T: core::fmt::Debug + Schema,
  {
    self.builder.find_by_id_sync(id.clone())
  }

  /// Finds a single document by its `_id` field. `Nongoose.find_by_id(id)` is almost equivalent to `MongoDB.find_one(doc! { "_id": id })`. If
  /// you want to query by a document's `_id`, use `Nongoose.find_by_id()` instead of `Nongoose.find_one()`.
  ///
  /// This function triggers `MongoDB.find_one()`.
  ///
  /// # Example
  /// ```rust,no_run
  /// // Find one `User` document by `_id`
  /// match nongoose.find_by_id::<User>(&ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap()).await
  /// {
  ///   Ok(user) => {
  ///     println!("User found: {}", user.id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error finding user: {}", error);
  ///   },
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn find_by_id<T>(&self, id: &T::__SchemaId) -> Result<T>
  where
    T: core::fmt::Debug + Schema + 'static,
  {
    let builder = self.builder.clone();
    let id = id.clone();

    spawn_blocking(move || builder.find_by_id_sync(id)).await?
  }

  /// Save one document to the database.
  ///
  /// # Example
  /// ```rust,no_run
  /// // Insert one new `User` document
  /// match nongoose.create::<User>(&user) {
  ///   Ok(result) => {
  ///     println!("User saved: {}", result.inserted_id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error saving the user: {}", error);
  ///   }
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn create<T>(&self, data: &T) -> Result<InsertOneResult>
  where
    T: Schema + Clone,
  {
    self.builder.create_sync(data.clone())
  }

  /// Save one document to the database.
  ///
  /// # Example
  /// ```rust,no_run
  /// // Insert one new `User` document
  /// match nongoose.create::<User>(&user).await {
  ///   Ok(result) => {
  ///     println!("User saved: {}", result.inserted_id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error saving the user: {}", error);
  ///   }
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn create<T>(&self, data: &T) -> Result<InsertOneResult>
  where
    T: Schema + Clone + Send + 'static,
  {
    let builder = self.builder.clone();
    let data = data.clone();

    spawn_blocking(move || builder.create_sync(data)).await?
  }
}
