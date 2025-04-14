FROM node:18 AS builder

ARG COMMIT_SHA
ARG TAG_NAME
ARG CREATED_AT

WORKDIR /taliumPanel

COPY package*.json ./

RUN npm install;

COPY . .

# Add Image info to temporary .env file inside the image to bake them into the image in building step
RUN touch .env
RUN echo "VITE_COMMIT_SHA=${COMMIT_SHA}" >> .env
RUN echo "VITE_VERSION=${TAG_NAME}" >> .env
RUN echo "VITE_CREATED_AT=${CREATED_AT}" >> .env
RUN echo "VITE_PLATFORM=DOCKER" >> .env

RUN npm run build

# Serve with Nginx
FROM nginx:stable-alpine as publish

ARG COMMIT_SHA
ARG TAG_NAME
ARG CREATED_AT

LABEL COMMIT_SHA=$COMMIT_SHA
LABEL VERSION=$TAG_NAME
LABEL CREATED_AT=$CREATED_AT

# Remove default nginx website
RUN rm -rf /usr/share/nginx/html/*

# Copy built files from previous stage
COPY --from=builder /taliumPanel/dist /usr/share/nginx/html

# config files are not copied in, they are later mounted into the container

EXPOSE 4772

CMD ["nginx", "-g", "daemon off;"]
