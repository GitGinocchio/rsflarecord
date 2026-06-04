use worker::*;

use flarecord::bot::Bot;

mod commands;
use commands::hello::Hello;

#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    let mut bot = Bot::new();

    bot.register_command(Box::new(Hello))?;

    Response::ok("Hello World!")
}