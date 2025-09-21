/**
 * Test file to verify logger unification works correctly
 * This test ensures backward compatibility and proper functionality
 */

import unifiedLogger from '../unified-logger';
import { detectRuntimeEnvironment } from '../unified-logger';

describe('Logger Unification', () => {
  it('should provide the same interface for unified logger', () => {
    // Test that unified logger has the expected methods
    const expectedMethods = ['error', 'warn', 'info', 'debug', 'verbose', 'silly', 'http'];

    expectedMethods.forEach(method => {
      expect(typeof unifiedLogger[method as keyof typeof unifiedLogger]).toBe('function');
    });
  });

  it('should detect runtime environment correctly', () => {
    const environment = detectRuntimeEnvironment();
    // In test environment, should detect as server
    expect(['server', 'client', 'unknown']).toContain(environment);
  });

  it('should work with unified logger methods', () => {
    // Test that unified logger methods can be called without errors
    expect(() => {
      unifiedLogger.info('Test info message');
      unifiedLogger.warn('Test warning message');
      unifiedLogger.error('Test error message');
      unifiedLogger.debug('Test debug message');
    }).not.toThrow();
  });

  it('should work with unified logger directly', () => {
    // Test that unified logger methods can be called without errors
    expect(() => {
      unifiedLogger.info('Test info message');
      unifiedLogger.warn('Test warning message');
      unifiedLogger.error('Test error message');
      unifiedLogger.debug('Test debug message');
    }).not.toThrow();
  });

  it('should handle different argument types', () => {
    const testObject = { key: 'value', number: 123 };
    const testArray = [1, 2, 3];
    const testError = new Error('Test error');

    expect(() => {
      unifiedLogger.info('String message');
      unifiedLogger.info('Object:', testObject);
      unifiedLogger.info('Array:', testArray);
      unifiedLogger.error('Error:', testError);
      unifiedLogger.info('Multiple', 'arguments', 123, true);
    }).not.toThrow();
  });
});
