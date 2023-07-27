FROM rust:1.71.0-alpine3.18 as build
WORKDIR /app
ADD . .
RUN cargo build --release
RUN strip target/release/healthy

FROM alpine:3.18 as runtime
COPY --from=build /app/target/release/healthy /
EXPOSE 8000
USER 65534:65534
ENTRYPOINT ["/healthy"]
