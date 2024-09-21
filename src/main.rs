mod counter; //imports counter file

use warp::Filter;
use serde_json::json; // for the json keyword to be accessible within function

#[tokio::main]
async fn main() {

  let get_counter = warp::path("getCounter")
                         .map(|| get_counter());

  let increment_counter = warp::put()
                               .and(warp::path("incrementCounter"))
                               .map(|| increment_counter());

  let decrement_counter = warp::put()
                               .and(warp::path("decrementCounter"))
                               .map(|| decrement_counter());

  let all_requests = 
      get_counter.or(increment_counter.or(decrement_counter));

  warp::serve(all_requests)
      .run(([127, 0, 0, 1], 3030))
      .await;    
  println!("Hello, world!");
}

// Function to handle the counter request and return a JSON response
fn get_counter() -> warp::reply::Json {
  let count: i32 = counter::get_counter();
  warp::reply::json(&json!({ "counter": count }))
}

fn increment_counter() -> impl warp::Reply {
  counter::increment_counter();
  warp::reply::with_status("", warp::http::StatusCode::NO_CONTENT)
}

fn decrement_counter() -> impl warp::Reply {
  counter::decrement_counter();
  warp::reply::with_status("", warp::http::StatusCode::NO_CONTENT)
}