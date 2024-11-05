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


use diesel::prelude::*;


table! {
    players (username) {
        username -> Text,
        money -> Double,
    }
}


#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = players)]
pub(crate) struct Player {
    pub(crate) username: String,
    pub(crate) money: f64,
}


impl Player {
    fn new(username: String, money: f64) -> Self {
        Self {
            username,
            money,
        }
    }
    
    
    pub(crate) fn create<U: Into<String>>(username: U) -> Self {
        Self::new(username.into(), 0_f64)
    }
}
