import dotenv from "dotenv"
import { client } from './db'
import format from 'pg-format'
import { readFile } from 'fs/promises'
import * as path from 'path'
import { z } from 'zod'

dotenv.config()

const ProcessArgSchema = z.array(z.string()).min(3)
ProcessArgSchema.parse(process.argv)

const ProvidedCorrectFile = z.enum([
  'create-tables',
  'drop-tables',
  'delete-rows',
  'create-list/test-sample',
  'sample-queries/feature1',
  'sample-queries/feature2',
  'sample-queries/feature3',
  'sample-queries/feature4'
])
ProvidedCorrectFile.parse(process.argv[2])

declare global {
  namespace NodeJS {
    interface ProcessArgv extends z.infer<typeof ProcessArgSchema> {}
  }
}

async function runSql() {
  try {
    const file = process.argv[2] as z.infer<typeof ProvidedCorrectFile>
    const path_to_file = path.join(__dirname, `./sql/${file}.sql`)

    await client.connect()
    const sql_query = await readFile(path_to_file, 'utf-8')

    for (const query of sql_query.split(';')) {
      const res = await client.query(format(query))
      console.log(res.rows)
    }
  } catch (error) {
    console.error('Error executing SQL file:', error)
  } finally {
    await client.end()
  }
}

runSql().catch((e) => {
  console.error(e)
  process.exit(1)
})