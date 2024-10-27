FROM node:18-alpine AS builder
WORKDIR /dashboard
COPY package*.json .
RUN npm ci
COPY . .
RUN npm run build
RUN npm prune --production

FROM node:18-alpine AS production
WORKDIR /dashboard
COPY --from=builder /dashboard/build ./build
COPY --from=builder /dashboard/node_modules ./node_modules
COPY --from=builder /dashboard/package.json ./package.json
EXPOSE 2515
ENV PORT=2515
ENV NODE_ENV=production
CMD ["node", "build"]
