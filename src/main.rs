
#[path="configuration\\settings.rs"]
mod settings;
mod templates;
#[path="viewmodels\\get_data.rs"]
mod get_data;

use settings::Settings;
use templates::{IndexTemplate, WeatherTemplate};
// use data;

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref SETTINGS: Settings = match Settings::new() {
        Some(s) => s,
        _ => {
            Settings::from_str("").unwrap()
        }
    };
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_main))
        // .route("/weather", get(get_data::get_weather))
        .route("/weather", get(handle_weather))
        .route("/_assets/*path", get(handle_assets));

    let addr: std::net::SocketAddr = format!("{}:{}", SETTINGS.ip, SETTINGS.port)
        .parse()
        .unwrap();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening to: {}", addr);
    axum::serve(listener, app).await.unwrap();
}

static THEME_CSS: &str = include_str!("../assets/theme.css");
static FAVICON: &str = include_str!("../assets/favicon.svg");

async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "theme.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, THEME_CSS)
    } else if path == "favicon.svg" {
        (StatusCode::OK, headers, FAVICON)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}

async fn handle_main() -> impl IntoResponse {
    let template = IndexTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
async fn handle_weather() -> impl IntoResponse {
    let response = get_data::get_rust_data().await;
    // println!("Data {:?}", response);
    // match response {
    //     Ok(res) => Ok(println!("Success {}", res.status)),
    //     Err(_e) => Err(println!("Err"))
    // }.expect("Error");
    // println!("{:?}",response.status);
    let mut res:Vec<get_data::ToDo> = Vec::new();
    match response {
        Ok(r) => {
            println!("{:?}",r[0].title);
            // title = &r[0].title
            // match r {
            //     Ok(item) => res.push(item)
            // };
            res = r;
        },
        Err(_) => println!("Error")
    };
    let template = WeatherTemplate {
        // name: &res[0].title
        todos: res
    };
    let send_template = template.render().unwrap();
    (StatusCode::OK, Html(send_template).into_response())
}