FROM ubuntu:24.04 

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y redis-server

COPY . .

CMD ["sh", "-c", "redis-server & ./permissions"]