FROM node:alpine

WORKDIR /app

COPY ./package*.json /app/
COPY ./*.js /app/

RUN npm ci
RUN mkdir /app/skins
RUN mkdir /app/skins/faces

ENTRYPOINT [ "node", "/app/bot.js" ]