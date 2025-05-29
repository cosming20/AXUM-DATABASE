-- This file should undo anything in `up.sql`

-- Drop constraints first
ALTER TABLE accounts DROP CONSTRAINT IF EXISTS chk_account_balance;
ALTER TABLE transactions DROP CONSTRAINT IF EXISTS chk_transaction_amount;
ALTER TABLE transactions DROP CONSTRAINT IF EXISTS chk_transaction_accounts;

-- Drop indexes
-- Sessions indexes
DROP INDEX IF EXISTS idx_sessions_expires_at;
DROP INDEX IF EXISTS idx_sessions_token;
DROP INDEX IF EXISTS idx_sessions_user_id;

-- Transactions indexes
DROP INDEX IF EXISTS idx_transactions_reference_number;
DROP INDEX IF EXISTS idx_transactions_created_at;
DROP INDEX IF EXISTS idx_transactions_status;
DROP INDEX IF EXISTS idx_transactions_type;
DROP INDEX IF EXISTS idx_transactions_to_account_id;
DROP INDEX IF EXISTS idx_transactions_from_account_id;
DROP INDEX IF EXISTS idx_transactions_uuid;

-- Accounts indexes
DROP INDEX IF EXISTS idx_accounts_is_active;
DROP INDEX IF EXISTS idx_accounts_account_type;
DROP INDEX IF EXISTS idx_accounts_account_number;
DROP INDEX IF EXISTS idx_accounts_branch_id;
DROP INDEX IF EXISTS idx_accounts_user_id;
DROP INDEX IF EXISTS idx_accounts_uuid;

-- Branches indexes
DROP INDEX IF EXISTS idx_branches_branch_code;
DROP INDEX IF EXISTS idx_branches_bank_id;

-- Banks indexes
DROP INDEX IF EXISTS idx_banks_name;

-- Users indexes
DROP INDEX IF EXISTS idx_users_is_active;
DROP INDEX IF EXISTS idx_users_role;
DROP INDEX IF EXISTS idx_users_uuid;
DROP INDEX IF EXISTS idx_users_email;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS accounts;
DROP TABLE IF EXISTS branches;
DROP TABLE IF EXISTS banks;
DROP TABLE IF EXISTS users;
