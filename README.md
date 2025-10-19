Talium is a Twitch Bot made primarily for [Clym](https://clym.tv) with a focus on reliability and designed with our goals and needs in mind. 
You can use or modify the bot as you want. At this time it is developed for our own use, but we are open for feature suggestions or outside contributions if someone is interested.
![Left: Command Edit Popout, Right: Commands List](https://github.com/user-attachments/assets/4e0acfd9-5fc5-4b6f-9504-1d4659e67731)
> Left: Command Edit Popout, Right: Commands List (with different kids of commands separate)

> [!TIP]
>The current goal is to get to a place where i feel comfortable running the bot in a production environment as fast as possible, to replace parts of our existing bot. 
>The current milstone for that can be found under [Issues > Milestones](https://github.com/Clym-Dev-Team/Talium/milestone/3)

> [!NOTE]
> If you want to **contribute**, have a question, or have generell interest in the project you can write [Orciument](https://discordapp.com/users/424579117577404417) on discord!
> You can of course also create an issue or pullrequest if you would rather contribute directly.
> Our generell needs are modeled via Issues, anything that does not have an assignee is definitly free for you to start on!

## [-> Setting up a local developing environment](https://github.com/Clym-Dev-Team/Talium/wiki/meta.dev‐setup)

## What is Talium?
Talium is a Twitch Bot that provides most of your basic Twitch Bot features, like commands, timers, giveaways and watchtime (+coins/points/loyalty), but also expands on some features that i think fell short in other bots, like complex templating in commands, automation, an edit history, error reporting and relabilty. 

After years of working with multiple different bots that all did not excactly fit our needs, fighting with some of them as our server administrator/infastructure guy, despair at the coding standards of some them, and being anoyed at ui and usability choices as a mod finally convinced me that writing a new bot from scratch to excatly fit out need would be the easier choice.

### Goals 
- reliability - *still working in a degraded state, providing an action to automatically rebuild all caches for a potential fast and easy fix, auto retries*
- concise logging and alerting - *to the moderators via notifications in the panel, a health overview page for different components, error reporting like email or external error reporting tools, and having good errors*
- documentation (why, and how)
- traceability of user actions - *a change history for almost every action/edit*
- easy of use as an moderator - *conviniently placed ui elements, visually disableling unallalowed actions, making dangerours/desctructive operations obvious and hard to trigger accidentally*
- no single user actions resulting in important data loss - *(worst case being some data needing to be manually extracted out of the change history)*
- having acceptable cpu, memory, and bandwidth requirements for the bot and panel
- taking data security and privacy seriously - *like designing a system so that mods do not have to directly communicate with winners of giveaways to arrange for shipping*

Explicit non-goals:
- Live code updates without downtime or image updates
- being general enough for most streamers (we are not opposed to being general enough, it is just not an important goal)
- non-self hosted options (bot as service)
- being the most performant bot possible


## Stack
- Bot Backend in *Java*
- Panel with *React + Vite + TypeScript*
- Target Environment *Docker*

## Features
These Features are currently planned for the 1.0 Version:
>(The full list of Features that are considered for implementation can be found [here](https://github.com/Clym-Dev-Team/Talium/wiki/meta.features))
- Commands
  - Aliases
  - Regex Command Patterns
  - User Cooldowns (Messages and Seconds)
  - Global Cooldowns (Messages and Seconds)
  - Twitch Permissions
  - Automatic List of available commands
  - Change Log
- Timer
  - Using Existing Commands, creating new texts inplace
  - Seconds and Message based interval calculation
  - Timer Groups
  - Only onstream timers 
- Giveaways
  - Multiple Draws
  - Variable amount of Tickets
  - Integrated Timer and Command
  - Change Log
- TipeeeStream
  - Alerts
- Coins & Watchtime
- OAuth Management UI
- Detailed Status Pages
- Login via Twitch
- Shadow Testing
- ChatHistory
  > The Twitch Chatlogs, reimplemented for the whole chat. _Infinite_ history of the entire chat, with searching of the entire history.

## Instalation
**We are currently in a pre 1.0 state, so we have no final artifact to install at this time.**

The eventual target environment is a Docker Image, you will find the new releases in the [Github Releases Page](https://github.com/Clym-Dev-Team/Talium/releases)

You can of course compile the code for yourself, (besides --preview-enabled because of String Templates) nothing special is required to build the Java executable. 

## License
Standart MIT License see -> [License](LICENSE)

+273 Stunden, Wakatime
+273 Stunden, Wakatime