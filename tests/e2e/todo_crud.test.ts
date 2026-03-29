import { test, expect } from '@playwright/test';

test.describe('Todo CRUD', () => {
  test('create a todo', async ({ request }) => {
    const resp = await request.post('/api/v1/todos', {
      data: { title: 'Test todo', priority: 'high' }
    });
    expect(resp.status()).toBe(201);
    const todo = await resp.json();
    expect(todo.id).toBeDefined();
    expect(todo.status).toBe('pending');
  });

  test('list todos', async ({ request }) => {
    const resp = await request.get('/api/v1/todos');
    expect(resp.status()).toBe(200);
  });

  test('toggle todo status', async ({ request }) => {
    // Create then toggle
    const create = await request.post('/api/v1/todos', {
      data: { title: 'Toggle test', priority: 'low' }
    });
    const todo = await create.json();
    const toggle = await request.put(`/api/v1/todos/${todo.id}`, {
      data: { ...todo, status: 'done' }
    });
    expect(toggle.status()).toBe(200);
  });
});
