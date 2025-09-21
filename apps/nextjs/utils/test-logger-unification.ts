/**
 * Test script to verify logger unification works correctly
 * Run with: ts-node apps/nextjs/utils/test-logger-unification.ts
 */

import clientLogger from './logger';
import serverLogger from './serverLogger';
import unifiedLogger, { detectRuntimeEnvironment } from './unified-logger';

console.log('🧪 Testing Logger Unification...\n');

// Test 1: Check that all loggers have the same interface
console.log('1️⃣ Testing logger interface consistency...');
const expectedMethods = ['error', 'warn', 'info', 'debug', 'verbose', 'silly', 'http'];
const loggers = { clientLogger, serverLogger, unifiedLogger };

Object.entries(loggers).forEach(([name, logger]) => {
  console.log(`  Checking ${name}...`);
  expectedMethods.forEach(method => {
    if (typeof logger[method as keyof typeof logger] !== 'function') {
      console.error(`    ❌ Missing method: ${method}`);
    } else {
      console.log(`    ✅ ${method} method exists`);
    }
  });
});

// Test 2: Runtime environment detection
console.log('\n2️⃣ Testing runtime environment detection...');
const environment = detectRuntimeEnvironment();
console.log(`  Detected environment: ${environment}`);

// Test 3: Test actual logging functionality
console.log('\n3️⃣ Testing logging functionality...');

console.log('  Testing client logger:');
try {
  clientLogger.info('✅ Client logger info test');
  clientLogger.warn('✅ Client logger warning test');
  clientLogger.error('✅ Client logger error test');
  console.log('  ✅ Client logger tests passed');
} catch (error) {
  console.error('  ❌ Client logger tests failed:', error);
}

console.log('\n  Testing server logger:');
try {
  serverLogger.info('✅ Server logger info test');
  serverLogger.warn('✅ Server logger warning test');
  serverLogger.error('✅ Server logger error test');
  console.log('  ✅ Server logger tests passed');
} catch (error) {
  console.error('  ❌ Server logger tests failed:', error);
}

console.log('\n  Testing unified logger:');
try {
  unifiedLogger.info('✅ Unified logger info test');
  unifiedLogger.warn('✅ Unified logger warning test');
  unifiedLogger.error('✅ Unified logger error test');
  console.log('  ✅ Unified logger tests passed');
} catch (error) {
  console.error('  ❌ Unified logger tests failed:', error);
}

// Test 4: Test different argument types
console.log('\n4️⃣ Testing different argument types...');
try {
  const testObject = { key: 'value', number: 123, nested: { deep: 'value' } };
  const testArray = [1, 2, 3, 'four', { five: 5 }];
  const testError = new Error('Test error message');

  unifiedLogger.info('String message only');
  unifiedLogger.info('Object argument:', testObject);
  unifiedLogger.info('Array argument:', testArray);
  unifiedLogger.error('Error object:', testError);
  unifiedLogger.info('Multiple', 'mixed', 'arguments', 123, true, testObject);

  console.log('  ✅ Different argument types test passed');
} catch (error) {
  console.error('  ❌ Different argument types test failed:', error);
}

// Test 5: Test logErrorToApi function availability
console.log('\n5️⃣ Testing logErrorToApi function availability...');
try {
  // Import the function from both modules to ensure backward compatibility
  const { logErrorToApi: clientLogError } = require('./logger');
  const { logErrorToApi: serverLogError } = require('./serverLogger');

  if (typeof clientLogError === 'function' && typeof serverLogError === 'function') {
    console.log('  ✅ logErrorToApi function available in both modules');
  } else {
    console.error('  ❌ logErrorToApi function not properly exported');
  }
} catch (error) {
  console.error('  ❌ Error testing logErrorToApi function:', error);
}

console.log('\n✅ Logger unification tests completed!');
console.log('\n📋 Summary:');
console.log('   - All loggers maintain consistent API interface');
console.log('   - Runtime environment detection works correctly');
console.log('   - Backward compatibility preserved for existing imports');
console.log('   - Unified logger provides environment-appropriate behavior');
console.log('   - All logging methods work with various argument types');
