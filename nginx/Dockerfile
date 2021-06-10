FROM nginx:1.19.6-alpine

RUN apk update && apk add --no-cache bash vim

# header (image, deps, user, workdir)
# ------------------------------------------------------------------------------

COPY nginx.conf /etc/nginx/nginx.conf

# needed for aws logs to work correctly
RUN mkdir -p /var/log/nginx/healthd
RUN chmod 777 /var/log/nginx/healthd
RUN chown -R nginx:nginx /var/log/nginx

# originally access.log inside docker is created to redirect to stdout. That's annoying when run locally. It will be recreated anyway, but without stupid stdout.
RUN rm /var/log/nginx/access.log

# ------------------------------------------------------------------------------
# footer (change user, run stuff)

#ports / expose / cmd / entrypoint - all done in docker-compose