#!/bin/bash

echo "=============================================="
echo "Nginx Worker Exhaustion - Quick Test"
echo "=============================================="
echo ""

# Step 1: Stop and remove existing container
echo "[1/3] Stopping and removing existing container..."
docker stop nginx-dos 2>/dev/null || true
docker rm nginx-dos 2>/dev/null || true
echo "  [✓] Container removed"
echo ""

# Step 2: Build Docker image
echo "[2/3] Building Docker image..."
docker build -t nginx-dos .
if [ $? -ne 0 ]; then
    echo "  [✗] Failed to build image"
    exit 1
fi
echo "  [✓] Image built successfully"
echo ""

# Step 3: Start new container
echo "[3/3] Starting new container..."
docker run -d --name nginx-dos -p 8080:80 nginx-dos
if [ $? -ne 0 ]; then
    echo "  [✗] Failed to start container"
    exit 1
fi
echo "  [✓] Container started"
echo ""

# Wait for services to be ready
echo "Waiting for services to start..."
sleep 3
echo ""

echo "[✓] Container is running"
echo ""

# Test 1: Normal request
echo "[1/4] Testing normal request..."
RESPONSE=$(docker exec nginx-dos curl -s http://localhost:80/)
if [[ "$RESPONSE" == *"success"* ]]; then
    echo "  [✓] Server responds normally"
    echo "  Response: $RESPONSE"
else
    echo "  [✗] Unexpected response: $RESPONSE"
fi
echo ""

# Test 2: Check server status
echo "[2/4] Checking server status..."
docker exec nginx-dos curl -s http://localhost:80/status
echo ""

# Test 3: Check backend health
echo "[3/4] Checking backend health..."
docker exec nginx-dos curl -s http://localhost:80/health
echo ""

# Test 4: Run exploit
echo "[4/4] Running Worker Process Exhaustion attack..."
echo "  (This will send 50 parallel requests)"
docker exec -it nginx-dos python3 /usr/local/bin/exploit.py
echo ""

echo "=============================================="
echo "Attack completed"
echo "=============================================="
echo ""
echo "Try accessing the server now from another terminal:"
echo "  curl http://localhost:8080/"
echo ""
echo "Expected result: Timeout or very slow response"
echo ""
echo "To stop the server:"
echo "  bash stop.sh"
echo ""
