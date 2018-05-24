-- Your SQL goes here
CREATE TABLE tbl_profiles(
  id SERIAL,
  first_name VARCHAR(255) NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  age INTEGER,
  sex BOOLEAN,
  CONSTRAINT tbl_profiles_pkey PRIMARY KEY (id)
);

ALTER TABLE tbl_profiles OWNER TO rust;

CREATE UNIQUE INDEX idx_profile_id ON tbl_profiles USING btree(id);

ALTER TABLE tbl_accounts
    ADD CONSTRAINT fk_account_profile_id
    FOREIGN KEY (fk_profile)
    REFERENCES tbl_profiles (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE;