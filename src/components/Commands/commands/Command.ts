import {StringTemplate} from "../templates/StringTemplate.ts";

export interface Trigger {
  pattern: string,
  isRegex: boolean,
  isVisible: boolean,
  isEnabled: boolean,
}

export enum CommandPermission {
  EVERYONE,
  PREDICTIONS_BLUE,
  PREDICTIONS_PINK,
  SUBSCRIBER,
  ARTIST,
  FOUNDER,
  VIP,
  MODERATOR,
  BROADCASTER,
  OWNER,
  SYSTEM
}

export enum CooldownTypes {
  MESSAGE,
  SECONDS
}

export interface CommandCooldown {
  value: number,
  type: CooldownTypes
}

export interface Command {
  id: string,
  description: string,
  patterns: Trigger[],
  permission: string,
  userCooldown: CommandCooldown,
  globalCooldown: CommandCooldown,
  isAutoGenerated: boolean,
  template: StringTemplate,
}