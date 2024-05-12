# 使用官方 Rust 镜像
FROM rust:latest as builder
WORKDIR /frontend-yew

# 安装 Trunk 和 wasm-bindgen-cli
RUN cargo install trunk wasm-bindgen-cli wasm-opt
RUN rustup target add wasm32-unknown-unknown

# 复制项目文件
COPY . .

# 构建静态网站
RUN trunk build --release

# 使用轻量级的 nginx 镜像来服务站点
FROM nginx:alpine
COPY --from=builder /frontend-yew/dist /usr/share/nginx/html
EXPOSE 8001
CMD ["nginx", "-g", "daemon off;"]
