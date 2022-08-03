// Require the necessary discord.js classes
import { Client, ColorResolvable, Intents, MessageEmbed } from 'discord.js'
import * as dotenv from 'dotenv'
import nano from 'nano'
import { guildCreate } from './routes/guildCreate.js'
dotenv.config()

export interface Settings extends nano.DocumentGetResponse {
    embedColor: string
}

// Create a new client instance
const client = new Client({ intents: [Intents.FLAGS.GUILDS] })

// When the client is ready, run this code (only once)
client.once('ready', async () => console.log('Ready!'))

client.on('interactionCreate', async i => {
    if (!i.isCommand()) return

    const { commandName } = i

    const couchdb = nano(`http://admin:${process.env.CDB_PASS}@localhost:5984`)
    const db = couchdb.db.use('connect2discord')
    const settings  = await db.get(i.guildId!) as Settings
    console.log(settings.embedColor)


    switch (commandName) {
        case 'ping':
            const embed = new MessageEmbed()
                .setTitle("Ping!")
                .setDescription("Pong!")
                .setColor(settings.embedColor as ColorResolvable)
            i.reply({embeds: [embed]})
            break
    }
})

client.on('guildCreate', async g => await guildCreate(g))

// Login to Discord with your client's token
client.login(process.env.TOKEN)
