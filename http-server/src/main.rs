use anyhow::Result;

use http_server::*;
use slight_http_handler_macro::register_handler;
wit_bindgen_rust::import!("./wit/http-server.wit");
wit_error_rs::impl_error!(http_server::HttpRouterError);

use configs::*;
wit_bindgen_rust::import!("./wit/configs.wit");
wit_error_rs::impl_error!(ConfigsError);

use rand::Rng;
use serde_json::{json, Value};

fn main() -> Result<()> {
    let router = Router::new()?;
    let router_with_route = router
        .get("/hello", "handle_hello")?
        .post("/upload", "upload")?;
    println!("Server is running on port 3000");
    let _ = Server::serve("0.0.0.0:3000", &router_with_route)?;
    Ok(())
}

#[register_handler]
fn handle_hello(req: Request) -> Result<Response, HttpError> {
    Ok(Response {
        headers: Some(req.headers),
        body: Some("hello".as_bytes().to_vec()),
        status: 200,
    })
}

#[register_handler]
fn upload(request: Request) -> Result<Response, HttpError> {
    assert_eq!(request.method, Method::Post);
    println!("Incoming frame - Running ONNX model inferencing");

    let configs = Configs::open("my-secret-store").unwrap();
   
    //Read the value of the BASE_X1 environment variable
    let base_x1_str = String::from_utf8(configs.get("BASE_X1").unwrap()).unwrap();
    let base_x1 = base_x1_str.parse::<i32>().unwrap();
    
    // Read the value of the BASE_Y1 environment variable
    let base_y1_str = String::from_utf8(configs.get("BASE_Y1").unwrap()).unwrap();
    let base_y1 = base_y1_str.parse::<i32>().unwrap();

    // Read the value of the BASE_X2 environment variable
    let base_x2_str = String::from_utf8(configs.get("BASE_X2").unwrap()).unwrap();
    let base_x2 = base_x2_str.parse::<i32>().unwrap();

    // Read the value of the BASE_Y2 environment variable
    let base_y2_str = String::from_utf8(configs.get("BASE_Y2").unwrap()).unwrap();
    let base_y2 = base_y2_str.parse::<i32>().unwrap();

    let random_x1 = rand::thread_rng().gen_range(0..20);
    let random_x2 = rand::thread_rng().gen_range(0..20);
    let random_y1 = rand::thread_rng().gen_range(0..20);
    let random_y2 = rand::thread_rng().gen_range(0..20);
    let conf_level = rand::thread_rng().gen_range(79..94);
	
    let final_x1 = base_x1 + random_x1;
    let final_x2 = base_x2 + random_x2;
    let final_y1 = base_y1 + random_y1;
    let final_y2 = base_y2 + random_y2;

    let json_obj: Value = json!({
        "x1": final_x1,
        "y1": final_y1,
        "x2": final_x2,
        "y2": final_y2,
        "confLevel" : conf_level
    });

    println!("Inferenced finished - Returning {}", json_obj);
    let bytes = serde_json::to_vec(&json_obj).ok();
    
    Ok(Response {
        headers: None,
        body: bytes,
        status: 200,
    })
}