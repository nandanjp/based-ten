// For more information about this file see https://dove.feathersjs.com/guides/cli/service.html

import { hooks as schemaHooks } from '@feathersjs/schema'

import {
  videoGamesDataValidator,
  videoGamesPatchValidator,
  videoGamesQueryValidator,
  videoGamesResolver,
  videoGamesExternalResolver,
  videoGamesDataResolver,
  videoGamesPatchResolver,
  videoGamesQueryResolver
} from './videoGames.schema'

import type { Application } from '../../../declarations'
import { VideoGamesService, getOptions } from './videoGames.class'
import { videoGamesPath, videoGamesMethods } from './videoGames.shared'

export * from './videoGames.class'
export * from './videoGames.schema'

// A configure function that registers the service and its hooks via `app.configure`
export const videoGames = (app: Application) => {
  // Register our service on the Feathers application
  app.use(videoGamesPath, new VideoGamesService(getOptions(app)), {
    // A list of all methods this service exposes externally
    methods: videoGamesMethods,
    // You can add additional custom events to be sent to clients here
    events: []
  })
  // Initialize hooks
  app.service(videoGamesPath).hooks({
    around: {
      all: [
        schemaHooks.resolveExternal(videoGamesExternalResolver),
        schemaHooks.resolveResult(videoGamesResolver)
      ]
    },
    before: {
      all: [
        schemaHooks.validateQuery(videoGamesQueryValidator),
        schemaHooks.resolveQuery(videoGamesQueryResolver)
      ],
      find: [],
      get: [],
      create: [
        schemaHooks.validateData(videoGamesDataValidator),
        schemaHooks.resolveData(videoGamesDataResolver)
      ],
      patch: [
        schemaHooks.validateData(videoGamesPatchValidator),
        schemaHooks.resolveData(videoGamesPatchResolver)
      ],
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
    [videoGamesPath]: VideoGamesService
  }
}
