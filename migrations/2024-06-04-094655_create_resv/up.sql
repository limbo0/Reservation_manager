-- Your SQL goes here

CREATE TABLE reservation (
	id SERIAL NOT NULL PRIMARY KEY,
	name VARCHAR NOT NULL,
	contact VARCHAR NOT NULL,
	seating VARCHAR NOT NULL,
	specific_seating_requested BOOL NOT NULL,
	advance BOOL NOT NULL,
	advance_method JSONB NOT NULL,
	advance_amount INTEGER,
	confirmed BOOL NOT NULL,
	reservation_date DATE NOT NULL,
	reservation_time TIME NOT NULL,
  property_id uuid NOT NULL REFERENCES property(property_id)
);
