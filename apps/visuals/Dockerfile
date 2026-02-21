FROM nginx:alpine
# Copy pre-built Trunk distribution
COPY apps/visuals/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
