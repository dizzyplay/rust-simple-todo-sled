mod filters;
mod handler;
mod model;
use model::TodoList;
use std::sync::Arc;
use tokio::sync::RwLock;

type DB = Arc<RwLock<TodoList>>;

#[tokio::main]
async fn main() {
    let todo_list = Arc::new(RwLock::new(TodoList::new("db".to_string())));
    let routes = filters::api(todo_list);
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
