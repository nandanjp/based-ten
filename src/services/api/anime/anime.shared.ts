// For more information about this file see https://dove.feathersjs.com/guides/cli/service.shared.html
import type { Params } from '@feathersjs/feathers'
import type { ClientApplication } from '../../../client'
import type { Anime, AnimeData, AnimePatch, AnimeQuery, AnimeService } from './anime.class'

export type { Anime, AnimeData, AnimePatch, AnimeQuery }

export type AnimeClientService = Pick<AnimeService<Params<AnimeQuery>>, (typeof animeMethods)[number]>

export const animePath = 'api/anime'

export const animeMethods: Array<keyof AnimeService> = ['find', 'get', 'create', 'patch', 'remove']

export const animeClient = (client: ClientApplication) => {
  const connection = client.get('connection')

  client.use(animePath, connection.service(animePath), {
    methods: animeMethods
  })
}

// Add this service to the client service type index
declare module '../../../client' {
  interface ServiceTypes {
    [animePath]: AnimeClientService
  }
}
