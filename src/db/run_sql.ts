import prisma from './db'
import { readFile } from 'fs/promises'
import * as path from 'path'
import { z } from 'zod'

const args_schema = z.array(z.string()).min(3)
args_schema.parse(process.argv)

declare global {
  namespace NodeJS {
    interface ProcessArgv extends z.infer<typeof args_schema> {}
  }
}

const path_to_file = path.join(__dirname, `./sql/${process.argv[2]}.sql`)
async function create() {
  try {
    const sql_query = await readFile(path_to_file, 'utf-8')
    const sqlStatements = sql_query.split(';').filter((statement) => statement.trim())
    for (const statement of sqlStatements) {
      await prisma.$executeRawUnsafe(statement)
    }
    console.log('SQL file executed successfully.')
  } catch (error) {
    console.error('Error executing SQL file:', error)
  }
}

create().catch((e) => {
  console.error(e)
  process.exit(1)
})
