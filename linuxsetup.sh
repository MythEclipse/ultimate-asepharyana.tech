# installs fnm (Fast Node Manager)
curl -fsSL https://fnm.vercel.app/install | bash
# download and install Node.js
fnm use --install-if-missing 22
# verifies the right Node.js version is in the environment
node -v # should print `v22.5.1`
# verifies the right NPM version is in the environment
npm -v # should print `10.8.2`
npm install -g pnpm
# verifies the right Yarn version is in the environment
pnpm -v # should print `1.22.17`
pnpm install