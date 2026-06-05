use std::sync::{Arc, LazyLock, RwLock};

use worker::*;

use flarecord::bot::{Bot, builder::BotBuilder};

mod commands;
use commands::hello::Hello;

static BOT: LazyLock<Arc<RwLock<Bot>>> = LazyLock::new(|| {
    BotBuilder::new()
        .register_command(Box::new(Hello))
        .build()
});

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let bot = BOT.read().map_err(|_| Error::Infallible)?;

    bot.handle(req, env).await
}