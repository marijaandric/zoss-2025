# Pokretanje

## Automatsko
```bash
bash start.sh
```

## Manuelno
```bash
docker stop rust-race-condition
docker rm rust-race-condition
docker build -t rust-race-condition .
docker run -d --name rust-race-condition -p 8080:8080 rust-race-condition
sleep 3
docker exec rust-race-condition python3 /app/exploit.py
```