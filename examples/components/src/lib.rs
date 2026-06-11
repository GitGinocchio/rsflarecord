use std::sync::{Arc, LazyLock};

use worker::*;

use flarecord::bot::{Bot, builder::BotBuilder};

mod components;

mod commands;
use commands::hello::Hello;

use crate::components::mycomponent::MyComponent;

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    BotBuilder::new()
        .register_component(MyComponent)
        .register_command(Hello)
        .build()
});

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    BOT.sync_commands_once(&env).await?;

    BOT.handle(req, env).await
}