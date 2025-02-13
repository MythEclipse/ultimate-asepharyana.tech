
#!/bin/bash
dir=$(pwd)
cd $dir/apps/NextJS/ && pm2 start npm --name "nextjs" -- start --max-memory-restart 1G
cd $dir/apps/Express/ && pm2 start npm --name "express" -- start --max-memory-restart 1G
# pm2 start ecosystem.config.js