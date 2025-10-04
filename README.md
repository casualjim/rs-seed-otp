# seed-otp

A Rust port of [brndnmtthws/seed-otp](https://github.com/brndnmtthws/seed-otp) - a one-time pad CLI tool for storing your Bitcoin seed mnemonic words securely using multi-factor authentication.

This implementation provides the same cryptographic functionality as the original Python version, with the added benefits of Rust's performance, memory safety, and zero-cost abstractions.

## Background

### The Problem

You have an HD wallet such as a Trezor or Ledger for storing your Bitcoin, and you would like to store your seed mnemonic phrase. You may also want to store multiple copies of your seed in different places. Unfortunately, if any one of those copies of your seed becomes compromised, anyone with access to the seed can now take all your coins.

Normally you would not need access to your seed mnemonic. However, should something happen to your wallet (perhaps you lose it, or it breaks), you may need to restore the wallet using the seed phrase.

### This Solution

Combine a [one-time pad](https://en.wikipedia.org/wiki/One-time_pad) with [multi-factor authentication](https://en.wikipedia.org/wiki/Multi-factor_authentication).

By using multi-factor auth (something you know plus something you have) and one-time pad encryption, you have a simple yet extremely hard to crack solution. With your OTP key and seed mnemonic stored separately, it becomes onerous to obtain both. Even if someone _does_ obtain either your mnemonic or OTP key, you would have time to move your coins to a new wallet with a brand new seed before anything happens to your coins. A one-time pad is considered perfect secrecy: it's nearly impossible to brute force attack so long as the key remains secret.

Your auth factors are:

- **Something you know**: A one-time pad key which you have stored securely in a password manager, which is locked with a password only you know. The password DB is backed up securely.
- **Something you have**: An encrypted mnemonic seed phrase stored on archival paper or another long term physical cold storage device. The phrase itself looks like a normal mnemonic phrase, which provides plausible deniability, and does not indicate to anyone who might find the phrase _how_ it's actually used.

### Caveats, Limitations, Considerations, Gotchas

- To use this tool, you need to enter the seed words into a computer. If your computer is compromised, someone could still use a keylogger or other tool to capture the seed mnemonic. Only use this tool if you trust the computer you are using.
- The BIP-0039 mnemonic includes a checksum. After encrypting the words, the checksum will break. Encrypted seed words are unlikely to be valid. This may be a problem since it breaks the plausible deniability of storing the encrypted seed words (as the encrypted mnemonic is not actually a valid phrase). The disadvantages of handling the checksum gracefully is that it's backward incompatible, and it would be much more difficult to apply the OTP by hand using pen and paper.
- The OTP encoding (see [the "OTP key" section](#otp-key) below) does not include any version/format metadata. The reason for doing this is to reduce the amount of information in the key which could be used to derive some other information (i.e., reduces the degree to which it is [information-theoretically secure](https://en.wikipedia.org/wiki/Information-theoretic_security)). The trade-off, of course, is that it's difficult to modify the key format and maintain backward compatibility.

## Installation

### From Source

```bash
cargo build --release
```

The binary will be available at `target/release/seed-otp`.

### Using Cargo

```bash
cargo install --path .
```

## Quickstart

### Checklist

Before using this tool, you should have a few things:

- [x] Get a decent hardware wallet from a reputable vendor. 2 popular options are [Trezor](https://trezor.io/) and [Ledger](https://www.ledger.com/).
- [x] Get a password manager, and learn to use it (if you haven't already). A few good options are [KeePassX](https://www.keepassx.org/), [1Password](https://1password.com/), or [BitWarden](https://bitwarden.com/). Make sure your passwords are backed up, and test the restore process.
- [x] Figure out a good way to store your mnemonic seed phrase, such as using archival paper or a metal seed storage product.
- [x] Have a safe place to store the seed mnemonic, such as in an actual safe, or a safe deposit box.
- [x] Make sure you have a secure computer to run the software. It should be running an up-to-date and secure OS. Avoid using any computers which might be controlled by third parties (such as a work computer, or your friend's computer). If you want to be extra safe, consider using a [privacy OS such as Tails](https://tails.boum.org/)

After using the tool, make sure you **test the seed restore process**!

### Generate an OTP key

```bash
$ seed-otp generate 12
{
  "otp-key": "AAwCnwGIAe0EWABWAI4AkAMjAFQBLgZjB1T1PJtz",
  "success": true
}
```

Store the key above in your password management tool.

### Encrypt your seed mnemonic using the OTP

```bash
$ seed-otp encrypt --language english AAwCnwGIAe0EWABWAI4AkAMjAFQBLgZjB1T1PJtz abandon ability able about above absent absorb abstract absurd abuse access accident
{
  "encrypted-words": [
    "fault",
    "couple",
    "digital",
    "merge",
    "area",
    "bar",
    "barrel",
    "grab",
    "argue",
    "cheap",
    "soap",
    "typical"
  ],
  "success": true
}
```

Store the phrase above in your safe place.

### Decrypt your seed mnemonic using the OTP

```bash
$ seed-otp decrypt --language english AAwCnwGIAe0EWABWAI4AkAMjAFQBLgZjB1T1PJtz fault couple digital merge area bar barrel grab argue cheap soap typical
{
  "encrypted-words": [
    "abandon",
    "ability",
    "able",
    "about",
    "above",
    "absent",
    "absorb",
    "abstract",
    "absurd",
    "abuse",
    "access",
    "accident"
  ],
  "success": true
}
```

## Usage

```
Usage: seed-otp <COMMAND>

Commands:
  generate   Generate a secure OTP key for up to NUM_WORDS number of words
  check-key  Check OTP key for encoding or checksum errors
  encrypt    Encrypt seed words using an OTP key
  decrypt    Decrypt seed words using an OTP key
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Generate Command

Generate a secure OTP key for up to NUM_WORDS number of words. Make sure NUM_WORDS is at least as large as the number of seed words you use. It is normally 12 or 24 words. If you are unsure, use 24.

```bash
seed-otp generate [NUM_WORDS]
```

### Check Key Command

Check OTP key for encoding or checksum errors:

```bash
seed-otp check-key [OTP_KEY]
```

### Encrypt Command

Encrypt seed words using an OTP key:

```bash
seed-otp encrypt --language <LANGUAGE> [OPTIONS] <OTP_KEY> <WORDS>...

Options:
  -l, --language <LANGUAGE>   Language of the wordlist [possible values: english, french, italian, spanish, japanese, korean, chinese-simplified, chinese-traditional]
  -i, --include-options      Include options in output
  -d, --detail              Include detailed mapping in output
```

### Decrypt Command

Decrypt seed words using an OTP key:

```bash
seed-otp decrypt --language <LANGUAGE> [OPTIONS] <OTP_KEY> <WORDS>...

Options:
  -l, --language <LANGUAGE>   Language of the wordlist [possible values: english, french, italian, spanish, japanese, korean, chinese-simplified, chinese-traditional]
  -i, --include-options      Include options in output
  -d, --detail              Include detailed mapping in output
```

## Supported Languages

This tool supports BIP-0039 wordlists in multiple languages:

- English (default)
- French
- Italian
- Spanish
- Japanese
- Korean
- Chinese (Simplified)
- Chinese (Traditional)

To use a different language, enable the corresponding feature when building:

```bash
cargo build --release --features french
```

Or enable multiple features:

```bash
cargo build --release --features "english,french,spanish"
```

## Implementation Details

### OTP Key

The OTP key is a URL-safe base64 encoded key (without padding) composed of N subkeys, where N is the number of keys specified at creation time. The values are stored as big-endian short unsigned integers (2-bytes each). The last 4 bytes of the OTP key is the first 4 bytes of the SHA256 digest of the preceding bytes.

```
0                   1
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|         Number of Keys        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
\                               /
/   Keylist (variable length)   \
\                               /
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                               |
+            Checksum           +
|                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

BIP-0039 uses 11 bits per word, but in this scheme we're using 16 bits per word. This is mainly for simplicity, with the trade-off of using more bytes. It also allows the possibility of using larger wordlists (of up to 65536 words).

### Encrypting/Decrypting Words

Below is some pseudocode for encrypting/decrypting. Assume that the words and keys are mapped to integers representing an index position in the wordlist.

To encrypt a word:

```
ciphertext = (word + key) mod 2048
```

To decrypt a word:

```
word = (ciphertext - key) mod 2048
```

You could perform the encryption/decryption using pen and paper if you feel the need to do so. This would prevent the necessity of typing your seed words into a computer. Naturally, you could also generate your own keys and store those offline as well. For practical purposes, however, this is probably unnecessary.

## Compatibility

This Rust implementation is fully compatible with the original Python version. OTP keys generated by either version can be used with the other, ensuring seamless migration and interoperability.

## License

MIT License - Copyright (c) 2022 Ivan Porto Carrero

See [LICENSE](LICENSE) file for details.

## Acknowledgments

This is a Rust port of the original [seed-otp](https://github.com/brndnmtthws/seed-otp) by [Brenden Matthews](https://github.com/brndnmtthws).

## Security Notice

This tool is provided as-is for educational and personal use. Always verify the integrity of the software you use to manage cryptocurrency seeds. Never trust any tool blindly with your financial security. Make sure to:

- Review the source code yourself
- Test the encryption/decryption process with test seeds before using with real seeds
- Keep backups of both your OTP key and encrypted seed in separate secure locations
- Never share your OTP key or unencrypted seed with anyone
