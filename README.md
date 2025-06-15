# Simple clone of T3 Chat.
Prioritised features that I use and care about.

- Local first
- Multi provider
- Concurrenct chats
- Persistent streams

## Todo:
- Sharable Chats
- Attachments
- More control via settings page

## Deployment (VPS)
Setup to be super easy to deploy your own. I have this running on the cheapest Hetzner instance. Handling DB, Redis, and the app.

1) Install [kamal](https://kamal-deploy.org/docs/installation/) on your computer.
2) Spin up a server (EC2, Droplet, Hetzner, etc)
3) Install docker on it.
4) Rename example-deploy.yml to deploy.yml
5) Update ip address, app name (my-app in the example), domain name, and choose a container registry (example is setup for github).
6) Provide secrets for the values defined in .kamal/secrets. It is setup to pull these from 1password right now, but you can capture from the env or whatever you want.
7) Run `kamal accessory boot all` to load db / redis onto instance
8) Run `kamal deploy to deploy` the web app.
9) Setup A record to point at your server for your domain. I use cloudflare, so leaving proxy: true, with your domain set from cloudflare lets you use their "Full" encryption mode.
