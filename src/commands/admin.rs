mod announce;
mod warn;

use announce::ANNOUNCE_COMMAND;
use warn::WARN_COMMAND;

use serenity::{
    framework::standard::{
        macros::group
    }
};

#[group]
#[commands(announce, warn)]
struct Admin;