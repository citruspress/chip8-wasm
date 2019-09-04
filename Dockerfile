FROM rust:1.37.0 as build-wasm

# Install WASM-target and wasm-pack tool
RUN rustup target add wasm32-unknown-unknown
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /src
COPY ./Cargo.lock ./Cargo.toml ./

# Dummy file to be able to build dependencies only
RUN mkdir -p src
RUN echo "fn main() {}" > src/dummy.rs
RUN sed -i 's/lib.rs/dummy.rs/' Cargo.toml

# Dummy build to get dependencies cached
RUN wasm-pack build

RUN sed -i 's/dummy.rs/lib.rs/' Cargo.toml

COPY ./src ./src
RUN wasm-pack build

FROM node:current-alpine as build-frontend
WORKDIR /src/www

COPY www/package.json www/package-lock.json ./
RUN npm install

COPY ./ /src

COPY --from=build-wasm /src/pkg/ ../pkg

RUN npm run build

FROM nginx:alpine
COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=build-frontend /src/www/dist/ /var/www/web