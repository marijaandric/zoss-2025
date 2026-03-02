mod llm;
mod embeddings;
mod rag;
mod security;

use security::SecurityFilter;
use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;
use std::fs;

use llm::GroqClient;
use rag::RAG;

struct AppState {
    groq: GroqClient,
    rag: Mutex<RAG>,  
    history: Mutex<HashMap<String, Vec<serde_json::Value>>>,
}

#[derive(Deserialize)]
struct ChatRequest {
    npc_name: String,
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    npc_name: String,
    response: String,
}

#[derive(Deserialize)]
struct PoisonRequest {filename: String,
    content: String,
    append: bool,
}

#[derive(Serialize)]
struct PoisonResponse {
    status: String,
    filename: String,
    bytes_written: usize,
    rag_reloaded: bool,
}

#[post("/admin/file")]
async fn poison_knowledge(
    data: web::Data<Arc<AppState>>,
    body: web::Json<PoisonRequest>,
) -> impl Responder {

    let safe_filename = body.filename
        .replace("..", "")
        .replace("/", "")
        .replace("\\", "");

    if safe_filename.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid filename"
        }));
    }

    let filepath = format!("knowledge/{}", safe_filename);

    let write_result = if body.append {
        let existing = fs::read_to_string(&filepath).unwrap_or_default();
        let new_content = format!("{}\n\n{}", existing, body.content);
        fs::write(&filepath, &new_content)
            .map(|_| new_content.len())
    } else {
        fs::write(&filepath, &body.content)
            .map(|_| body.content.len())
    };

    match write_result {
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to write file: {}", e)
            }));
        }
        Ok(bytes) => {
            let mut rag = data.rag.lock().unwrap();
            *rag = RAG::load_knowledge("knowledge");

            HttpResponse::Ok().json(PoisonResponse {
                status: "uploaded".to_string(),
                filename: safe_filename,
                bytes_written: bytes,
                rag_reloaded: true,
            })
        }
    }
}

#[post("/chat")]
async fn chat(
    data: web::Data<Arc<AppState>>,
    body: web::Json<ChatRequest>,
) -> impl Responder {
    let clean_message = match SecurityFilter::validate(&body.message) {
        Ok(msg) => msg,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

     let prompt = data.rag.lock().unwrap().build_prompt(&clean_message, &body.npc_name);

    let history = {
        let map = data.history.lock().unwrap();
        map.get(&body.npc_name).cloned().unwrap_or_default()
    };

    match data.groq.chat(&prompt, &clean_message, &history).await {
        Ok((response, user_msg, assistant_msg)) => {
            let mut map = data.history.lock().unwrap();
            let npc_history = map.entry(body.npc_name.clone()).or_insert_with(Vec::new);
            npc_history.push(user_msg);
            npc_history.push(assistant_msg);

            HttpResponse::Ok().json(ChatResponse {
                npc_name: body.npc_name.clone(),
                response,
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().body("Error communicating with LLM")
        }
    }
}

#[post("/reset/{npc_name}")]
async fn reset_history(
    data: web::Data<Arc<AppState>>,
    path: web::Path<String>,
) -> impl Responder {
    let npc_name = path.into_inner();
    let mut map = data.history.lock().unwrap();
    map.remove(&npc_name);
    HttpResponse::Ok().body(format!("Histopry {} deleted.", npc_name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY nije postavljen u .env!");

    let rag = Mutex::new(RAG::load_knowledge("knowledge"));
    let groq = GroqClient::new(api_key);

    let state = Arc::new(AppState {
        groq,
        rag,
        history: Mutex::new(HashMap::new()),
    });

    println!("NPC server pokrenut na http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(chat)
            .service(reset_history)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}