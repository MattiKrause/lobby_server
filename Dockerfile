FROM rust:1.67 as  builder
COPY . .
ENV CARGO_HOME=~/.cargo
ENV BUILD_DIR=~/build
RUN --mount=type=cache,target=~/.cargo,id=cargoregistry,mode=0666 \
    --mount=type=cache,target=~/build,id=cargobuild,mode=0666 \
    ls $CARGO_HOME \
    && ls $BUILD_DIR \
    && cargo build --release --package web_server --target-dir $BUILD_DIR \
    && cp $BUILD_DIR/release/web_server web_server \
    && ls $BUILD_DIR

FROM debian:11.6-slim as runtime
COPY --from=builder web_server .
ENTRYPOINT ["./web_server"]