-- Initialize PostgreSQL database for bank application
-- This script runs automatically when the container starts

-- Create the database if it doesn't exist (though it's already created via environment variables)
-- CREATE DATABASE IF NOT EXISTS bank_app;

-- Grant all privileges to the bank_user
GRANT ALL PRIVILEGES ON DATABASE bank_app TO bank_user;

-- Connect to the bank_app database
\c bank_app;

-- Grant schema privileges
GRANT ALL ON SCHEMA public TO bank_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO bank_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO bank_user;

-- Set default privileges for future tables and sequences
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO bank_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO bank_user; 