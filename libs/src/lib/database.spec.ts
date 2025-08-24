import { database } from './database.js';

describe('database', () => {
  it('should work', () => {
    expect(database()).toEqual('database');
  });
});
