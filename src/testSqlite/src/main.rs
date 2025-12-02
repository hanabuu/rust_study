use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // ファイルDBに接続（なければ作成）。メモリDBなら ":memory:"。
    let conn = Connection::open("app.db")?;

    // テーブル作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            age     INTEGER NOT NULL
        )",
        [],
    )?;

    // INSERT（プリペアドステートメント＋パラメータ）
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params!["Taro", 28],
    )?;

    // 単一行SELECT（query_row）
    let (name, age): (String, i64) = conn.query_row(
        "SELECT name, age FROM users WHERE id = ?1",
        params![1],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    println!("id=1 -> {name}, {age}");

    // 複数行SELECT（prepare + query_map）
    let mut stmt = conn.prepare("SELECT id, name, age FROM users WHERE age >= ?1")?;
    let rows = stmt.query_map(params![20], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for u in rows {
        println!("{:?}", u?);
    }

    Ok(())
}

#[derive(Debug)]
struct User {
    id: i64,
    name: String,
    age: i64,
}