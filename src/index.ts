import dotenv from 'dotenv'
import { z } from 'zod'
import express from 'express'
import helmet from 'helmet'
import cors from 'cors'
import morgan from 'morgan'
import session from 'express-session'
import passport from 'passport'
import router from './routes/index.routes'

dotenv.config()

const env_schema = z.object({
  DATABASE_URL: z.string(),
  PORT: z.string(),
  VERCEL_URL: z.string(),
  VERCEL_PORT: z.string(),
  VERCEL_URL_NO_SSL: z.string(),
  VERCEL_USER: z.string(),
  VERCEL_HOST: z.string(),
  VERCEL_PASSWORD: z.string(),
  VERCEL_DATABASE: z.string(),
  OMDB_API_KEY: z.string(),
  SPOTIFY_CLIENT_ID: z.string(),
  RAWG_API_KEY: z.string()
})

env_schema.parse(process.env)

declare global {
  namespace NodeJS {
    interface ProcessEnv extends z.infer<typeof env_schema> {}
  }
}

const app = express()

// setup middleware
app.use(express.json())
app.use(cors())
app.use(helmet())
app.use(morgan('dev'))

app.use(
  session({
    secret: 'one for all randomness this is guess is KINDa CRazy damn haikyUU on top though',
    saveUninitialized: false,
    resave: false,
    cookie: {
      maxAge: 60000 * 60 * 24
    }
  })
)
app.use(passport.initialize())
app.use(passport.session())
app.get('/api/healthcheck', (req, res) => {
  return res.status(200).send({
    msg: 'successfully passed the healthcheck!'
  })
})
app.use(router)

app.listen(process.env.PORT, () => {
  console.log(`Running on PORT ${process.env.PORT}`)
})
