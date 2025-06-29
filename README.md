# Clone of [t3.chat](https://t3.chat) for cloneathon
Aimed to stay true to the same mission statement of focusing on excellent performance. 
Chats, messages, and active model are all handled local first.
Prioritised features that I use and care about:

- Local first
- Multi provider (OpenAI, Gemini, Anthropic, OpenRouter)
- Code formatting (optional wrap, easy copy)
- Keybinds (ctrl + number to jump to chat in hot bar, ctrl + h to pin chat to hotbar, ctrl + s to start new chat, ctrl + r to search for a chat)
- Concurrent chats
- Persistent streams
- Fork chats
- Hot bar
- Basic chat sharing

## Todo:
- Cancel message
- Add more than base share to chats (add to account etc)
- More control via settings page
- Extend reasoning support (only shown for indicated openai models for now)
- Restyle model selection & expand open router model list.
- Attachments
- Image gen

## Deployment (VPS)
Setup to be super easy to deploy your own. I have this running on the cheapest Hetzner instance. Handling DB, Redis, and the app.

1) Install [kamal](https://kamal-deploy.org/docs/installation/) on your computer.
2) Spin up a server (EC2, Droplet, Hetzner, etc)
3) Rename example-deploy.yml to deploy.yml
4) Update ip address, app name (my-app in the example), domain name, and choose a container registry (example is setup for github).
5) Provide secrets for the values defined in .kamal/secrets. It is setup to pull these from 1password right now, but you can capture from the env or whatever you want.
6) Run `kamal setup` to setup docker, load accessories (db, redis), and deploy the app.
7) Setup A record to point at your server for your domain. I use cloudflare, so leaving proxy: true, with your domain set from cloudflare lets you use their "Full" encryption mode.

### Secrets
- KAMAL_REGISTRY_PASSWORD - For example, a Personal access tokens (classic) for github registry.
- MYSQL_ROOT_PASSWORD - Whatever you want
- MYSQL_PASSWORD - Whatever you want
- APP_APPLICATION__SECRET - Generate with `openssl rand -base64 64`
- REPLICACHE_KEY= - Replicache is NOW FREE, but still needs a license key. Instructions [here](https://doc.replicache.dev/concepts/licensing). Replicache is used to handle the local first sync.

## Running Locally
There is a local.yaml config already setup. You just need to get a replicache key and add it to the frontend/.env as `VITE_REPLICACHE_KEY`.
- Load DB / redis using `docker compose up -d`
- Run backend by running `cargo run` from root dir.
- Run frontend by running `bun dev` from the ./frontend dir.
