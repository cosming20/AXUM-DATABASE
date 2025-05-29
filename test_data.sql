-- Test data for SecureBank application
INSERT INTO banks (name, address, phone, email) 
VALUES ('SecureBank Test', '123 Banking St', '555-0123', 'info@securebank.test')
ON CONFLICT DO NOTHING;

INSERT INTO branches (bank_id, name, address, branch_code) 
VALUES (1, 'Main Branch', '123 Banking St', 'MAIN001')
ON CONFLICT (branch_code) DO NOTHING;

INSERT INTO users (uuid, email, password_hash, first_name, last_name, role) 
VALUES (
	'test-user-uuid-123', 
	'test@securebank.test', 
	'$2b$12$5IheTx6NPtTRorTAexylx.hziE4IrvJ0Wn6r0TOGGwJc3Yeyq9ZuC', 
	'Test', 
	'User', 
	'customer'
) ON CONFLICT (email) DO NOTHING;

INSERT INTO users (uuid, email, password_hash, first_name, last_name, role) 
VALUES (
	'admin-user-uuid-456', 
	'admin@securebank.test', 
	'$2b$12$5IheTx6NPtTRorTAexylx.hziE4IrvJ0Wn6r0TOGGwJc3Yeyq9ZuC', 
	'Admin', 
	'User', 
	'admin'
) ON CONFLICT (email) DO NOTHING;

INSERT INTO accounts (uuid, user_id, branch_id, account_number, account_type, balance) 
VALUES (
	'test-account-uuid-123', 
	1, 
	1, 
	'ACC1234567890', 
	'checking', 
	1000.00
) ON CONFLICT (account_number) DO NOTHING; 