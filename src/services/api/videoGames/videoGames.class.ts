// For more information about this file see https://dove.feathersjs.com/guides/cli/service.class.html#database-services
import type { Params } from '@feathersjs/feathers'
import { KnexService } from '@feathersjs/knex'
import type { KnexAdapterParams, KnexAdapterOptions } from '@feathersjs/knex'

import type { Application } from '../../../declarations'
import type { VideoGames, VideoGamesData, VideoGamesPatch, VideoGamesQuery } from './videoGames.schema'

export type { VideoGames, VideoGamesData, VideoGamesPatch, VideoGamesQuery }

export interface VideoGamesParams extends KnexAdapterParams<VideoGamesQuery> {}

// By default calls the standard Knex adapter service methods but can be customized with your own functionality.
export class VideoGamesService<ServiceParams extends Params = VideoGamesParams> extends KnexService<
  VideoGames,
  VideoGamesData,
  VideoGamesParams,
  VideoGamesPatch
> {}

export const getOptions = (app: Application): KnexAdapterOptions => {
  return {
    paginate: app.get('paginate'),
    Model: app.get('postgresqlClient'),
    name: 'videoGames'
  }
}
