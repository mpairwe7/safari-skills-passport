# Safari Skills Passport Guide

A blockchain-powered digital credentialing and verification platform for
African professionals, built with Rust, Substrate, and modern web
technologies.

## Overview

Safari Skills Passport enables: - Professionals to store and share
verified credentials digitally - Institutions to issue tamper-proof
credentials on-chain - Employers to verify credentials instantly via QR
codes or credential IDs


## 🚀 Getting Started

### Prerequisites

- **Rust** (1.75+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **PostgreSQL** (14+): [Installation Guide](https://www.postgresql.org/download/)
- **IPFS** (optional, for full functionality): [Installation Guide](https://docs.ipfs.tech/install/)

### Installation

1. **Clone the repository**
```bash
git clone <repository-url>
cd Safari
```

2. **Set up environment variables**
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. **Create PostgreSQL database**
```bash
createdb safari_skills_passport
# Or create user and database:
psql -U postgres
CREATE USER safari_user WITH PASSWORD 'safari_pass';
CREATE DATABASE safari_skills_passport OWNER safari_user;
\q
```

4. **Build the project**
```bash
cargo build --release
```

5. **Run database migrations**
```bash
# Migrations run automatically on server start
# Or manually with sqlx-cli:
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run --database-url "postgresql://safari_user:safari_pass@localhost/safari_skills_passport"
```

6. **Start the API server**
```bash
cargo run --bin api-server
```

The server will start on `http://localhost:8080`

### Frontend UI (NEW!)

Launch the web-based user interface:

```bash
# Option 1: Using the built-in server script
cd frontend
./serve.sh 8081

# Option 2: Using Python
cd frontend
python3 -m http.server 8081

# Option 3: Using Node.js
cd frontend
npx serve -p 8081
```

Then visit: `http://localhost:8081/demo.html` or `http://localhost:8081`

#### User Endpoints

**Login User Credentials**
```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "olowophily11@gmail.com",
  "password": "NFkr4ddaZDh7iDG",
}