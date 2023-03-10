use sea_orm_migration::{prelude::*, sea_orm::ConnectionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        /* ROLE */
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Role::Name)
                            .string()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        // Filling the table
        let values = ["admin", "moderator", "user"];
        for value in values {
            db.execute_unprepared(&format!("INSERT INTO role (name) VALUES ('{value}')"))
                .await?;
        }

        /* USER */
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::Followers)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(User::Following)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(User::Role)
                            .tiny_integer()
                            .not_null()
                            .default(3 as i16),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk__user__to__role")
                            .from_col(User::Role)
                            .to_col(Role::Id)
                            .from_tbl(User::Table)
                            .to_tbl(Role::Table),
                    )
                    .to_owned(),
            )
            .await?;

        /* SUBSCRIBER */
        manager
            .create_table(
                Table::create()
                    .table(Subscriber::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Subscriber::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Subscriber::SubscriberId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Subscriber::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk__user__to__user")
                            .from_col(Subscriber::UserId)
                            .to_col(User::Id)
                            .from_tbl(Subscriber::Table)
                            .to_tbl(User::Table),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk__subscriber__to__user")
                            .from_col(Subscriber::SubscriberId)
                            .to_col(User::Id)
                            .from_tbl(Subscriber::Table)
                            .to_tbl(User::Table),
                    )
                    .primary_key(
                        Index::create()
                            .col(Subscriber::UserId)
                            .col(Subscriber::SubscriberId),
                    )
                    .to_owned(),
            )
            .await?;

        /* POST */
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::RelatedToPost).integer().null())
                    .col(ColumnDef::new(Post::UserId).integer().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::Likes).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Post::Comments)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk__post__to__user")
                            .from_col(Post::UserId)
                            .to_col(User::Id)
                            .from_tbl(Post::Table)
                            .to_tbl(User::Table),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk__post__to__post")
                            .from_col(Post::RelatedToPost)
                            .to_col(Post::Id)
                            .from_tbl(Post::Table)
                            .to_tbl(Post::Table),
                    )
                    .to_owned(),
            )
            .await?;

        /* POST_LIKE */
        manager
            .create_table(
                Table::create()
                    .table(PostLike::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PostLike::LikeUserId).integer().not_null())
                    .col(ColumnDef::new(PostLike::PostId).integer().not_null())
                    .col(
                        ColumnDef::new(PostLike::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk__post_like__to__user")
                            .from_col(PostLike::LikeUserId)
                            .to_col(User::Id)
                            .from_tbl(PostLike::Table)
                            .to_tbl(User::Table),
                    )
                    .primary_key(
                        Index::create()
                            .col(PostLike::PostId)
                            .col(PostLike::LikeUserId),
                    )
                    .to_owned(),
            )
            .await?;

        /* SESSION */
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::JWT)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Session::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk__session__to__user")
                            .from_col(Session::UserId)
                            .to_col(User::Id)
                            .from_tbl(Session::Table)
                            .to_tbl(User::Table),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(PostLike::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Post::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Session::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Subscriber::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Role::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    Password,
    Followers,
    Following,
    Role,
    CreatedAt,
}

#[derive(Iden)]
enum Subscriber {
    Table,
    UserId,
    SubscriberId,
    CreatedAt,
}

#[derive(Iden)]
enum Role {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Post {
    Table,
    Id,
    RelatedToPost,
    UserId,
    Text,
    Likes,
    Comments,
    CreatedAt,
}

#[derive(Iden)]
enum PostLike {
    Table,
    PostId,
    LikeUserId,
    CreatedAt,
}

#[derive(Iden)]
enum Session {
    Table,
    JWT,
    UserId,
    CreatedAt,
}
