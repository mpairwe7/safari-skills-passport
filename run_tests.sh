#!/bin/bash

echo "🧪 Safari Skills Passport - Test Runner"
echo "========================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test results
UNIT_TESTS_PASSED=false
INTEGRATION_TESTS_PASSED=false
SYSTEM_TESTS_PASSED=false

echo "📦 Step 1: Running Unit Tests"
echo "=============================="
cargo test --lib --all -- --nocapture

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Unit tests passed${NC}"
    UNIT_TESTS_PASSED=true
else
    echo -e "${RED}❌ Unit tests failed${NC}"
    exit 1
fi

echo ""
echo "🔌 Step 2: Setting up test environment with Podman"
echo "=================================================="

# Check if podman is installed
if ! command -v podman &> /dev/null; then
    echo -e "${RED}❌ Podman is not installed${NC}"
    echo "Install it with: sudo apt install podman (Ubuntu/Debian)"
    exit 1
fi

# Stop any existing test containers
echo "Stopping existing test containers..."
podman stop safari-test-postgres 2>/dev/null || true
podman rm safari-test-postgres 2>/dev/null || true
podman stop safari-test-ipfs 2>/dev/null || true
podman rm safari-test-ipfs 2>/dev/null || true
podman stop safari-test-api 2>/dev/null || true
podman rm safari-test-api 2>/dev/null || true

# Create a test network
echo "Creating test network..."
podman network create safari-test-network 2>/dev/null || true

# Start PostgreSQL for testing
echo "Starting PostgreSQL test container..."
podman run -d \
    --name safari-test-postgres \
    --network safari-test-network \
    -e POSTGRES_USER=safari_test \
    -e POSTGRES_PASSWORD=safari_test \
    -e POSTGRES_DB=safari_test_db \
    -p 5433:5432 \
    docker.io/library/postgres:15-alpine

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
sleep 5

# Check if PostgreSQL is ready
for i in {1..30}; do
    if podman exec safari-test-postgres pg_isready -U safari_test &> /dev/null; then
        echo -e "${GREEN}PostgreSQL is ready${NC}"
        break
    fi
    if [ $i -eq 30 ]; then
        echo -e "${RED}PostgreSQL failed to start${NC}"
        podman logs safari-test-postgres
        exit 1
    fi
    sleep 1
done

# Start IPFS for testing
echo "Starting IPFS test container..."
podman run -d \
    --name safari-test-ipfs \
    --network safari-test-network \
    -p 5002:5001 \
    docker.io/ipfs/kubo:latest

echo "Waiting for IPFS to be ready..."
sleep 3

# Set test environment variables
export DATABASE_URL="postgresql://safari_test:safari_test@localhost:5433/safari_test_db"
export JWT_SECRET="test-secret-key-for-testing-only-min-32-characters"
export JWT_EXPIRATION_HOURS=24
export IPFS_URL="http://localhost:5002"
export BLOCKCHAIN_NODE_URL="ws://127.0.0.1:9944"
export RUST_LOG=info
export ENVIRONMENT=test
export HOST=0.0.0.0
export PORT=8081

echo ""
echo "🔧 Step 3: Running Database Migrations"
echo "======================================"

# Use our custom migration runner
echo "Running migrations using migration binary..."
cargo run --bin migrate

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Database migrations failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Database migrations completed${NC}"
sleep 2

echo ""
echo "🔗 Step 4: Running Integration Tests"
echo "===================================="

# Build API server first (so compilation doesn't happen in background)
echo "Building API server..."
cargo build --bin api-server

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ API server build failed${NC}"
    exit 1
fi

# Start the built API server
echo "Starting API server..."
./target/debug/api-server &
API_PID=$!

# Wait for API to be ready
echo "Waiting for API server to be ready..."
sleep 3

# Check if server is responding
for i in {1..60}; do
    if curl -s http://localhost:8081/health &> /dev/null; then
        echo -e "${GREEN}✅ API server is ready${NC}"
        break
    fi
    if [ $i -eq 60 ]; then
        echo -e "${RED}❌ API server failed to respond after 60 seconds${NC}"
        echo "Checking if process is still running..."
        if kill -0 $API_PID 2>/dev/null; then
            echo "Process is running but not responding. Checking logs..."
            # Process is running but not responding - might need to check logs
        else
            echo "Process has died. This might be a startup error."
        fi
        kill $API_PID 2>/dev/null || true
        exit 1
    fi
    sleep 1
done

# Run integration tests
BASE_URL="http://localhost:8081" cargo test --package api-server --test integration_tests -- --nocapture

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Integration tests passed${NC}"
    INTEGRATION_TESTS_PASSED=true
else
    echo -e "${RED}❌ Integration tests failed${NC}"
    kill $API_PID 2>/dev/null || true
    exit 1
fi

# Stop API server
kill $API_PID 2>/dev/null || true
sleep 2

echo ""
echo "🌐 Step 5: Running System Tests with Podman"
echo "==========================================="

# Build the application image
echo "Building application image..."
podman build -t safari-skills-passport:test -f Dockerfile .

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to build Docker image${NC}"
    exit 1
fi

# Start the application container
echo "Starting application container..."
podman run -d \
    --name safari-test-api \
    --network safari-test-network \
    -p 8082:8080 \
    -e DATABASE_URL="postgresql://safari_test:safari_test@safari-test-postgres:5432/safari_test_db" \
    -e JWT_SECRET="test-secret-key-for-testing-only-min-32-characters" \
    -e JWT_EXPIRATION_HOURS=24 \
    -e IPFS_URL="http://safari-test-ipfs:5001" \
    -e BLOCKCHAIN_NODE_URL="ws://127.0.0.1:9944" \
    -e RUST_LOG=info \
    -e ENVIRONMENT=test \
    -e HOST=0.0.0.0 \
    -e PORT=8080 \
    safari-skills-passport:test

# Wait for API to be ready
echo "Waiting for containerized API to be ready..."
sleep 10

for i in {1..30}; do
    if curl -s http://localhost:8082/health &> /dev/null; then
        echo -e "${GREEN}Containerized API is ready${NC}"
        break
    fi
    if [ $i -eq 30 ]; then
        echo -e "${RED}Containerized API failed to respond${NC}"
        podman logs safari-test-api
        exit 1
    fi
    sleep 1
done

# Run system tests
echo "Running system tests..."
BASE_URL="http://localhost:8082" cargo test --package api-server --test system_tests -- --nocapture

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ System tests passed${NC}"
    SYSTEM_TESTS_PASSED=true
else
    echo -e "${RED}❌ System tests failed${NC}"
    podman logs safari-test-api
    exit 1
fi

echo ""
echo "🧹 Step 6: Cleanup"
echo "=================="

# Stop and remove containers
podman stop safari-test-api safari-test-postgres safari-test-ipfs 2>/dev/null || true
podman rm safari-test-api safari-test-postgres safari-test-ipfs 2>/dev/null || true

# Remove test network
podman network rm safari-test-network 2>/dev/null || true

echo ""
echo "📊 Test Summary"
echo "==============="
echo -e "Unit Tests:        ${GREEN}✅ PASSED${NC}"
echo -e "Integration Tests: ${GREEN}✅ PASSED${NC}"
echo -e "System Tests:      ${GREEN}✅ PASSED${NC}"
echo ""
echo -e "${GREEN}🎉 All tests passed successfully!${NC}"
