-- This file should undo anything in `up.sql`
ALTER TABLE tbl_profiles DROP CONSTRAINT fk_profile_account_id;
DROP TABLE tbl_profiles;