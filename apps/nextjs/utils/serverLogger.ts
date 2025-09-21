/**
 * Server-side logger - re-exports from unified logger for backward compatibility
 *
 * This module provides a server-side logger that uses Winston with file rotation
 * and structured logging. It automatically detects the runtime environment and
 * provides appropriate logging behavior.
 *
 * @deprecated Use the unified logger directly from 'utils/unified-logger' for new code
 */

import unifiedLogger, { logErrorToApi } from './unified-logger';

// Re-export the unified logger as the server logger for backward compatibility
export default unifiedLogger;

// Re-export the logErrorToApi function for backward compatibility
export { logErrorToApi };
