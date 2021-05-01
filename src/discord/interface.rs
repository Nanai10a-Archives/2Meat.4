use crate::commands::{DropCommand, NewCommand};
use crate::discord::transceiver::{
    DiscordTransceiver, DiscordTransceivers, Transceiver, Transceivers,
};
use crate::discord::transferer::Transferer;
use crate::interface::Interface;
use crate::model::arg::CommandArgs;
use crate::model::data::{Author, FormattedData, Place};
use crate::utils::RefWrap;
use clap::clap_app;
use serenity::builder::{
    CreateInteraction, CreateInteractionResponse, CreateInteractionResponseData,
};
use serenity::http::Http;
use serenity::model::prelude::{
    ApplicationCommand, ApplicationCommandOptionType, ChannelId, GuildId, Interaction, Message,
    MessageType, Ready,
};
use serenity::prelude::{Context, EventHandler, Mutex};
use serenity::Error;
use std::fmt::Display;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;
use uuid::Uuid;

pub struct DiscordInterface {
    data_sender: broadcast::Sender<FormattedData>,
    data_receiver: broadcast::Receiver<FormattedData>,
    transferer: Arc<Transferer>,
    command_parser: clap::App<'static>,
    serenity_ctx: RefWrap<Context>,
}

impl DiscordInterface {
    async fn post_slash_command<F>(
        &self,
        post_to: PostTo,
        http: impl AsRef<Http>,
        id: u64,
    ) -> anyhow::Result<ApplicationCommand> {
        let res: Result<ApplicationCommand, serenity::Error> = match post_to {
            PostTo::Global => {
                Interaction::create_global_application_command(http, id, |ci| {
                    create_interaction(ci)
                })
                .await
            }

            PostTo::Guild(guild_id) => {
                Interaction::create_guild_application_command(http, guild_id, id, |ci| {
                    create_interaction(ci)
                })
                .await
            }
        };

        match res {
            Ok(command) => Ok(command),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }

    pub async fn msg_is_command(&self, msg: &Message) -> anyhow::Result<bool> {
        if msg.author.bot {
            return Ok(false);
        }

        let res = self
            .command_parser
            .clone()
            .try_get_matches_from(split_raw_command(msg.content.clone()).await)
            .is_ok();
        Ok(res)
    }

    pub fn is_ia_command(&self, ia: &Interaction) -> anyhow::Result<bool> {
        // FIXME: このMember.user.botはAuthorを指しているのか?
        Ok(!ia.member.user.bot)
    }

    pub async fn on_msg_command(&self, ctx: Context, msg: Message) -> anyhow::Result<Message> {
        let res = msg
            .channel_id
            .say(ctx.http, self.on_command_process(todo!()).await.unwrap())
            .await
            .unwrap();

        Ok(res)
    }

    pub async fn on_ia_command(&self, ctx: Context, ia: Interaction) -> anyhow::Result<()> {
        let res = self.on_command_process(todo!()).await.unwrap();

        ia.create_interaction_response(ctx.http, |cir: &mut CreateInteractionResponse| {
            cir.interaction_response_data(|cird: &mut CreateInteractionResponseData| {
                cird.content(res)
            })
        })
        .await
        .unwrap();

        Ok(())
    }

    async fn on_command_process(&self, arg: CommandArgs) -> anyhow::Result<String> {
        match arg {
            CommandArgs::New => {
                todo!()
            }
            CommandArgs::Drop { id } => {
                todo!()
            }
            CommandArgs::Subsc {
                broadcaster_id: brcs_id,
                subscriber_id: sbsc_id,
            } => {
                todo!()
            }
            CommandArgs::Exit {
                broadcaster_id: brcs_id,
                subscriber_id: sbsc_id,
            } => {
                todo!()
            }
            _ => todo!(),
        }
        todo!()
    }

    pub async fn on_receive(&self, ctx: Context, msg: Message) -> anyhow::Result<()> {
        self.receive(FormattedData {
            content: msg.content.as_str().into(),
            attachments: vec![],
            author: Author {
                name: msg.author.name.as_str().into(),
                nickname: msg.author_nick(ctx.http).await,
                id: msg.author.id.0.to_string(),
                place: Place::Discord {
                    channel_id: msg.channel_id.0,
                },
            },
            additional_contents: None,
            timestamp: msg.timestamp,
        })
        .await
    }

    pub async fn on_send(
        &self,
        channel_id: ChannelId,
        content: impl Display,
    ) -> anyhow::Result<Message> {
        let res = channel_id
            .say(
                (*self.serenity_ctx.clone())
                    .lock()
                    .await
                    .as_ref()
                    .unwrap()
                    .http
                    .clone(),
                content,
            )
            .await;

        match res {
            Ok(item) => Ok(item),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }

    // 受け
    pub async fn receive(&self, data: FormattedData) -> anyhow::Result<()> {
        self.data_sender.send(data).unwrap();
        Ok(())
    }

    // 攻め
    pub async fn send(&self, data: FormattedData) -> anyhow::Result<()> {
        if let Place::Discord { channel_id } = data.author.place {
            ChannelId(channel_id)
                .say(
                    self.serenity_ctx.as_ref().lock().await.as_ref().unwrap(),
                    format!(
                        "```\
{}\
```\
",
                        data
                    ),
                )
                .await;
            return Ok(());
        }
        todo!()
    }

fn create_interaction(ci: &mut CreateInteraction) -> &mut CreateInteraction {
    ci.name("2c-tr")
        .description("2Meat Discord Interface: Transceiver")
        .create_interaction_option(|cio| {
            // New
            cio.name("new")
                .description("create transceiver.")
                .kind(ApplicationCommandOptionType::SubCommandGroup)
                .create_sub_option(|cio| {
                    cio.name("there")
                        .description("create transceiver: bind this channel.")
                        .kind(ApplicationCommandOptionType::SubCommand)
                })
        })
        // Mut
        // TODO: this handling
        // .create_interaction_option(|cio| {cio.name("mut")})
        // Drop
        .create_interaction_option(|cio| {
            cio.name("drop")
                .description("drop transceiver from id.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|cio| {
                    cio.name("id")
                        .description("target's id")
                        .kind(ApplicationCommandOptionType::String)
                })
        })
        // Subsc
        .create_interaction_option(|cio| {
            cio.name("subsc")
                .description("subsc from id.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|cio| {
                    cio.name("sbsc_id")
                        .description("subscriber's id")
                        .kind(ApplicationCommandOptionType::String)
                })
                .create_sub_option(|cio| {
                    cio.name("brcs_id")
                        .description("broadcaster's id")
                        .kind(ApplicationCommandOptionType::String)
                })
        })
        // Exit
        .create_interaction_option(|cio| {
            cio.name("exit")
                .description("exit send at recv from id.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|cio| {
                    cio.name("sbsc_id")
                        .description("subscriber's id")
                        .kind(ApplicationCommandOptionType::String)
                })
                .create_sub_option(|cio| {
                    cio.name("brcs_id")
                        .description("broadcaster's id")
                        .kind(ApplicationCommandOptionType::String)
                })
        })
}

fn create_command_parser() -> clap::App<'static> {
    clap_app!("/2c-tr" =>
        (about: "2Meat Discord Interface: Transceiver")
        (@subcommand "new" =>
            (about: "create transceiver.")
            (@arg PLACE: +required "set there/[coming soon...]")
        )
        // (@subcommand "mut" =>
        //     (about: "mut send/recv from id.")
        //     (@arg ID: +required "send/recv's id")
        // )
        (@subcommand "drop" =>
            (about: "drop from id.")
            (@arg ID: +required "transceiver's id")
        )
        (@subcommand "subsc" =>
            (about: "subsc from id.")
            (@arg SBSC_ID: +required "subscriber's id")
            (@arg BRCS_ID: +required "broadcaster's id")
        )
        (@subcommand "exit" =>
            (about: "exit from id.")
            (@arg SBSC_ID: +required "subscriber's id")
            (@arg BRCS_ID: +required "broadcaster's id")
        )
    )
}

pub enum PostTo {
    Global,
    Guild(GuildId),
}

#[serenity::async_trait]
impl EventHandler for DiscordInterface {
    async fn message(&self, ctx: Context, msg: Message) {
        if MessageType::Regular != msg.kind {
            return;
        }

        if self.msg_is_command(&msg).await.unwrap() {
            self.on_msg_command(ctx, msg).await.unwrap();
            return;
        }

        self.on_receive(ctx, msg).await.unwrap()
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        *self.serenity_ctx.lock().await = Some(ctx);
        println!("{:?}", ready);
    }

    async fn interaction_create(&self, ctx: Context, ia: Interaction) {
        if self.is_ia_command(&ia).unwrap() {
            self.on_ia_command(ctx, ia).await.unwrap();
        }
    }
}
