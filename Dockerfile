# Etapa de build
FROM rust:1.70 AS builder

# Defina o diretório de trabalho dentro do contêiner
WORKDIR /usr/src/app

# Copie o arquivo Cargo.toml e Cargo.lock
COPY Cargo.toml Cargo.lock ./


# Copie o código fonte da aplicação
COPY . .

# Compile a aplicação em modo release
RUN cargo build --release

WORKDIR /usr/src/app/target/release

CMD ["./permissions"]