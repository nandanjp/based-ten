// For more information about this file see https://dove.feathersjs.com/guides/cli/service.shared.html
import type { Params } from '@feathersjs/feathers'
import type { ClientApplication } from '../../../client'
import type { Songs, SongsData, SongsPatch, SongsQuery, SongsService } from './songs.class'

export type { Songs, SongsData, SongsPatch, SongsQuery }

export type SongsClientService = Pick<SongsService<Params<SongsQuery>>, (typeof songsMethods)[number]>

export const songsPath = 'api/songs'

export const songsMethods: Array<keyof SongsService> = ['find', 'get', 'create', 'patch', 'remove']

export const songsClient = (client: ClientApplication) => {
  const connection = client.get('connection')

  client.use(songsPath, connection.service(songsPath), {
    methods: songsMethods
  })
}

// Add this service to the client service type index
declare module '../../../client' {
  interface ServiceTypes {
    [songsPath]: SongsClientService
  }
}
