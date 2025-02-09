
#!/bin/bash
dir=$(pwd)
cd $dir/apps/NextJS/ && pm2 start npm --name "nextjs" -- start
cd $dir/apps/Express/ && pm2 start npm --name "express" -- start
# pm2 start ecosystem.config.js