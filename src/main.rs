use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
        macros::{command, group, help, hook},
    },
    http::Http,
    model::{
        channel::Message,
        gateway::Ready,
        id::UserId,
    },
    prelude::*,
};
use std::{
    env,
    collections::{HashMap, HashSet},
    sync::Arc,
};
use dotenv::dotenv;
use ojichat::ojichat;

struct SharedManagerContainer;

impl TypeMapKey for SharedManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[description = "おじさんが喋ります。"]
#[commands(ojichat)]
struct Ojichat;

#[help]
#[command_not_found_text = "Could not find: `{}`."]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(duration) = error {
        let _ = msg
            .channel_id
            .say(&ctx.http, &format!("Type this again in {} seconds.", duration.as_secs()))
            .await;
    }
}

use serenity::{futures::future::BoxFuture, FutureExt};
fn _dispatch_error_no_macro<'fut>(ctx: &'fut mut Context, msg: &'fut Message, error: DispatchError) -> BoxFuture<'fut, ()> {
    async move {
        if let DispatchError::Ratelimited(duration) = error {
            let _ = msg
                .channel_id
                .say(&ctx.http, &format!("Type this again in {} seconds.", duration.as_secs()))
                .await;
        };
    }.boxed()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment or .env file",);

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("~")
            .owners(owners))
        .unrecognised_command(unknown_command)
        .on_dispatch_error(dispatch_error)
        .help(&MY_HELP)
        .group(&OJICHAT_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<SharedManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        eprintln!("An Error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ojichat(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let emoji_num = args.single::<usize>().ok();
    let punctuation_level = args.single::<usize>().ok();
    let target = args.single::<String>().ok();

    let res = ojichat::get_message(target, emoji_num, punctuation_level);

    msg.channel_id.say(&ctx.http, &res.to_string()).await?;

    Ok(())
}
