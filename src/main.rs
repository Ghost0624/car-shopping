mod todo_rest;  // Assuming you have a `todo_rest` module  
mod security;   // Assuming you have a `security` module  
use warp::Filter;  

use std::{convert::Infallible, sync::Arc};  

use crate::todo_rest::todos_filter;

const WEB_FOLDER: &str = "web-folder/";

#[tokio::main]
async fn main() {  

    let db_pool = Arc::new(DbPool{});

    // Define the "hi" route  
    let hi = warp::path("hi").map(|| "Hello from hi");  
    let apis = hi.or(todos_filter(db_pool.clone()));  // Assuming `todos_filter` returns a Filter  

    // Serve static files from WEB_FOLDER  
    let content = warp::fs::dir(WEB_FOLDER);  
    let root = warp::get()
        .and(warp::path::end())  
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));  
    
    let static_site = content.or(root);  
    
    // Combine the API routes and static file serving routes  
    let routes = apis.or(static_site);  
    
    println!("Server is running at http://127.0.0.1:8000");  
    
    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;  
}  

pub struct DbPool {}  

// Function to create a filter that provides access to the DbPool  
pub fn with_db_pool(db_pool: Arc<DbPool>) -> impl Filter<Extract = (Arc<DbPool>,), Error = Infallible> + Clone {  
    warp::any().map(move || db_pool.clone())  
}