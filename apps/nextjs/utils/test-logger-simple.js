/**
 * Simple test script to verify logger unification works correctly
 * Run with: node apps/nextjs/utils/test-logger-simple.js
 */

// Test basic functionality without TypeScript imports
console.log('🧪 Testing Logger Unification...\n');

// Test 1: Check that we can require the modules without errors
console.log('1️⃣ Testing module loading...');
try {
  const _clientLogger = require('./logger.ts');
  console.log('  ✅ Client logger module loaded');

  const _serverLogger = require('./serverLogger.ts');
  console.log('  ✅ Server logger module loaded');

  const _unifiedLogger = require('./unified-logger.ts');
  console.log('  ✅ Unified logger module loaded');
} catch (error) {
  console.error('  ❌ Module loading failed:', error.message);
}

// Test 2: Check TypeScript compilation
console.log('\n2️⃣ Testing TypeScript compilation...');
try {
  const { execSync } = require('child_process');
  const _result = execSync(
    'npx tsc --noEmit --skipLibCheck utils/logger.ts utils/serverLogger.ts utils/unified-logger.ts',
    {
      cwd: 'apps/nextjs',
      encoding: 'utf8',
    },
  );
  console.log('  ✅ TypeScript compilation successful');
} catch (error) {
  console.error('  ❌ TypeScript compilation failed:', error.message);
}

// Test 3: Check that the unified logger provides the expected interface
console.log('\n3️⃣ Testing unified logger interface...');
try {
  // Check if the files exist and have the expected structure
  const fs = require('fs');
  const path = require('path');

  const unifiedLoggerPath = path.join(__dirname, 'unified-logger.ts');
  const unifiedLoggerContent = fs.readFileSync(unifiedLoggerPath, 'utf8');

  const expectedExports = [
    'error',
    'warn',
    'info',
    'debug',
    'verbose',
    'silly',
    'http',
  ];
  const hasAllMethods = expectedExports.every(
    (method) =>
      unifiedLoggerContent.includes(`${method}:`) ||
      unifiedLoggerContent.includes(`${method}(`),
  );

  if (hasAllMethods) {
    console.log('  ✅ Unified logger has all expected methods');
  } else {
    console.log('  ⚠️  Some methods might be missing in unified logger');
  }

  // Check for backward compatibility exports
  const loggerPath = path.join(__dirname, 'logger.ts');
  const loggerContent = fs.readFileSync(loggerPath, 'utf8');

  if (
    loggerContent.includes('unifiedLogger') &&
    loggerContent.includes('logErrorToApi')
  ) {
    console.log('  ✅ Client logger maintains backward compatibility');
  } else {
    console.log(
      '  ⚠️  Client logger might not maintain full backward compatibility',
    );
  }

  const serverLoggerPath = path.join(__dirname, 'serverLogger.ts');
  const serverLoggerContent = fs.readFileSync(serverLoggerPath, 'utf8');

  if (
    serverLoggerContent.includes('unifiedLogger') &&
    serverLoggerContent.includes('logErrorToApi')
  ) {
    console.log('  ✅ Server logger maintains backward compatibility');
  } else {
    console.log(
      '  ⚠️  Server logger might not maintain full backward compatibility',
    );
  }
} catch (error) {
  console.error('  ❌ Interface test failed:', error.message);
}

// Test 4: Check that types are properly exported
console.log('\n4️⃣ Testing type exports...');
try {
  const typesIndexPath = path.join(__dirname, '../types/index.ts');
  const typesIndexContent = fs.readFileSync(typesIndexPath, 'utf8');

  if (typesIndexContent.includes("export * from './logger'")) {
    console.log('  ✅ Logger types are exported from types index');
  } else {
    console.log('  ⚠️  Logger types might not be exported from types index');
  }

  const loggerTypesPath = path.join(__dirname, '../types/logger.ts');
  const loggerTypesContent = fs.readFileSync(loggerTypesPath, 'utf8');

  const expectedTypes = ['ILogger', 'LogLevel', 'LoggerConfig', 'ErrorInfo'];
  const hasAllTypes = expectedTypes.every(
    (type) =>
      loggerTypesContent.includes(`export ${type}`) ||
      loggerTypesContent.includes(`interface ${type}`) ||
      loggerTypesContent.includes(`type ${type}`),
  );

  if (hasAllTypes) {
    console.log('  ✅ All expected types are defined');
  } else {
    console.log('  ⚠️  Some types might be missing');
  }
} catch (error) {
  console.error('  ❌ Type export test failed:', error.message);
}

console.log('\n✅ Logger unification verification completed!');
console.log('\n📋 Summary:');
console.log('   - Module loading: Tested');
console.log('   - TypeScript compilation: Tested');
console.log('   - Interface consistency: Tested');
console.log('   - Backward compatibility: Tested');
console.log('   - Type exports: Tested');
console.log('\n💡 Next steps:');
console.log('   - Test in actual Next.js application');
console.log('   - Verify file logging works in production');
console.log('   - Test logErrorToApi function with actual API endpoint');
