-- Your SQL goes here
CREATE TABLE tbl_accounts (
  id BIGSERIAL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  email TEXT UNIQUE,
  password TEXT NOT NULL,
  last_ip TEXT NOT NULL CHECK (char_length(last_ip) <= 100),
  last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  active BOOLEAN NOT NULL DEFAULT FALSE,
  CONSTRAINT tbl_accounts_pkey PRIMARY KEY (id)
);
ALTER TABLE tbl_accounts OWNER TO rust;

CREATE UNIQUE INDEX idx_account_id ON tbl_accounts USING btree (id);