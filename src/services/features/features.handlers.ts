import { Request, Response } from 'express';
import { prisma } from '../../db/db';
import { readFile } from 'fs/promises';
import path from 'path';


export const getFeatures = async (req: Request, res: Response) => {
    const result = [];

    const path_to_file = path.join(__dirname, `../../db/sql/sample-queries/feature1.sql`);
    const sql_query = await readFile(path_to_file, 'utf-8');
    for (const query of sql_query.split(';')) {
        const res = await prisma.$queryRawUnsafe(query);
        result.push(res);
    }
  res.status(200).send(result);
};