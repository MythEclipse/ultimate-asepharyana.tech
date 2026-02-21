FROM nginx:alpine
# Copy pre-built Trunk distribution
COPY apps/leptos/dist /usr/share/nginx/html
RUN sed -i 's/listen \(.*\)80;/listen 8081;/' /etc/nginx/conf.d/default.conf
EXPOSE 8081
CMD ["nginx", "-g", "daemon off;"]
