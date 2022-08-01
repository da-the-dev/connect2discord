import type Guild from '../interfaces/guild'

export default class Bot {
    static guilds: Guild[]

    static async apiRequest(endpoint: string) {
        return await fetch(`http://localhost:8000/botapi${endpoint}`)
    }

    static async getGuilds(): Promise<Guild[]> {
        const res = await this.apiRequest('/users/@me/guilds')
        this.guilds = JSON.parse(await res.text())
        return this.guilds
    }
}
