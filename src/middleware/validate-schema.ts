import { NextFunction, Request, Response } from 'express'
import { ZodSchema } from 'zod'

export const validate_body =
  (schema: ZodSchema) => async (req: Request, res: Response, next: NextFunction) => {
    const outcome = schema.safeParse(req.body)
    if (!outcome.success) {
      return next(new Error(outcome.error.toString()))
    }
    next()
  }
