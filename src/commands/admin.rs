mod announce;
mod warn;
mod case;

use announce::ANNOUNCE_COMMAND;
use warn::WARN_COMMAND;
use case::CASE_COMMAND;

use serenity::{
    framework::standard::{
        macros::group
    }
};

#[group]
#[commands(announce, warn, case)]
struct Admin;