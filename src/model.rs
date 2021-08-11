use serde::{Deserialize, Serialize};
use serde_json;
use sled::{self, Db, Result};
use std::cmp::Ordering;
use std::convert::Infallible;
use std::str;
use warp::{self, Filter};

#[derive(Debug, Serialize, Deserialize, Eq)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub is_done: bool,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id.parse::<usize>().unwrap() == other.id.parse::<usize>().unwrap()
    }
}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.id.parse::<usize>().unwrap();
        let b = other.id.parse::<usize>().unwrap();
        a.cmp(&b)
    }
}

pub struct TodoList {
    db: Db,
    latest_id: String,
}

impl TodoList {
    pub fn new(path: String) -> Self {
        let db = sled::open(path.as_str()).unwrap();
        let latest_id = match db.get("latest_id".as_bytes()) {
            Ok(id) => match id {
                Some(id) => str::from_utf8(&id).unwrap().to_string(),
                None => {
                    format!("0")
                }
            },
            _ => panic!("erro"),
        };
        TodoList { db, latest_id }
    }

    pub fn list(&self) -> Vec<Todo> {
        let mut list = Vec::new();
        for v in self.db.iter() {
            if "latest_id" == str::from_utf8(&v.as_ref().unwrap().0).unwrap() {
                continue;
            }

            let stodo = str::from_utf8(&v.as_ref().unwrap().1).unwrap();
            let todo: Todo = serde_json::from_str(stodo).unwrap();
            list.push(todo);
        }
        list
    }

    pub fn add(&mut self, title: String) -> Result<()> {
        let id = format!("{}", self.latest_id.parse::<usize>().unwrap() + 1);
        self.latest_id = id.clone();
        let todo = Todo {
            id: id.clone(),
            title,
            is_done: false,
        };
        let r = serde_json::to_string(&todo).unwrap();
        self.db.insert(id.as_bytes(), r.as_bytes())?;
        self.db.insert("latest_id".as_bytes(), id.as_bytes())?;
        Ok(())
    }

    pub fn remove(&mut self, id: String) {
        self.db.remove(id.as_bytes()).unwrap();
    }

    pub fn toggle_done(&mut self, id: String) {
        let todo = self.db.get(id.as_bytes()).unwrap();
        match todo {
            Some(todo) => {
                let s = str::from_utf8(&todo).unwrap();
                let mut todo: Todo = serde_json::from_str(s).unwrap();
                todo.is_done = !todo.is_done;
                self.db
                    .insert(
                        id.as_bytes(),
                        serde_json::to_string(&todo).unwrap().as_bytes(),
                    )
                    .unwrap();
            }
            None => {}
        }
    }

    pub fn get(&self, id: String) -> Option<Todo> {
        let todo = self.db.get(id.as_bytes()).unwrap();
        match todo {
            Some(todo) => {
                let s = str::from_utf8(&todo).unwrap();
                let todo: Todo = serde_json::from_str(s).unwrap();
                Some(todo)
            }
            None => None,
        }
    }

    pub fn edit(&mut self, id: String, title: String) {
        let todo = self.db.get(id.as_bytes()).unwrap().unwrap();
        let mut todo: Todo = serde_json::from_str(str::from_utf8(&todo).unwrap()).unwrap();
        todo.title = title;
        let todo = serde_json::to_string(&todo).unwrap();
        self.db.insert(&id.as_bytes(), todo.as_bytes()).unwrap();
    }
}

use crate::DB;
pub fn with_db(todo_list: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || todo_list.clone())
}
