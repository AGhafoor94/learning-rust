#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ToDo {
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    pub user_id: i16,
    // pub id: Option<i32>,
    // pub id: String,
    pub id: i32,
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
pub async fn get_all_users() -> Result<Vec<ToDo>, reqwest::Error> {
    let client:reqwest::Client = reqwest::Client::new();
    let url:String = format!("https://jsonplaceholder.typicode.com/todos");
    
    let response: Vec<ToDo> = client
        .get(url)
        .send()
        .await?
        .json()
        .await?;
    // println!("{:?}",response);
    
    Ok(response)
}
pub async fn get_rust_data() -> Result<Vec<ToDo>, reqwest::Error> {
    let client:reqwest::Client = reqwest::Client::new();
    let url:String = format!("https://jsonplaceholder.typicode.com/todos?userId=1");
    
    let response: Vec<ToDo> = client
        .get(url)
        .send()
        .await?
        .json()
        .await?;
    // println!("{:?}",response);
    
    Ok(response)
}

pub async fn get_rust_data_single(id: &str) -> Result<ToDo, reqwest::Error> {
    let client:reqwest::Client = reqwest::Client::new();
    let url:String = format!("https://jsonplaceholder.typicode.com/todos?id={}",id);
    println!("{:?}", &url);
    
    let response:Vec<ToDo> = client
        .get(&url)
        .send()
        .await?
        .json()
        .await?;
    // println!("{:?}",response[0]);
    for todo in response {
        println!("{:?}", todo);
       return Ok(todo);
    }
    Ok(ToDo {
        user_id: 1,
        title: "Test".to_owned(),
        id:1,
        completed:false
    })
}
pub async fn get_rust_data_single_user(user: i16, id: i16) -> Result<ToDo, reqwest::Error> {
    let client:reqwest::Client = reqwest::Client::new();
    let url:String = format!("https://jsonplaceholder.typicode.com/todos?userId={}&id={}",user, id);
    println!("{:?}", &url);
    
    let response:Vec<ToDo> = client
        .get(&url)
        .send()
        .await?
        .json()
        .await?;
    // println!("{:?}",response[0]);
    for todo in response {
        println!("{:?}", todo);
       return Ok(todo);
    }
    Ok(ToDo {
        user_id: 1,
        title: "Test".to_owned(),
        id:1,
        completed:false
    })
}
pub async fn send_data() -> Result<ToDo, reqwest::Error> {
    
    let client = reqwest::Client::new();
    // let todo = ToDo {
    //     user_id: 2,
    //     id: None,
    //     title: "Test".to_owned(),
    //     completed:true
    // };
    let vec_length: Result<Vec<ToDo>,_> = get_rust_data().await;
    println!("{:?}",vec_length);
    let todo = ToDo {
        user_id: 2,
        id: 20,
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