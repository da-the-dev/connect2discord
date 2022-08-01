import { writable } from 'svelte/store'
import type UserGuild from '../interfaces/userGuild'
import DB from '../modules/database'
// export const activeGuild = writable({name: "Not selected"} as UserGuild) 
export const activeGuild = writable({} as UserGuild) 
export const activeGuildSettings = writable(new DB())

