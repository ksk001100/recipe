# 開発環境
FROM node:14.18-alpine as develop-stage
RUN yarn global add elm create-elm-app
WORKDIR /app
# COPY elm.json ./
# RUN elm install
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN yarn build

# 本番環境
FROM nginx:1.19-alpine as production-stage
COPY --from=build-stage /app/build /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
