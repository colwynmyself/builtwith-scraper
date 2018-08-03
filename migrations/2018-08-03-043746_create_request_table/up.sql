CREATE TABLE request (
    id SERIAL PRIMARY KEY,
    domain VARCHAR NOT NULL,
    request_date DATE NOT NULL default CURRENT_DATE,
    response JSON NOT NULL,
    throttled BOOLEAN NOT NULL
);