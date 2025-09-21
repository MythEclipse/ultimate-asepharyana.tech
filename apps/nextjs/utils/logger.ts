/**
 * Client-side logger - re-exports from unified logger for backward compatibility
 *
 * This module provides a client-side logger that automatically detects the runtime
 * environment and provides appropriate logging behavior. In the browser, it uses
 * console methods. On the server, it uses Winston with file rotation and
 * structured logging.
 *
 * @deprecated Use the unified logger directly from 'utils/unified-logger' for new code
 */

import unifiedLogger, { logErrorToApi } from './unified-logger';

// Re-export the unified logger as the client logger for backward compatibility
export default unifiedLogger;

// Re-export the logErrorToApi function for backward compatibility
export { logErrorToApi };
