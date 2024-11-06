CREATE TABLE players (
    username text PRIMARY KEY,
    money double precision NOT NULL,

    system text NOT NULL,
    location text NOT NULL,
    service text,

    ship text NOT NULL,
    weapon_1 text,
    weapon_2 text,
    weapon_3 text,
    device_1 text,
    device_2 text,
    device_3 text,
    component_1 text,
    component_2 text,
    component_3 text
);
