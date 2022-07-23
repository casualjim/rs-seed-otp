use std::{
    io::{stdout, Write},
    process::exit,
};

use clap::Parser;
use seed_otp::{decrypt, encrypt, key, wordlist, Language};
use serde_json::json;

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let (output, code) = match &cli.command {
        Commands::Generate { num_words } => {
            match key::generate(num_words.to_owned().unwrap_or(24)) {
                Ok(key) => (
                    json!({
                        "success": true,
                        "otp-key": key,
                    }),
                    0,
                ),
                Err(e) => (
                    json!({
                        "success": false,
                        "message": format!("{}", e)
                    }),
                    1,
                ),
            }
        }
        Commands::CheckKey { otp_key } => {
            match key::decode(otp_key.to_owned().expect("otp_key is not provided")) {
                Ok((num_keys, keys)) => (
                    json!({
                        "success": true,
                        "num-keys": num_keys,
                        "keylist": keys,
                    }),
                    0,
                ),
                Err(e) => (
                    json!({
                        "success": false,
                        "message": format!("{}", e)
                    }),
                    1,
                ),
            }
        }
        Commands::Encrypt {
            language,
            include_options,
            detail,
            otp_key,
            words,
        } => match key::decode(otp_key.to_owned().expect("otp key is not provided")) {
            Ok((num_keys, keys)) => {
                let (word_list, word_to_idx) = wordlist(language);
                let maybe_encrypted = encrypt(
                    num_keys as usize,
                    &keys,
                    &word_list,
                    &word_to_idx,
                    words
                        .iter()
                        .map(|w| w.as_str())
                        .collect::<Vec<&str>>()
                        .as_slice(),
                );

                match maybe_encrypted {
                    Ok(messages) => {
                        let encrypted_words: Vec<&str> =
                            messages.iter().map(|m| m.cipher_text).collect();
                        let mut js = json!({
                            "success": true,
                            "encrypted-words": encrypted_words,
                        });
                        let updater = js.as_object_mut().unwrap();
                        if *include_options {
                            updater.append(
                                json!({
                                    "otp-key": otp_key.clone().unwrap(),
                                    "language": language,
                                    "detail": detail,
                                })
                                .as_object_mut()
                                .unwrap(),
                            );
                        }
                        if *detail {
                            updater.append(
                                json!({
                                    "mapping": messages,
                                })
                                .as_object_mut()
                                .unwrap(),
                            );
                        }
                        (js, 0)
                    }
                    Err(e) => (
                        json!({
                            "success": false,
                            "message": format!("{}", e)
                        }),
                        1,
                    ),
                }
            }
            Err(e) => (
                json!({
                    "success": false,
                    "message": format!("{}", e)
                }),
                1,
            ),
        },
        Commands::Decrypt {
            language,
            include_options,
            detail,
            otp_key,
            words,
        } => match key::decode(otp_key.to_owned().expect("otp-key is not provided")) {
            Ok((num_keys, keys)) => {
                let (word_list, word_to_idx) = wordlist(language);
                let maybe_decrypted = decrypt(
                    num_keys as usize,
                    &keys,
                    &word_list,
                    &word_to_idx,
                    words
                        .iter()
                        .map(|w| w.as_str())
                        .collect::<Vec<&str>>()
                        .as_slice(),
                );

                match maybe_decrypted {
                    Ok(messages) => {
                        let encrypted_words: Vec<&str> =
                            messages.iter().map(|m| m.cipher_text).collect();
                        let mut js = json!({
                            "success": true,
                            "encrypted-words": encrypted_words,
                        });
                        let updater = js.as_object_mut().unwrap();
                        if *include_options {
                            updater.append(
                                json!({
                                    "otp-key": otp_key.clone().unwrap(),
                                    "language": language,
                                    "detail": detail,
                                })
                                .as_object_mut()
                                .unwrap(),
                            );
                        }
                        if *detail {
                            updater.append(
                                json!({
                                    "mapping": messages,
                                })
                                .as_object_mut()
                                .unwrap(),
                            );
                        }
                        (js, 0)
                    }
                    Err(e) => (
                        json!({
                            "success": false,
                            "message": format!("{}", e)
                        }),
                        1,
                    ),
                }
            }
            Err(e) => (
                json!({
                    "success": false,
                    "message": format!("{}", e)
                }),
                1,
            ),
        },
    };
    let out = stdout();
    {
        let mut out = out.lock();
        colored_json::write_colored_json(&output, &mut out).unwrap();
        writeln!(&mut out).unwrap();
        out.flush().unwrap();
    }
    exit(code);
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(clap::Subcommand)]
enum Commands {
    /// Generate a secure OTP key for up to NUM_WORDS number of words.
    ///
    /// Make sure NUM_WORDS is at least as large as the number of seed words you
    /// use. It is normally 12 or 24 words. If you are unsure, use 24.
    Generate {
        #[clap(value_parser)]
        num_words: Option<u16>,
    },
    /// Check OTP key for encoding or checksum errors.
    CheckKey {
        #[clap(value_parser)]
        otp_key: Option<String>,
    },
    /// Encrypt seed words using an OTP key.
    Encrypt {
        #[clap(short, long)]
        language: Language,
        #[clap(short, long)]
        include_options: bool,
        #[clap(short, long)]
        detail: bool,
        #[clap(value_parser)]
        otp_key: Option<String>,
        #[clap(value_parser, multiple_occurrences(true))]
        words: Vec<String>,
    },
    /// Decrypt seed words using an OTP key.
    Decrypt {
        #[clap(short, long)]
        language: Language,
        #[clap(short, long)]
        include_options: bool,
        #[clap(short, long)]
        detail: bool,
        #[clap(value_parser)]
        otp_key: Option<String>,
        #[clap(value_parser, multiple_occurrences(true))]
        words: Vec<String>,
    },
}
