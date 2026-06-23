// SQLiteデータベース接続の管理とスキーマ初期化を担当するモジュール。
use std::fs;
use std::path::Path;

use rusqlite::{Connection, OpenFlags};

use crate::error::AppResult;

mod imports;
mod schema;

pub struct Database {
    connection: Connection,
}

impl Database {
    // 指定パスのSQLite DBを開き、必要なPRAGMAを初期化する。
    pub fn open(path: &Path) -> AppResult<Self> {
        // ディレクトリが存在しない場合は作成する。
        // path.parent()で親ディレクトリを取得("/a/b/c/db.sqlite" -> "/a/b/c"、"/"の場合はNone)
        // Some()はOption<T>型の値が存在する場合、Noneは存在しない場合を表す
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let flags = OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE;
        let connection = Connection::open_with_flags(path, flags)?;
        connection.pragma_update(None, "journal_mode", &"WAL")?;
        connection.pragma_update(None, "synchronous", &"NORMAL")?;
        connection.pragma_update(None, "foreign_keys", &"ON")?;

        Ok(Self { connection })
    }

    // 読み取り専用の接続参照を取得する。
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    // 書き換え可能な接続参照を取得する。
    #[allow(dead_code)]
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }

    // スキーマ定義を1トランザクションで適用し、途中失敗時はロールバックする。
    pub fn initialize_schema(&mut self) -> AppResult<()> {
        let transaction = self.connection.transaction()?;

        for &(name, ref statement) in schema::statements() {
            transaction.execute_batch(statement)?;
            tracing::trace!(statement = %name, "schema statement applied");
        }

        transaction.commit()?;
        Ok(())
    }
}
