// For more information about this file see https://dove.feathersjs.com/guides/cli/service.html

import { hooks as schemaHooks } from '@feathersjs/schema'

import {
  moviesDataValidator,
  moviesPatchValidator,
  moviesQueryValidator,
  moviesResolver,
  moviesExternalResolver,
  moviesDataResolver,
  moviesPatchResolver,
  moviesQueryResolver
} from './movies.schema'

import type { Application } from '../../../declarations'
import { MoviesService, getOptions } from './movies.class'
import { moviesPath, moviesMethods } from './movies.shared'

export * from './movies.class'
export * from './movies.schema'

// A configure function that registers the service and its hooks via `app.configure`
export const movies = (app: Application) => {
  // Register our service on the Feathers application
  app.use(moviesPath, new MoviesService(getOptions(app)), {
    // A list of all methods this service exposes externally
    methods: moviesMethods,
    // You can add additional custom events to be sent to clients here
    events: []
  })
  // Initialize hooks
  app.service(moviesPath).hooks({
    around: {
      all: [schemaHooks.resolveExternal(moviesExternalResolver), schemaHooks.resolveResult(moviesResolver)]
    },
    before: {
      all: [schemaHooks.validateQuery(moviesQueryValidator), schemaHooks.resolveQuery(moviesQueryResolver)],
      find: [],
      get: [],
      create: [schemaHooks.validateData(moviesDataValidator), schemaHooks.resolveData(moviesDataResolver)],
      patch: [schemaHooks.validateData(moviesPatchValidator), schemaHooks.resolveData(moviesPatchResolver)],
      remove: []
    },
    after: {
      all: []
    },
    error: {
      all: []
    }
  })
}

// Add this service to the service type index
declare module '../../../declarations' {
  interface ServiceTypes {
    [moviesPath]: MoviesService
  }
}
