use std::sync::Arc;

use serde_json::{json, Value};  
use warp::{Filter, reply::Json};

use crate::{security::{do_auth, UserCtx}, DbPool, with_db_pool};  

pub fn todos_filter(db_pool: Arc<DbPool>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {  
    let todos_base = warp::path("todo");  

    // List todos  
    let list = todos_base  
        .and(warp::get())  
        .and(warp::path::end())  
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and_then(todo_list);

    // Get a single todo  
    let get = todos_base  
        .and(warp::get())  
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::path::param())  
        .and_then(todo_get);

    // Create a new todo  
    let create = todos_base  
        .and(warp::post())  
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::body::json())  
        .and_then(todo_create);

    list.or(get).or(create)  
}  

async fn todo_list(
    _user_ctx: UserCtx, 
    _db_pool:Arc<DbPool>,
) -> Result<Json, warp::Rejection> {  
    let todos = json!([  
        {"id": 1, "title": "todo 1"},  
        {"id": 2, "title": "todo 2"}  
    ]);  

    Ok(warp::reply::json(&todos))  
}  

async fn todo_get(
    _user_ctx: UserCtx, 
    _db_pool:Arc<DbPool>, 
    id: i64,
) -> Result<Json, warp::Rejection> {  
    // Simulating a case where we might not find the todo  
    if id > 2 { // Change this logic based on your actual data  
        return Err(warp::reject::not_found());  
    }  

    let todo = json!({"id": id, "user_id": _user_ctx.user_id, "title": format!("todo {}", id)});  
    Ok(warp::reply::json(&todo))  
}  

async fn todo_create(
    _user_ctx: UserCtx, 
    _db_pool:Arc<DbPool>,
    data: Value,
) -> Result<Json, warp::Rejection> {  
    // Here you could add logic to validate the `data` or save to a database.  
    Ok(warp::reply::json(&data))  
}
