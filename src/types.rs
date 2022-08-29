use teloxide::{adaptors::DefaultParseMode, prelude::*};

pub type MyBot = AutoSend<DefaultParseMode<Bot>>;
