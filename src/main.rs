use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let photo1 = teloxide::types::InputFile::url("https://sun9-68.userapi.com/impg/NOfMVGE0BOZ_hmkf8SdkMYJs_XxHlx2I5RHfew/bTs_SxHhqYk.jpg?size=1080x1080&quality=95&sign=f979aa78dff451628ac1b0c17db46d97&type=album".parse().unwrap());
        bot.send_message(message.chat.id,"Добро пожаловать! Вы запустили бота на Rust!").send().await?;
        match message.text().unwrap(){
            "/help" => bot.send_message(message.chat.id,"Why?").send().await?,
            _ => {}
        }
        if message.text().unwrap() == "/help"{
        }
        respond(())
    })
        .await;
}