import { RiceCooker } from './RiceCooker';
import { describe, test, afterEach, beforeEach, expect } from '@jest/globals';



describe('RiceCooker', () => {
  let riceCooker: RiceCooker;

  beforeEach(() => {
    riceCooker = new RiceCooker();
  });

  test('cookRice should cook rice', async () => {
    const result = await riceCooker.cookRice();
    expect(result).toBeTruthy(); // Assuming success returns true
  });

  test('keepWarm should keep food warm', async () => {
    const result = await riceCooker.keepWarm();
    expect(result).toBeTruthy(); // Assuming success returns true
  });

  test('steamCook should cook food with steam', async () => {
    const result = await riceCooker.steamCook();
    expect(result).toBeTruthy(); // Assuming success returns true
  });

  test('cookSoup should cook soup', async () => {
    const result = await riceCooker.cookSoup();
    expect(result).toBeTruthy(); // Assuming success returns true
  });

  test('cookDessert should cook dessert', async () => {
    const result = await riceCooker.cookDessert();
    expect(result).toBeTruthy(); // Assuming success returns true
  });
});
