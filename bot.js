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
    const member = msg.member
    const neededPermission = 'MANAGE_EMOJIS'
    if(!member.hasPermission(neededPermission)){
        msg.reply("You don't have permissions to edit emojis!");
        return
    }
    for (index in args){
        const name = args[index]
        try{
            await getOnlineFaces.downloadOnlineUsersSkinLocations(name)
            if(msg.guild.available){
                emojiStr = `${name}Minecraft`
                emojiArray = msg.guild.emojis.cache.array()
                foundExisting = false
                for(id in emojiArray ){
                    guildEmoji = emojiArray[id]
                    if(guildEmoji.name == emojiStr){
                        foundExisting = true
                    }
                }
                if(foundExisting){
                    msg.reply(`Emoji with name ${emojiStr} already exists!`)
                }else{
                    //TODO - Role checks on user creating emote
                    msg.guild.emojis.create(`./skins/faces/${name}.png`, emojiStr)
                    .then(emoji => {
                        msg.reply(`Created new emoji with name ${emoji.name}!`)
                        emojiID =  msg.guild.emojis.resolve(emoji)
                        msg.react(`${emojiID}`)
                    })
                    .catch(console.error);
                }
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