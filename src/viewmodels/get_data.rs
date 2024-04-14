#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ToDo {
    #[serde(rename = "userId")]
    pub user_id: i32,
    id: Option<i32>,
    pub title: String,
    pub completed: bool
}

// pub async fn get_rust_data() -> Result<Vec<ToDo>, reqwest::Error> {
    
//     match todo_items.is_empty() {
//         true => {
//             let client = reqwest::Client::new();
    
//             let url = format!("https://jsonplaceholder.typicode.com/todos?userId=1");
            
//             let response: Vec<ToDo> = client
//                 .get(url)
//                 .send()
//                 .await?
//                 .json()
//                 .await?;
//             Ok(response)
//         },   // If vector is empty, return this message
//         false => Ok(todo_items) // If vector is not empty, return this message
//     }
    

//     // println!("{:?}",response);
    
//     // Ok(response)
// }
pub async fn get_rust_data() -> Result<Vec<ToDo>, reqwest::Error> {
    let client = reqwest::Client::new();
    
    let url = format!("https://jsonplaceholder.typicode.com/todos?userId=1");
    
    let response: Vec<ToDo> = client
        .get(url)
        .send()
        .await?
        .json()
        .await?;
    
    // println!("{:?}",response);
    
    Ok(response)
}

pub async fn send_data() -> Result<ToDo, reqwest::Error> {
    
    let client = reqwest::Client::new();
    let todo = ToDo {
        user_id: 2,
        id:None,
        title: "Test".to_owned(),
        completed:true
    };
    let url = format!("https://jsonplaceholder.typicode.com/todos");
    let new_todo_item:ToDo = client
        .post(url)
        .json(&todo)
        .send()
        .await?
        .json()
        .await?;
    // in json we could use:
    // &serde_json::json!({ "userId": 1, "title": "Title" })
    // then change ToDo to serde_json::Value
    Ok(new_todo_item)
}