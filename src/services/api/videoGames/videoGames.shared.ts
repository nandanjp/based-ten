// For more information about this file see https://dove.feathersjs.com/guides/cli/service.shared.html
import type { Params } from '@feathersjs/feathers'
import type { ClientApplication } from '../../../client'
import type {
  VideoGames,
  VideoGamesData,
  VideoGamesPatch,
  VideoGamesQuery,
  VideoGamesService
} from './videoGames.class'

export type { VideoGames, VideoGamesData, VideoGamesPatch, VideoGamesQuery }

export type VideoGamesClientService = Pick<
  VideoGamesService<Params<VideoGamesQuery>>,
  (typeof videoGamesMethods)[number]
>

export const videoGamesPath = 'api/videoGames'

export const videoGamesMethods: Array<keyof VideoGamesService> = ['find', 'get', 'create', 'patch', 'remove']

export const videoGamesClient = (client: ClientApplication) => {
  const connection = client.get('connection')

  client.use(videoGamesPath, connection.service(videoGamesPath), {
    methods: videoGamesMethods
  })
}

// Add this service to the client service type index
declare module '../../../client' {
  interface ServiceTypes {
    [videoGamesPath]: VideoGamesClientService
  }
}
