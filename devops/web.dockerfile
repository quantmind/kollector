FROM node as builder

WORKDIR /web
COPY package.json yarn.lock ./
RUN yarn
COPY webpack.config.js tsconfig.json web ./
COPY web ./web
COPY @types ./@types
RUN yarn build

FROM node

WORKDIR /web
RUN npm install -g http-server
COPY web/index.html ./sources/index.html
COPY --from=builder /web/dist sources/dist
RUN ls -la

CMD ["http-server", "/web/sources", "-p", "3000", "-d"]
