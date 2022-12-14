import { AccessCode, AccessCodeResponse, isAccessCode } from '../interfaces/accessCodeResponce'
import type DiscordUser from '../interfaces/discordUser'
import type UserGuild from '../interfaces/userGuild'

export default class User {
    private accessCode: AccessCode

    public loggedIn: boolean
    public discordUser: DiscordUser
    public guilds: UserGuild[]

    public async login() {
        if (this.loggedIn) return
        // Try to find access code cookie. If it exists, just load it
        if (localStorage.getItem('access_code')) {
            const accessCodeCookie = JSON.parse(localStorage.getItem('access_code')) as AccessCode

            this.accessCode = accessCodeCookie

            this.loggedIn = true
            console.log("fetching user...")
            await this.fetchDiscordUser()
            console.log("user fetched!")
            await this.fetchGuilds()
        } else {
            // If it doesn't exit, try to find the auth code in the URL query params
            const params = new URLSearchParams(window.location.search)
            const authCode = params.get('code')

            if (authCode) {
                // If there is, remove it from query, get access code and save it to a cookie
                window.history.replaceState({}, document.title, '/')

                const response = await fetch(`http://localhost:8000/identify?code=${authCode}`)
                const json = JSON.parse(await response.text()) as AccessCodeResponse

                if (!isAccessCode(json)) throw new SyntaxError('No access code recieved!')

                this.accessCode = json
                localStorage.setItem('access_code', JSON.stringify(json))

                this.loggedIn = true
                await this.fetchDiscordUser()
                await this.fetchGuilds()
            } else {
                // If there is no auth code, the client couldn't login
                this.loggedIn = false
            }
        }
    }

    private async apiRequest(endpoint: string) {
        return await fetch(`https://discord.com/api${endpoint}`, {
            headers: [['Authorization', `${this.accessCode.token_type} ${this.accessCode.access_token}`]],
        })
    }

    public async fetchDiscordUser(): Promise<DiscordUser> {
        const res = await this.apiRequest('/users/@me')
        this.discordUser = JSON.parse(await res.text())
        return this.discordUser
    }


    public async fetchGuilds(): Promise<UserGuild[]> {
        const res = await this.apiRequest('/users/@me/guilds')
        this.guilds = JSON.parse(await res.text())
        this.guilds = this.guilds.filter(g => g.owner)
        this.guilds.sort((a, b) => (a.name > b.name ? 1 : -1))
        return this.guilds
    }
}
