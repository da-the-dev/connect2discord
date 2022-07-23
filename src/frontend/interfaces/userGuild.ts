export default interface UserGuild {
	id: string
	name: string
	icon: string
	owner?: boolean
	permissions?: number
	features: [string]
}
