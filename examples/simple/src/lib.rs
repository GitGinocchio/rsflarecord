use worker::*;

use flarecord::prelude::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let bot = Bot::new();

    let mut bot_guard = bot.write().map_err(|_| Error::Infallible)?;

    bot_guard.register_command_handler("Hello", "Say Hi to someone in chat!", async move |_interaction, _ctx| {
        let response = CommandResponse::new();

        Ok(response)
    });

    bot_guard.handle(req, env).await
}