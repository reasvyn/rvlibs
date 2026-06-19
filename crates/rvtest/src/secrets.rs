use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

/// Global flag to enable secrets masking.
static MASK_SECRETS_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable or disable secrets masking globally.
pub fn set_mask_secrets_enabled(enabled: bool) {
    MASK_SECRETS_ENABLED.store(enabled, Ordering::SeqCst);
}

/// Returns `true` if secrets masking is currently enabled.
pub fn is_mask_secrets_enabled() -> bool {
    MASK_SECRETS_ENABLED.load(Ordering::SeqCst)
}

/// A compiled secret pattern for matching and masking.
struct SecretPattern {
    regex: regex::Regex,
    replacement: &'static str,
}

fn compile(pattern: &str, replacement: &'static str) -> SecretPattern {
    SecretPattern {
        regex: regex::Regex::new(pattern).expect("invalid secret pattern"),
        replacement,
    }
}

/// Compile patterns once and cache them.
fn compiled_patterns() -> &'static Vec<SecretPattern> {
    static PATTERNS: OnceLock<Vec<SecretPattern>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            // AWS access keys
            compile(r"(^|[^a-zA-Z0-9/+])(AKIA[0-9A-Z]{16})($|[^a-zA-Z0-9/+])", "${1}***$3"),
            compile(r"(^|[^a-zA-Z0-9/+])(ASIA[0-9A-Z]{16})($|[^a-zA-Z0-9/+])", "${1}***$3"),
            // General API key/value patterns
            compile(r#"(?i)((?:api[_-]?key|apikey|token|secret|password|passwd|auth[_-]?token|bearer)[:=]\s*['\"]?)([a-zA-Z0-9_\-.]{16,64})"#, "$1***"),
            // GitHub tokens (preceded by non-alphanum or start)
            compile(r"(^|[^a-zA-Z0-9])(ghp_[a-zA-Z0-9]{36})($|[^a-zA-Z0-9])", "${1}***$3"),
            compile(r"(^|[^a-zA-Z0-9])(gho_[a-zA-Z0-9]{36})($|[^a-zA-Z0-9])", "${1}***$3"),
            compile(r"(^|[^a-zA-Z0-9])(ghu_[a-zA-Z0-9]{36})($|[^a-zA-Z0-9])", "${1}***$3"),
            compile(r"(^|[^a-zA-Z0-9])(ghs_[a-zA-Z0-9]{36})($|[^a-zA-Z0-9])", "${1}***$3"),
            compile(r"(^|[^a-zA-Z0-9])(ghr_[a-zA-Z0-9]{36})($|[^a-zA-Z0-9])", "${1}***$3"),
            // GitLab tokens
            compile(r"(^|[^a-zA-Z0-9])(glpat-[a-zA-Z0-9\-_]{20,40})($|[^a-zA-Z0-9])", "${1}***$3"),
            // Stripe keys
            compile(r"(^|[^a-zA-Z0-9])(sk_live_[a-zA-Z0-9]{24,48})($|[^a-zA-Z0-9])", "${1}***$3"),
            compile(r"(^|[^a-zA-Z0-9])(pk_live_[a-zA-Z0-9]{24,48})($|[^a-zA-Z0-9])", "${1}***$3"),
            compile(r"(^|[^a-zA-Z0-9])(rk_live_[a-zA-Z0-9]{24,48})($|[^a-zA-Z0-9])", "${1}***$3"),
            // Slack tokens
            compile(r"(^|[^a-zA-Z0-9])(xox[baprs]-[a-zA-Z0-9\-]{10,80})($|[^a-zA-Z0-9])", "${1}***$3"),
            // SSH private keys
            compile(r"-----BEGIN\s?(?:RSA|DSA|EC|OPENSSH|PRIVATE)\s?KEY-----[\s\S]*?-----END\s?(?:RSA|DSA|EC|OPENSSH|PRIVATE)\s?KEY-----",
                   "-----BEGIN KEY-----\n*** REDACTED ***\n-----END KEY-----"),
            // JWT tokens
            compile(r"(eyJ[a-zA-Z0-9\-_]{10,}\.[a-zA-Z0-9\-_]{10,}\.[a-zA-Z0-9\-_]{10,})", "eyJ***.***.***"),
            // Password field in JSON
            compile(r#"(?i)"password"\s*:\s*"[^"]{3,}""#, r#""password": "***""#),
            compile(r#"(?i)"secret"\s*:\s*"[^"]{3,}""#, r#""secret": "***""#),
            // Connection strings with password
            compile(r"(?i)(postgres|mysql|mongodb|redis|amqp)://[^:]+:([^@]+)@", "$1://***:***@"),
            // Basic auth in URLs
            compile(r"(?i)(https?://)[^:]+:[^@]+@", "$1***:***@"),
            // Generic secret/credential env-var style
            compile(r"(?i)(?:SECRET|PASSWORD|TOKEN|API_KEY|ACCESS_KEY)[=:][^\]\)\}\s;,]{8,}", "***"),
        ]
    })
}

/// Mask known secret patterns in the given text.
///
/// Replaces API keys, tokens, passwords, and other sensitive data
/// with redacted placeholders like `***`.
///
/// # Example
///
/// ```
/// use rvtest::secrets::mask_secrets;
///
/// let masked = mask_secrets("my token is ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
/// assert!(!masked.contains("ghp_xxxxxxxxx"));
/// assert!(masked.contains("ghp_***"));
/// ```
pub fn mask_secrets(text: &str) -> String {
    let mut result = text.to_owned();
    for pattern in compiled_patterns() {
        result = pattern.regex.replace_all(&result, pattern.replacement).into_owned();
    }
    result
}

/// Mask secrets in a string if masking is enabled.
pub fn mask_if_enabled(text: &str, enabled: bool) -> String {
    if enabled {
        mask_secrets(text)
    } else {
        text.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_aws_key() {
        let result = mask_secrets("Using key AKIAIOSFODNN7EXAMPLE in config");
        assert!(!result.contains("AKIAIOSFODNN7EXAMPLE"));
        assert!(result.contains("***"));
    }

    #[test]
    fn masks_github_token() {
        let result = mask_secrets("token=ghp_abcdefghijklmnopqrstuvwxyz1234567890");
        assert!(!result.contains("ghp_abcdefghijklmnopqrstuvwxyz1234567890"));
        assert!(result.contains("***"));
    }

    #[test]
    fn masks_password_in_url() {
        let result = mask_secrets("postgres://user:supersecret@localhost:5432/db");
        assert!(!result.contains("supersecret"));
        assert!(result.contains("postgres://***:***@"));
    }

    #[test]
    fn masks_password_in_json() {
        let result = mask_secrets("{\"user\": \"admin\", \"password\": \"hunter2\"}");
        assert!(!result.contains("hunter2"));
        assert!(result.contains("\"password\": \"***\""));
    }

    #[test]
    fn masks_ssh_key() {
        let key = "-----BEGIN RSA KEY-----\nMIIEpAIBAAKCAQEA\n-----END RSA KEY-----";
        let result = mask_secrets(key);
        assert!(!result.contains("MIIEpAIBAAKCAQEA"));
        assert!(result.contains("*** REDACTED ***"));
    }

    #[test]
    fn masks_jwt() {
        let jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jNqqG7PYrqYzIH8N-mCQfFsQXmQ";
        let result = mask_secrets(jwt);
        assert!(!result.contains("eyJhbGciOiJIUzI1NiJ9"));
        assert!(result.contains("eyJ***.***.***"));
    }

    #[test]
    fn masks_stripe_key() {
        let key = "sk_live_";
        let result = mask_secrets(&format!("{key}abcdefghijklmnopqrstuvwxyz123456"));
        assert!(!result.contains(key));
        assert!(result.contains("***"));
    }

    #[test]
    fn masks_gitlab_token() {
        let result = mask_secrets("glpat-abcdefghijklmnopqrstuvwxyz123456");
        assert!(!result.contains("glpat-abcdefghijklmnopqrstuvwxyz123456"));
        assert!(result.contains("***"));
    }

    #[test]
    fn does_not_affect_normal_text() {
        let text = "The quick brown fox jumps over the lazy dog";
        assert_eq!(mask_secrets(text), text);
    }

    #[test]
    fn does_not_affect_short_strings() {
        let text = "hello world 12345";
        assert_eq!(mask_secrets(text), text);
    }

    #[test]
    fn mask_if_enabled_true() {
        let result = mask_if_enabled("ghp_abcdefghijklmnopqrstuvwxyz1234567890", true);
        assert!(result.contains("***"));
        assert!(!result.contains("ghp_abcdefghijklmnopqrstuvwxyz1234567890"));
    }

    #[test]
    fn mask_if_enabled_false() {
        let input = "ghp_abcdefghijklmnopqrstuvwxyz1234567890";
        assert_eq!(mask_if_enabled(input, false), input);
    }

    #[test]
    fn set_and_check_enabled() {
        set_mask_secrets_enabled(true);
        assert!(is_mask_secrets_enabled());
        set_mask_secrets_enabled(false);
        assert!(!is_mask_secrets_enabled());
    }
}
