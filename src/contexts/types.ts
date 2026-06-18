import type workerpool from 'workerpool';
import type { CustomWorker } from '../worker/worker.js';

export const Types = {
  WorkerPool: Symbol.for('WorkerPool'),
  FileProcessorConfig: Symbol.for('FileProcessorConfig'), // Added token for combined config
};

export type WorkerPool = workerpool.Proxy<CustomWorker>;
