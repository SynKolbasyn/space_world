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


use anyhow::Result;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rand::random;
use crate::database::players_db::players;


#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug, Clone)]
#[diesel(table_name = players)]
pub(crate) struct Player {
    pub(crate) username: String,
    pub(crate) money: f64,

    pub(crate) system: String,
    pub(crate) location: String,
    pub(crate) service: Option<String>,

    pub(crate) ship: String,
    pub(crate) weapon_1: Option<String>,
    pub(crate) weapon_2: Option<String>,
    pub(crate) weapon_3: Option<String>,
    pub(crate) device_1: Option<String>,
    pub(crate) device_2: Option<String>,
    pub(crate) device_3: Option<String>,
    pub(crate) component_1: Option<String>,
    pub(crate) component_2: Option<String>,
    pub(crate) component_3: Option<String>,
}


impl Player {
    fn new(
        username: String,
        money: f64,
        
        system: String,
        location: String,
        service: Option<String>,
        
        ship: String,
        weapon_1: Option<String>,
        weapon_2: Option<String>,
        weapon_3: Option<String>,
        device_1: Option<String>,
        device_2: Option<String>,
        device_3: Option<String>,
        component_1: Option<String>,
        component_2: Option<String>,
        component_3: Option<String>,
    ) -> Self {
        Self {
            username,
            money,
            
            system,
            location,
            service,

            ship,
            weapon_1,
            weapon_2,
            weapon_3,
            device_1,
            device_2,
            device_3,
            component_1,
            component_2,
            component_3,
        }
    }


    pub(crate) fn create<U: ToString>(username: U) -> Self {
        Self::new(
            username.to_string(),
            0_f64,
            
            String::from("Sol"),
            String::from("station"),
            Some(String::from("station")),
            
            String::from("Explorer T1 R1"),
            Some(String::from("Blaster T1 R1")),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }


    pub(crate) fn process_action(&mut self, action: String) -> Result<(String, Vec<String>)> {
        println!("{action}");
        self.money += random::<f64>() / f64::MAX;
        Ok((format!("Your money is {}", self.money), vec!["Button".to_string()]))
    }


    pub(crate) fn username(&self) -> String {
        self.username.clone()
    }
}