FROM rust:latest

# Создаем рабочую директорию внутри контейнера
WORKDIR /usr/src/app

# Копируем файлы зависимостей и исходный код
COPY . .

RUN cargo build --release

# Для логирования
ENV RUST_LOG=trace

RUN cargo build --release

CMD ["./target/release/project"]
