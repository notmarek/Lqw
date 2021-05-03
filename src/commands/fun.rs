use crate::utils::nhentai::Gallery;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[group]
#[commands(nhentai)]
struct Fun;

#[command]
#[help_available]
#[num_args(1)]
#[aliases("nh")]
#[description("A command to get info from a nhentai id")]
async fn nhentai(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // let tags = gallery.tags.;
    let channel = msg.channel(ctx).await.unwrap();
    if !channel.is_nsfw() {
        msg.channel_id
            .send_message(&ctx.http, |builder| {
                builder
                    .reference_message(msg)
                    .allowed_mentions(|f| f.replied_user(true));
                builder.embed(|e| {
                    e.colour(0xff0069).title("Error~").field(
                        "Missing nsfw tag.",
                        "This command is only available in NSFW tagged channels.",
                        false,
                    )
                })
            })
            .await?;
    } else {
        let nhentai_id = args.single::<i64>()?;
        let typing = msg.channel_id.start_typing(&ctx.http)?;
        let gallery: Gallery = Gallery::get(nhentai_id).await?;
        msg.channel_id
            .send_message(&ctx.http, |builder| {
                builder
                    .reference_message(msg)
                    .allowed_mentions(|f| f.replied_user(true));
                builder.embed(|e| {
                    e.colour(0xff0069)
                        .title(&gallery.title.pretty)
                        .field(
                            "Link",
                            format!("https://nhentai.net/g/{}", nhentai_id),
                            false,
                        )
                        .field("Tags", format!("{}", gallery.get_tags()), false)
                })
            })
            .await?;
        typing.stop();
    }
    Ok(())
}
