
#!/bin/bash
dir=$(pwd)
cd $dir/apps/NextJS/ && pm2 start npm --name "nextjs" --max-memory-restart 1G -- start
cd $dir/apps/Express/ && pm2 start npm --name "express" --max-memory-restart 1G -- start
# pm2 start ecosystem.config.js