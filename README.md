# WIK-DPS-TP02

Voici le rendu du TP 2 de devops, le but de ce tp était de faire 2 images docker (la premiere avec un seul stage et la seconde avec 2) permettant de lancer l'API faite lors du TP1

## Image 1
### contenu

voici le [dockerfile](Dockerfile) de l'image 1:
```
FROM rust:latest
WORKDIR /usr/src/TP-WIK-DPS-02
COPY . .
RUN cargo build
CMD ["./target/debug/TP-WIK-DPS-TP02"]
```
### Fonctionnement

Il suffit de le build pour creer l'image après avoir cloné le repertoire:

```
docker build -f Dockerfile -t [nom]
```

puis ensuite on peut le lancer comme un conteneur classique

```
docker run -p 3333:3333 [nom]
```

on peut vérifier que cela fonctionne:

```
[hurlu@monke DevOps]$ curl localhost:3333/ping
{"Host":"localhost:3333","User-Agent":"curl/7.86.0","Accept":"*/*"}
```

### Trivy

l'image a été scannée avec trivy, le scan se trouve ici:
[image 1](trivy1.md)

On se rend compte que beaucoup de vulnérabilités ont été retournées à la suite de ce scan

## Image 2
### contenu

voici le [dockerfile](Dockerfile2) de l'image 2:
```
FROM rust:latest as builder
WORKDIR /usr/src/TP-WIK-DPS-02
COPY . .
RUN cargo build

FROM builder as exec
CMD ["./target/debug/TP-WIK-DPS-TP02"]
```
### Fonctionnement

Il suffit de le build pour creer l'image après avoir cloné le repertoire:

```
docker build -f Dockerfile2 -t [nom]
```

puis ensuite on peut le lancer comme un conteneur classique

```
docker run -p 3333:3333 [nom]
```

on peut vérifier que cela fonctionne:

```
[hurlu@monke DevOps]$ curl localhost:3333/ping
{"Host":"localhost:3333","User-Agent":"curl/7.86.0","Accept":"*/*"}
```

### Trivy

l'image a été scannée avec trivy, le scan se trouve ici:
[image 2](trivy2.md)

Encore une fois il y a tout autant de vulnérabilités
