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


use diesel::table;


table! {
    players (username) {
        username -> Text,
        money -> Double,

        system -> Text,
        location -> Text,
        service -> Nullable<Text>,
        
        ship -> Text,
        weapon_1 -> Nullable<Text>,
        weapon_2 -> Nullable<Text>,
        weapon_3 -> Nullable<Text>,
        device_1 -> Nullable<Text>,
        device_2 -> Nullable<Text>,
        device_3 -> Nullable<Text>,
        component_1 -> Nullable<Text>,
        component_2 -> Nullable<Text>,
        component_3 -> Nullable<Text>,
    }
}
