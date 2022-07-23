import Coookie from 'js-cookie'
import {
	AccessCode,
	AccessCodeResponse,
	isAccessCode,
} from '../interfaces/accessCodeResponce'

interface DiscordUser {
	id: string
	username: string
	discriminator: string
	avatar: string
	verified?: boolean
	email?: string
	flags?: number
	banner?: number
	accent_color?: number
	premium_type?: number
	public_flags?: number
}

export default class User {
	public loggedIn: boolean
	public accessCode: AccessCode
	public discordUser: DiscordUser

	public username: string

	constructor() {}

	public async login() {
		// Try to find access code cookie. If it exists, just load it
		if (Coookie.get('access_code')) {
			const accessCodeCookie = JSON.parse(
				decodeURI(Coookie.get('access_code'))
			) as AccessCode

			this.accessCode = accessCodeCookie
			this.loggedIn = true
		} else {
			console.log('here for the params')
			// If it doesn't exit, try to find the auth code in the URL query params
			const params = new URLSearchParams(window.location.search)
			const authCode = params.get('code')

			if (authCode) {
				// If there is, remove it from query, get access code and save it to a cookie
				window.history.replaceState({}, document.title, '/')

				const res = JSON.parse(
					await (
						await fetch(
							`http://localhost:8000/identify?code=${authCode}`
						)
					).text()
				) as AccessCodeResponse

				if (!isAccessCode(res))
					throw new SyntaxError('No access code recieved!')

				this.accessCode = res
				Coookie.set('access_code', encodeURI(JSON.stringify(res)), {
					expires: 7,
				})

				this.loggedIn = true
			} else {
				// If there is no auth code, the client couldn't login
				this.loggedIn = false
			}
		}
	}

	public async getDiscordUser() {
		const res = await fetch('https://discord.com/api/users/@me', {
			headers: [
				[
					'authorization',
					`${this.accessCode.token_type} ${this.accessCode.access_token}`,
				],
			],
		})
		this.discordUser = JSON.parse(await res.text())
	}
}
