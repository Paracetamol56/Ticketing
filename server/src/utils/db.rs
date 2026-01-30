pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tickets (
            uuid TEXT PRIMARY KEY,
            number INTEGER NOT NULL,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            message TEXT NOT NULL,
            note TEXT,
            status TEXT NOT NULL CHECK (status IN ('open', 'closed', 'pending')),
            created_at TEXT NOT NULL,
            updated_at TEXT,
            closed_at TEXT
        );",
        [],
    )?;
    Ok(())
}
