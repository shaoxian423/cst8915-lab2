use warp::Filter; // Import the warp web framework and the Filter trait for handling HTTP requests.

#[tokio::main] // This macro marks the main function as asynchronous and uses the Tokio runtime.
async fn main() {
    // Create a CORS (Cross-Origin Resource Sharing) filter to allow access from any origin.
    let cors = warp::cors()
        .allow_any_origin() // Allow requests from any domain (not restricted).
        .allow_methods(vec!["GET", "POST", "DELETE"]); // Extend the allowed HTTP methods to include POST and DELETE.

    // Define a route that listens for GET requests to the "/products" path.
    let products = warp::get() // Ensure this route only responds to GET requests.
        .and(warp::path("products")) // Define the "/products" path.
        .map(|| {
            warp::reply::json(&vec![
                serde_json::json!({ "id": 1, "name": "Dog Food", "price": 19.99 }),
                serde_json::json!({ "id": 2, "name": "Cat Food", "price": 34.99 }),
                serde_json::json!({ "id": 3, "name": "Bird Seeds", "price": 10.99 }),
            ])
        })
        .with(cors.clone()); // Apply the CORS filter to this route.

    // Define a route to accept POST requests at "/add-product".
    let add_product = warp::post() // Ensure this route only responds to POST requests.
        .and(warp::path("add-product")) // Define the "/add-product" path.
        .and(warp::body::json()) // Use warp's built-in JSON body parser.
        .map(|product: serde_json::Value| { // Accept a JSON object representing a product.
            println!("Received product to add: {:?}", product);
            warp::reply::json(&product) // Echo back the received product.
        })
        .with(cors); // Apply the CORS filter to this route.

    // Combine the "products" and "add-product" routes into one service.
    let routes = products.or(add_product);

    // Start the web server on IP address 0.0.0.0 and port 3030.
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

