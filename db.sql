CREATE EXTENSION pgcrypto;

CREATE TABLE users (
       id		    SERIAL PRIMARY KEY,
       email		    varchar(255),
       username		    varchar(255),
       password		    varchar(255)
);

CREATE TABLE droids (
       id		    SERIAL PRIMARY KEY,
       user_id		    integer REFERENCES users (id),
       attack_level	    integer,
       defense_level	    integer,
       escape_level	    integer
);

CREATE TABLE user_ressources (
       id		    SERIAL PRIMARY KEY,
       user_id		    integer REFERENCES users (id),
       energy		    integer,
       crystal		    integer,
       metal		    integer
);

CREATE TABLE galaxy (
       id		    SERIAL PRIMARY KEY,
       name		    varchar(255),
       sun_number	    integer
);

CREATE TABLE solar_system (
       id		    SERIAL PRIMARY KEY,
       galaxy		    integer REFERENCES galaxy (id),
       name		    varchar(255),
       planet_number	    integer
);

CREATE TABLE planetes (
       id		    SERIAL PRIMARY KEY,
       solar_system	    integer REFERENCES solar_system (id),
       case_number	    integer,
       distance_to_sun	    integer
);

CREATE TABLE product (
       id		    SERIAL PRIMARY KEY,
       name		    varchar(255),
       description	    text
);

CREATE TABLE type_buildings (
       id		    SERIAL PRIMARY KEY,
       name		    varchar(255),
       img_url		    varchar(1000),
       product		    varchar(255),
       product_id	    integer REFERENCES product (id),
       base_value	    integer,
       base_price	    integer,
       level_multiplier	    decimal
);

CREATE TABLE buildings (
       id		    SERIAL PRIMARY KEY,
       level		    integer,
       type		    integer REFERENCES type_buildings (id),
       user_id		    integer REFERENCES users (id),
       planet_id	    integer REFERENCES planetes (id)
);

CREATE TABLE buildings_update_level (
       id		    SERIAL PRIMARY KEY,
       building_id	    integer REFERENCES buildings (id),
       level		    integer,
       up_at		    timestamp DEFAULT now()
);


INSERT INTO type_buildings(id, name, img_url, product, level_multiplier) VALUES(1, 'factory', 'https://upload.wikimedia.org/wikipedia/commons/thumb/a/a2/Factory.svg/502px-Factory.svg.png', 'crystal', 1.5);

INSERT INTO galaxy(id, name, sun_number) VALUES(1, 'andromeda', 500);
INSERT INTO solar_system(id, galaxy, name, planet_number) VALUES(1, 1, 'sun_4815162342', 7);
INSERT INTO planetes(id, solar_system, name, case_number, distance_to_sun) VALUES(1, 1, 'planet_momo', 163, 100);
INSERT INTO planetes(id, solar_system, name, case_number, distance_to_sun) VALUES(2, 1, 'planet_batsu', 18, 50);
