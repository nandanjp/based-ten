import knex from "knex";

const pg = knex({
  client: "pg",
  connection: {
    connectionString: process.env.POSTGRES_URL,
    host: process.env.POSTGRES_URL,
    port: 0,
    user: process.env.POSTGRES_URL,
    database: process.env.POSTGRES_URL,
    password: process.env.POSTGRES_URL,
  },
});
export default pg;
