# koval-telegram-bot
Telegram bot for Japanese to Ukrainian transliteration using my wana-kana-rust fork. Built with teloxide. Supports inline queries.

## Setting up your environment

 1. [Download Rust](http://rustup.rs/).
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 3. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows command line
$ set TELOXIDE_TOKEN=<Your token here>

# Windows PowerShell
$ $env:TELOXIDE_TOKEN=<Your token here>
```

 4. Make sure that your Rust compiler is up to date (`teloxide` currently requires rustc at least version 1.68):
```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```

 5. Run `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:
```toml
[dependencies]
teloxide = { version = "0.12", features = ["macros"] }
log = "0.4"
pretty_env_logger = "0.5"
tokio = { version =  "1.8", features = ["rt-multi-thread", "macros"] }
```

