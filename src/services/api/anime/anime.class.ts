// For more information about this file see https://dove.feathersjs.com/guides/cli/service.class.html#database-services
import type { Params } from '@feathersjs/feathers'
import { KnexService } from '@feathersjs/knex'
import type { KnexAdapterParams, KnexAdapterOptions } from '@feathersjs/knex'

import type { Application } from '../../../declarations'
import type { Anime, AnimeData, AnimePatch, AnimeQuery } from './anime.schema'

export type { Anime, AnimeData, AnimePatch, AnimeQuery }

export interface AnimeParams extends KnexAdapterParams<AnimeQuery> {}

// By default calls the standard Knex adapter service methods but can be customized with your own functionality.
export class AnimeService<ServiceParams extends Params = AnimeParams> extends KnexService<
  Anime,
  AnimeData,
  AnimeParams,
  AnimePatch
> {}

export const getOptions = (app: Application): KnexAdapterOptions => {
  return {
    paginate: app.get('paginate'),
    Model: app.get('postgresqlClient'),
    name: 'anime'
  }
}
