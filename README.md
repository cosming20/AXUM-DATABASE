# SecureBank - Modern Banking Application

A modern, full-stack banking application built with Rust, Leptos, and PostgreSQL. This application provides a comprehensive banking solution with user authentication, account management, transaction processing, and administrative features.

## Features

### 🏦 Core Banking Features
- **User Authentication**: Secure login system with password hashing
- **Account Management**: Multiple account types (Checking, Savings, Credit)
- **Transaction Processing**: Deposits, withdrawals, and transfers
- **Transaction History**: Complete audit trail of all transactions
- **Real-time Balance Updates**: Instant balance updates after transactions

### 👥 User Management
- **Role-based Access Control**: Customer, Staff, and Admin roles
- **User Profiles**: Complete user information management
- **Account Linking**: Users can have multiple accounts

### 🏢 Administrative Features
- **User Management**: Admin panel for managing users
- **Account Oversight**: Monitor and manage all accounts
- **Transaction Monitoring**: Review and audit transactions
- **Branch Management**: Manage bank branches and locations

### 🎨 Modern UI/UX
- **Responsive Design**: Works on desktop and mobile devices
- **Modern Interface**: Clean, intuitive user interface
- **Real-time Updates**: Dynamic content updates
- **Accessibility**: Built with accessibility in mind

## Technology Stack

### Backend
- **Rust**: High-performance, memory-safe systems programming
- **Axum**: Modern, ergonomic web framework
- **Diesel**: Type-safe ORM for database operations
- **PostgreSQL**: Reliable relational database
- **bcrypt**: Secure password hashing
- **JWT**: JSON Web Tokens for authentication

### Frontend
- **Leptos**: Reactive web framework for Rust
- **Thaw**: Modern UI component library
- **TailwindCSS**: Utility-first CSS framework
- **WebAssembly**: High-performance web applications

### Infrastructure
- **Docker**: Containerized PostgreSQL database
- **Docker Compose**: Development environment orchestration

## Project Structure

```
AXUM-DATABASE/
├── src/
│   ├── api/                 # API endpoints and handlers
│   │   ├── auth.rs         # Authentication endpoints
│   │   ├── users.rs        # User management endpoints
│   │   ├── accounts.rs     # Account management endpoints
│   │   ├── transactions.rs # Transaction endpoints
│   │   ├── banks.rs        # Bank management endpoints
│   │   └── branches.rs     # Branch management endpoints
│   ├── app/                # Frontend application
│   │   ├── bank_app.rs     # Main application components
│   │   └── models.rs       # Frontend data models
│   ├── db/                 # Database layer
│   │   ├── models/         # Database models
│   │   │   ├── users.rs    # User database operations
│   │   │   ├── accounts.rs # Account database operations
│   │   │   ├── transactions.rs # Transaction database operations
│   │   │   ├── banks.rs    # Bank database operations
│   │   │   └── branches.rs # Branch database operations
│   │   ├── migrations/     # Database migrations
│   │   └── schema.rs       # Database schema definitions
│   ├── lib.rs              # Library root
│   └── main.rs             # Application entry point
├── style/
│   └── main.scss           # Application styles
├── dev.yaml                # Docker Compose for development
├── init.sql                # PostgreSQL initialization script
├── env.example             # Environment variables example
├── Cargo.toml              # Rust dependencies and configuration
└── README.md               # This file
```

## Prerequisites

Before running the application, ensure you have the following installed:

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Docker and Docker Compose**
   ```bash
   # macOS (using Homebrew)
   brew install docker docker-compose
   
   # Ubuntu/Debian
   sudo apt-get install docker.io docker-compose
   
   # Or install Docker Desktop from https://www.docker.com/products/docker-desktop
   ```

3. **Diesel CLI** (for PostgreSQL)
   ```bash
   cargo install diesel_cli --no-default-features --features postgres
   ```

## Setup Instructions

### 1. Clone the Repository
```bash
git clone <repository-url>
cd AXUM-DATABASE
```

### 2. Start PostgreSQL with Docker
```bash
# Start PostgreSQL container on port 5555
docker-compose -f dev.yaml up -d

# Check if the container is running
docker-compose -f dev.yaml ps
```

### 3. Environment Configuration
Copy the example environment file and configure it:
```bash
cp env.example .env
```

Edit `.env` if needed:
```env
DATABASE_URL=postgres://bank_user:secure_password@localhost:5555/bank_app
JWT_SECRET=your-super-secret-jwt-key-here-change-in-production
RUST_LOG=info
```

### 4. Run Database Migrations
```bash
# Run the comprehensive migration to create all tables
diesel migration run

# Verify the migration was successful
diesel migration list
```

### 5. Build and Run
```bash
# Development build
cargo build

# Run the application
cargo run

# Or run in development mode with hot reload
cargo leptos watch
```

The application will be available at `http://localhost:3000`

## Database Schema

The application uses the following main entities:

### Users
- User authentication and profile information
- Role-based access control (Customer, Staff, Admin)
- Password hashing with bcrypt

### Banks
- Bank information and details
- Multiple banks support

### Branches
- Bank branch locations
- Branch-specific operations

### Accounts
- User bank accounts (Checking, Savings, Credit)
- Account numbers and balances
- Account status management

### Transactions
- All financial transactions
- Transaction types (Transfer, Deposit, Withdrawal, Payment)
- Transaction status tracking
- Reference numbers for audit trails

### Sessions
- User session management
- JWT token storage and validation

## Docker Development Environment

The project includes a Docker Compose configuration for easy development:

```yaml
# dev.yaml
services:
  postgres:
    image: postgres:15-alpine
    ports:
      - "5555:5432"
    environment:
      POSTGRES_DB: bank_app
      POSTGRES_USER: bank_user
      POSTGRES_PASSWORD: secure_password
```

### Docker Commands
```bash
# Start the database
docker-compose -f dev.yaml up -d

# Stop the database
docker-compose -f dev.yaml down

# View logs
docker-compose -f dev.yaml logs postgres

# Connect to PostgreSQL directly
docker exec -it bank_app_postgres psql -U bank_user -d bank_app
```

## API Endpoints

### Authentication
- `POST /api/login` - User login
- `POST /api/logout` - User logout
- `GET /api/current-user` - Get current user info

### Users
- `GET /api/users` - List all users (Admin only)
- `POST /api/users` - Create new user
- `GET /api/users/{id}` - Get user by ID

### Accounts
- `GET /api/accounts` - Get user accounts
- `POST /api/accounts` - Create new account
- `GET /api/accounts/{id}` - Get account details

### Transactions
- `GET /api/transactions` - Get transaction history
- `POST /api/transactions/transfer` - Create transfer
- `GET /api/transactions/{id}` - Get transaction details

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Database Operations
```bash
# Create new migration
diesel migration generate <migration_name>

# Run migrations
diesel migration run

# Revert last migration
diesel migration revert

# Reset database (drop and recreate)
diesel database reset
```

### Database Management
```bash
# Connect to the database
docker exec -it bank_app_postgres psql -U bank_user -d bank_app

# Backup database
docker exec bank_app_postgres pg_dump -U bank_user bank_app > backup.sql

# Restore database
docker exec -i bank_app_postgres psql -U bank_user -d bank_app < backup.sql
```

## Security Features

- **Password Hashing**: All passwords are hashed using bcrypt
- **JWT Authentication**: Secure token-based authentication
- **Role-based Access**: Different access levels for different user types
- **SQL Injection Prevention**: Diesel ORM provides type-safe queries
- **Input Validation**: Comprehensive input validation on all endpoints
- **Database Constraints**: Foreign keys and check constraints for data integrity

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For support and questions, please open an issue in the GitHub repository.

## Roadmap

- [ ] Mobile application
- [ ] Advanced reporting and analytics
- [ ] Multi-currency support
- [ ] Integration with external payment systems
- [ ] Advanced security features (2FA, biometric authentication)
- [ ] Real-time notifications
- [ ] API rate limiting
- [ ] Comprehensive audit logging