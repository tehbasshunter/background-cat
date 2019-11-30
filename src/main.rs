use lazy_static::lazy_static;
use regex::Regex;

use reqwest::get;

use serenity::{
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
    utils::Colour,
};

use log::{debug, error, info};

mod parsers;
use parsers::PARSERS;

fn main() {
    kankyo::load(false).expect("Expected a .env file");
    env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in $DISCORD_TOKEN");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}

fn common_mistakes(input: &str) -> Vec<(&str, String)> {
    PARSERS.iter().flat_map(|m| m(input)).collect()
}

/// Takes a string of an URL, returns the content.
/// Helper for Error Handling.
fn get_log(link: &str) -> Result<String, Box<dyn std::error::Error>> {
    let link: reqwest::Url = link.parse()?;
    Ok(get(link)?.text()?)
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.is_private() {
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("<:backgroundcat:280120125284417536>A bot to parse logfiles on the MultiMC discord<:backgroundcat:280120125284417536>");
                    #[allow(clippy::unreadable_literal)]
                    let creator_name = match UserId::from(185461862878543872).to_user(&ctx) {
                        Ok(o) => o.tag(),
                        Err(why) => {error!("Couldn't get info about creator: {}", why); "<Error getting name>".to_string()}
                    };
                    e.colour(Colour::DARK_TEAL);
                    e.description(format!(r"
                Developed by {}.
                To start, just post a https://paste.ee link in the Discord.

                [Source Code available under AGPLv3](https://gitlab.com/Scotsguy/background-cat)
                ", creator_name));
                    e
                });
                m
            }) {
                error!("Couldn't send info message: {}", why)
            }
        }

        lazy_static! {
            static ref PASTEE_REGEX: Regex = Regex::new(r"https:/{2}paste.ee/p/[^\s/]+").unwrap();
        }

        if let Some(link) = PASTEE_REGEX.find(&msg.content) {
            info!(
                "Found paste.ee link: {} in message {}",
                link.as_str(),
                msg.id
            );

            let link_raw = link.as_str().replacen("/p/", "/r/", 1);
            let log = match get_log(&link_raw) {
                Ok(o) => o,
                Err(_) => return,
            };
            debug!("Content of log: {}", log);

            let mistakes = common_mistakes(&log);

            if mistakes.is_empty() {
                info!("Didn't find any mistakes in log ({})", link.as_str());
                return;
            }
            debug!("Mistakes found: {:?}", mistakes);

            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Automated Response: (Warning: Experimental)");
                    e.colour(Colour::DARK_TEAL);
                    for i in mistakes.iter() {
                        e.field(i.0, &i.1, true);
                    }
                    e.footer(|f| {
                        f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1");
                        f.text("This might not solve your problem, but it could be worth a try")
                    });
                    debug!("Embed: {:?}", e);
                    e
                });
                debug!("Embed: {:?}", m);
                m
            }) {
                error!("Couldn't send message: {}", why)
            }
        };
    }

    // TODO: delete on reaction

    fn ready(&self, ctx: Context, ready: Ready) {
        use serenity::model::{gateway::Activity, user::OnlineStatus};

        info!(
            "{}#{} is connected!",
            ready.user.name, ready.user.discriminator
        );
        ctx.set_presence(Some(Activity::playing("DM me!")), OnlineStatus::Online);
    }
}