use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(FileType::Enum)
                    .values([FileType::File, FileType::Directory])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(File::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(File::Name).string().not_null())
                    .col(
                        ColumnDef::new(File::FileType)
                            .enumeration(FileType::Enum, [FileType::Directory, FileType::File])
                            .not_null(),
                    )
                    .col(ColumnDef::new(File::ParentId).uuid().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(FileType::Enum).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum File {
    Table,
    Id,
    Name,
    FileType,
    ParentId,
}

#[derive(Iden)]
enum FileType {
    #[iden = "file_type"]
    Enum,
    #[iden = "Directory"]
    Directory,
    #[iden = "File"]
    File,
}
