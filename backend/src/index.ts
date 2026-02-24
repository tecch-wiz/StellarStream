/**
 * StellarStream Event Watcher Service
 * 
 * Monitors the Stellar blockchain for events emitted by the StellarStream contract
 */

import { getConfig } from "./config";
import { EventWatcher } from "./event-watcher";
import { logger } from "./logger";

/**
 * Main entry point
 */
async function main(): Promise<void> {
  logger.info("Starting StellarStream Event Watcher Service");

  try {
    // Load configuration
    const config = getConfig();
    logger.info("Configuration loaded", {
      rpcUrl: config.rpcUrl,
      contractId: config.contractId,
      pollInterval: `${config.pollIntervalMs}ms`,
    });

    // Create and start event watcher
    const watcher = new EventWatcher(config);

    // Handle graceful shutdown
    const shutdown = async (signal: string) => {
      logger.info(`Received ${signal}, shutting down gracefully...`);
      await watcher.stop();
      process.exit(0);
    };

    process.on("SIGINT", () => shutdown("SIGINT"));
    process.on("SIGTERM", () => shutdown("SIGTERM"));

    // Handle uncaught errors
    process.on("uncaughtException", (error) => {
      logger.error("Uncaught exception", error);
      process.exit(1);
    });

    process.on("unhandledRejection", (reason) => {
      logger.error("Unhandled rejection", reason);
      process.exit(1);
    });

    // Start watching
    await watcher.start();
  } catch (error) {
    logger.error("Fatal error during startup", error);
    process.exit(1);
  }
}

// Run the service
main().catch((error) => {
  logger.error("Fatal error in main", error);
  process.exit(1);
});
