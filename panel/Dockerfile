# Stage 1: Build the app
FROM node:18 AS builder

WORKDIR /taliumPanel

COPY package*.json ./
RUN npm install

COPY . .

RUN npm run build

# Stage 2: Serve with Nginx
FROM nginx:stable-alpine as publish

# Remove default nginx website
RUN rm -rf /usr/share/nginx/html/*

# Copy built files from previous stage
COPY --from=builder /taliumPanel/dist /usr/share/nginx/html

# Optional: custom nginx config
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 4772

CMD ["nginx", "-g", "daemon off;"]
