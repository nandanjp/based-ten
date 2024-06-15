// For more information about this file see https://dove.feathersjs.com/guides/cli/service.shared.html
import type { Params } from '@feathersjs/feathers'
import type { ClientApplication } from '../../../client'
import type { Movies, MoviesData, MoviesPatch, MoviesQuery, MoviesService } from './movies.class'

export type { Movies, MoviesData, MoviesPatch, MoviesQuery }

export type MoviesClientService = Pick<MoviesService<Params<MoviesQuery>>, (typeof moviesMethods)[number]>

export const moviesPath = 'api/movies'

export const moviesMethods: Array<keyof MoviesService> = ['find', 'get', 'create', 'patch', 'remove']

export const moviesClient = (client: ClientApplication) => {
  const connection = client.get('connection')

  client.use(moviesPath, connection.service(moviesPath), {
    methods: moviesMethods
  })
}

// Add this service to the client service type index
declare module '../../../client' {
  interface ServiceTypes {
    [moviesPath]: MoviesClientService
  }
}
