use anyhow::{anyhow, Result};
use clap::ValueEnum;
use serde::Serialize;
use std::{collections::HashMap, str::FromStr, vec};

#[cfg(feature = "english")]
const ENGLISH_WORD_LIST: &str = include_str!("../wordlists/english.txt");
#[cfg(feature = "chinese_simplified")]
const CHINESE_SIMPLIFIED_WORD_LIST: &str = include_str!("../wordlists/chinese_simplified.txt");
#[cfg(feature = "chinese_traditional")]
const CHINESE_TRADITIONAL_WORD_LIST: &str = include_str!("../wordlists/chinese_traditional.txt");
#[cfg(feature = "french")]
const FRENCH_WORD_LIST: &str = include_str!("../wordlists/french.txt");
#[cfg(feature = "italian")]
const ITALIAN_WORD_LIST: &str = include_str!("../wordlists/italian.txt");
#[cfg(feature = "japanese")]
const JAPANESE_WORD_LIST: &str = include_str!("../wordlists/japanese.txt");
#[cfg(feature = "korean")]
const KOREAN_WORD_LIST: &str = include_str!("../wordlists/korean.txt");
#[cfg(feature = "spanish")]
const SPANISH_WORD_LIST: &str = include_str!("../wordlists/spanish.txt");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
pub enum Language {
    #[cfg(feature = "chinese_simplified")]
    ChineseSimplified,
    #[cfg(feature = "chinese_traditional")]
    ChineseTraditional,
    #[cfg(feature = "english")]
    English,
    #[cfg(feature = "french")]
    French,
    #[cfg(feature = "italian")]
    Italian,
    #[cfg(feature = "japanese")]
    Japanese,
    #[cfg(feature = "korean")]
    Korean,
    #[cfg(feature = "spanish")]
    Spanish,
}

impl FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "chinese_simplified")]
            "chinese_simplified" => Ok(Language::ChineseSimplified),
            #[cfg(feature = "chinese_traditional")]
            "chinese_traditional" => Ok(Language::ChineseTraditional),
            #[cfg(feature = "english")]
            "english" => Ok(Language::English),
            #[cfg(feature = "french")]
            "french" => Ok(Language::French),
            #[cfg(feature = "italian")]
            "italian" => Ok(Language::Italian),
            #[cfg(feature = "japanese")]
            "japanese" => Ok(Language::Japanese),
            #[cfg(feature = "korean")]
            "korean" => Ok(Language::Korean),
            #[cfg(feature = "spanish")]
            "spanish" => Ok(Language::Spanish),
            _ => Err(anyhow!("unsupported language: {}", s)),
        }
    }
}

pub fn wordlist(language: &Language) -> (Vec<&'static str>, HashMap<&'static str, usize>) {
    let word_list = match language {
        #[cfg(feature = "english")]
        Language::English => ENGLISH_WORD_LIST,
        #[cfg(feature = "chinese_simplified")]
        Language::ChineseSimplified => CHINESE_SIMPLIFIED_WORD_LIST,
        #[cfg(feature = "chinese_traditional")]
        Language::ChineseTraditional => CHINESE_TRADITIONAL_WORD_LIST,
        #[cfg(feature = "french")]
        Language::French => FRENCH_WORD_LIST,
        #[cfg(feature = "italian")]
        Language::Italian => ITALIAN_WORD_LIST,
        #[cfg(feature = "japanese")]
        Language::Japanese => JAPANESE_WORD_LIST,
        #[cfg(feature = "korean")]
        Language::Korean => KOREAN_WORD_LIST,
        #[cfg(feature = "spanish")]
        Language::Spanish => SPANISH_WORD_LIST,
    };

    let mut words = vec![];
    let mut words_to_idx = HashMap::new();
    for (idx, word) in word_list.lines().enumerate() {
        words_to_idx.insert(word, idx);
        words.push(word);
    }

    (words, words_to_idx)
}

#[derive(Debug, Serialize)]
pub struct EncryptedMessage {
    pub message: &'static str,
    pub cipher_text: &'static str,
}

pub fn encrypt(
    num_keys: usize,
    key_list: &[u16],
    word_list: &[&'static str],
    word_to_idx: &HashMap<&'static str, usize>,
    words: &[&str],
) -> Result<Vec<EncryptedMessage>> {
    if words.len() > num_keys {
        return Err(anyhow!("Number of input words exceeds key length"));
    }

    let mut result = vec![];
    for (i, word) in words.iter().enumerate() {
        let word_i = word_to_idx.get(word).unwrap().to_owned();
        let key = key_list[i];
        let cipher = (word_i + key as usize) % word_list.len();
        result.push(EncryptedMessage {
            message: word_list[word_i],
            cipher_text: word_list[cipher],
        })
    }
    Ok(result)
}

pub fn decrypt(
    num_keys: usize,
    key_list: &[u16],
    word_list: &[&'static str],
    word_to_idx: &HashMap<&'static str, usize>,
    words: &[&str],
) -> Result<Vec<EncryptedMessage>> {
    if words.len() > num_keys {
        return Err(anyhow!("Number of input words exceeds key length"));
    }

    let mut result = vec![];
    for (i, word) in words.iter().enumerate() {
        let word_i = word_to_idx.get(word).unwrap().to_owned();
        let key = key_list[i];
        let cipher = (word_i - key as usize) % word_list.len();
        result.push(EncryptedMessage {
            message: word_list[word_i],
            cipher_text: word_list[cipher],
        })
    }
    Ok(result)
}

pub mod key {

    use anyhow::{anyhow, Result};
    use base64::{engine::general_purpose, Engine};
    use rand::Rng;
    use sha2::{Digest, Sha256};

    fn get_next() -> u16 {
        rand::thread_rng().gen_range(0..2048)
    }

    pub fn generate(num_words: u16) -> Result<String> {
        let mut key_list: Vec<u16> = vec![];
        for _ in 0..num_words {
            key_list.push(get_next());
        }
        Ok(encode(num_words, key_list))
    }

    fn encode(num_keys: u16, key_list: Vec<u16>) -> String {
        let mut key_string = vec![];
        key_string.extend_from_slice(&num_keys.to_be_bytes());
        for key in key_list.iter() {
            key_string.extend_from_slice(&key.to_be_bytes());
        }
        let mut hasher = Sha256::new();
        hasher.update(key_string.clone());
        let result = hasher.finalize();
        key_string.extend(&result[..4]);
        general_purpose::URL_SAFE_NO_PAD.encode(&key_string)
    }

    pub fn decode<I: Into<String>>(key_string: I) -> Result<(u16, Vec<u16>)> {
        let key_string = key_string.into();
        let mut decoded = general_purpose::URL_SAFE_NO_PAD.decode(key_string)?;
        let final_length = decoded.len().saturating_sub(4);
        let checksum = decoded.split_off(final_length);

        let mut hasher = Sha256::new();
        hasher.update(decoded.clone());
        let result: Vec<u8> = hasher.finalize()[..4].to_vec();

        if result != checksum {
            return Err(anyhow!(
                "keylength {:?} does not match the expected value: {:?}",
                result,
                checksum
            ));
        }

        let key_bytes: Vec<u8> = decoded.drain(2..).collect();
        let num_keys = u16::from_be_bytes(decoded.try_into().unwrap());

        let keys: Vec<u16> = key_bytes
            .chunks_exact(2)
            .map(|b| u16::from_be_bytes(b.try_into().unwrap()))
            .collect();
        Ok((num_keys, keys))
    }
}
