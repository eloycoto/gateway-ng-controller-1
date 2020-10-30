FROM rust AS build

RUN export DEBIAN_FRONTEND=noninteractive \
 && apt-get update -y \
 && apt-get -y dist-upgrade \
 && apt-get install curl -y \
 && rustup component add rustfmt

WORKDIR /usr/src/gateway-ng-controller

COPY Cargo.* ./

RUN mkdir -p src \
 && echo "fn main() -> Result<(), u8> { eprintln!(\"if you see this, the build broke\"); Err(1) }" > src/main.rs \
 && cargo build --release \
 && rm -f target/release/deps/gateway-ng-controller*

COPY . .

RUN cargo build --release

FROM debian

WORKDIR /home/app/bin
ENV PATH="/home/app/bin:${PATH}"

COPY --from=build /usr/src/gateway-ng-controller/target/release/gateway-ng-controller .

RUN chown -R 1001:1001 /home/app

EXPOSE 5000

USER 1001:1001

CMD ./gateway-ng-controller
