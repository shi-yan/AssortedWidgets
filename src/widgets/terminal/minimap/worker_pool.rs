/// Background worker pool for minimap rasterization
///
/// Phase 5: Multi-threaded background rasterization.
/// Uses crossbeam-channel for job queue and result delivery.

use std::thread;
use crossbeam_channel::{Sender, Receiver, bounded, unbounded};
use super::raster_job::{RasterJob, RasterResult, rasterize_job_background};

/// Worker pool for background rasterization
///
/// Maintains a pool of worker threads that process raster jobs.
pub struct WorkerPool {
    /// Send jobs to workers
    job_sender: Sender<WorkerMessage>,

    /// Receive results from workers
    result_receiver: Receiver<RasterResult>,

    /// Number of worker threads
    num_workers: usize,

    /// Current grid generation (for job cancellation)
    current_generation: usize,
}

/// Messages sent to worker threads
enum WorkerMessage {
    /// Process a raster job
    Job(RasterJob),

    /// Shutdown the worker
    Shutdown,
}

impl WorkerPool {
    /// Create a new worker pool
    ///
    /// # Arguments
    /// * `num_workers` - Number of background threads (default: 2)
    pub fn new(num_workers: usize) -> Self {
        let (job_sender, job_receiver) = unbounded::<WorkerMessage>();
        let (result_sender, result_receiver) = unbounded::<RasterResult>();

        // Spawn worker threads
        for worker_id in 0..num_workers {
            let job_rx = job_receiver.clone();
            let result_tx = result_sender.clone();

            thread::Builder::new()
                .name(format!("minimap-worker-{}", worker_id))
                .spawn(move || {
                    worker_thread(job_rx, result_tx);
                })
                .expect("Failed to spawn worker thread");
        }

        Self {
            job_sender,
            result_receiver,
            num_workers,
            current_generation: 0,
        }
    }

    /// Submit a raster job for background processing
    ///
    /// Jobs are processed in FIFO order (no prioritization yet).
    ///
    /// # Arguments
    /// * `job` - Raster job to process
    pub fn submit_job(&self, job: RasterJob) {
        let _ = self.job_sender.send(WorkerMessage::Job(job));
    }

    /// Try to receive completed results (non-blocking)
    ///
    /// Returns all available results. Call this frequently from the main thread
    /// to retrieve completed rasterizations.
    ///
    /// # Returns
    /// Vec of completed RasterResults
    pub fn try_recv_results(&self) -> Vec<RasterResult> {
        let mut results = Vec::new();

        // Drain all available results
        while let Ok(result) = self.result_receiver.try_recv() {
            // Filter out stale results (grid generation changed)
            if result.grid_generation == self.current_generation {
                results.push(result);
            }
            // Stale results are silently dropped
        }

        results
    }

    /// Update current grid generation (invalidates pending jobs)
    ///
    /// Call this when the terminal resizes. Pending jobs with old generation
    /// will be filtered out when results are received.
    ///
    /// # Arguments
    /// * `generation` - New grid generation
    pub fn set_generation(&mut self, generation: usize) {
        self.current_generation = generation;
    }

    /// Get number of worker threads
    pub fn num_workers(&self) -> usize {
        self.num_workers
    }

    /// Shutdown the worker pool
    ///
    /// Sends shutdown signal to all workers and waits for them to finish.
    pub fn shutdown(self) {
        for _ in 0..self.num_workers {
            let _ = self.job_sender.send(WorkerMessage::Shutdown);
        }
        // Channels will be dropped automatically when self is dropped
    }
}

/// Worker thread function
///
/// Loops indefinitely, processing jobs until shutdown signal received.
fn worker_thread(
    job_receiver: Receiver<WorkerMessage>,
    result_sender: Sender<RasterResult>,
) {
    loop {
        match job_receiver.recv() {
            Ok(WorkerMessage::Job(job)) => {
                // Rasterize the job
                let result = rasterize_job_background(job);

                // Send result back (ignore errors if channel closed)
                let _ = result_sender.send(result);
            }
            Ok(WorkerMessage::Shutdown) | Err(_) => {
                // Shutdown signal or channel closed
                break;
            }
        }
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        // Send shutdown to remaining workers
        for _ in 0..self.num_workers {
            let _ = self.job_sender.send(WorkerMessage::Shutdown);
        }
    }
}
