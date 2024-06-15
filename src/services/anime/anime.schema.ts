import { z } from 'zod'

export const CreateAnimeSchema = z.object({
  title: z.string(),
  image: z.string().url(),
  creators: z.array(z.string()),
  episodes: z.number().min(1)
})
