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


pub(crate) mod players_db;


use std::env::var;

use anyhow::{Context, Result};
use diesel::{insert_into, update, Connection, OptionalExtension, PgConnection, QueryDsl, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::{
    database::players_db::players,
    game::player::Player,
};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");


pub(crate) struct Database {
    connection: AsyncPgConnection,
}


impl Database {
    fn new(connection: AsyncPgConnection) -> Self {
        Self {
            connection,
        }
    }


    pub(crate) async fn create() -> Result<Self> {
        let user: String = var("POSTGRES_USER")?;
        let password: String = var("POSTGRES_PASSWORD")?;
        let db_name: String = var("POSTGRES_DB")?;
        let url: String = format!("postgres://{user}:{password}@database/{db_name}");

        PgConnection::establish(&url)?.run_pending_migrations(MIGRATIONS).ok().context("Migrations failed")?;

        let connection: AsyncPgConnection = AsyncPgConnection::establish(&url).await?;
        Ok(Self::new(connection))
    }


    pub(crate) async fn add_player(&mut self, player: &Player) -> Result<()> {
        insert_into(players::table).values(player).execute(&mut self.connection).await?;
        Ok(())
    }


    pub(crate) async fn load_player<T: ToString>(&mut self, username: T) -> Result<Option<Player>> {
        let data_table: Option<Player> = players::table.find(username.to_string()).select(Player::as_select()).first(&mut self.connection).await.optional()?;
        Ok(data_table)
    }


    pub(crate) async fn update_player(&mut self, player: Player) -> Result<()> {
        update(players::table.find(player.username())).set(player).execute(&mut self.connection).await?;
        Ok(())
    }
}
