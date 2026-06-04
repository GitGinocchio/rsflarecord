use worker::*;

use flarecord::prelude::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let mut bot = Bot::new();

    bot.register_command_handler("Hello", "Say Hi to someone in chat!", async move |_interaction, _data, _env| {
        let response = CommandResponse::new();

        Ok(response)
    })?;

    bot.handle(req, env).await
}