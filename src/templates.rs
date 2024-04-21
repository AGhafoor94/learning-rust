#[path="viewmodels/get_data.rs"]
mod get_data;

#[derive(askama::Template)]
#[template(path = "../public/index.html")]
pub struct IndexTemplate {}
#[derive(askama::Template)]
#[template(path = "../public/data/index.html")]
// pub struct WeatherTemplate<'a> {
//     // pub name: &'a str
//     pub todos: Vec<get_data::ToDo>
// }
pub struct ToDoTemplate {
    // pub name: &'a str
    pub todos: Vec<crate::get_data::ToDo>
}
#[derive(askama::Template)]
#[template(path = "../public/data/todo.html")]
pub struct ToDoItemTemplate {
    pub todo: crate::get_data::ToDo
}
#[derive(askama::Template)]
#[template(path="../public/error/error.html")]
pub struct ErrorTemplate {
    pub error: String
}