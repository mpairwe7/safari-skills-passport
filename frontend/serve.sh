#!/bin/bash

# Safari Skills Passport - Frontend Server
# Simple static file server for development

PORT=${1:-8080}
FRONTEND_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "================================================"
echo "  Safari Skills Passport - Frontend Server"
echo "================================================"
echo ""
echo "📁 Serving from: $FRONTEND_DIR"
echo "🌐 Port: $PORT"
echo "🔗 URL: http://localhost:$PORT"
echo ""
echo "Press Ctrl+C to stop the server"
echo "================================================"
echo ""

# Check if Python 3 is available
if command -v python3 &> /dev/null; then
    echo "✅ Using Python 3 HTTP server"
    cd "$FRONTEND_DIR"
    python3 -m http.server $PORT
elif command -v python &> /dev/null; then
    echo "✅ Using Python 2 HTTP server"
    cd "$FRONTEND_DIR"
    python -m SimpleHTTPServer $PORT
elif command -v php &> /dev/null; then
    echo "✅ Using PHP built-in server"
    cd "$FRONTEND_DIR"
    php -S localhost:$PORT
else
    echo "❌ Error: No suitable HTTP server found"
    echo ""
    echo "Please install one of the following:"
    echo "  - Python 3: sudo apt-get install python3"
    echo "  - PHP: sudo apt-get install php"
    echo "  - Node.js: sudo apt-get install nodejs npm"
    echo "             then run: npx serve -p $PORT"
    exit 1
fi
