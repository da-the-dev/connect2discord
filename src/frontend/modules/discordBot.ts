import type Guild from '../interfaces/guild'

export default class Bot {
    guilds: [Guild]
    constructor() {}

    private async apiRequest(endpoint: string) {
        return await fetch(`http://localhost:8000${endpoint}`)
    }

    public async getGuilds(): Promise<[Guild]> {
        const res = await this.apiRequest('/guilds')
        this.guilds = JSON.parse(await res.text())
        return this.guilds
    }
}
