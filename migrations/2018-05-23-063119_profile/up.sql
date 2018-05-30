-- Your SQL goes here
CREATE TABLE tbl_profiles(
  id BIGSERIAL,
  account_id  BIGINT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  birth_date TIMESTAMPTZ,
  sex BOOLEAN,
  CONSTRAINT tbl_profiles_pkey PRIMARY KEY (id)
);

ALTER TABLE tbl_profiles OWNER TO rust;

CREATE UNIQUE INDEX idx_profile_id ON tbl_profiles USING btree(id);
CREATE INDEX idx_profile_account ON tbl_profiles USING btree(account_id);
ALTER TABLE tbl_profiles
    ADD CONSTRAINT fk_profile_account_id
    FOREIGN KEY (account_id)
    REFERENCES tbl_accounts (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE;