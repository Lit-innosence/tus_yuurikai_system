# ベースイメージとしてUbuntuを指定
FROM ubuntu:22.04

# 必要なツールをインストールするために環境を更新
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    build-essential \
    libssl-dev \
    pkg-config \
    software-properties-common \
    libpq-dev \
    git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Rustをインストール
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=$PATH:/root/.cargo/bin

# Node.jsとnpmをインストール
RUN curl -fsSL https://deb.nodesource.com/setup_current.x -o nodesource_setup.sh && \
    bash nodesource_setup.sh && \
    apt-get install -y nodejs && \
    npm install -g npm@latest && \
    rm nodesource_setup.sh
ENV PATH=$PATH:/usr/local/bin

# PostgreSQLクライアントをインストール
RUN apt-get update && apt-get install -y postgresql-client

# Diesel CLIをインストール
RUN cargo install diesel_cli --no-default-features --features postgres         