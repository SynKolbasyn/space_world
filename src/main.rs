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


mod database;
mod game;


use anyhow::Result;

use crate::game::start_bot;


#[tokio::main]
async fn main() -> Result<()> {
    start_bot().await?;
    Ok(())
}
