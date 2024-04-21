
#[path="configuration\\settings.rs"]
mod settings;
mod templates;
#[path="viewmodels\\database_connect.rs"]
mod database_connect;
#[path="viewmodels\\get_data.rs"]
mod get_data;
// use uuid::Uuid;
use settings::Settings;
use templates::{IndexTemplate, ToDoTemplate, ToDoItemTemplate, ErrorTemplate};
// use data;
use get_data::{ ToDo };
use askama::Template;
use axum::{
    extract::{Path, Query},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
// macro_rules! env {
//     ($name:expr $(,)?) => { ... };
//     ($name:expr, $error_msg:expr $(,)?) => { ... };
// }
static THEME_CSS: &str = include_str!("../assets/theme.css");
static FAVICON: &str = include_str!("../assets/favicon.svg");
#[derive(serde::Deserialize, Debug)]
struct QueryParams {
    user: i16,
    id: i16,
}
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
    // let db_string = env!("../env");
    let app = Router::new()
        .route("/", get(handle_main))
        // .route("/weather", get(get_data::get_weather))
        .route("/_assets/*path", get(handle_assets))
        .route("/todo", get(handle_todo))
        .route("/todo/:id", get(get_todo_item))
        .route("/todo_query", get(get_query_params));

    let addr: std::net::SocketAddr = format!("{}:{}", SETTINGS.ip, SETTINGS.port)
        .parse()
        .unwrap();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening to: {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers:HeaderMap = HeaderMap::new();
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
async fn handle_todo() -> impl IntoResponse {
    let response = get_data::get_all_users().await;
    let mut res:Vec<ToDo> = Vec::new();
    match response {
        Ok(r) => {
            // println!("{:?}",r.len());
            // title = &r[0].title
            // match r {
            //     Ok(item) => res.push(item)
            // };
            res = r;
        },
        Err(_) => println!("Error")
    };
    let template:ToDoTemplate = ToDoTemplate {
        // name: &res[0].title
        todos: res
    };
    // vec_len = &response.len();
    let send_template:String = template.render().unwrap();
    (StatusCode::OK, Html(send_template).into_response())
}
async fn get_todo_item(Path(id): Path<String>) -> impl IntoResponse {
    // let id = params.get("id").unwrap();
    // println!("{:?}",id.to_string());
    // println!("{:?}", id);
    let response:Result<ToDo, reqwest::Error> = get_data::get_rust_data_single(&id).await;
    // let mut res:Vec<get_data::ToDo> = Vec::new();
    // let mut todo_item: get_data::ToDo = get_data::ToDo {
    //     id:1,
    //     title: "Placeholder".to_owned(),
    //     user_id: 1,
    //     completed:false
    // };
    println!("{:?}",response);
    match response {
        Ok(r) => {
            let template:ToDoItemTemplate = ToDoItemTemplate {
                todo: r
            };
            // vec_len = &response.len();
            let send_template:String = template.render().unwrap();
            (StatusCode::OK, Html(send_template).into_response())
        }
        Err(e) => {
            let error_template: ErrorTemplate = ErrorTemplate {
                error: e.to_string()
            };
            let load_template: String = error_template.render().unwrap();
            (StatusCode::NOT_FOUND,Html(load_template).into_response())
        }
    }
}
async fn get_query_params(Query(query):Query<QueryParams>) -> impl IntoResponse {
    println!("{:?}", query);
    let response:Result<ToDo, reqwest::Error> = get_data::get_rust_data_single_user(query.user, query.id).await;
    // let mut res:Vec<get_data::ToDo> = Vec::new();
    // let mut todo_item: get_data::ToDo = get_data::ToDo {
    //     id:1,
    //     title: "Placeholder".to_owned(),
    //     user_id: 1,
    //     completed:false
    // };
    println!("{:?}",response);
    match response {
        Ok(r) => {
            let template:ToDoItemTemplate = ToDoItemTemplate {
                todo: r
            };
            // vec_len = &response.len();
            let send_template:String = template.render().unwrap();
            (StatusCode::OK, Html(send_template).into_response())
        }
        Err(e) => {
            let error_template: ErrorTemplate = ErrorTemplate {
                error: e.to_string()
            };
            let load_template: String = error_template.render().unwrap();
            (StatusCode::NOT_FOUND,Html(load_template).into_response())
        }
    }
}