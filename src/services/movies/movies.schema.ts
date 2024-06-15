import { z } from 'zod'

export const CreateMoviesSchema = z.object({
  title: z.string(),
  image: z.string().url(),
  creators: z.array(z.string())
})
