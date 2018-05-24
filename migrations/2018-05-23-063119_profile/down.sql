-- This file should undo anything in `up.sql`
ALTER TABLE tbl_accounts DROP CONSTRAINT fk_account_profile_id;
DROP TABLE tbl_profiles;