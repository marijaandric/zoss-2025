use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

pub struct GroqClient {
    api_key: String,
    client: Client,
}

impl GroqClient {
    pub fn new(api_key: String) -> Self {
        GroqClient {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn chat(
        &self,
        system_prompt: &str,
        user_message: &str,
        history: &Vec<Value>,
    ) -> Result<(String, Value, Value)> {
        let mut messages = vec![
            json!({ "role": "system", "content": system_prompt })
        ];

        for msg in history {
            messages.push(msg.clone());
        }

        let user_msg = json!({ "role": "user", "content": user_message });
        messages.push(user_msg.clone());

        let body = json!({
            "model": "llama-3.1-8b-instant",
            "messages": messages,
            "temperature": 0.9,
            "max_tokens": 300
        });

        let response = self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let json: Value = response.json().await?;

        println!("Groq odgovor: {}", serde_json::to_string_pretty(&json).unwrap());

        let text = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("...")
            .to_string();

        let assistant_msg = json!({ "role": "assistant", "content": text });

        Ok((text, user_msg, assistant_msg))
    }
}