# Safari Skills Passport

A blockchain-powered digital credentialing and verification platform for African professionals, built with Rust, Substrate, and modern web technologies.

## 🌍 Overview

Safari Skills Passport enables:
- **Professionals** to store and share verified credentials digitally
- **Institutions** to issue tamper-proof credentials on-chain
- **Employers** to verify credentials instantly via QR codes or credential IDs

## 🏗️ Architecture

### Technology Stack

- **Blockchain Layer**: Substrate (custom pallets for users, institutions, and credentials)
- **API Server**: Axum web framework
- **Database**: PostgreSQL (metadata storage)
- **Off-chain Storage**: IPFS (credential documents)
- **Authentication**: JWT tokens
- **Languages**: Rust (100%)

### Project Structure

```
Safari/
├── crates/
│   ├── blockchain/          # Substrate pallets
│   │   ├── pallet_users/    # User management on-chain
│   │   ├── pallet_institutions/  # Institution registry
│   │   └── pallet_credentials/   # Credential issuance & verification
│   ├── api-server/          # REST API backend
│   │   ├── handlers/        # HTTP request handlers
│   │   ├── services/        # Business logic
│   │   ├── middleware/      # Auth middleware
│   │   └── utils/          # Utilities (QR codes, etc.)
│   ├── common/             # Shared types and errors
│   └── database/           # PostgreSQL models & repositories
├── frontend/               # Web-based user interface (NEW!)
│   ├── index.html         # Main application
│   ├── demo.html          # Demo landing page
│   ├── css/               # Stylesheets
│   ├── js/                # Application logic
│   ├── serve.sh           # Development server
│   └── README.md          # Frontend documentation
├── Cargo.toml              # Workspace configuration
└── .env.example           # Environment variables template
```

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

**Features:**
- 🎨 Modern, responsive design
- ♿ WCAG 2.1 AA accessibility compliant
- 📱 Mobile-first, works on all devices
- 🔐 Secure JWT authentication
- 📷 QR code scanner for verification
- 🎯 Real-time credential dashboard
- 🌍 Optimized for African professionals

See [frontend/README.md](frontend/README.md) for detailed documentation.

### Optional: Run IPFS Node

For full credential document storage:

```bash
# Install IPFS
ipfs init
ipfs daemon
```

## 📚 API Documentation

### Base URL
```
http://localhost:8080/api
```

### Authentication

Most endpoints require JWT authentication. Include the token in the Authorization header:
```
Authorization: Bearer <your-jwt-token>
```

### Endpoints

#### Health Check
```http
GET /health
```

#### Auth Endpoints

**Register User**
```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "olowophily11@gmail.com",
  "password": "NFkr4ddaZDh7iDG",
  "name": "John Doe",
  "role": "professional" | "institution" | "employer"
}

Response:
{
  "token": "jwt-token",
  "user": {
    "id": "uuid",
    "wallet_address": "substrate-address",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "professional",
    "is_verified": false,
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-01-01T00:00:00Z"
  }
}
```

**Login**
```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure_password"
}

Response: Same as register
```

#### Institution Endpoints

**Register Institution** (requires institution role)
```http
POST /api/institutions/register
Authorization: Bearer <token>
Content-Type: application/json

{
  "institution_name": "University of Example",
  "institution_type": "University",
  "country": "Kenya",
  "accreditation_number": "ACC123456"
}

Response:
{
  "id": "uuid",
  "user_id": "uuid",
  "institution_name": "University of Example",
  "institution_type": "University",
  "country": "Kenya",
  "accreditation_number": "ACC123456",
  "is_accredited": false,
  "created_at": "2025-01-01T00:00:00Z"
}
```

**Get My Institution**
```http
GET /api/institutions/me
Authorization: Bearer <token>

Response: Same as register institution
```

#### Credential Endpoints

**Issue Credential** (requires accredited institution)
```http
POST /api/credentials/issue
Authorization: Bearer <token>
Content-Type: application/json

{
  "holder_email": "professional@example.com",
  "credential_type": "certificate" | "license" | "degree" | "workexperience" | "skill",
  "title": "Bachelor of Science in Computer Science",
  "description": "Completed degree program with honors",
  "issue_date": "2025-01-01T00:00:00Z",
  "expiry_date": null,
  "metadata": {
    "gpa": "3.8",
    "honors": "Magna Cum Laude"
  },
  "document_data": "base64-encoded-pdf-or-image"
}

Response:
{
  "credential_id": "SSP-uuid",
  "ipfs_hash": "QmXxx...",
  "chain_hash": "0x123...",
  "qr_code": "base64-encoded-qr-image"
}
```

**Verify Credential** (public endpoint)
```http
GET /api/credentials/verify/:credential_id

Response:
{
  "valid": true,
  "credential": { ... },
  "issuer": { ... },
  "holder": { ... },
  "message": "Credential is valid and verified"
}
```

**Verify QR Code**
```http
POST /api/credentials/verify-qr
Content-Type: application/json

{
  "qr_data": "SSP-uuid"
}

Response: Same as verify credential
```

**Get My Credentials**
```http
GET /api/credentials/my
Authorization: Bearer <token>

Response:
{
  "credentials": [ ... ],
  "total": 5
}
```

**Get Issued Credentials** (institutions only)
```http
GET /api/credentials/issued
Authorization: Bearer <token>

Response: Same as get my credentials
```

**Get Specific Credential**
```http
GET /api/credentials/:credential_id
Authorization: Bearer <token>

Response: Single credential object
```

**Revoke Credential** (issuer only)
```http
POST /api/credentials/:credential_id/revoke
Authorization: Bearer <token>

Response:
{
  "message": "Credential revoked successfully",
  "credential_id": "SSP-uuid"
}
```

## 🔐 Security Features

1. **JWT Authentication**: Secure token-based authentication
2. **Password Hashing**: bcrypt with salt
3. **On-chain Verification**: Credential hashes stored on blockchain
4. **Accreditation System**: Only accredited institutions can issue credentials
5. **Role-based Access Control**: Enforced at API and handler levels

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p api-server
cargo test -p database
cargo test -p blockchain
```

## 📊 Database Schema

### Users Table
- `id`: UUID (primary key)
- `wallet_address`: String (unique, blockchain address)
- `email`: String (unique)
- `password_hash`: String
- `name`: String
- `role`: Enum (professional, institution, employer)
- `is_verified`: Boolean
- `created_at`, `updated_at`: Timestamps

### Institutions Table
- `id`: UUID (primary key)
- `user_id`: UUID (foreign key to users)
- `institution_name`: String
- `institution_type`: String
- `country`: String
- `accreditation_number`: String (optional)
- `is_accredited`: Boolean
- `created_at`: Timestamp

### Credentials Table
- `id`: UUID (primary key)
- `credential_id`: String (unique, e.g., SSP-uuid)
- `holder_id`: UUID (foreign key to users)
- `issuer_id`: UUID (foreign key to users)
- `credential_type`: Enum
- `title`: String
- `description`: Text
- `ipfs_hash`: String (IPFS content hash)
- `chain_hash`: String (blockchain hash)
- `issue_date`: Timestamp
- `expiry_date`: Timestamp (optional)
- `status`: Enum (pending, issued, revoked, expired)
- `metadata`: JSONB
- `created_at`: Timestamp

## 🛠️ Development

### Adding New Features

1. **Add models** in `crates/common/src/models.rs`
2. **Add database migrations** in `crates/database/migrations/`
3. **Implement repository methods** in `crates/database/src/repositories.rs`
4. **Create handlers** in `crates/api-server/src/handlers/`
5. **Add routes** in `crates/api-server/src/main.rs`

### Code Style

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for common mistakes
cargo check
```

## 🌐 Deployment

### Docker Deployment (Recommended)

```dockerfile
# Dockerfile (create this)
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin api-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api-server /usr/local/bin/
EXPOSE 8080
CMD ["api-server"]
```

```bash
docker build -t safari-skills-passport .
docker run -p 8080:8080 --env-file .env safari-skills-passport
```

### Production Considerations

1. **Use strong JWT secrets** (32+ characters)
2. **Enable HTTPS** (use nginx/caddy reverse proxy)
3. **Set up database backups**
4. **Configure CORS** appropriately
5. **Monitor logs** (structured logging with tracing)
6. **Run IPFS node** for decentralized storage
7. **Deploy Substrate node** for blockchain functionality

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

MIT License - see LICENSE file for details

## 🔗 Resources

- [Substrate Documentation](https://docs.substrate.io/)
- [Axum Documentation](https://docs.rs/axum/)
- [IPFS Documentation](https://docs.ipfs.tech/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)

## 💡 Future Enhancements

- [ ] Mobile application (React Native/Flutter)
- [ ] Email verification system
- [ ] Multi-signature credential issuance
- [ ] Credential templates
- [ ] Analytics dashboard
- [ ] Integration with African job platforms
- [ ] NFT-based credentials
- [ ] Decentralized identity (DID) support
- [ ] Multi-language support (English, Swahili, French, etc.)

## 📧 Support

For issues and questions, please open an issue on GitHub or contact the development team.

---

**Built with ❤️ for African professionals**
