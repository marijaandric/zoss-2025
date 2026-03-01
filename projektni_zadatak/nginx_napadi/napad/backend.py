#!/usr/bin/env python3
"""
Slow Backend Server
Simulira slow backend aplikaciju koja obrađuje zahteve 1 sekundu.
Koristi se za demonstraciju Worker Process Exhaustion napada.
"""

from http.server import HTTPServer, BaseHTTPRequestHandler
import time
import json

class SlowHandler(BaseHTTPRequestHandler):
    
    def log_message(self, format, *args):
        """Custom logging"""
        print(f"[Backend] {self.address_string()} - {format % args}")
    
    def do_GET(self):
        """Handle GET request - sleep 1 second to simulate slow processing"""
        
        if self.path == '/':
            # Main endpoint - simulate slow processing
            print(f"[Backend] Processing request from {self.address_string()}...")
            time.sleep(0.5)
            
            response = {
                "status": "success",
                "message": "Request processed after 1 second delay",
                "processing_time": 1.0
            }
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(response).encode())
            
        elif self.path == '/health':
            # Health check endpoint
            response = {
                "status": "healthy",
                "backend": "slow-server",
                "delay_per_request": "1 second"
            }
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(response).encode())
            
        else:
            self.send_response(404)
            self.send_header('Content-Type', 'text/plain')
            self.end_headers()
            self.wfile.write(b"Not Found\n")

def run_server(host='0.0.0.0', port=8000):
    """Run the slow backend server"""
    server = HTTPServer((host, port), SlowHandler)
    print(f"[Backend] Slow Backend Server running on {host}:{port}")
    print(f"[Backend] Each request will take 1 second to process")
    print(f"[Backend] Endpoints:")
    print(f"[Backend]   / - Slow endpoint (1 second delay)")
    print(f"[Backend]   /fast - Fast endpoint (no delay)")
    print(f"[Backend]   /health - Health check")
    print()
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n[Backend] Server stopped")
        server.shutdown()

if __name__ == '__main__':
    run_server()
