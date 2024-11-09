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


use std::sync::Arc;

use anyhow::{Context, Result};
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message};
use tokio::sync::RwLock;

use crate::{
    database::Database,
    game::player::Player,
};


async fn process_player(username: String, action: String, database: Arc<RwLock<Database>>) -> Result<(String, InlineKeyboardMarkup)> {
    let mut player: Player = load_player(username, database.clone()).await?;
    let (answer, buttons): (String, Vec<String>) = player.process_action(action)?;
    database.write().await.update_player(player).await?;
    Ok((answer, generate_buttons(buttons)))
}


async fn load_player(username: String, database: Arc<RwLock<Database>>) -> Result<Player> {
    let mut db = database.write().await;
    Ok(match db.load_player(&username).await? {
        Some(player) => player,
        None => {
            let player: Player = Player::create(username);
            db.add_player(&player).await?;
            player
        },
    })
}


fn generate_buttons(buttons: Vec<String>) -> InlineKeyboardMarkup {
    let buttons: Vec<[InlineKeyboardButton; 1]> = buttons.iter().map(|button: &String| [InlineKeyboardButton::callback(button, button)]).collect();
    InlineKeyboardMarkup::new(buttons)
}


pub(crate) async fn process_message(message: &Message, database: Arc<RwLock<Database>>) -> Result<(String, InlineKeyboardMarkup)> {
    Ok(process_player(
        get_username_from_message(message)?,
        get_action_from_message(message)?,
        database,
    ).await?)
}


pub(crate) async fn process_callback_query(callback_query: &CallbackQuery, database: Arc<RwLock<Database>>) -> Result<(String, InlineKeyboardMarkup)> {
    Ok(process_player(
        get_username_from_callback_query(callback_query)?,
        get_action_from_callback_query(callback_query)?,
        database,
    ).await?)
}


fn get_username_from_message(message: &Message) -> Result<String> {
    let username: String = message
        .from.clone().context("Couldn't get the 'from' object")?
        .username.context("Couldn't get username from the 'from' object")?;
    Ok(username)
}


fn get_action_from_message(message: &Message) -> Result<String> {
    let action: String = message
        .text()
        .context("Couldn't get a player action")?
        .to_string();
    Ok(action)
}


fn get_username_from_callback_query(callback_query: &CallbackQuery) -> Result<String> {
    let username: String = callback_query
        .from.clone()
        .username.context("Couldn't get username from the 'from' object")?;
    Ok(username)
}


fn get_action_from_callback_query(callback_query: &CallbackQuery) -> Result<String> {
    Ok(callback_query.data.clone().context("Couldn't get a player action")?)
}
