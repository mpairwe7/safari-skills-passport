# Safari Skills Passport - Installation Guide

This guide provides step-by-step instructions for installing and setting up the Safari Skills Passport system for development and production environments.

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Quick Start (Docker)](#quick-start-docker)
3. [Manual Installation](#manual-installation)
4. [Configuration](#configuration)
5. [Running the Application](#running-the-application)
6. [Troubleshooting](#troubleshooting)

---

## System Requirements

### Minimum Requirements
- **OS**: Linux, macOS, or Windows (with WSL2)
- **CPU**: 4 cores
- **RAM**: 8GB
- **Disk Space**: 20GB (for dependencies and database)
- **Network**: Internet connection for dependencies

### Software Prerequisites

#### Required
- **Rust**: 1.70+ ([Install](https://rustup.rs/))
- **Node.js**: 18+ ([Install](https://nodejs.org/))
- **Docker & Docker Compose**: 20.10+ ([Install](https://docs.docker.com/get-docker/))
- **Git**: 2.30+ ([Install](https://git-scm.com/))
- **PostgreSQL**: 13+ (included in Docker)
- **IPFS**: (included in Docker)

#### Optional (for development)
- **VS Code**: Code editor ([Install](https://code.visualstudio.com/))
- **Postman**: API testing ([Install](https://www.postman.com/))
- **pgAdmin**: Database UI ([Install](https://www.pgadmin.org/))

---

## Quick Start (Docker)

### Prerequisites
- Docker and Docker Compose installed
- 8GB+ available RAM
- Port 3000, 8080, 5432, 5001, 4001 available

### Installation Steps

#### 1. Clone the Repository
```bash
git clone https://github.com/mpairwe7/safari-skills-passport.git
cd safari-skills-passport
```

#### 2. Copy Environment Configuration
```bash
cp .env.example .env
```

#### 3. Build and Start Services
```bash
docker-compose up -d
```

This starts:
- **PostgreSQL** (Port 5432)
- **IPFS** (Ports 5001, 4001)
- **API Server** (Port 8080)
- **Frontend** (Port 3000)

#### 4. Wait for Services to Initialize
```bash
# Check service status
docker-compose ps

# View logs
docker-compose logs -f api-server
```

#### 5. Access the Application
- **Frontend**: http://localhost:3000
- **API**: http://localhost:8080/api
- **API Documentation**: http://localhost:8080/api/docs (if available)

#### 6. Run Tests
```bash
docker-compose exec api-server cargo test
```

---

## Manual Installation

### Step 1: Clone Repository
```bash
git clone https://github.com/mpairwe7/safari-skills-passport.git
cd safari-skills-passport
```

### Step 2: Install Rust Dependencies
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Update Rust
rustup update
```

### Step 3: Install Node.js Dependencies (Frontend)
```bash
cd frontend
npm install
```

### Step 4: Start PostgreSQL
```bash
# Using Docker
docker run -d \
  --name safari-postgres \
  -e POSTGRES_USER=safari_user \
  -e POSTGRES_PASSWORD=safari_password \
  -e POSTGRES_DB=safari_db \
  -p 5432:5432 \
  postgres:15

# Or using native PostgreSQL (macOS)
brew install postgresql@15
brew services start postgresql@15
createdb safari_db
psql safari_db -c "CREATE USER safari_user WITH PASSWORD 'safari_password';"
```

### Step 5: Start IPFS
```bash
# Using Docker
docker run -d \
  --name safari-ipfs \
  -p 5001:5001 \
  -p 4001:4001 \
  ipfs/kubo:latest

# Or using native IPFS
ipfs daemon
```

### Step 6: Configure Environment Variables
Create `.env` file in the project root:
```env
# Database
DATABASE_URL=postgresql://safari_user:safari_password@localhost:5432/safari_db

# Server
API_HOST=0.0.0.0
API_PORT=8080
ENVIRONMENT=development

# IPFS
IPFS_API_URL=http://localhost:5001

# JWT (generate with: openssl rand -hex 32)
JWT_SECRET=your_generated_secret_here

# CORS
CORS_ORIGIN=http://localhost:3000

# Frontend
FRONTEND_URL=http://localhost:3000
```

### Step 7: Run Database Migrations
```bash
cd crates/database
cargo run --bin migrate
```

### Step 8: Build and Run Backend
```bash
cd crates/api-server
cargo build --release
cargo run --release
```

The API server will start on `http://localhost:8080`

### Step 9: Build and Run Frontend
```bash
cd frontend
npm run build  # or: npm start for development
python3 -m http.server 3000  # Serve built files
```

Access the application at `http://localhost:3000`

---

## Configuration

### Environment Variables

#### Database Configuration
```env
# PostgreSQL connection string
DATABASE_URL=postgresql://user:password@host:port/database

# Connection pool settings
DATABASE_POOL_MIN_SIZE=5
DATABASE_POOL_MAX_SIZE=20
```

#### API Server Configuration
```env
# Host and port
API_HOST=0.0.0.0
API_PORT=8080

# Environment
ENVIRONMENT=development|production

# CORS settings
CORS_ORIGIN=http://localhost:3000
```

#### Authentication
```env
# JWT secret (use openssl rand -hex 32 to generate)
JWT_SECRET=your_secret_here

# Token expiration (in seconds)
JWT_EXPIRATION=86400
```

#### IPFS Configuration
```env
# IPFS API endpoint
IPFS_API_URL=http://localhost:5001

# IPFS Gateway
IPFS_GATEWAY_URL=https://ipfs.io/ipfs/
```

#### Blockchain Configuration
```env
# Mock blockchain for testing
BLOCKCHAIN_MOCK=true

# Or connect to real blockchain
BLOCKCHAIN_RPC_URL=http://localhost:8545
BLOCKCHAIN_CHAIN_ID=1
```

#### Frontend Configuration
Edit `frontend/js/app.js`:
```javascript
const CONFIG = {
    API_BASE_URL: 'http://localhost:8080/api',
    STORAGE_KEY: 'ssp_auth_token',
    USER_KEY: 'ssp_user_data',
    TOAST_DURATION: 5000,
    ANIMATION_DURATION: 300
};
```

---

## Running the Application

### Development Mode

#### Backend (Rust)
```bash
cd crates/api-server
cargo run  # Uses debug build
```

Server runs on `http://localhost:8080`

#### Frontend (with live reload)
```bash
cd frontend
npm start
```

Frontend available on `http://localhost:3000`

### Production Mode

#### Build Rust Backend
```bash
cd crates/api-server
cargo build --release
./target/release/api-server
```

#### Build Frontend
```bash
cd frontend
npm run build
npm start  # Serves optimized build
```

### Using Docker Compose (Recommended)
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Stop and remove volumes (clean reset)
docker-compose down -v
```

---

## Troubleshooting

### Port Already in Use
```bash
# Find process using port 8080
lsof -i :8080

# Kill process
kill -9 <PID>
```

### Database Connection Failed
```bash
# Check PostgreSQL is running
docker ps | grep postgres

# Test connection
psql -h localhost -U safari_user -d safari_db

# Check environment variables
echo $DATABASE_URL
```

### IPFS Connection Issues
```bash
# Check IPFS daemon is running
curl http://localhost:5001/api/v0/version

# Restart IPFS
docker restart safari-ipfs
```

### Frontend API Calls Return 404
1. Check API server is running: `curl http://localhost:8080/health`
2. Verify `API_BASE_URL` in `frontend/js/app.js`
3. Check CORS settings in `.env`
4. Look at browser console for actual error details

### Build Failures

#### Rust Compilation Error
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

#### Node Dependency Issues
```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Performance Issues
```bash
# Monitor resource usage
docker stats

# Check database query performance
psql -d safari_db -c "EXPLAIN ANALYZE SELECT ..."

# Increase pool size if needed
DATABASE_POOL_MAX_SIZE=50
```

### Common Error Messages

#### "Connection refused"
- Ensure all services are running
- Check firewall settings
- Verify ports are not blocked

#### "Invalid JWT token"
- Token may have expired
- JWT_SECRET might be different between sessions
- Clear browser storage and re-login

#### "IPFS timeout"
- IPFS daemon may be overloaded
- Restart IPFS service
- Check network connectivity

---

## Next Steps

After installation:

1. **Create an account**: Visit http://localhost:3000 and register
2. **Choose a role**: Professional, Employer, or Institution
3. **Explore the dashboard**: Familiarize yourself with the UI
4. **Run tests**: `./run_tests.sh` to verify everything works
5. **Read the User Guide**: See `USER_GUIDE.md`

## Getting Help

- **Documentation**: Check `README.md` and other `.md` files
- **API Examples**: See `API_EXAMPLES.md`
- **Architecture**: Review `ARCHITECTURE.md`
- **Issues**: Open an issue on [GitHub](https://github.com/mpairwe7/safari-skills-passport/issues)
- **Tests**: Review `TEST_RESULTS.md` for test coverage

---

## Security Notes

⚠️ **Important for Production:**

1. Change all default passwords
2. Generate strong JWT_SECRET: `openssl rand -hex 32`
3. Use HTTPS/TLS certificates
4. Enable database backups
5. Use environment-specific configuration
6. Keep dependencies updated
7. Enable authentication for IPFS API
8. Use reverse proxy (nginx) for frontend

---

## License

This project is licensed under the MIT License. See `LICENSE` file for details.

---

## Support

For issues, questions, or contributions, please visit:
https://github.com/mpairwe7/safari-skills-passport
