use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Simulacija korisničkog naloga
#[derive(Clone)]
struct Account {
    balance: Arc<Mutex<i32>>,
}

#[derive(Deserialize)]
struct WithdrawRequest {
    amount: i32,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    balance: i32,
}

// RANJIV endpoint - Race condition: TOCTOU (Time-of-Check-Time-of-Use)
#[actix_web::post("/withdraw")]
async fn withdraw(
    account: web::Data<Account>,
    req: web::Json<WithdrawRequest>,
) -> Result<HttpResponse> {
    let amount = req.amount;
    
    // 1. CHECK - Pročitaj balans (lock se oslobađa nakon ovog bloka)
    let current_balance = {
        let balance = account.balance.lock().unwrap();
        println!("[CHECK] Balance: {}", *balance);
        *balance
    }; // Lock oslobođen ovde!
    
    // Čekanje 1 sekunda BEZ AKTIVNOG LOCKA - drugi thread može pročitati isti balans!
    println!("[WAIT] 1 second (NO LOCK)...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 2. ACT - Izvrši povlačenje koristeći STARI current_balance
    let mut balance = account.balance.lock().unwrap();
    *balance = current_balance - amount;
    let new_balance = *balance;
    
    println!("[ACT] Withdrew {} | New balance: {} (used stale: {})", 
             amount, new_balance, current_balance);
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        balance: new_balance,
    }))
}

// Provera balansa
#[actix_web::get("/balance")]
async fn get_balance(account: web::Data<Account>) -> Result<HttpResponse> {
    let balance = account.balance.lock().unwrap();
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        balance: *balance,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("==============================================");
    println!("TOCTOU Race Condition Demo");
    println!("==============================================");
    println!("[*] Starting server on http://0.0.0.0:8080");
    println!("[*] Initial balance: $1000");
    println!("[*] Endpoint: POST /withdraw");
    println!("[*] Check balance: GET /balance");
    println!("==============================================\n");

    let account = Account {
        balance: Arc::new(Mutex::new(1000)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(account.clone()))
            .service(withdraw)
            .service(get_balance)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
