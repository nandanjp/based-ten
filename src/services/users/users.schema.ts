import { z } from 'zod'

export const CreateUserSchema = z
  .object({
    name: z.string(),
    display_name: z.string().optional(),
    email: z.string().email(),
    password: z.string().min(6).max(15)
  })
  .refine(
    (d) => {
      if (!d.display_name) d.display_name = d.name
      return true //must always return boolean
    },
    {
      message: 'display_name should default to name if not provided',
      path: ['display_name'] //error path for clarity
    }
  )
