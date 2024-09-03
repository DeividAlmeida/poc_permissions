# Etapa de build
FROM ubuntu:24.04 

# Defina o diretório de trabalho dentro do contêiner
WORKDIR /usr/src/app

# Atualize os pacotes e instale o Redis
RUN apt-get update && apt-get install -y redis-server

# Copie o arquivo Cargo.toml e Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copie o código fonte da aplicação
COPY . .

# Comando para iniciar o Redis e a aplicação
CMD ["sh", "-c", "redis-server & ./permissions"]