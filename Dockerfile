FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Install trunk
RUN cargo install trunk

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN trunk build --release


FROM nginx:alpine as runtime
COPY --from=build /app/dist /usr/share/nginx/html/static
RUN echo "\
    server { \
    listen 8080; \
    root /usr/share/nginx/html/static; \
    location / { \
    {{ if .SPA }}            try_files \$uri /index.html; \
    {{ else }}            try_files \$uri \$uri.html \$uri/index.html /404.html =404; \
    {{ end }}        } \
    }"> /etc/nginx/conf.d/default.conf
EXPOSE 8080
