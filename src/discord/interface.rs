use serenity::builder::{
    CreateInteraction, CreateInteractionResponse, CreateInteractionResponseData,
};
use serenity::http::Http;
use serenity::model::prelude::{
    ApplicationCommand, ApplicationCommandOptionType, GuildId, Interaction, Message, MessageType,
};
use serenity::prelude::{Context, EventHandler};
use tokio::sync::broadcast;

use crate::commands;
use crate::discord::receivers::{DiscordReceiver, DiscordReceivers};
use crate::discord::senders::{DiscordSender, DiscordSenders};
use crate::discord::transferer::Transferer;
use crate::interface::Interface;
use crate::model::arg::{CommandArgs, Target};
use crate::model::data::{Author, FormattedData, Place};
use crate::utils::RefWrap;
use clap::clap_app;
use std::sync::Arc;

pub struct DiscordInterface {
    data_sender: broadcast::Sender<FormattedData>,
    data_receiver: broadcast::Receiver<FormattedData>,
    senders: Arc<DiscordSenders>,
    receivers: Arc<DiscordReceivers>,
    transferer: Arc<Transferer>,
}

impl DiscordInterface {
    async fn post_slash_command<F>(
        &self,
        post_to: PostTo,
        http: impl AsRef<Http>,
        id: u64,
    ) -> anyhow::Result<ApplicationCommand, serenity::Error> {
        match post_to {
            PostTo::Global => {
                Interaction::create_global_application_command(http, id, |ci| {
                    Self::create_interaction(ci)
                })
                .await
            }

            PostTo::Guild(guild_id) => {
                Interaction::create_guild_application_command(http, guild_id, id, |ci| {
                    Self::create_interaction(ci)
                })
                .await
            }
        }
    }

    pub fn is_command(&self, _msg: &Message) -> anyhow::Result<bool> {
        todo!()
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
            CommandArgs::New { target, place } => match target {
                Target::Receiver => {
                    let rs: anyhow::Result<RefWrap<DiscordReceiver>> =
                        commands::New::new(self.receivers.as_ref(), place);
                    todo!()
                }
                Target::Sender => {
                    let rs: anyhow::Result<RefWrap<DiscordSender>> =
                        commands::New::<DiscordSenders>::new(self.senders.as_ref(), place);
                    todo!()
                }
            },
            CommandArgs::Drop { id } => {
                match self.transferer.which_is(id).unwrap() {
                    Target::Receiver => match self.receivers.get(id) {
                        Ok(item) => commands::Drop::drop(item),
                        Err(_) => todo!(),
                    },
                    Target::Sender => match self.senders.get(id) {
                        Ok(item) => commands::Drop::drop(item),
                        Err(_) => todo!(),
                    },
                };
                todo!()
            }
            CommandArgs::Subsc {
                receiver_id,
                sender_id,
            } => {
                match self.receivers.get(receiver_id) {
                    Ok(recv) => match self.senders.get(sender_id) {
                        Ok(send) => {
                            commands::Subsc::subsc(
                                recv.lock().unwrap().as_mut().unwrap(),
                                send.lock().unwrap().as_ref().unwrap(),
                            );
                            todo!()
                        }
                        Err(_) => todo!(),
                    },
                    Err(_) => todo!(),
                };
                todo!()
            }
            CommandArgs::Exit {
                receiver_id,
                sender_id,
            } => {
                match self.receivers.get(receiver_id) {
                    Ok(recv) => match self.senders.get(sender_id) {
                        Ok(send) => {
                            commands::Exit::exit(
                                recv.lock().unwrap().as_mut().unwrap(),
                                send.lock().unwrap().as_ref().unwrap(),
                            );
                            todo!()
                        }
                        Err(_) => todo!(),
                    },
                    Err(_) => todo!(),
                };
                todo!()
            }
            _ => todo!(),
        }
        todo!()
    }

    pub async fn on_receive(&self, ctx: Context, msg: Message) -> anyhow::Result<()> {
        self.on_receive_process(FormattedData {
            content: msg.content.as_str().into(),
            attachments: vec![],
            author: Author {
                name: msg.author.name.as_str().into(),
                nickname: msg.author_nick(ctx.http).await,
                id: msg.author.id.0.to_string(),
                place: Place::Discord,
            },
            additional_contents: None,
            timestamp: msg.timestamp,
        })
    }

    fn on_receive_process(&self, _data: FormattedData) -> anyhow::Result<()> {
        // Transfererにdataを投げて,
        // callbackではなくsenderにhttpとかの情報を持たせて
        // on_send(...)してほしい
        todo!()
    }

    pub fn on_send(&self, _data: FormattedData) -> anyhow::Result<()> {
        todo!()
    }

    fn create_interaction(ci: &mut CreateInteraction) -> &mut CreateInteraction {
        ci.name("2c-tr")
            .description("2Meat Discord Interface: Transceiver")
            .create_interaction_option(|cio| {
                // New
                cio.name("new")
                    .description("create send/recv.")
                    .kind(ApplicationCommandOptionType::SubCommandGroup)
                    .create_sub_option(|cio| {
                        cio.name("send")
                            .description("create sender.")
                            .kind(ApplicationCommandOptionType::SubCommandGroup)
                            .create_sub_option(|cio| {
                                cio.name("there")
                                    .description("create sender: bind this channel.")
                                    .kind(ApplicationCommandOptionType::SubCommand)
                            })
                    })
                    .create_sub_option(|cio| {
                        cio.name("recv")
                            .description("create receiver.")
                            .kind(ApplicationCommandOptionType::SubCommandGroup)
                            .create_sub_option(|cio| {
                                cio.name("there")
                                    .description("create receiver: bind this channel.")
                                    .kind(ApplicationCommandOptionType::SubCommand)
                            })
                    })
            })
            // Mut
            // TODO: this handling
            // .create_interaction_option(|cio| {cio.name("mut")})
            // Drop
            .create_interaction_option(|cio| {
                cio.name("drop")
                    .description("drop send/recv from id.")
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
                    .description("subsc send at recv from id.")
                    .kind(ApplicationCommandOptionType::SubCommand)
                    .create_sub_option(|cio| {
                        cio.name("recv_id")
                            .description("receiver's id")
                            .kind(ApplicationCommandOptionType::String)
                    })
                    .create_sub_option(|cio| {
                        cio.name("send_id")
                            .description("sender's id")
                            .kind(ApplicationCommandOptionType::String)
                    })
            })
            // Exit
            .create_interaction_option(|cio| {
                cio.name("exit")
                    .description("exit send at recv from id.")
                    .kind(ApplicationCommandOptionType::SubCommand)
                    .create_sub_option(|cio| {
                        cio.name("recv_id")
                            .description("receiver's id")
                            .kind(ApplicationCommandOptionType::String)
                    })
                    .create_sub_option(|cio| {
                        cio.name("send_id")
                            .description("sender's id")
                            .kind(ApplicationCommandOptionType::String)
                    })
            })
    }

    fn create_command_parser() -> clap::App<'static> {
        clap_app!("/2c-tr" =>
            (about: "2Meat Discord Interface: Transceiver")
            (@subcommand "new" =>
                (about: "create send/recv.")
                (@arg TARGET: +required "set send/recv")
                (@arg PLACE: +required "set there/[coming soon...]")
            )
            // (@subcommand "mut" =>
            //     (about: "mut send/recv from id.")
            //     (@arg ID: +required "send/recv's id")
            // )
            (@subcommand "drop" =>
                (about: "drop send/recv from id.")
                (@arg ID: +required "send/recv's id")
            )
            (@subcommand "subsc" =>
                (about: "subsc send at recv from id.")
                (@arg RECV_ID: +required "receiver's id")
                (@arg SEND_ID: +required "sender's id")
            )
            (@subcommand "exit" =>
                (about: "exit send at recv from id.")
                (@arg RECV_ID: +required "receiver's id")
                (@arg SEND_ID: +required "sender's id")
            )
        )
    }
}

pub enum PostTo {
    Global,
    Guild(GuildId),
}

impl Interface for DiscordInterface {
    fn receive(&self, _data: FormattedData) -> anyhow::Result<()> {
        todo!()
    }

    fn send(&self, _data: FormattedData) -> anyhow::Result<()> {
        todo!()
    }
}

#[serenity::async_trait]
impl EventHandler for DiscordInterface {
    async fn message(&self, ctx: Context, msg: Message) {
        if MessageType::Regular != msg.kind {
            return;
        }

        if self.is_command(&msg).unwrap() {
            self.on_msg_command(ctx, msg).await.unwrap();
            return;
        }

        self.on_receive(ctx, msg).await.unwrap()
    }

    async fn interaction_create(&self, ctx: Context, ia: Interaction) {
        self.on_ia_command(ctx, ia).await.unwrap();
    }
}
