use crate::DB_POOL;

/// Initalizes a table for a word in case it doesn't exists and adds an URL's
/// score of the current word.
/// WARN: The URL should exist as a PRIMARY KEY in the `sites` table. Call 
/// `db::sites::new_url_record` to make sure it's the case.
pub fn save_word_score(
    url: String,
    word: String,
    score: usize
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    
    conn.execute_batch(&format!("
        BEGIN IMMEDIATE;
        CREATE TABLE IF NOT EXISTS w_{word} (
            url text,
            score int,
            CONSTRAINT url FOREIGN KEY (url) REFERENCES sites(url)
        );
        INSERT OR IGNORE INTO w_{word} (url, score) VALUES ('{url}', 0);
        UPDATE w_{word} SET score = score + {score} WHERE url = '{url}';
        COMMIT;
    "))?;
    Ok(())
}
