# Etapa de build
FROM ubuntu:24.04 

# Defina o diretório de trabalho dentro do contêiner
WORKDIR /usr/src/app

# Copie o arquivo Cargo.toml e Cargo.lock
COPY Cargo.toml Cargo.lock ./


# Copie o código fonte da aplicação
COPY . .

CMD ["ls"]

CMD ["./permissions"]