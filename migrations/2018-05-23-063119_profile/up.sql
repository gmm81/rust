-- Your SQL goes here
CREATE TABLE tbl_profiles(
  id BIGSERIAL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  birth_date TIMESTAMPTZ,
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