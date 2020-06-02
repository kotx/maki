use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult, CommandError};
use serenity::model::prelude::*;
use serenity::prelude::*;
use aspotify::{CCFlow, ClientCredentials, model::ItemType::Track};

#[command]
#[aliases(s, sp, spot)]
#[description("Gets things from Spotify. Defaults to \"songs\".\nSubcommands: `songs`")]
#[sub_commands(SPOTIFY_SONGS)]
async fn spotify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    spotify_songs(ctx, msg, args).await?;
    Ok(())
}

#[command("songs")]
#[aliases(s)]
async fn spotify_songs(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let flow =
        CCFlow::new(ClientCredentials::from_env().expect("CLIENT_ID and CLIENT_SECRET not found."));

    let result = aspotify::search(
            &flow.send().await.unwrap(),
            args.rest(),
            &[Track], false, 1, 0, None
        ).await.unwrap();

        if result.clone().tracks.unwrap().items.len() == 0 {
            msg.channel_id.say(&ctx.http, "No songs were found that matched the input.").await?;
            return Err(CommandError::from("h-No songs were found."));
        }

    let _ = msg.channel_id.send_message(&ctx.http, |m| {

        let desc = "by ".to_string()
            + &result.clone().tracks.unwrap().items[0].artists[0].name.to_string()
            + "\non "
            + &result.clone().tracks.unwrap().items[0].album.name.to_string()
            + "\n[view on Spotify >]("
            + &result.clone().tracks.unwrap().items[0].external_urls["spotify"].to_string()
            + ")";

        m.embed(|e| {
            e.author(|a| {
                a.name(&format!("{}", &result.clone().tracks.unwrap().items[0].name))
                    .url(&result.clone().tracks.unwrap().items[0].external_urls["spotify"].to_string())
            })
            .color(0xb90000)
            .description(desc)
            .thumbnail(&result.clone().tracks.unwrap().items[0].album.images[0].url)
            .footer(|f| f.text(format!("Data from Spotify Web API")))
        })
    }).await;

    Ok(())
}