-- Your SQL goes here
CREATE TABLE tbl_accounts (
  id SERIAL,
  fk_profile  INTEGER NOT NULL,
  login VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE,
  CONSTRAINT tbl_accounts_pkey PRIMARY KEY (id)
);
ALTER TABLE tbl_accounts OWNER TO rust;

CREATE UNIQUE INDEX idx_account_id ON tbl_accounts USING btree (id);
CREATE INDEX idx_account_profile ON tbl_accounts USING btree(fk_profile);