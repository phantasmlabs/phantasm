FROM node:18 AS builder
WORKDIR /dashboard
COPY package*.json .
RUN npm install
COPY . .
RUN npm run build
RUN npm prune --production

FROM node:18 AS production
WORKDIR /dashboard

# Copy only the necessary files from the build stage.
COPY --from=builder /dashboard/build ./build
COPY --from=builder /dashboard/package.json ./package.json
RUN npm install --only=production

EXPOSE 2515
ENV PORT=2515
CMD ["node", "build"]
