use std::collections::HashMap;

const STOP_WORDS: &[&str] = &[
    "the", "and", "for", "are", "but", "not", "you", "all", "can", "her",
    "was", "one", "our", "out", "day", "get", "has", "him", "his", "how",
    "its", "may", "new", "now", "old", "see", "two", "way", "who", "boy",
    "did", "does", "had", "let", "put", "say", "she", "too", "use", "that",
    "this", "with", "have", "from", "they", "will", "been", "were", "said",
    "what", "when", "your", "which", "there", "their", "would", "could",
    "should", "about", "into", "than", "then", "them", "these", "some",
];

pub struct SimpleEmbedder;

impl SimpleEmbedder {
    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphabetic()).to_string())
            .filter(|w| w.len() > 2)
            .filter(|w| !STOP_WORDS.contains(&w.as_str()))
            .map(|w| Self::stem(&w))
            .collect()
    }

    fn stem(word: &str) -> String {
        let suffixes = ["ing", "tion", "ons", "ers", "est", "ed", "es", "er", "ly", "s"];
        for suffix in &suffixes {
            if word.ends_with(suffix) && word.len() > suffix.len() + 2 {
                return word[..word.len() - suffix.len()].to_string();
            }
        }
        word.to_string()
    }

    pub fn embed(text: &str) -> HashMap<String, f32> {
        let mut counts: HashMap<String, f32> = HashMap::new();
        let words = Self::tokenize(text);
        let total = words.len() as f32;

        if total == 0.0 {
            return counts;
        }

        for word in &words {
            *counts.entry(word.clone()).or_insert(0.0) += 1.0;
        }

        for val in counts.values_mut() {
            *val /= total;
        }

        counts
    }

    pub fn compute_idf(documents: &[String]) -> HashMap<String, f32> {
        let total_docs = documents.len() as f32;
        let mut doc_counts: HashMap<String, f32> = HashMap::new();

        for doc in documents {
            let unique_words: std::collections::HashSet<String> =
                Self::tokenize(doc).into_iter().collect();

            for word in unique_words {
                *doc_counts.entry(word).or_insert(0.0) += 1.0;
            }
        }

        doc_counts
            .into_iter()
            .map(|(word, count)| (word, (total_docs / count).ln()))
            .collect()
    }

    pub fn embed_with_idf(text: &str, idf: &HashMap<String, f32>) -> HashMap<String, f32> {
        let tf = Self::embed(text);

        tf.into_iter()
            .map(|(word, tf_score)| {
                let idf_score = idf.get(&word).copied().unwrap_or(0.0);
                (word, tf_score * idf_score)
            })
            .collect()
    }

    pub fn cosine_similarity(a: &HashMap<String, f32>, b: &HashMap<String, f32>) -> f32 {
        let dot: f32 = a.iter()
            .filter_map(|(k, v)| b.get(k).map(|bv| v * bv))
            .sum();

        let mag_a: f32 = a.values().map(|v| v * v).sum::<f32>().sqrt();
        let mag_b: f32 = b.values().map(|v| v * v).sum::<f32>().sqrt();

        if mag_a == 0.0 || mag_b == 0.0 {
            return 0.0;
        }

        dot / (mag_a * mag_b)
    }
}