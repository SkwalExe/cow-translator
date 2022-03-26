FROM rust:latest
WORKDIR /app
COPY . /app
LABEL maintainer="Léopold Koprivnik Ibghy <skwal.net@gmail.com>"
RUN RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build  --release
ENTRYPOINT ["/app/target/release/cow-translator"]
