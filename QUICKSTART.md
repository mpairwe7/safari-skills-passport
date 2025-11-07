# Quick Start Guide

Get Safari Skills Passport running in 5 minutes!

## Prerequisites Check

```bash
# Check Rust
rustc --version
# If not installed: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Check PostgreSQL
psql --version
# If not installed: sudo apt install postgresql (Ubuntu/Debian)

# Check Git
git --version
```

## Option 1: Automated Setup (Recommended)

```bash
# Clone repository
git clone <repository-url>
cd Safari

# Run setup script
./setup.sh

# Start the server
cargo run --bin api-server
```

## Option 2: Docker Setup (Easiest)

```bash
# Clone repository
git clone <repository-url>
cd Safari

# Copy and edit environment variables
cp .env.example .env
nano .env  # Edit JWT_SECRET

# Start all services
docker-compose up -d

# Check logs
docker-compose logs -f api-server

# Stop services
docker-compose down
```

## Option 3: Manual Setup

### 1. Setup Database

```bash
# Connect to PostgreSQL
sudo -u postgres psql

# Create user and database
CREATE USER safari_user WITH PASSWORD 'safari_pass';
CREATE DATABASE safari_skills_passport OWNER safari_user;
GRANT ALL PRIVILEGES ON DATABASE safari_skills_passport TO safari_user;
\q
```

### 2. Configure Environment

```bash
cp .env.example .env
nano .env
```

Update these values:
```env
DATABASE_URL=postgresql://safari_user:safari_pass@localhost:5432/safari_skills_passport
JWT_SECRET=your-super-secret-key-min-32-chars-change-in-production
```

### 3. Build and Run

```bash
# Build
cargo build --release

# Run
cargo run --bin api-server

# Or run the binary directly
./target/release/api-server
```

## Verify Installation

### 1. Health Check

```bash
curl http://localhost:8080/health
```

Expected response:
```json
{
  "status": "healthy",
  "service": "Safari Skills Passport API",
  "version": "0.1.0"
}
```

### 2. Register a Test User

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "TestPass123!",
    "name": "Test User",
    "role": "professional"
  }'
```

You should get a token and user object back!

## Common Issues

### Issue: Database Connection Failed

```bash
# Check if PostgreSQL is running
sudo systemctl status postgresql

# Start PostgreSQL
sudo systemctl start postgresql

# Verify connection
psql -U safari_user -d safari_skills_passport
```

### Issue: Port 8080 Already in Use

```bash
# Find process using port 8080
sudo lsof -i :8080

# Kill the process
kill -9 <PID>

# Or change port in .env
PORT=8081
```

### Issue: Build Failed

```bash
# Update Rust
rustup update

# Clean build cache
cargo clean

# Rebuild
cargo build --release
```

### Issue: IPFS Not Available

IPFS is optional for basic functionality. The system will work without it, but credentials won't be stored on IPFS.

To install IPFS:
```bash
# Download and install
wget https://dist.ipfs.tech/kubo/v0.24.0/kubo_v0.24.0_linux-amd64.tar.gz
tar -xvzf kubo_v0.24.0_linux-amd64.tar.gz
cd kubo
sudo bash install.sh

# Initialize and start
ipfs init
ipfs daemon
```

## Next Steps

1. **Explore the API**: Check `API_EXAMPLES.md` for detailed examples
2. **Register an institution**: Create an institution account and register details
3. **Issue credentials**: Have an admin accredit your institution, then issue test credentials
4. **Verify credentials**: Test the verification endpoints
5. **Read architecture**: See `ARCHITECTURE.md` for system design details

## Development Mode

```bash
# Run with auto-reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x 'run --bin api-server'

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Production Deployment

### Using Docker Compose (Recommended)

```bash
# Production setup
cp .env.example .env
# Edit .env with production values

# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Scale API servers
docker-compose up -d --scale api-server=3
```

### Manual Deployment

```bash
# Build optimized release
cargo build --release

# Copy binary to server
scp target/release/api-server user@server:/opt/safari/

# Setup systemd service
sudo nano /etc/systemd/system/safari-api.service
```

Add:
```ini
[Unit]
Description=Safari Skills Passport API
After=network.target postgresql.service

[Service]
Type=simple
User=safari
WorkingDirectory=/opt/safari
Environment="RUST_LOG=info"
EnvironmentFile=/opt/safari/.env
ExecStart=/opt/safari/api-server
Restart=always

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start
sudo systemctl enable safari-api
sudo systemctl start safari-api
sudo systemctl status safari-api
```

## Monitoring

```bash
# Check API logs
docker-compose logs api-server

# Check database
docker-compose exec postgres psql -U safari_user -d safari_skills_passport

# Monitor resources
docker stats

# View API metrics
curl http://localhost:8080/health
```

## Getting Help

- **Documentation**: Check `README.md`, `API_EXAMPLES.md`, `ARCHITECTURE.md`
- **Issues**: Open an issue on GitHub
- **Logs**: Check `RUST_LOG=debug cargo run` for detailed logs
- **Community**: Join our Slack/Discord (if available)

## Useful Commands

```bash
# Reset database
psql -U safari_user -d safari_skills_passport -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"

# Rebuild from scratch
cargo clean && cargo build --release

# Check port usage
sudo netstat -tulpn | grep 8080

# Test endpoint
curl -v http://localhost:8080/health

# Format JSON response
curl http://localhost:8080/api/credentials/my -H "Authorization: Bearer TOKEN" | jq '.'
```

## Security Checklist

Before going to production:

- [ ] Change `JWT_SECRET` to a strong random value (32+ chars)
- [ ] Use strong database passwords
- [ ] Enable HTTPS (use nginx/caddy reverse proxy)
- [ ] Set up firewall rules
- [ ] Enable database backups
- [ ] Configure CORS for your domain
- [ ] Set `ENVIRONMENT=production` in .env
- [ ] Review and set appropriate `RUST_LOG` level
- [ ] Set up monitoring and alerting
- [ ] Enable rate limiting
- [ ] Review API authentication on all endpoints

## Success!

You now have Safari Skills Passport running! 🎉

Start by:
1. Creating test users
2. Registering an institution
3. Issuing a test credential
4. Verifying the credential

Happy coding! 🚀
