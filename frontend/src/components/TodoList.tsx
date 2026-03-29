import React from 'react';
import { Todo } from '../types';
import { TodoItem } from './TodoItem';
import { useTodoContext } from '../context/TodoContext';

export const TodoList: React.FC = () => {
  const { todos, filter } = useTodoContext();
  const filtered = todos.filter(t => filter === 'all' || t.status === filter);
  return <div className="todo-list">{filtered.map(t => <TodoItem key={t.id} todo={t} />)}</div>;
};
