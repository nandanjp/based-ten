// For more information about this file see https://dove.feathersjs.com/guides/cli/service.html

import { hooks as schemaHooks } from '@feathersjs/schema'

import {
  animeDataValidator,
  animePatchValidator,
  animeQueryValidator,
  animeResolver,
  animeExternalResolver,
  animeDataResolver,
  animePatchResolver,
  animeQueryResolver
} from './anime.schema'

import type { Application } from '../../../declarations'
import { AnimeService, getOptions } from './anime.class'
import { animePath, animeMethods } from './anime.shared'

export * from './anime.class'
export * from './anime.schema'

// A configure function that registers the service and its hooks via `app.configure`
export const anime = (app: Application) => {
  // Register our service on the Feathers application
  app.use(animePath, new AnimeService(getOptions(app)), {
    // A list of all methods this service exposes externally
    methods: animeMethods,
    // You can add additional custom events to be sent to clients here
    events: []
  })
  // Initialize hooks
  app.service(animePath).hooks({
    around: {
      all: [schemaHooks.resolveExternal(animeExternalResolver), schemaHooks.resolveResult(animeResolver)]
    },
    before: {
      all: [schemaHooks.validateQuery(animeQueryValidator), schemaHooks.resolveQuery(animeQueryResolver)],
      find: [],
      get: [],
      create: [schemaHooks.validateData(animeDataValidator), schemaHooks.resolveData(animeDataResolver)],
      patch: [schemaHooks.validateData(animePatchValidator), schemaHooks.resolveData(animePatchResolver)],
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
    [animePath]: AnimeService
  }
}
