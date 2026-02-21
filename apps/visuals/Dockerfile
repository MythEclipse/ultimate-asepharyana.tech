FROM nginx:alpine
RUN sed -i 's/dl-cdn.alpinelinux.org/mirror.kartolo.sby.datautama.net.id/g' /etc/apk/repositories
# Copy pre-built Trunk distribution
COPY apps/visuals/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
