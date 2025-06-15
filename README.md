# Simple clone of T3 Chat.
Prioritised features that I use and care about.

- Local first
- Multi provider
- Concurrenct chats
- Persistent streams
- Hot bar
- Keybinds (ctrl + number to jump to chat in hot bar, ctrl + h to pin chat to hotbar, ctrl + s to start new chat, ctrl + r to search for a chat)

## Todo:
- Sharable Chats
- More control via settings page
- Attachments
- Image gen

## Deployment (VPS)
Setup to be super easy to deploy your own. I have this running on the cheapest Hetzner instance. Handling DB, Redis, and the app.

1) Install [kamal](https://kamal-deploy.org/docs/installation/) on your computer.
2) Spin up a server (EC2, Droplet, Hetzner, etc)
3) Install docker on it.
4) Rename example-deploy.yml to deploy.yml
5) Update ip address, app name (my-app in the example), domain name, and choose a container registry (example is setup for github).
6) Provide secrets for the values defined in .kamal/secrets. It is setup to pull these from 1password right now, but you can capture from the env or whatever you want.
7) Run `kamal accessory boot all` to load db / redis onto instance
8) Run `kamal deploy` to deploy the web app.
9) Setup A record to point at your server for your domain. I use cloudflare, so leaving proxy: true, with your domain set from cloudflare lets you use their "Full" encryption mode.

### Secrets
- KAMAL_REGISTRY_PASSWORD - For example, a Personal access tokens (classic) for github registry.
- MYSQL_ROOT_PASSWORD - Whatever you want
- MYSQL_PASSWORD - Whatever you want
- APP_APPLICATION__SECRET - Generate with `openssl rand -base64 64`
- REPLICACHE_KEY= - Replicache is NOW FREE, but still needs a license key Instructions [here](https://doc.replicache.dev/concepts/licensing). Replicache is used to handle the local first sync.
