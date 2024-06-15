// For more information about this file see https://dove.feathersjs.com/guides/cli/service.class.html#database-services
import type { Params } from '@feathersjs/feathers'
import { KnexService } from '@feathersjs/knex'
import type { KnexAdapterParams, KnexAdapterOptions } from '@feathersjs/knex'

import type { Application } from '../../../declarations'
import type { Songs, SongsData, SongsPatch, SongsQuery } from './songs.schema'

export type { Songs, SongsData, SongsPatch, SongsQuery }

export interface SongsParams extends KnexAdapterParams<SongsQuery> {}

// By default calls the standard Knex adapter service methods but can be customized with your own functionality.
export class SongsService<ServiceParams extends Params = SongsParams> extends KnexService<
  Songs,
  SongsData,
  SongsParams,
  SongsPatch
> {}

export const getOptions = (app: Application): KnexAdapterOptions => {
  return {
    paginate: app.get('paginate'),
    Model: app.get('postgresqlClient'),
    name: 'songs'
  }
}
