-- Your SQL goes here

CREATE TABLE propertyusers (
  user_id SERIAL NOT NULL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  user_password VARCHAR NOT NULL,
  user_role INTEGER NOT NULL REFERENCES roles(role_id),
  property_id uuid NOT NULL REFERENCES property(property_id)
);
