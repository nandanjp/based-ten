// For more information about this file see https://dove.feathersjs.com/guides/cli/service.class.html#database-services
import type { Params } from '@feathersjs/feathers'
import { KnexService } from '@feathersjs/knex'
import type { KnexAdapterParams, KnexAdapterOptions } from '@feathersjs/knex'

import type { Application } from '../../../declarations'
import type { Movies, MoviesData, MoviesPatch, MoviesQuery } from './movies.schema'

export type { Movies, MoviesData, MoviesPatch, MoviesQuery }

export interface MoviesParams extends KnexAdapterParams<MoviesQuery> {}

// By default calls the standard Knex adapter service methods but can be customized with your own functionality.
export class MoviesService<ServiceParams extends Params = MoviesParams> extends KnexService<
  Movies,
  MoviesData,
  MoviesParams,
  MoviesPatch
> {}

export const getOptions = (app: Application): KnexAdapterOptions => {
  return {
    paginate: app.get('paginate'),
    Model: app.get('postgresqlClient'),
    name: 'movies'
  }
}
