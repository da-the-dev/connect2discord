export interface Settings {
    embedColor: string
}

export default class DB {
    public async getSettings(id: string): Promise<Settings | undefined> {
        const response = await fetch(`http://localhost:8000/db/get/${id}`)
        if (response.status != 400) return JSON.parse(await response.text())
    }
}
