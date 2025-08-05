import 'reflect-metadata';
import express, { Request, Response } from 'express';
import cors from 'cors';
import swaggerUi from 'swagger-ui-express';
import grantRoutes from './routes/grant.routes';
import { swaggerSpec } from './config/swagger.config';

const app = express();

app.use(express.json());
app.use(cors());

app.use('/grants', grantRoutes);

// Fix for TypeScript error: spread the serve middleware array
app.use('/api-docs', ...swaggerUi.serve, swaggerUi.setup(swaggerSpec));

app.get('/', (req: Request, res: Response) => {
    res.send('Grant Contract API is running! Go to /api-docs for documentation.');
});

export default app;