FROM rust:latest as build
 
ARG LOCAL
 
WORKDIR /build
 
COPY Cargo.toml Cargo.lock /build/
 
# Now copy over the source and compile our app
COPY src/ /build/src/
RUN cargo build --release ${LOCAL}
 
### API Image
# FROM  gcr.io/distroless/cc-debian10 as api
 
# WORKDIR /app/
# COPY log4rs.yml /app/
# COPY --from=build /build/target/release/user-sync /app/user-sync
 
EXPOSE 8080
 
CMD ["./user-sync"]