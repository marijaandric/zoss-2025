FROM nginx:alpine

# Instaliraj Python i potrebne pakete
RUN apk add --no-cache python3 py3-pip curl bash && \
    pip3 install --break-system-packages requests

# Copy ranjiva nginx konfiguracija
COPY nginx.conf /etc/nginx/nginx.conf

# Copy backend server
COPY backend.py /usr/local/bin/backend.py
RUN chmod +x /usr/local/bin/backend.py

# Copy startup script
COPY start-services.sh /usr/local/bin/start-services.sh
RUN chmod +x /usr/local/bin/start-services.sh

# Copy exploit skripte
COPY exploit.py /usr/local/bin/exploit.py
RUN chmod +x /usr/local/bin/exploit.py

# Expose port 80 (nginx)
EXPOSE 80

# Start both backend and nginx
CMD ["/usr/local/bin/start-services.sh"]
