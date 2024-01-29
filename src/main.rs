use teloxide::{prelude::*, types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText, MessageEntity}, utils::command::BotCommands};
use wana_kana::{ConvertJapanese, Options};

extern crate wana_kana;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    let _ = bot.set_my_commands(Command::bot_commands()).await;

    // Command::repl(bot, answer).await;

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(answer),
                )
                .branch(
                    dptree::filter(|m: Message| m.text().is_some())
                        .endpoint(answer_text),
                ),
        )
        .branch(
            Update::filter_inline_query()
                .endpoint(handle_inline),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Підтримувані команди:")]
enum Command {
    #[command(description = "довідка яка виводить цей опис.")]
    Help,
    #[command(description = "транслітерує хірагану/катакану українською мовою.")]
    Translit(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Translit(name) => {
            bot.send_message(msg.chat.id, transliterate(name)).await?
        }
    };

    Ok(())
}

async fn answer_text(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.text() {
        Some(word) => bot.send_message(msg.chat.id, transliterate(word.to_string())).await?,
        None => bot.send_message(msg.chat.id, "Що?").await?,
    };

    Ok(())
}

fn transliterate(word: String) -> String {
    word.to_ukrainian_with_opt(
        Options {
            imemode: false,
            ..Default::default()
        }
    )
}


pub async fn handle_inline(bot: Bot, query: InlineQuery,
) -> ResponseResult<()> {

    let transliteration = query.query.to_ukrainian_with_opt(
        Options {
            imemode: true,
            ..Default::default()
        }
    );

    if !transliteration.is_empty() {

        //offset might go out of bounds due to the implicit encoding depending on characters.
        let copytext = MessageEntity::code(0, transliteration.encode_utf16().count());

        let prepared_message = InputMessageContentText::new(
            (transliteration).to_string()).entities(vec![copytext]);
    
        let article = InlineQueryResultArticle::new(
            // Each item needs a unique ID, as well as the response container for the
            // items. These can be whatever, as long as they don't
            // conflict.
            "01".to_string(),
            // What the user will actually see
            format!(
                "{} — {}",
                query.query,
                transliteration
            ),
            // What message will be sent when clicked/tapped
            InputMessageContent::Text(
                prepared_message
            )
        );
    
        let results = vec![
            InlineQueryResult::Article(article)
        ];
    
        bot.answer_inline_query(&query.id, results).cache_time(0).await?;
    }

    Ok(())
}

