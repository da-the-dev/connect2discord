// Require the necessary discord.js classes
import { Client, Intents } from 'discord.js'
import * as dotenv from 'dotenv'
import { guildCreate } from './routes/guildCreate.js'
dotenv.config()

// Create a new client instance
const client = new Client({ intents: [Intents.FLAGS.GUILDS] })

// When the client is ready, run this code (only once)
client.once('ready', async () => console.log('Ready!'))

client.on('interactionCreate', async i => {
    if (!i.isCommand()) return

    const { commandName } = i

    switch (commandName) {
        case 'ping':
            i.reply('Pong!')
            break
    }
})

client.on('guildCreate', async g => await guildCreate(g))

// Login to Discord with your client's token
client.login(process.env.TOKEN)
