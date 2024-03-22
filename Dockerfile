FROM rust:latest as backend

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y curl git unzip

# Set the default Rust toolchain
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release

ARG FLUTTER_SDK=/usr/local/flutter
ARG FLUTTER_VERSION=3.10.5
ARG APP=/app/

RUN git clone https://github.com/flutter/flutter.git $FLUTTER_SDK
RUN cd $FLUTTER_SDK && git fetch && git checkout $FLUTTER_VERSION
ENV PATH="$FLUTTER_SDK/bin:$FLUTTER_SDK/bin/cache/dart-sdk/bin:${PATH}"

RUN flutter doctor -v

RUN mkdir -p /workspace
WORKDIR /workspace
COPY . /workspace

RUN cd client
RUN flutter clean
RUN flutter pub get
RUN flutter build web
RUN cd ..

FROM rust:latest

COPY --from=backend /workspace /workspace
WORKDIR /workspace

EXPOSE 8080

CMD ["cargo", "run"]
