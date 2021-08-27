FROM rust:latest as build

WORKDIR /build
#RUN ls
COPY ./common/ ../common/
COPY ./server/src/ ./src/
COPY ./server/Cargo.* ./

# WORKDIR /build/src
RUN cargo build --release

# RUN ls -a

## API Image
FROM  gcr.io/distroless/cc-debian10 as api
 
WORKDIR /app/
# COPY log4rs.yml /app/
COPY --from=build /build/target/release/rust-learning /app/rust-learning
# EXPOSE 1065

CMD ["./rust-learning"]