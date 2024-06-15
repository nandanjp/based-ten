// For more information about this file see https://dove.feathersjs.com/guides/cli/client.html
import { feathers } from '@feathersjs/feathers'
import type { TransportConnection, Application } from '@feathersjs/feathers'
import authenticationClient from '@feathersjs/authentication-client'
import type { AuthenticationClientOptions } from '@feathersjs/authentication-client'

import { videoGamesClient } from './services/api/videoGames/videoGames.shared'
export type {
  VideoGames,
  VideoGamesData,
  VideoGamesQuery,
  VideoGamesPatch
} from './services/api/videoGames/videoGames.shared'

import { songsClient } from './services/api/songs/songs.shared'
export type { Songs, SongsData, SongsQuery, SongsPatch } from './services/api/songs/songs.shared'

import { moviesClient } from './services/api/movies/movies.shared'
export type { Movies, MoviesData, MoviesQuery, MoviesPatch } from './services/api/movies/movies.shared'

import { animeClient } from './services/api/anime/anime.shared'
export type { Anime, AnimeData, AnimeQuery, AnimePatch } from './services/api/anime/anime.shared'

export interface Configuration {
  connection: TransportConnection<ServiceTypes>
}

export interface ServiceTypes {}

export type ClientApplication = Application<ServiceTypes, Configuration>

/**
 * Returns a typed client for the server app.
 *
 * @param connection The REST or Socket.io Feathers client connection
 * @param authenticationOptions Additional settings for the authentication client
 * @see https://dove.feathersjs.com/api/client.html
 * @returns The Feathers client application
 */
export const createClient = <Configuration = any,>(
  connection: TransportConnection<ServiceTypes>,
  authenticationOptions: Partial<AuthenticationClientOptions> = {}
) => {
  const client: ClientApplication = feathers()

  client.configure(connection)
  client.configure(authenticationClient(authenticationOptions))
  client.set('connection', connection)

  client.configure(animeClient)
  client.configure(moviesClient)
  client.configure(songsClient)
  client.configure(videoGamesClient)
  return client
}
