# 開発環境
FROM node:14.15-alpine as develop-stage
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN npm run build

# 本番環境
FROM nginx:1.19-alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
