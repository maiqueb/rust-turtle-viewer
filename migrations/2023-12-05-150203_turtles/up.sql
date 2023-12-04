CREATE TABLE ninja_turtles (
                               user_id serial PRIMARY KEY,
                               name VARCHAR ( 50 ) UNIQUE NOT NULL,
                               email VARCHAR ( 255 ) UNIQUE NOT NULL,
                               weapon VARCHAR ( 50 ) NOT NULL,
                               created_on TIMESTAMP DEFAULT now()
);
