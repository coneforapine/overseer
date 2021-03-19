mod announce;
use announce::ANNOUNCE_COMMAND;

use serenity::{
    framework::standard::{
        macros::group
    }
};

#[group]
#[commands(announce)]
struct Admin;