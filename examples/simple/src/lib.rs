use worker::*;

use flarecord::{bot::builder::BotBuilder, prelude::*};

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let bot = BotBuilder::new()
        .register_command_handler("Hello", "Say Hi to someone in chat!", async move |_interaction, _ctx| {
            let response = CommandResponse::new();

            Ok(response)
        })
        .build();

    bot.handle(req, env).await
}