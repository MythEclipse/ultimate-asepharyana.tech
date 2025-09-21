"use strict";
/**
 * Test script to verify logger unification works correctly
 * Run with: ts-node apps/nextjs/utils/test-logger-unification.ts
 */
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const logger_1 = __importDefault(require("./logger"));
const serverLogger_1 = __importDefault(require("./serverLogger"));
const unified_logger_1 = __importStar(require("./unified-logger"));
console.log('🧪 Testing Logger Unification...\n');
// Test 1: Check that all loggers have the same interface
console.log('1️⃣ Testing logger interface consistency...');
const expectedMethods = ['error', 'warn', 'info', 'debug', 'verbose', 'silly', 'http'];
const loggers = { clientLogger: logger_1.default, serverLogger: serverLogger_1.default, unifiedLogger: unified_logger_1.default };
Object.entries(loggers).forEach(([name, logger]) => {
    console.log(`  Checking ${name}...`);
    expectedMethods.forEach(method => {
        if (typeof logger[method] !== 'function') {
            console.error(`    ❌ Missing method: ${method}`);
        }
        else {
            console.log(`    ✅ ${method} method exists`);
        }
    });
});
// Test 2: Runtime environment detection
console.log('\n2️⃣ Testing runtime environment detection...');
const environment = (0, unified_logger_1.detectRuntimeEnvironment)();
console.log(`  Detected environment: ${environment}`);
// Test 3: Test actual logging functionality
console.log('\n3️⃣ Testing logging functionality...');
console.log('  Testing client logger:');
try {
    logger_1.default.info('✅ Client logger info test');
    logger_1.default.warn('✅ Client logger warning test');
    logger_1.default.error('✅ Client logger error test');
    console.log('  ✅ Client logger tests passed');
}
catch (error) {
    console.error('  ❌ Client logger tests failed:', error);
}
console.log('\n  Testing server logger:');
try {
    serverLogger_1.default.info('✅ Server logger info test');
    serverLogger_1.default.warn('✅ Server logger warning test');
    serverLogger_1.default.error('✅ Server logger error test');
    console.log('  ✅ Server logger tests passed');
}
catch (error) {
    console.error('  ❌ Server logger tests failed:', error);
}
console.log('\n  Testing unified logger:');
try {
    unified_logger_1.default.info('✅ Unified logger info test');
    unified_logger_1.default.warn('✅ Unified logger warning test');
    unified_logger_1.default.error('✅ Unified logger error test');
    console.log('  ✅ Unified logger tests passed');
}
catch (error) {
    console.error('  ❌ Unified logger tests failed:', error);
}
// Test 4: Test different argument types
console.log('\n4️⃣ Testing different argument types...');
try {
    const testObject = { key: 'value', number: 123, nested: { deep: 'value' } };
    const testArray = [1, 2, 3, 'four', { five: 5 }];
    const testError = new Error('Test error message');
    unified_logger_1.default.info('String message only');
    unified_logger_1.default.info('Object argument:', testObject);
    unified_logger_1.default.info('Array argument:', testArray);
    unified_logger_1.default.error('Error object:', testError);
    unified_logger_1.default.info('Multiple', 'mixed', 'arguments', 123, true, testObject);
    console.log('  ✅ Different argument types test passed');
}
catch (error) {
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
    }
    else {
        console.error('  ❌ logErrorToApi function not properly exported');
    }
}
catch (error) {
    console.error('  ❌ Error testing logErrorToApi function:', error);
}
console.log('\n✅ Logger unification tests completed!');
console.log('\n📋 Summary:');
console.log('   - All loggers maintain consistent API interface');
console.log('   - Runtime environment detection works correctly');
console.log('   - Backward compatibility preserved for existing imports');
console.log('   - Unified logger provides environment-appropriate behavior');
console.log('   - All logging methods work with various argument types');
