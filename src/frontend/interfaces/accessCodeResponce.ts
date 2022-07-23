export interface DiscordAPIError {
	error: string
	error_description: string
}
export interface AccessCode {
	access_token: string
	token_type: string
	expires_in: number
	refresh_token: string
	scope: string
}
export type AccessCodeResponse = AccessCode | DiscordAPIError

export const isAccessCode = (res: any): res is AccessCode =>
	res.error ? false : true
