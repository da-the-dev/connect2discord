export function getDiscordCode() {
	let authcode = ''
	if (localStorage.getItem('discordcode')) {
		authcode = localStorage.getItem('discordcode')!
	} else {
		const urlParams = new URLSearchParams(window.location.search)
		const param = urlParams.get('code')

		if (!param) return ''
		authcode = param

		localStorage.setItem('discordcode', authcode)

		let location = window.location.href.split('?')[-1]
		console.log(location)
		window.location.replace(location)
	}
	return authcode
}
