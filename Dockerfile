# Stage 1
FROM node:alpine as build

WORKDIR /app
COPY . /app
RUN yarn install


# Stage 2
FROM node:alpine

COPY --from=build /app /app

WORKDIR /app

ENTRYPOINT ["node", "/app/src/index.js"]

EXPOSE 3000
