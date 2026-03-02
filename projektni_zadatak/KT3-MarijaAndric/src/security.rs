pub struct SecurityFilter;

impl SecurityFilter {
    const INJECTION_PATTERNS: &'static [&'static str] = &[
        "ignore previous instructions",
        "ignore all previous",
        "you are now",
        "forget your instructions",
        "system prompt",
        "reveal your",
        "admin console",
        "admin mode",
        "developer mode",
        "jailbreak",
        "pretend you are",
        "act as if",
        "you are an ai",
        "ignore your role",
        "new instructions",
        "override instructions",
        "disregard",
        "bypass",
    ];

    pub fn validate_npc_name(name: &str, allowed_npcs: &[String]) -> Result<String, &'static str> {
        if name.trim().is_empty() {
            return Err("NPC ime ne sme biti prazno.");
        }

        if Self::is_too_long(name) {
            return Err("NPC ime je predugacko.");
        }

        if Self::is_injection(name) {
            return Err("Detektovan pokusaj napada u npc_name polju.");
        }

        let name_lower = name.to_lowercase();
        let exists = allowed_npcs.iter().any(|n| n.to_lowercase() == name_lower);
        if !exists {
            return Err("Nepoznati NPC.");
        }

        Ok(name.to_string())
    }

    pub fn is_injection(input: &str) -> bool {
        let lower = input.to_lowercase();
        Self::INJECTION_PATTERNS.iter().any(|pattern| lower.contains(pattern))
    }

    pub fn is_too_long(input: &str) -> bool {
        input.len() > 500
    }

    pub fn sanitize(input: &str) -> String {
        input.chars()
            .filter(|c| c.is_alphanumeric() || " .,!?'-".contains(*c))
            .collect()
    }

    pub fn validate(input: &str) -> Result<String, &'static str> {
        if input.trim().is_empty() {
            return Err("Poruka ne sme biti prazna.");
        }

        if Self::is_too_long(input) {
            return Err("Poruka je predugacka.");
        }

        if Self::is_injection(input) {
            return Err("Detektovan pokusaj napada.");
        }

        Ok(Self::sanitize(input))
    }

    pub fn validate_history(history: &[serde_json::Value]) -> Result<(), &'static str> {
        for msg in history {
            if msg["role"] == "user" {
                if let Some(content) = msg["content"].as_str() {
                    if Self::is_injection(content) {
                        return Err("Detektovana manipulacija u istoriji razgovora.");
                    }
                }
            }
        }
        Ok(())
    }   

}