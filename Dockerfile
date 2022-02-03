FROM rust:latest AS builder
MAINTAINER Kyle Cotton <kylecottonkc@gmail.com>
WORKDIR /usr/src/workspace

COPY . .
RUN cd preprocessor-service && cargo build --release

FROM gcr.io/distroless/cc-debian10
MAINTAINER Kyle Cotton <kylecottonkc@gmail.com>
EXPOSE 8000
COPY --from=builder /usr/src/workspace/preprocessor-service/target/release/preprocessor-service /workspace/preprocessor-service

CMD ["./workspace/preprocessor-service"]
