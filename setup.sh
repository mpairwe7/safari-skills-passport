#!/bin/bash

echo "🦁 Safari Skills Passport - Setup Script"
echo "========================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi
echo "✅ Rust is installed"

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "⚠️  PostgreSQL is not installed. Install it from https://www.postgresql.org/download/"
    echo "   On Ubuntu/Debian: sudo apt install postgresql postgresql-contrib"
    echo "   On macOS: brew install postgresql"
    exit 1
fi
echo "✅ PostgreSQL is installed"

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file..."
    cp .env.example .env
    echo "✅ .env file created. Please update it with your configuration."
else
    echo "✅ .env file already exists"
fi

# Ask if user wants to create the database
read -p "📊 Do you want to create the PostgreSQL database? (y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    read -p "Enter database name [safari_skills_passport]: " DB_NAME
    DB_NAME=${DB_NAME:-safari_skills_passport}
    
    read -p "Enter database user [safari_user]: " DB_USER
    DB_USER=${DB_USER:-safari_user}
    
    read -sp "Enter database password [safari_pass]: " DB_PASS
    DB_PASS=${DB_PASS:-safari_pass}
    echo ""
    
    # Create user and database
    sudo -u postgres psql -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASS';" 2>/dev/null || echo "User might already exist"
    sudo -u postgres psql -c "CREATE DATABASE $DB_NAME OWNER $DB_USER;" 2>/dev/null || echo "Database might already exist"
    sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"
    
    echo "✅ Database setup complete"
    
    # Update .env file
    sed -i "s|DATABASE_URL=.*|DATABASE_URL=postgresql://$DB_USER:$DB_PASS@localhost:5432/$DB_NAME|" .env
fi

# Build the project
echo ""
echo "🔨 Building the project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi

# Optional: Check for IPFS
echo ""
if command -v ipfs &> /dev/null; then
    echo "✅ IPFS is installed"
else
    echo "⚠️  IPFS is not installed. For full functionality, install from https://docs.ipfs.tech/install/"
    echo "   Credentials will still work but without decentralized storage."
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Next steps:"
echo "1. Update .env with your configuration (JWT_SECRET, etc.)"
echo "2. Start the server: cargo run --bin api-server"
echo "3. Access API at: http://localhost:8080"
echo "4. Check health: curl http://localhost:8080/health"
echo ""
echo "📚 See README.md for API documentation"
