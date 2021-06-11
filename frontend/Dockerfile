#this is used for both stage and prod environments

################################################################################
# build
FROM node:15.6.0-alpine3.10 as build-stage
WORKDIR /app
RUN yarn global add @vue/cli
COPY package*.json yarn.lock ./
RUN yarn
COPY . .
RUN yarn build

################################################################################
# deploy
FROM nginx:1.19.6-alpine

RUN apk update && apk add --no-cache bash vim

# header (image, deps, user, workdir)
# ------------------------------------------------------------------------------

RUN mkdir /app
COPY --from=build-stage /app/dist /app
COPY nginx.conf /etc/nginx/nginx.conf

ENV APP_ENVIRONMENT prod

# ------------------------------------------------------------------------------
# footer (change user, run stuff)

#ports / expose / cmd / entrypoint - all done in docker-compose
