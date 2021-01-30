const Discord = require('discord.js');
const getOnlineFaces = require('./getOnlineFaces.js');

const bot = new Discord.Client();
const token = process.env.TOKEN;

bot.on('ready', () => {
  console.log('bot is ready')
});

bot.login(token);

const prefix = '!';

async function makeMinecraftEmotes(msg, args){
    for (index in args){
        const name = args[index]
        try{
            await getOnlineFaces.downloadOnlineUsersSkinLocations(name)
            if(msg.guild.available){
                //TODO - Role checks on user creating emote and to update instead of create new if exists
                msg.guild.emojis.create(`./skins/faces/${name}.png`, `${name}Minecraft`)
                .then(emoji => msg.reply(`Created new emoji with name ${emoji.name}!`))
                .catch(console.error);
            }
        }catch(ex){
            msg.reply(`There was an issue getting the skin for Minecraft user ${name}`)
            console.log(`${ex}`)
        }
    }
}

bot.on('message', async (msg) => {
  //if our message doesnt start with our defined prefix, dont go any further into function
  if(msg.content.startsWith(prefix)) {
    //slices off prefix from our message, then trims extra whitespace, then returns our array of words from the message
    const args = msg.content.slice(prefix.length).trim().split(' ')
    
    //splits off the first word from the array, which will be our command
    const command = args.shift().toLowerCase()
    //log the command
    switch(command){
        case "minecraftemote":
            makeMinecraftEmotes(msg, args)
            break;
        default:
            console.log(`Invalide command: ${command} ${args}.`);
    }
  }
});