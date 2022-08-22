use sqlite::State;
use std::env;

fn get_db_location() -> String {
    return match env::var("DB_FILE_NAME") {
        Ok(file) => file,
        _ => "/tmp/default".to_owned(),
    };
}

pub fn get_db_connection() -> sqlite::Connection {
    return sqlite::open(get_db_location()).expect("Could not get db connection");
}

pub fn init_db(connection: &sqlite::Connection) {
    connection
        .execute("CREATE TABLE IF NOT EXISTS links (original_url TEXT, link_id TEXT);")
        .expect("Could not init db");
}

pub fn get_link(link_id: &str, connection: &sqlite::Connection) -> Option<crate::model::Link> {
    let mut statement = connection
        .prepare("SELECT * FROM links WHERE link_id = ?")
        .unwrap();

    statement.bind(1, link_id).unwrap();
    let state = statement.next().unwrap();

    if state == State::Done {
        return None;
    }

    Some(crate::model::Link {
        original_url: statement.read::<String>(0).unwrap(),
        link_id: statement.read::<String>(1).unwrap(),
    })
}

pub fn is_id_used(link_id: &str, connection: &sqlite::Connection) -> bool {
    let mut statement = connection
        .prepare("SELECT COUNT(link_id) FROM links WHERE link_id = ?")
        .unwrap();

    statement.bind(1, link_id).unwrap();

    match statement.read::<i64>(0).unwrap() {
        0 => false,
        _ => true,
    }
}

pub fn insert_link(link: &crate::model::Link, connection: &sqlite::Connection) {
    let mut statement = connection
        .prepare("INSERT INTO links (original_url, link_id) VALUES(?, ?)")
        .unwrap();

    statement.bind(1, &link.original_url[..]).unwrap();
    statement.bind(2, &link.link_id[..]).unwrap();

    statement.next().unwrap();
}
