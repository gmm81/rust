-- Your SQL goes here
CREATE TABLE tbl_accounts (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  fk_profile INTEGER NOT NULL,
  login VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE UNIQUE INDEX idx_account_id ON tbl_accounts(id) USING btree;
CREATE INDEX idx_account_profile ON tbl_accounts(fk_profile) USING btree;