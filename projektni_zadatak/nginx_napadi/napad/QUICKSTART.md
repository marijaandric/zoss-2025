# Pokretanje

## Automatsko
```bash
bash test.sh
```

## Manuelno
```bash
docker stop nginx-dos
docker rm nginx-dos
docker build -t nginx-worker-dos .
docker run -d -p 8080:80 --name nginx-dos nginx-worker-dos
sleep 3
docker exec -it nginx-dos python3 /usr/local/bin/exploit.py
```