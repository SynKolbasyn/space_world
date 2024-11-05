///     space_world. A game about the space world.
///     Copyright (C) 2024  Andrew Kozmin <syn.kolbasyn.06@gmail.com>
///
///     This program is free software: you can redistribute it and/or modify
///     it under the terms of the GNU Affero General Public License as published
///     by the Free Software Foundation, either version 3 of the License, or
///     (at your option) any later version.
///
///     This program is distributed in the hope that it will be useful,
///     but WITHOUT ANY WARRANTY; without even the implied warranty of
///     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
///     GNU Affero General Public License for more details.
///
///     You should have received a copy of the GNU Affero General Public License
///     along with this program.  If not, see <https://www.gnu.org/licenses/>.


use std::{env::var, sync::Arc};

use anyhow::Result;
use tokio::sync::RwLock;
use teloxide::{
    dptree::{deps, entry},
    Bot,
    dispatching::UpdateFilterExt,
    prelude::{Dispatcher, Message, Requester, Update, CallbackQuery},
    payloads::SendMessageSetters,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::database::{Database, players_db::Player};


pub(crate) async fn start_bot() -> Result<()> {
    let token: String = var("BOT_TOKEN")?;
    let bot: Bot = Bot::new(token);
    let database: Arc<RwLock<Database>> = Arc::new(RwLock::new(Database::create().await?));
    let handler = entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_query_handler));
    Dispatcher::builder(bot, handler)
        .dependencies(deps![database])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}


async fn message_handler(bot: Bot, msg: Message, database: Arc<RwLock<Database>>) -> Result<()> {
    match get_username(&msg) {
        Some(username) => {
            let mut player: Player = database.write().await.load_player(username).await?;
            player.money += 10_f64;
            database.write().await.update_player(&player).await?;
            bot.send_message(msg.chat.id, format!("Hello, you have {} money", player.money)).reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::callback("Button", "Button")]]))
        }
        None => bot.send_message(msg.chat.id, "Couldn't get access to your username"),
    }.await?;
    Ok(())
}


async fn callback_query_handler(bot: Bot, q: CallbackQuery) -> Result<()> {
    bot.answer_callback_query(q.id).await?;
    bot.send_message(q.from.id, "Hello").reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::callback("Button", "Button")]])).await?;
    Ok(())
}


fn get_username(message: &Message) -> Option<String> {
    Some(message.from.clone()?.username?)
}
