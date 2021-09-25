import express from 'express'
import {Cache} from './Cache.js'
import {Octokit} from '@octokit/rest'

const RELEASES_TTL = 5 * 60 * 1000 // 5 minutes
const MOTD_TTL = 10 * 60 * 1000 // 10 minutes

const server = express()
const cache = new Cache()

server.get('/releases', async (req, res) => {
  res.send(await cache.retrieve('releases', RELEASES_TTL, async () => {
    const octokit = new Octokit
    return (await octokit.rest.repos.listReleases({
      owner: 'ori-rando',
      repo: 'build',
    })).data
  }))
})

server.get('/motd/wotw', async (req, res) => {
  res.send(await cache.retrieve('wotw-motd', MOTD_TTL, async () => {
    const octokit = new Octokit
    const base64Content = (await octokit.rest.repos.getContent({
      owner: 'ori-rando',
      repo: 'motd',
      path: 'motd.wotw.html'
    })).data.content

    return {
      motd: Buffer.from(base64Content, 'base64').toString('utf-8'),
    }
  }))
})

server.listen(3000, () => console.log('Server running on port 3000'))
