// Load rust dependencies
use std::{env, time::Duration};

// S L A S H  C O M M A N D S
use poise::{
    serenity_prelude::{self as serenity},
    FrameworkOptions,
};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[allow(dead_code)]
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
struct Data {}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e) // lol
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let options = FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        listener: |_ctx, event, _framework, _data| {
            // This is a custom event handler
            // It is called for every event that the framework receives
            // We can use it to log events, or do other things

            Box::pin(async move {
                // We can also return a future to be run after the event is handled
                // This is useful for things like logging
                // We can also return an error to stop the event from being handled

                match event {
                    poise::Event::Ready { data_about_bot } => {
                        println!("Ready! Logged in as {}", data_about_bot.user.name);
                        println!("Session ID: {}", data_about_bot.session_id);

                        _ctx.set_activity(serenity::Activity::watching("sticks & sham cry"))
                            .await;
                    }
                    _ => {}
                };

                Ok(())
            })
        },
        commands: vec![register(), h(), age()],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(token)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
    println!("Client started");
}

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// h
#[poise::command(prefix_command, slash_command)]
async fn h(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("h").await?;
    Ok(())
}
