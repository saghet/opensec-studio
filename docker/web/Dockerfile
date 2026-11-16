FROM nginxinc/nginx-unprivileged:1.27.4-alpine3.21
COPY docker/web/nginx.conf /etc/nginx/conf.d/default.conf
COPY services/opensec-web/index.html /usr/share/nginx/html/index.html
EXPOSE 8080
