use daichi::*;
use poise::futures_util::StreamExt;
use serenity::ChannelType;

pub async fn channel_select(ctx: Context<'_>) -> Result<Option<serenity::ChannelId>> {
    let buttons = vec![
        serenity::CreateActionRow::SelectMenu(serenity::CreateSelectMenu::new(
            "channel",
            serenity::CreateSelectMenuKind::Channel {
                channel_types: Some(vec![ChannelType::Voice]),
                default_channels: None,
            },
        )),
        serenity::CreateActionRow::Buttons(vec![serenity::CreateButton::new("no").label("no")]),
    ];

    let msg = ctx
        .send(
            poise::CreateReply::default()
                .content("Do you want to specifiy an afk channel that will count as being offline")
                .reply(true)
                .components(buttons),
        )
        .await?;

    let mut reactions = msg
        .message()
        .await?
        .await_component_interactions(&ctx.serenity_context().shard)
        .stream();

    while let Some(reaction) = reactions.next().await {
        if &reaction.user == ctx.author() {
            let afk_channel = match reaction.data.kind {
                serenity::ComponentInteractionDataKind::Button => None,
                serenity::ComponentInteractionDataKind::ChannelSelect { values } => {
                    Some(values.last().copied().unwrap())
                }
                _ => None,
            };

            msg.delete(ctx).await?;

            return Ok(afk_channel);
        }
    }

    Ok(None)
}

pub async fn role_select(ctx: Context<'_>, text: impl Into<String>) -> Result<serenity::RoleId> {
    let buttons = vec![serenity::CreateActionRow::SelectMenu(
        serenity::CreateSelectMenu::new(
            "role",
            serenity::CreateSelectMenuKind::Role {
                default_roles: None,
            },
        ),
    )];

    let msg = ctx
        .send(
            poise::CreateReply::default()
                .content(text)
                .reply(true)
                .components(buttons),
        )
        .await?;

    let mut reactions = msg
        .message()
        .await?
        .await_component_interactions(&ctx.serenity_context().shard)
        .stream();

    while let Some(reaction) = reactions.next().await {
        if &reaction.user == ctx.author() {
            let role_to_watch =
                if let serenity::ComponentInteractionDataKind::RoleSelect { values } =
                    reaction.data.kind
                {
                    values.last().copied().unwrap()
                } else {
                    return Err(Error::from_any(std::fmt::Error));
                };

            msg.delete(ctx).await?;

            return Ok(role_to_watch);
        }
    }

    Err(Error::from_any(std::fmt::Error))
}

pub async fn role_select_opt(
    ctx: Context<'_>,
    text: impl Into<String>,
) -> Result<Option<serenity::RoleId>> {
    let buttons = vec![
        serenity::CreateActionRow::SelectMenu(serenity::CreateSelectMenu::new(
            "role",
            serenity::CreateSelectMenuKind::Role {
                default_roles: None,
            },
        )),
        serenity::CreateActionRow::Buttons(vec![serenity::CreateButton::new("no").label("no")]),
    ];

    let msg = ctx
        .send(
            poise::CreateReply::default()
                .content(text)
                .reply(true)
                .components(buttons),
        )
        .await?;

    let mut reactions = msg
        .message()
        .await?
        .await_component_interactions(&ctx.serenity_context().shard)
        .stream();

    while let Some(reaction) = reactions.next().await {
        if &reaction.user == ctx.author() {
            let role_to_watch = match reaction.data.kind {
                serenity::ComponentInteractionDataKind::Button => None,
                serenity::ComponentInteractionDataKind::RoleSelect { values } => {
                    Some(values.last().copied().unwrap())
                }
                _ => None,
            };

            msg.delete(ctx).await?;

            return Ok(role_to_watch);
        }
    }

    Ok(None)
}

pub async fn bool_select(ctx: Context<'_>, text: impl Into<String>) -> Result<bool> {
    let text: String = text.into();

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("yes").label("yes"),
        serenity::CreateButton::new("no").label("no"),
    ])];

    let msg = ctx
        .send(
            poise::CreateReply::default()
                .content(text)
                .reply(true)
                .components(buttons),
        )
        .await?;

    let mut reactions = msg
        .message()
        .await?
        .await_component_interactions(&ctx.serenity_context().shard)
        .stream();

    while let Some(reaction) = reactions.next().await {
        if &reaction.user == ctx.author() {
            msg.delete(ctx).await?;
            return Ok(&reaction.data.custom_id == "yes");
        }
    }

    Ok(false)
}
