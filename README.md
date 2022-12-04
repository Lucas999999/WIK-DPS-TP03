# WIK-DPS-TP03

Voici le rendu du TP 3 de devops, le but de ce tp était de faire un fichier docker-compose permettant de lancer 4 répliques du conteneur créé au TP02 ainsi qu'un reverse-proxy qui s'occupera du loadbalancing sur les 4 conteneurs.

## docker-compose

voici le [fichier](docker-compose.yaml):
```
version: '3.8'

services:

  rust_api:

    build:
      context: .
      dockerfile: Dockerfile

    expose:
      - "8080"

    deploy:
      replicas: 4

    restart: always

    networks:
      - front-network

  proxy:

    image: nginx:latest


    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    
    ports:
      - "8080:8080"

    depends_on: 
      - rust_api

    networks:
      - front-network

networks:
  front-network:
    driver: bridge
```
## nginx

voici le [fichier](nginx.conf) de configuration d'nginx qui permet le loadbalancing:
```
events {

}

http {
  server {
    listen 8080;

    location / {
      proxy_pass http://rust_api:8080/;
    }
  }
}
```

ainsi le seul port exposé est le 8080 du reverse-proxy, pour voir le résultat il suffit de lancer le docker-compose:

```
[hurlu@monke TP-WIK-DPS-TP03]$ docker compose up --build
[+] Building 89.8s (9/9) FINISHED  
[...]
tp-wik-dps-tp03-proxy-1     | 172.19.0.1 - - [04/Dec/2022:21:31:47 +0000] "GET / HTTP/1.1" 404 5 "-" "Mozilla/5.0 (X11; Linux x86_64; rv:107.0) Gecko/20100101 Firefox/107.0"
tp-wik-dps-tp03-rust_api-4  | REQUEST : GET /ping HTTP/1.0
tp-wik-dps-tp03-rust_api-4  | Hostname: "a50e4e70ea15"
```

l'api a également été modifiée afin d'afficher son hostname sur la route /ping