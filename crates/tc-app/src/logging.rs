pub struct LogRedactor;

impl LogRedactor {
    pub fn redact(message: &str) -> String {
        let mut out = message.to_string();

        // 1. Redact Bearer tokens & Authorization headers
        if let Some(pos) = out.to_lowercase().find("bearer ") {
            let start = pos + 7;
            let end = out[start..]
                .find(|c: char| c.is_whitespace() || c == '"' || c == ',' || c == ';')
                .map(|e| start + e)
                .unwrap_or(out.len());
            if end > start {
                let token_slice = &out[start..end];
                out = out.replace(token_slice, "[REDACTED_TOKEN]");
            }
        }

        // 2. Redact emails
        let words: Vec<&str> = out.split_whitespace().collect();
        for w in words {
            let clean = w.trim_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '.');
            if clean.contains('@') && clean.contains('.') && clean.len() > 5 {
                out = out.replace(clean, "[REDACTED_EMAIL]");
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redaction_email_and_token() {
        let log = "User john.doe@example.com logged in with header Authorization: Bearer abc123supersecret456!";
        let redacted = LogRedactor::redact(log);

        assert!(!redacted.contains("john.doe@example.com"));
        assert!(!redacted.contains("abc123supersecret456"));
        assert!(redacted.contains("[REDACTED_EMAIL]"));
        assert!(redacted.contains("[REDACTED_TOKEN]"));
    }
}
