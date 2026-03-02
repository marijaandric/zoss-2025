#!/bin/sh

echo "[Startup] Starting backend server..."
python3 /usr/local/bin/backend.py &
BACKEND_PID=$!

# Wait for backend to be ready
sleep 2

echo "[Startup] Starting Nginx..."
nginx -g "daemon off;" &
NGINX_PID=$!

# Wait for both processes
wait $BACKEND_PID $NGINX_PID
