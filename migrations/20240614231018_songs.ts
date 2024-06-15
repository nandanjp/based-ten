// For more information about this file see https://dove.feathersjs.com/guides/cli/knexfile.html
import type { Knex } from 'knex'
import path from 'path'
import { promises } from 'fs'

export async function up(knex: Knex): Promise<void> {
  const createTable = path.join(__dirname, '../sql/songs/createTables.sql')
  const create = await promises.readFile(createTable, 'utf-8')
  const populateTable = path.join(__dirname, '../sql/songs/populateTables.sql')
  const populate = await promises.readFile(populateTable, 'utf-8')

  await knex.raw(create)
  return knex.raw(populate)
}

export async function down(knex: Knex): Promise<void> {
  const dropTables = path.join(__dirname, '../sql/songs/dropTables.sql')
  const drop = await promises.readFile(dropTables, 'utf-8')
  return knex.raw(drop)
}
