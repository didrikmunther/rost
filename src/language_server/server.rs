use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum LspId {
    Number(usize),
    String(String),
}

#[derive(Debug, Deserialize)]
struct HoverParamsPosition {
    line: usize,
    character: usize,
}

#[derive(Debug, Deserialize)]
struct HoverParams {
    textDocument: String,
    position: HoverParamsPosition,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RequestParams {
    HoverParams(HoverParams),
}

#[derive(Debug, Deserialize)]
struct RequestMessage {
    jsonrpc: String,
    id: LspId,
    method: String,
    params: Option<RequestParams>,
}

#[derive(Debug, Serialize)]
struct HoverResult {
    value: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ResponseResult {
    HoverResult(HoverResult),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ResponseErrorData {
    String(String),
    Number(isize),
    Bool(bool),
    // Array
    // Object
}

#[derive(Debug, Serialize)]
struct ResponseError {
    code: isize,
    message: String,
    data: Option<ResponseErrorData>,
}

#[derive(Debug, Serialize)]
struct ResponseMessage {
    jsonrpc: String,
    id: Option<LspId>,
    result: Option<ResponseResult>,
    error: Option<ResponseError>,
}

async fn index(item: web::Json<RequestMessage>) -> HttpResponse {
    println!("model: {:?}", &item);
    // HttpResponse::Ok().json(item.id) // <- send response
    HttpResponse::Ok().finish()
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    println!("Starting server ...");

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/extractor").route(web::post().to(index)))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    println!("... server started");

    server.await
}
