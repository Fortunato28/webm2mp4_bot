use teloxide::prelude::*;
use std::fs::File;
use std::io::Write;

pub async fn download_resource(link: &str)
{
    let client = reqwest::Client::new();
    let response = client.get(link).send().await.expect("Problem while GET request");
    let body = response.bytes().await.expect("Problem while getting response body");

    write_to_file("test", &body);
}

fn write_to_file(filename: &str, body: &[u8])
{
    let mut destination = File::create(filename).expect("Problem while create  file");
    destination.write_all(body);
}

pub async fn run() {
teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                message.answer("pong").send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
}
