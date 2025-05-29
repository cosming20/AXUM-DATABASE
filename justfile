# SecureBank Development Commands
# Run `just` to see all available commands

# Default recipe - shows help
default:
	@just --list

# 🚀 Quick start - sets up everything and runs the app
start: setup migrate test-data
	@echo "🎉 Banking app is ready!"
	@echo "Run 'just dev' to start the development server"

# 🐳 Start PostgreSQL container
db-start:
	@echo "🐳 Starting PostgreSQL container..."
	docker-compose -f dev.yaml up -d
	@echo "⏳ Waiting for PostgreSQL to be ready..."
	@sleep 5
	@docker exec bank_app_postgres pg_isready -U bank_user -d bank_app || (echo "❌ PostgreSQL not ready" && exit 1)
	@echo "✅ PostgreSQL is ready on port 5555"

# 🛑 Stop PostgreSQL container
db-stop:
	@echo "🛑 Stopping PostgreSQL container..."
	docker-compose -f dev.yaml down

# 🗑️ Reset database (stop, remove volumes, start fresh)
db-reset:
	@echo "🗑️ Resetting database..."
	docker-compose -f dev.yaml down -v
	@just db-start
	@just migrate

# 📊 Show database status
db-status:
	@echo "📊 Database Status:"
	@docker-compose -f dev.yaml ps
	@echo ""
	@echo "🔗 Connection test:"
	@docker exec bank_app_postgres pg_isready -U bank_user -d bank_app && echo "✅ Connected" || echo "❌ Not connected"

# 🔧 Setup environment
setup:
	@echo "🔧 Setting up environment..."
	@if [ ! -f .env ]; then cp env.example .env && echo "✅ Created .env file"; else echo "✅ .env file exists"; fi
	@echo "🔍 Checking Diesel CLI..."
	@diesel --version > /dev/null 2>&1 || (echo "📦 Installing Diesel CLI..." && cargo install diesel_cli --no-default-features --features postgres)
	@echo "✅ Environment setup complete"

# 🗄️ Run database migrations
migrate:
	@echo "🗄️ Running database migrations..."
	diesel migration run
	@echo "✅ Migrations complete"

# 📝 Create new migration
migration name:
	@echo "📝 Creating migration: {{name}}"
	diesel migration generate {{name}}

# ⏪ Revert last migration
migrate-revert:
	@echo "⏪ Reverting last migration..."
	diesel migration revert

# 🧪 Run all tests
test:
	@echo "🧪 Running tests..."
	cargo test

# 🧪 Run specific test
test-filter filter:
	@echo "🧪 Running tests matching: {{filter}}"
	cargo test {{filter}}

# 🧪 Test JWT functionality
test-jwt:
	@echo "🧪 Testing JWT..."
	cargo test jwt

# 🧪 Test database models
test-db:
	@echo "🧪 Testing database models..."
	cargo test db

# 🔍 Check code compilation
check:
	@echo "🔍 Checking compilation..."
	cargo check

# 🎨 Format code
fmt:
	@echo "🎨 Formatting code..."
	cargo fmt

# 📎 Run clippy lints
lint:
	@echo "📎 Running clippy..."
	cargo clippy

# 🏗️ Build the application
build:
	@echo "🏗️ Building application..."
	cargo build

# 🏗️ Build for release
build-release:
	@echo "🏗️ Building for release..."
	cargo build --release

# 🚀 Start development server
dev:
	@echo "🚀 Starting development server..."
	@echo "📱 Open http://localhost:3000 in your browser"
	cargo leptos watch

# 🌐 Start production server
serve:
	@echo "🌐 Starting production server..."
	cargo leptos serve --release

# 📊 Insert test data
test-data:
	@echo "📊 Inserting test data..."
	@docker exec -i bank_app_postgres psql -U bank_user -d bank_app < test_data.sql
	@echo "✅ Test data inserted"
	@echo ""
	@echo "🔑 Test login credentials:"
	@echo "   Email: test@securebank.test"
	@echo "   Password: password123"

# 🗄️ Connect to database
db-connect:
	@echo "🗄️ Connecting to database..."
	docker exec -it bank_app_postgres psql -U bank_user -d bank_app

# 📋 Show database tables
db-tables:
	@echo "📋 Database tables:"
	@docker exec bank_app_postgres psql -U bank_user -d bank_app -c "\dt"

# 👥 Show users in database
db-users:
	@echo "👥 Users in database:"
	@docker exec bank_app_postgres psql -U bank_user -d bank_app -c "SELECT id, email, first_name, last_name, role, is_active FROM users;"

# 🏦 Show accounts in database
db-accounts:
	@echo "🏦 Accounts in database:"
	@docker exec bank_app_postgres psql -U bank_user -d bank_app -c "SELECT id, account_number, account_type, balance, currency FROM accounts;"

# 💳 Show transactions in database
db-transactions:
	@echo "💳 Transactions in database:"
	@docker exec bank_app_postgres psql -U bank_user -d bank_app -c "SELECT id, transaction_type, amount, status, created_at FROM transactions ORDER BY created_at DESC LIMIT 10;"

# 🧹 Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# 🔄 Full reset and restart
reset: db-reset setup migrate test-data
	@echo "🔄 Full reset complete!"
	@echo "Run 'just dev' to start development"

# 📦 Install dependencies
deps:
	@echo "📦 Installing dependencies..."
	@echo "🦀 Updating Rust..."
	rustup update
	@echo "🎯 Adding WebAssembly target..."
	rustup target add wasm32-unknown-unknown
	@echo "📦 Installing Diesel CLI..."
	cargo install diesel_cli --no-default-features --features postgres
	@echo "📦 Installing cargo-leptos..."
	cargo install cargo-leptos
	@echo "✅ Dependencies installed"

# 🔐 Generate new JWT secret
jwt-secret:
	@echo "🔐 New JWT secret for production:"
	@openssl rand -base64 32

# 📊 Show app status
status:
	@echo "📊 SecureBank Status"
	@echo "==================="
	@echo ""
	@echo "🐳 Docker:"
	@docker --version 2>/dev/null || echo "❌ Docker not available"
	@echo ""
	@echo "🦀 Rust:"
	@rustc --version 2>/dev/null || echo "❌ Rust not available"
	@echo ""
	@echo "⚙️ Diesel:"
	@diesel --version 2>/dev/null || echo "❌ Diesel CLI not available"
	@echo ""
	@echo "🗄️ Database:"
	@just db-status
	@echo ""
	@echo "📁 Environment:"
	@if [ -f .env ]; then echo "✅ .env file exists"; else echo "❌ .env file missing"; fi

# 🚀 Complete setup for new developers
bootstrap: deps db-start setup migrate test-data
	@echo ""
	@echo "🎉 Bootstrap complete!"
	@echo ""
	@echo "Next steps:"
	@echo "1. Run 'just dev' to start development server"
	@echo "2. Open http://localhost:3000"
	@echo "3. Login with test@securebank.test / password123"
	@echo ""
	@echo "Useful commands:"
	@echo "- just dev          # Start development server"
	@echo "- just test         # Run tests"
	@echo "- just db-connect   # Connect to database"
	@echo "- just status       # Check system status" 