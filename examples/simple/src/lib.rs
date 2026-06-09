/// Quick Start Example - Flarecord Discord Bot
/// 
/// This example shows how to create a simple bot with inline command handlers.
/// Perfect for getting started quickly without complex command structs.
/// 
/// See docs/COMMANDS.md and docs/BOT.md for more details.

use worker::*;
use flarecord::{bot::builder::BotBuilder, prelude::*};

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> worker::Result<Response> {
    // Build bot with inline command handler
    let bot = BotBuilder::new()
        // Register a simple command that says hello
        .register_command_handler(
            "hello",
            "Say hello to someone in chat!",
            |_interaction, _ctx| async {
                Ok(CommandResponseBuilder::new()
                    .content("👋 Hello! Thanks for using Flarecord!")
                    .build())
            }
        )
        // You can add more commands here
        .register_command_handler(
            "ping",
            "Check if bot is responsive",
            |_interaction, _ctx| async {
                Ok(CommandResponseBuilder::new()
                    .content("🏓 Pong!")
                    .build())
            }
        )
        .build();

    // Handle the Discord interaction
    bot.handle(req, env).await
}

/// 
/// To deploy this bot:
/// 1. Configure Cloudflare: `wrangler init`
/// 2. Set secrets: `wrangler secret put DISCORD_BOT_TOKEN` etc.
/// 3. Build: `cargo build --target wasm32-unknown-unknown --release`
/// 4. Deploy: `wrangler deploy`
/// 
/// See GETTING_STARTED.md for complete setup instructions
///