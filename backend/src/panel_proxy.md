# Options for Backend-Panel Setups
## Separate Servers, two Endpoints
The simplest setup is just having two endpoints. 
The panel application knows that the backend is at a different url, and a config exists to provide this url.

Drawback is that this required two ports. 
This makes sharing the app for a presentation more difficult because most sharing/gateway applications only support one port. 
These applications make it way easier to share an application for demonstration or testing purposes in cases of non-static ip's, restrictive routers, only IPv4 or IPv6, address, and provider grade NAT.
## Bot Reverse Proxy
An approach which, from the outside, would be the same as the integrated Option. 
The Backend server would be the only one exposed, and would proxy http and ws requests/connections to the panel dev server.

The problem with this is the current implementation of the websocket proxy. It does work even with both http requests and websocket requests on the same /path.
But we currently cannot handle more than one websocket connection, because on all subsequent current connections, 
the dev server refuses to initiate a new connection. 

My guess would be that is so not have two connections with the same service. But the exact cause is unknown, because of that,
the amount of work needed to finish this is unknown.
## Vite Dev Server Reverse Proxy
This Option would be the reverse of the bot Proxy. The Panel dev Server would know be exposed, and it would proxy everything on /bot.

This has the same advantages as the bot reverse proxy, with the slight downside of normally being on a different port than the internal server option.
## Integrated Static Panel Webserver
This would be the best option for production, because it completely removes the need to a second server.
It should be easily doable just by building and copying the dist from the panel next to the executable. 

The current server addresses could be supplied via replacing a link tag in the index.html. That way the urls could be changed over the panel, and without touching the js files.
