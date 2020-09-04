// use std::env;
use sqlx::sqlite::SqliteConnection;
use sqlx::Connection;
use uuid::Uuid;

// no traits are needed
#[derive(sqlx::FromRow, Debug)]
struct Entry {
    id: i64,
    uuid: String,
    title: Option<String>,
    description: Option<String>,
    username: Option<String>,
    notes: Option<String>,
    url: Option<String>,
    entry_data: Option<String>,
    certificate: Option<String>,
    ssh_key: Option<String>,
    other_binary_type: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnection::connect("sqlite://test.sqlite?mode=memory").await?;

    let rows = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS entries (
            id INTEGER NOT NULL PRIMARY KEY,
            uuid TEXT NOT NULL UNIQUE,
            title TEXT,
            description TEXT,
            username TEXT,
            notes TEXT,
            url TEXT,
            entry_data TEXT,
            certificate BLOB,
            ssh_key BLOB,
            other_binary_type BLOB
        );
        ",
    )
    .execute(&mut conn)
    .await?;

    println!("Table Created: {:?}", rows);

    let inserted_rows = sqlx::query(
        "
        INSERT INTO entries (uuid, title, description)
        VALUES( ?, 'title' , 'This is a description');
        ",
    )
    .bind(format!("{}", Uuid::new_v4()))
    .execute(&mut conn)
    .await?;

    println!("Inserted Rows: {:?}", inserted_rows);

    let first_row: Entry = sqlx::query_as::<_, Entry>(
        "
        SELECT * FROM entries;
        ",
    )
    .fetch_one(&mut conn)
    .await?;

    println!("Selected Items: {:?}", first_row);

    Ok(())
}
