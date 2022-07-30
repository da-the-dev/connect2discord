import { writable } from 'svelte/store'
import type UserGuild from '../interfaces/userGuild'
import type { Settings } from '../modules/database'

export const activeGuild = writable({} as UserGuild)
export const activeGuildSettings = writable({} as Settings)
