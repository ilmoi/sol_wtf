#this is used for both stage and prod environments

################################################################################
# build
FROM node:15.6.0-alpine3.10 as build-stage
WORKDIR /app
RUN yarn global add @vue/cli
COPY package*.json yarn.lock ./
RUN yarn
COPY . .
# need to pass env during build stage as well as during prod
RUN echo "VUE_APP_ENVIRONMENT=prod" >> .env
RUN yarn build

################################################################################
# deploy
FROM nginx:1.19.6-alpine

RUN apk update && apk add --no-cache bash vim

# header (image, deps, user, workdir)
# ------------------------------------------------------------------------------

RUN echo "VUE_APP_ENVIRONMENT=prod" >> .env
RUN mkdir /app
COPY --from=build-stage /app/dist /app
COPY nginx.conf /etc/nginx/nginx.conf


# ------------------------------------------------------------------------------
# footer (change user, run stuff)

#ports / expose / cmd / entrypoint - all done in docker-compose
