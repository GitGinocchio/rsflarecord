use worker::*;

use flarecord::{bot::Bot, models::command::response::CommandResponse};

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let mut bot = Bot::new();

    bot.register_command_handler("Hello", "Say Hi to someone in chat!", async move |_interaction, _data, _env| {
        let response = CommandResponse::new();

        Some(Ok(response))
    })?;

    match bot.handle(req, env).await {
        Ok(response) => Ok(response),
        Err(e) => e.as_response()
    }
}