use std::collections::HashSet;

use crate::Result;
use miette::{Context as _, IntoDiagnostic};
use serenity::{
    all::{ChannelId, Message, UserId},
    async_trait,
    prelude::*,
};

pub fn channels_of_interest() -> Result<HashSet<ChannelId>> {
    std::env::var("DISCORD_CHANNEL_IDS")
        .into_diagnostic()?
        .split(",")
        .map(|c| {
            u64::from_str_radix(c.trim(), 10)
                .into_diagnostic()
                .context(format!("Invalid channel ID: {c}"))
        })
        .try_fold(HashSet::new(), |mut set, channel| {
            set.insert(ChannelId::from(channel?));
            Ok(set)
        })
}

#[derive(Debug)]
pub struct HoneyPotBot {
    bot_id: UserId,
    /// Channels of Interest are channels that the bot will operate on.
    coi: HashSet<ChannelId>,
    excluded_users: HashSet<UserId>,
}

impl HoneyPotBot {
    pub fn new(
        honey_pot_channels: Option<HashSet<ChannelId>>,
        excluded_users: Option<HashSet<UserId>>,
    ) -> Result<Self> {
        let bot_id: UserId = u64::from_str_radix(dotenvy_macro::dotenv!("DISCORD_BOT_ID"), 10)
            .into_diagnostic()?
            .into();
        let coi = honey_pot_channels.unwrap_or(channels_of_interest()?);
        let excluded_users = excluded_users.unwrap_or_default();
        Ok(Self {
            bot_id,
            coi,
            excluded_users,
        })
    }

    pub fn excluded_users(&self) -> &HashSet<UserId> {
        &self.excluded_users
    }

    pub fn honey_potted_channels(&self) -> &HashSet<ChannelId> {
        &self.coi
    }

    pub fn bot_id(&self) -> UserId {
        self.bot_id
    }
}
// TODO: Add caching
#[async_trait]
impl EventHandler for HoneyPotBot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == self.bot_id {
            return;
        }

        let http = ctx.http();

        if let Some(gid) = msg.guild_id {
            let guild = http.get_guild(gid).await.unwrap();
            let owner = http.get_user(guild.owner_id).await.unwrap();
            // If owner exit early
            if guild.owner_id == msg.author.id {
                eprintln!("Owner {} messaged: {}", owner.display_name(), msg.content);
                return;
            }
            if self.coi.contains(&msg.channel_id) && !self.excluded_users().contains(&msg.author.id)
            {
                let channel = http.get_channel(msg.channel_id).await.unwrap();
                match http.ban_user(gid, msg.author.id, 0, None).await {
                    Ok(_) => println!(
                        "User ({}) was banned from '{}' by messaging in channel {}",
                        msg.author.display_name(),
                        guild.name,
                        channel.guild().unwrap().name()
                    ),
                    Err(e) => eprintln!("{e}"),
                };

                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        &format!(
                            "{} YOU'RE BANNED from {}",
                            msg.author.display_name(),
                            guild.name
                        ),
                    )
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
        } else {
            eprint!("No Guild to ban from.");
        }
    }
}
