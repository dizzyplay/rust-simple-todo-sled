use crate::handler;
use crate::model::with_db;
use crate::DB;
use warp;
use warp::{Filter, Rejection, Reply};

pub fn api(todo_list: DB) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let todos = warp::path("todos");
    let list = todos
        .and(warp::path("list"))
        .and(with_db(todo_list.clone()))
        .and_then(handler::todo_list);
    let add = todos.and(warp::path("add").and(warp::get()).and_then(handler::add));

    let do_add = todos.and(
        warp::path("add")
            .and(warp::post())
            .and(warp::body::form())
            .and(with_db(todo_list.clone()))
            .and_then(handler::to_add),
    );
    let delete = todos.and(
        warp::path("delete")
            .and(warp::post())
            .and(warp::body::form())
            .and(with_db(todo_list.clone()))
            .and_then(handler::delete),
    );
    let toggle_done = todos.and(
        warp::path("toggle_done")
            .and(warp::post())
            .and(warp::body::form())
            .and(with_db(todo_list.clone()))
            .and_then(handler::toggle_done),
    );

    let edit = todos.and(
        warp::path("edit")
            .and(warp::get())
            .and(warp::query::<handler::EditOpt>())
            .and(with_db(todo_list.clone()))
            .and_then(handler::edit),
    );

    let do_edit = todos.and(
        warp::path("edit")
            .and(warp::post())
            .and(warp::body::form())
            .and(with_db(todo_list.clone()))
            .and_then(handler::do_edit),
    );

    todos
        .and(warp::path::end())
        .map(|| "hello its rust todo list")
        .or(list)
        .or(add)
        .or(do_add)
        .or(delete)
        .or(toggle_done)
        .or(edit)
        .or(do_edit)
}
