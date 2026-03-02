#!/bin/bash

echo "=============================================="
echo "TOCTOU Race Condition Demo"
echo "=============================================="
echo ""

# Stop and remove existing container
echo "[1/4] Cleaning up..."
docker stop rust-race-condition 2>/dev/null
docker rm rust-race-condition 2>/dev/null
echo ""

# Build Docker image
echo "[2/4] Building..."
docker build -t rust-race-condition .
if [ $? -ne 0 ]; then
    echo "  [✗] Build failed"
    exit 1
fi
echo ""

# Start container
echo "[3/4] Starting server..."
docker run -d --name rust-race-condition -p 8080:8080 rust-race-condition
echo ""
echo "Waiting for server..."
sleep 3
echo ""

# Run exploit inside container
echo "[4/4] Running exploit..."
echo ""
docker exec rust-race-condition python3 /app/exploit.py
echo ""
echo "=============================================="
echo "Demo complete!"
echo "=============================================="
echo ""
echo "Stop server:"
echo "  bash stop.sh"
echo ""
