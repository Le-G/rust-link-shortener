use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use warp::http;

#[derive(Deserialize)]
pub struct Request {
    url: String,
}

fn get_unused_prefix(connection: &sqlite::Connection) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    if crate::db::is_id_used(s.as_str(), connection) == true {
        return get_unused_prefix(connection);
    }

    return s;
}

pub async fn shorten_url(
    request: Request,
    connection: sqlite::Connection,
) -> Result<impl warp::Reply, warp::Rejection> {
    let link = crate::model::Link {
        link_id: get_unused_prefix(&connection).to_owned(),
        original_url: request.url,
    };

    crate::db::insert_link(&link, &connection);

    Ok(warp::reply::with_status(
        warp::reply::json(&link),
        http::StatusCode::CREATED,
    ))
}

pub async fn get_link(
    link_id: String,
    connection: sqlite::Connection,
) -> Result<impl warp::Reply, warp::Rejection> {
    match crate::db::get_link(&link_id[..], &connection) {
        None => Ok(warp::reply::with_status(
            warp::reply::json(&{}),
            http::StatusCode::NOT_FOUND,
        )),
        Some(link) => Ok(warp::reply::with_status(
            warp::reply::json(&link),
            http::StatusCode::OK,
        )),
    }
}

pub async fn options(
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        warp::reply::json(&{}),
        http::StatusCode::OK,
    ))
}
