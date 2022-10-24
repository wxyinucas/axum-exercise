CREATE TABLE admins (
                        id SERIAL PRIMARY KEY,
                        email VARCHAR(255) NOT NULL,
                        password VARCHAR(255) NOT NULL,
                        is_del BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO admins (email, password) VALUES('wxyinucas@gmail.com', 'axum.rs');
