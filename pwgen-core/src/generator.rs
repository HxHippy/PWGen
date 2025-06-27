use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    pub length: usize,
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
    pub exclude_ambiguous: bool,
    pub custom_symbols: Option<String>,
    pub min_uppercase: usize,
    pub min_lowercase: usize,
    pub min_numbers: usize,
    pub min_symbols: usize,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 16,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: true,
            exclude_ambiguous: true,
            custom_symbols: None,
            min_uppercase: 1,
            min_lowercase: 1,
            min_numbers: 1,
            min_symbols: 1,
        }
    }
}

pub struct PasswordGenerator;

impl PasswordGenerator {
    const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &'static str = "0123456789";
    const SYMBOLS: &'static str = "!@#$%^&*()-_=+[]{}|;:'\",.<>/?";
    const AMBIGUOUS: &'static str = "0O1lI";
    
    pub fn generate(config: &PasswordConfig) -> Result<String> {
        if config.length < 8 || config.length > 128 {
            return Err(Error::InvalidPasswordLength);
        }
        
        let mut charset = String::new();
        let mut password = Vec::new();
        let mut rng = thread_rng();
        
        if config.include_lowercase {
            charset.push_str(Self::LOWERCASE);
            for _ in 0..config.min_lowercase {
                let chars: Vec<char> = Self::LOWERCASE.chars().collect();
                password.push(chars[rng.gen_range(0..chars.len())]);
            }
        }
        
        if config.include_uppercase {
            charset.push_str(Self::UPPERCASE);
            for _ in 0..config.min_uppercase {
                let chars: Vec<char> = Self::UPPERCASE.chars().collect();
                password.push(chars[rng.gen_range(0..chars.len())]);
            }
        }
        
        if config.include_numbers {
            charset.push_str(Self::NUMBERS);
            for _ in 0..config.min_numbers {
                let chars: Vec<char> = Self::NUMBERS.chars().collect();
                password.push(chars[rng.gen_range(0..chars.len())]);
            }
        }
        
        if config.include_symbols {
            let symbols = config.custom_symbols.as_deref().unwrap_or(Self::SYMBOLS);
            charset.push_str(symbols);
            for _ in 0..config.min_symbols {
                let chars: Vec<char> = symbols.chars().collect();
                password.push(chars[rng.gen_range(0..chars.len())]);
            }
        }
        
        if charset.is_empty() {
            return Err(Error::Other("No character set selected".to_string()));
        }
        
        if config.exclude_ambiguous {
            charset = charset.chars()
                .filter(|c| !Self::AMBIGUOUS.contains(*c))
                .collect();
        }
        
        let chars: Vec<char> = charset.chars().collect();
        let remaining_length = config.length.saturating_sub(password.len());
        
        for _ in 0..remaining_length {
            password.push(chars[rng.gen_range(0..chars.len())]);
        }
        
        let mut shuffled = password;
        for i in (1..shuffled.len()).rev() {
            let j = rng.gen_range(0..=i);
            shuffled.swap(i, j);
        }
        
        Ok(shuffled.into_iter().collect())
    }
    
    pub fn generate_escaped(config: &PasswordConfig) -> Result<String> {
        let password = Self::generate(config)?;
        Ok(Self::escape_for_shell(&password))
    }
    
    pub fn escape_for_shell(password: &str) -> String {
        password
            .chars()
            .map(|c| match c {
                '\'' => "'\\''".to_string(),
                '\\' => "\\\\".to_string(),
                '"' => "\\\"".to_string(),
                '$' => "\\$".to_string(),
                '`' => "\\`".to_string(),
                '!' => "\\!".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }
    
    pub fn generate_passphrase(word_count: usize, separator: &str, capitalize: bool) -> Result<String> {
        const WORD_LIST: &[&str] = &[
            "ability", "account", "achieve", "across", "action", "activity", "actual", "address",
            "advance", "advice", "afford", "afraid", "against", "agency", "agenda", "almost",
            "already", "although", "always", "amazing", "amount", "analysis", "ancient", "animal",
            "another", "answer", "anxiety", "anyone", "anyway", "appear", "approach", "approve",
            "archive", "argument", "around", "arrange", "arrival", "article", "artist", "assault",
            "attempt", "attract", "auction", "audience", "author", "autumn", "average", "awesome",
            "balance", "balloon", "banana", "banner", "bargain", "barrier", "battery", "beauty",
            "because", "bedroom", "believe", "benefit", "besides", "between", "bicycle", "billion",
            "biology", "blanket", "blossom", "bottle", "boulder", "bracket", "brother", "browser",
            "buffalo", "builder", "burning", "business", "cabinet", "calcium", "calendar", "camera",
            "campaign", "capable", "capital", "captain", "capture", "carbon", "careful", "carrier",
            "cartoon", "cascade", "catalog", "category", "ceiling", "cellular", "century", "certain",
            "chairman", "chamber", "champion", "channel", "chapter", "charity", "chicken", "children",
            "chimney", "citizen", "clarity", "classic", "climate", "cluster", "coastal", "coconut",
            "collapse", "collect", "college", "combine", "comfort", "command", "comment", "common",
            "company", "compare", "compete", "complete", "complex", "concept", "concern", "concert",
            "conduct", "confirm", "connect", "consider", "console", "contain", "content", "contest",
            "context", "control", "convert", "cooking", "correct", "costume", "cottage", "council",
            "counter", "country", "courage", "creative", "cricket", "critical", "crystal", "culture",
            "current", "curtain", "customer", "cutting", "dancing", "daughter", "daylight", "deadline"
        ];
        
        if word_count < 3 || word_count > 20 {
            return Err(Error::Other("Word count must be between 3 and 20".to_string()));
        }
        
        let mut rng = thread_rng();
        let mut words = Vec::with_capacity(word_count);
        
        for _ in 0..word_count {
            let word = WORD_LIST[rng.gen_range(0..WORD_LIST.len())];
            let word = if capitalize {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            } else {
                word.to_string()
            };
            words.push(word);
        }
        
        Ok(words.join(separator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_password_default() {
        let config = PasswordConfig::default();
        let password = PasswordGenerator::generate(&config).unwrap();
        assert_eq!(password.len(), 16);
    }
    
    #[test]
    fn test_generate_password_custom_length() {
        let mut config = PasswordConfig::default();
        config.length = 32;
        let password = PasswordGenerator::generate(&config).unwrap();
        assert_eq!(password.len(), 32);
    }
    
    #[test]
    fn test_generate_password_numbers_only() {
        let config = PasswordConfig {
            length: 10,
            include_uppercase: false,
            include_lowercase: false,
            include_numbers: true,
            include_symbols: false,
            exclude_ambiguous: false,
            custom_symbols: None,
            min_uppercase: 0,
            min_lowercase: 0,
            min_numbers: 0,
            min_symbols: 0,
        };
        let password = PasswordGenerator::generate(&config).unwrap();
        assert!(password.chars().all(|c| c.is_numeric()));
    }
    
    #[test]
    fn test_escape_for_shell() {
        let password = "test$password'with\"special`chars!";
        let escaped = PasswordGenerator::escape_for_shell(password);
        assert!(escaped.contains("\\$"));
        assert!(escaped.contains("\\'"));
        assert!(escaped.contains("\\\""));
        assert!(escaped.contains("\\`"));
        assert!(escaped.contains("\\!"));
    }
    
    #[test]
    fn test_generate_passphrase() {
        let passphrase = PasswordGenerator::generate_passphrase(4, "-", true).unwrap();
        let parts: Vec<&str> = passphrase.split('-').collect();
        assert_eq!(parts.len(), 4);
        for part in parts {
            assert!(part.chars().next().unwrap().is_uppercase());
        }
    }
}