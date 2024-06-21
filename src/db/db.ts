// prisma.ts
import dotenv from 'dotenv'
import { PrismaClient } from '@prisma/client'
import { Client } from 'pg'
import { z } from 'zod'

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

const globalForPrisma = global as unknown as { prisma: PrismaClient }
const globalForPgClient = global as unknown as { pgclient: Client }

const prisma = globalForPrisma.prisma || new PrismaClient()
const client = new Client({
  connectionString: process.env.DATABASE_URL
})

if (process.env.NODE_ENV !== 'production') {
  globalForPrisma.prisma = prisma
  globalForPgClient.pgclient = client
}

export { prisma, client }
