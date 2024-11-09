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


mod dispatcher;
pub(crate) mod player;


use std::{env::var, sync::Arc};

use anyhow::Result;
use tokio::sync::RwLock;
use teloxide::{
    dptree::{deps, entry},
    Bot,
    dispatching::UpdateFilterExt,
    prelude::{Dispatcher, Message, Requester, Update, CallbackQuery},
    payloads::SendMessageSetters,
    types::InlineKeyboardMarkup,
};

use crate::{
    database::Database,
    game::dispatcher::{process_message, process_callback_query},
};


pub(crate) async fn start_bot() -> Result<()> {
    log::info!("Starting bot...");
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


async fn message_handler(bot: Bot, message: Message, database: Arc<RwLock<Database>>) -> Result<()> {
    let (answer, inline_keyboard_markup): (String, InlineKeyboardMarkup) = process_message(&message, database).await.unwrap_or_else(|e| {log::error!("{e}"); (e.to_string(), InlineKeyboardMarkup::default())});
    bot.send_message(message.chat.id, answer).reply_markup(inline_keyboard_markup).await?;
    Ok(())
}


async fn callback_query_handler(bot: Bot, callback_query: CallbackQuery, database: Arc<RwLock<Database>>) -> Result<()> {
    let (answer, inline_keyboard_markup): (String, InlineKeyboardMarkup) = process_callback_query(&callback_query, database).await?;
    bot.answer_callback_query(callback_query.id).await?;
    bot.send_message(callback_query.from.id, answer).reply_markup(inline_keyboard_markup).await?;
    Ok(())
}
