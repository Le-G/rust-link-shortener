mod db;
mod handlers;
mod model;

use warp::Filter;

fn json_body() -> impl Filter<Extract = (handlers::Request,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_db(
) -> impl Filter<Extract = (sqlite::Connection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db::get_db_connection())
}

#[tokio::main]
async fn main() {
    let connection = db::get_db_connection();
    db::init_db(&connection);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    let get_link = warp::path!("urls" / String)
        .and(with_db())
        .and_then(handlers::get_link);

    let add_link = warp::path!("urls")
        .and(json_body())
        .and(with_db())
        .and_then(handlers::shorten_url);


    let options = warp::options().and(warp::path!("urls")
        .and_then(handlers::options));

    let get = warp::get().and(get_link);
    let post = warp::post().and(add_link);
    let routes = get.or(post).or(options);

    println!("Preparing to listen");
    warp::serve(routes.with(cors)).run(([127, 0, 0, 1], 4242)).await;
}
