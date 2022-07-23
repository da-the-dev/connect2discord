export default interface DiscordUser {
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
