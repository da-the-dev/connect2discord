export interface Settings {
    _id?: string
    _rev?: string
    embedColor: string
}

export default class DB {
    static async getSettings(id: string): Promise<Settings | undefined> {
        const response = await fetch(
            `http://localhost:5984/connect2discord/_find`,
            {
                method: 'POST',
                headers: [["Content-Type", "application/json"]],
                body: JSON.stringify({
                    selector: {
                        _id: id || ""
                    }
                })
            }
        )
        const settings = JSON.parse(await response.text()).docs[0]

        if (response.status != 400) return settings
        else return {} as Settings
    }

    static async saveSettings(settings: Settings): Promise<void> {
        console.log("saving settings")
        const response = await fetch(
            `http://localhost:8000/db/saveSettings`,
            {
                method: 'POST',
                headers: [["Content-Type", "application/json"]],
                body: JSON.stringify(settings)
            }
        ) 
        if (!response.ok)
            throw new SyntaxError("Unsuccessful request")
    }
}
