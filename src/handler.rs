use crate::DB;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::result::Result;
use warp::http::Uri;
use warp::Reply;

type WebResult<T> = Result<T, Infallible>;

#[derive(Serialize)]
struct JsonResponse {
    data: String,
}

impl JsonResponse {
    fn new(data: String) -> Self {
        Self { data }
    }
}

pub async fn todo_list(todo_list: DB) -> WebResult<impl Reply> {
    let todos = todo_list.read().await.list();
    let todos = todos
        .iter()
        .map(|v| {
            let line = match v.is_done {
                true => "<s>",
                false => ""
            };
            let cline = match v.is_done {
                true => "</s>",
                false => ""
            };
            format!(
                r#"
                <div style="display:flex; align-items:center;">
                    {}<div style="border: 1px #C0C0C0 solid; padding: 5px; margin: 10px; border-radius:3px; width:500px;"> {} {} {}
                    </div>
                    <div>
                    <form method="post" action="/todos/toggle_done">
                        <input type="hidden" name="id" value="{}"/>
                        <input type="submit" value="COMPLETE"/>
                    </form>
                    </div>
                    <div>
                        <form method="get" action="/todos/edit">
                            <input type="hidden" name="id" value="{}"/>
                            <input type="submit" value="EDIT"/>
                        </form>
                    </div>
                    <div>
                    <form method="post" action="/todos/delete">
                        <input type="hidden" name="id" value="{}"/>
                        <input type="submit" value="DELETE"/>
                    </form>
                    </div>
                </div>
                "#,
                v.id, line, v.title, cline, v.id, v.id, v.id
            )
        })
        .collect::<Vec<String>>();
    let todos_str = todos.join("");
    let mut html = format!(
        r#"
        <h2>멋쟁이 창열이의 할일들</h2>
        <div><a href = "/todos/add"> <h3>ADD</h3> </a></div>"#
    );
    html.push_str(todos_str.as_str());
    Ok(warp::reply::html(html))
}

pub async fn add() -> WebResult<impl Reply> {
    let html = format!(
        r#"
        <form method="post" action="/todos/add">
            <input type="text" name="title"/>
            <input type="submit"/>
        </form>
        "#
    );
    Ok(warp::reply::html(html))
}

pub async fn to_add(body: HashMap<String, String>, todo_list: DB) -> WebResult<impl Reply> {
    let title = body.get("title").unwrap();
    todo_list.write().await.add(title.to_string()).unwrap();

    Ok(warp::redirect(Uri::from_static("/todos/list")))
}

pub async fn delete(body: HashMap<String, String>, todo_list: DB) -> WebResult<impl Reply> {
    let to_delete_id = match body.get("id") {
        Some(v) => v,
        None => "0",
    };
    todo_list.write().await.remove(to_delete_id.to_string());
    Ok(warp::redirect(Uri::from_static("/todos/list")))
    //Ok(warp::reply::json(&JsonResponse::new(String::from("ok"))))
}

#[derive(Debug, Deserialize)]
pub struct EditOpt {
    id: Option<String>,
}

pub async fn edit(query: EditOpt, todo_list: DB) -> WebResult<impl Reply> {
    let edit_id = query.id.unwrap();
    let todo = todo_list.read().await.get(edit_id);
    match todo {
        Some(todo) => {
            let html = format!(
                r#"
                <h3>Edit Todo</h>
                <form method="post" action="/todos/edit">
                    <input type="hidden" name="id" value="{}"/>
                    <input type="text" name="title" value="{}"/>
                    <input type="submit"/>
                </form>
                "#,
                todo.id, todo.title
            );
            Ok(warp::reply::html(html))
        }
        None => Ok(warp::reply::html("not found".to_string())),
    }
}

pub async fn toggle_done(body: HashMap<String, String>, todo_list: DB) -> WebResult<impl Reply> {
    let todo_id = match body.get("id") {
        Some(id) => id,
        None => "0",
    };
    todo_list.write().await.toggle_done(todo_id.to_string());

    Ok(warp::redirect(Uri::from_static("/todos/list")))
}

pub async fn do_edit(body: HashMap<String, String>, todo_list: DB) -> WebResult<impl Reply> {
    let mut todos = todo_list.write().await;
    let edit_id = body.get("id").unwrap();
    let title = body.get("title").unwrap();
    todos.edit(edit_id.clone(), title.clone());
    Ok(warp::redirect(Uri::from_static("/todos/list")))
}
