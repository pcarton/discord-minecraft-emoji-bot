const fetch = require('node-fetch');
var http = require('http');
var fs = require('fs');
const sharp = require('sharp');

var download = function(url, dest) {
    return new Promise((resolve,reject) => {
        var file = fs.createWriteStream(dest);
    
        http.get(url, function(response) {
          response.pipe(file);
          file.on('finish',() => {resolve(true)});
          file.on("error", reject); // don't forget this!
        });
    });
}

const printPromiseJson = async (promise) => {
    promise.then((values) => {
        console.log(values);
      });
};

const downloadOnlineUsersSkinLocations = async (username) => {
    const userURL = 'https://api.mojang.com/users/profiles/minecraft/'
    const skinURL = 'https://sessionserver.mojang.com/session/minecraft/profile/'

    return new Promise((resolve,reject) => {
        fetch(`${userURL}${username}`)
        .then(userResponse => userResponse.json())
        .then( async function(userResponse) {
            userUUID = userResponse.id
            console.log(`${username}:${userUUID}`);
    
            fetch(`${skinURL}${userUUID}`)
            .then(response => response.json())
            .then( async function(response) {
                textureBase64 = response.properties[0].value
                const buff = Buffer.from(textureBase64, 'base64');
                const textureStr = buff.toString('utf-8');
                const textureJson = JSON.parse(textureStr);
                const skinURL = textureJson.textures.SKIN.url
                const fullSkinPath = `./skins/${username}.png`
                const faceSkinPath = `./skins/faces/${username}.png`
                await download(skinURL, fullSkinPath)
        
                sharp(fullSkinPath)
                .extract({ width: 8, height: 8, left: 8, top: 8 })
                .resize({ width:255, height:255, fit: 'fill', kernel: 'nearest' })
                .toFile(faceSkinPath)
                .then(function(new_file_info) {
                    console.log("Image cropped and saved");
                    resolve(true);
                })
                .catch(function(err) {
                    console.log("An error occured");
                    console.log(`${err}`);
                    reject
                });
        
            })
            .catch(function(error) {
                console.log(error);
                reject
            }); 
        })
        .catch(function(error) {
            console.log(error);
            reject
        }); 
    });

};


module.exports = { downloadOnlineUsersSkinLocations};