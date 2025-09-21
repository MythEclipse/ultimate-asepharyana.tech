import { ErrorCategory, ErrorSeverity } from '../types/error';
import {
  createError,
  createNetworkError,
  createHttpError,
  toAppError,
  logError,
  withRetry,
  ErrorHandler,
} from './error-handler';

console.log('🧪 Testing Error Handler System...\n');

// Test 1: Basic error creation
console.log('1️⃣ Testing createError:');
const basicError = createError('Test basic error', ErrorCategory.UNKNOWN);
console.log('✅ Basic error created:', {
  message: basicError.message,
  category: basicError.category,
  severity: basicError.severity,
  timestamp: basicError.timestamp,
});

// Test 2: Network error creation
console.log('\n2️⃣ Testing createNetworkError:');
const networkError = createNetworkError('Connection failed', {
  code: 'ETIMEDOUT',
  url: 'https://api.example.com',
  context: { timeout: 5000 },
});
console.log('✅ Network error created:', {
  message: networkError.message,
  category: networkError.category,
  code: networkError.code,
  url: networkError.url,
  context: networkError.context,
});

// Test 3: HTTP error creation
console.log('\n3️⃣ Testing createHttpError:');
const httpError = createHttpError('Not found', 404, {
  statusText: 'Not Found',
  url: '/api/users/123',
});
console.log('✅ HTTP error created:', {
  message: httpError.message,
  category: httpError.category,
  statusCode: httpError.statusCode,
  statusText: httpError.statusText,
  url: httpError.url,
});

// Test 4: toAppError conversion
console.log('\n4️⃣ Testing toAppError:');
const regularError = new Error('Regular error');
const convertedError = toAppError(regularError, {
  url: '/api/test',
  method: 'POST'
});
console.log('✅ Error converted:', {
  originalMessage: regularError.message,
  convertedMessage: convertedError.message,
  category: convertedError.category,
  context: convertedError.context,
});

// Test 5: ErrorHandler utility object
console.log('\n5️⃣ Testing ErrorHandler utility:');
console.log('✅ ErrorHandler exports available functions:', {
  createError: typeof ErrorHandler.createError,
  createNetworkError: typeof ErrorHandler.createNetworkError,
  createHttpError: typeof ErrorHandler.createHttpError,
  toAppError: typeof ErrorHandler.toAppError,
  logError: typeof ErrorHandler.logError,
  withRetry: typeof ErrorHandler.withRetry,
});

// Test 6: Retry logic (mock)
console.log('\n6️⃣ Testing retry logic:');
async function testRetry() {
  let attempts = 0;
  const operation = async () => {
    attempts++;
    if (attempts < 3) {
      throw new Error(`Attempt ${attempts} failed`);
    }
    return `Success after ${attempts} attempts`;
  };

  try {
    const result = await withRetry(operation, {
      enabled: true,
      maxAttempts: 3,
      delayMs: 100,
      backoffMultiplier: 2,
      maxDelayMs: 1000,
    });
    console.log('✅ Retry succeeded:', result);
  } catch (error: unknown) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.log('❌ Retry failed:', errorMessage);
  }
}

// Test 7: Error response creation
console.log('\n7️⃣ Testing error response creation:');
const errorResponse = ErrorHandler.createErrorResponse(
  createError('API Error', ErrorCategory.HTTP, { statusCode: 500 })
);
console.log('✅ Error response created:', errorResponse);

// Run async tests
testRetry().then(() => {
  console.log('\n🎉 All error handler tests completed!');
}).catch((error) => {
  console.error('❌ Error during testing:', error);
});
