"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.logErrorToApi = logErrorToApi;
exports.createUnifiedLogger = createUnifiedLogger;
exports.detectRuntimeEnvironment = detectRuntimeEnvironment;
const winston_1 = __importDefault(require("winston"));
const winston_daily_rotate_file_1 = __importDefault(require("winston-daily-rotate-file"));
const util_1 = __importDefault(require("util"));
/**
 * Detects the current runtime environment
 */
function detectRuntimeEnvironment() {
    // Check if we're in a browser environment
    if (typeof window !== 'undefined') {
        return 'client';
    }
    // Check if we're in a Node.js environment
    if (typeof process !== 'undefined' && process.versions && process.versions.node) {
        return 'server';
    }
    return 'unknown';
}
/**
 * Formats log arguments for consistent output
 */
function formatLogArgs(args) {
    return args
        .map((arg) => {
        if (typeof arg === 'object' && arg !== null) {
            try {
                return JSON.stringify(arg);
            }
            catch {
                // Fallback to util.inspect for complex objects
                if (typeof util_1.default !== 'undefined') {
                    return util_1.default.inspect(arg, { depth: 3, breakLength: Infinity });
                }
                return String(arg);
            }
        }
        return String(arg);
    })
        .join(' ');
}
/**
 * Creates a client-side logger using console methods
 */
function createClientLogger() {
    return {
        error: (...args) => console.error(...args),
        warn: (...args) => console.warn(...args),
        info: (...args) => console.info(...args),
        debug: (...args) => console.debug(...args),
        verbose: (...args) => console.log(...args),
        silly: (...args) => console.log(...args),
        http: (...args) => console.log(...args),
    };
}
/**
 * Creates a server-side logger using Winston with file rotation and structured logging
 */
function createServerLogger(config = {}) {
    const { level = process.env.NODE_ENV === 'production' ? 'info' : 'debug', enableColors = process.env.NODE_ENV !== 'production', enableFileLogging = process.env.NODE_ENV === 'production', logDirectory = 'logs', } = config;
    // Base format for structured logging
    const baseFormat = winston_1.default.format.combine(winston_1.default.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }), winston_1.default.format.errors({ stack: true }), winston_1.default.format.printf(({ timestamp, level, message, stack }) => {
        return `${timestamp} [${level}]: ${stack || message}`;
    }));
    const transports = [];
    // Console transport
    if (process.env.NODE_ENV !== 'production') {
        transports.push(new winston_1.default.transports.Console({
            format: enableColors
                ? winston_1.default.format.combine(winston_1.default.format.colorize(), baseFormat)
                : baseFormat,
        }));
    }
    else {
        transports.push(new winston_1.default.transports.Console({
            level: 'warn',
            format: baseFormat,
        }));
    }
    // File transports for production
    if (enableFileLogging) {
        transports.push(new winston_daily_rotate_file_1.default({
            filename: `${logDirectory}/app-%DATE%.log`,
            datePattern: 'YYYY-MM-DD',
            zippedArchive: true,
            maxSize: '20m',
            maxFiles: '30d',
            level,
        }));
    }
    const winstonLogger = winston_1.default.createLogger({
        level,
        format: baseFormat,
        transports,
        exceptionHandlers: enableFileLogging ? [
            new winston_daily_rotate_file_1.default({
                filename: `${logDirectory}/exceptions-%DATE%.log`,
                datePattern: 'YYYY-MM-DD',
                zippedArchive: true,
                maxSize: '20m',
                maxFiles: '30d',
            }),
        ] : [],
        rejectionHandlers: enableFileLogging ? [
            new winston_daily_rotate_file_1.default({
                filename: `${logDirectory}/rejections-%DATE%.log`,
                datePattern: 'YYYY-MM-DD',
                zippedArchive: true,
                maxSize: '20m',
                maxFiles: '30d',
            }),
        ] : [],
        exitOnError: false,
    });
    // Handle transport errors
    winstonLogger.transports.forEach((transport) => {
        if (transport instanceof winston_1.default.transports.File) {
            transport.on('error', (error) => {
                console.error('Error in file transport:', error);
            });
        }
    });
    return {
        error: (...args) => winstonLogger.error(formatLogArgs(args)),
        warn: (...args) => winstonLogger.warn(formatLogArgs(args)),
        info: (...args) => winstonLogger.info(formatLogArgs(args)),
        debug: (...args) => winstonLogger.debug(formatLogArgs(args)),
        verbose: (...args) => winstonLogger.verbose(formatLogArgs(args)),
        silly: (...args) => winstonLogger.silly(formatLogArgs(args)),
        http: (...args) => winstonLogger.http(formatLogArgs(args)),
    };
}
/**
 * Sends error details to the /api/log-error endpoint with retry logic.
 * This function is only available on the server side.
 *
 * @param error Error object or string
 * @param info Additional error info (e.g., React ErrorInfo)
 * @param maxRetries Number of retry attempts (default: 3)
 */
async function logErrorToApi(error, info, maxRetries = 3) {
    // This function should only work on the server side
    if (detectRuntimeEnvironment() !== 'server') {
        console.warn('logErrorToApi is only available on the server side');
        return;
    }
    let attempt = 0;
    let lastError = null;
    while (attempt < maxRetries) {
        try {
            let errorPayload;
            if (error instanceof Error) {
                errorPayload = { message: error.message, stack: error.stack };
            }
            else if (typeof error === 'object' && error !== null) {
                try {
                    errorPayload = JSON.stringify(error);
                }
                catch {
                    // Use util.inspect for better object representation as a fallback
                    errorPayload = util_1.default.inspect(error, { depth: 3, breakLength: 120 });
                }
            }
            else {
                errorPayload = String(error);
            }
            await fetch('/api/log-error', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    error: errorPayload,
                    info: typeof info === 'object' && info !== null
                        ? JSON.stringify(info)
                        : String(info),
                }),
            });
            return;
        }
        catch (e) {
            lastError = e;
            attempt++;
            await new Promise((res) => setTimeout(res, 500 * attempt)); // Exponential backoff
        }
    }
    // If all retries fail, fallback to console
    console.error('Failed to log error to API after retries:', error, info, lastError);
}
/**
 * Creates a unified logger that automatically adapts to the runtime environment
 */
function createUnifiedLogger(config) {
    const environment = detectRuntimeEnvironment();
    switch (environment) {
        case 'client':
            return createClientLogger();
        case 'server':
            return createServerLogger(config);
        default:
            // Fallback to client logger for unknown environments
            console.warn('Unknown runtime environment, falling back to client logger');
            return createClientLogger();
    }
}
// Create the default unified logger instance
const unifiedLogger = createUnifiedLogger();
// Export the logger instance and utility functions
exports.default = unifiedLogger;
