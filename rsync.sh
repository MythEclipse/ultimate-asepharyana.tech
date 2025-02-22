rsync -avz --progress -e "ssh -p 22" --exclude=".turbo" /workspaces/ultimate-asepharyana.cloud/ root@217.15.165.147:/root/ultimate-asepharyana.cloud/
# ssh -p 22 root@217.15.165.147 "pm2 restart express --update-env && pm2 restart nextjs --update-env"
