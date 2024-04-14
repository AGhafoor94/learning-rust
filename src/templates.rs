#[path="viewmodels\\get_data.rs"]
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
pub struct WeatherTemplate {
    // pub name: &'a str
    pub todos: Vec<crate::get_data::ToDo>
}