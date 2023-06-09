use ::entity::{file, file::Entity as File, sea_orm_active_enums::FileType};
use migration::{Migrator, MigratorTrait};
use sea_orm::*;

use uuid::Uuid;

#[tokio::main]
async fn main() {
    let conn: DatabaseConnection = Database::connect(ConnectOptions::new(
        "postgres://postgres:changeme@localhost:5432/enum_test".to_string(),
    ))
    .await
    .unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let to_insert = vec![
        file::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set("file1".to_owned()),
            file_type: Set(FileType::File),
            parent_id: Set(None),
        },
        file::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set("file2".to_owned()),
            file_type: Set(FileType::Directory),
            parent_id: Set(None),
        },
    ];

    let ids = to_insert
        .iter()
        .cloned()
        .map(|f| f.id.unwrap().into())
        .collect::<Vec<_>>();
    File::insert_many(to_insert).exec(&conn).await.unwrap();

    let files = File::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                WITH RECURSIVE file_hierarchy AS (
                    SELECT "id", "name", CAST(file_type AS file_type), parent_id
                    FROM public.file
                    WHERE id IN ($1)
    
                    UNION ALL
    
                    SELECT f.id, f.name, CAST(f.file_type AS file_type), f.parent_id
                    FROM public.file f
                    INNER JOIN file_hierarchy fh ON f.parent_id = fh.id
                )
                SELECT *
                FROM file_hierarchy;
            "#,
            ids,
        ))
        .all(&conn)
        .await
        .unwrap();
    dbg!(files);
}
