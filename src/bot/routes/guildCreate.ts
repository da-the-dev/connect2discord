import { Guild } from 'discord.js'
import nano from 'nano'

interface GuildData {
    _id?: string
    _rev?: string
    embedColor: string
}

export async function guildCreate(g: Guild) {
    const couch = nano({ url: 'http://localhost:5984', requestDefaults: { jar: true } })

    const authResponse = await couch.auth('admin', process.env.CDB_PASS!)
    if (!authResponse.ok) throw new SyntaxError('Could not authoraze into the database!')

    const db = couch.use<GuildData>('connect2discord')
    const insertResponse = await db.insert({ embedColor: '#cc00CC' }, g.id)
    if (!insertResponse.ok) throw new SyntaxError('Count not insert a new guild entry!')
}
