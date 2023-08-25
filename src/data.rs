use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub checked: bool,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct TodoRequest {
    pub name: String,
    pub checked: bool
}
impl std::fmt::Display for TodoRequest {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.name, self.checked)
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct TodoUpdateRequest {
    pub name: String,
    pub checked: bool,
}


impl std::fmt::Display for TodoUpdateRequest{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}, {}", self.name, self.checked)
    }
}


#[derive(Deserialize)]
#[derive(Serialize)]


pub struct TodoResponse {
    pub id: i32,
    pub name: String,
    pub checked: bool,
}

impl Clone for TodoResponse {
    fn clone(&self) -> Self {
        TodoResponse {
            id: self.id,
            name: self.name.clone(), // Clone the String field
            checked: self.checked,
        }
    }
}


impl TodoResponse {
    pub fn of(todo: Todo) -> TodoResponse {
        TodoResponse {
            id: todo.id,
            name: todo.name,
            checked: todo.checked,
        }
    }

 

}
