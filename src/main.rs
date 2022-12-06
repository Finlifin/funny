use clap::{App, Arg, SubCommand};
use in_and_out::{signin, signout};
use refer::*;
use tokio;
use config::*;
use top::*;
extern crate clap;
mod top;
mod config;
mod in_and_out;
mod refer;

#[tokio::main]
async fn main() {
    let default_id = get_id().await;
    let matches = App::new("MayApp")
        .version("0.1")
        .author("Finlifin")
        .about("Funny was built for signin or signout 科协的签到系统.")
        .subcommand(
            SubCommand::with_name("in").about("To signin").arg(
                Arg::with_name("id")
                    .takes_value(true)
                    .short("i")
                    .help("take id"),
            ),
        )
        .subcommand(
            SubCommand::with_name("out").about("To signout").arg(
                Arg::with_name("id")
                    .takes_value(true)
                    .short("i")
                    .help("take id"),
            ),
        )
        .subcommand(
            SubCommand::with_name("set").about("To config your id").arg(
                Arg::with_name("id")
                    .takes_value(true)
                    .short("i")
                    .help("take id"),
            ),
        )
        .subcommand(
            SubCommand::with_name("top")
                .about("To query the top five of this week")
                .arg(
                    Arg::with_name("id")
                        .takes_value(true)
                        .short("i")
                        .help("take id"),
                ),
        )
        .subcommand(
            SubCommand::with_name("refer")
                .about("To refer wether some is online")
                .arg(
                    Arg::with_name("id")
                        .takes_value(true)
                        .short("i")
                        .help("take id"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("in") {
        if let Some(id) = matches.value_of("id") {
            signin(id).await;
        } else {
            signin(default_id.as_str()).await;
        }
    }

    if let Some(matches) = matches.subcommand_matches("out") {
        if let Some(id) = matches.value_of("id") {
            signout(id).await;
        } else {
            signout(default_id.as_str()).await;
        }
    }

    if let Some(matches) = matches.subcommand_matches("refer") {
        if let Some(id) = matches.value_of("id") {
            let _ = refer(id).await;
        } else {
            let _ = refer(default_id.as_str()).await;
        }
    }

    if let Some(matches) = matches.subcommand_matches("set") {
        if let Some(id) = matches.value_of("id") {
            set_id(id).await;
        }else {
            println!("请输入正确的学号。");
        }
    }

    if let Some(_) = matches.subcommand_matches("top") {
        get_top_five().await;
    }
}
