#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20241115_000002_classes;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // inject-below (do not remove this comment)
            Box::new(m20241115_000002_classes::Migration),
            Box::new(m20220101_000001_users::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
