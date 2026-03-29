import React from 'react';
import { Todo } from '../types';

interface Props { todo: Todo; }
export const TodoItem: React.FC<Props> = ({ todo }) => (
  <div className={`todo-item ${todo.status}`}>
    <input type="checkbox" checked={todo.status === 'done'} />
    <span>{todo.title}</span>
    <span className="priority">{todo.priority}</span>
  </div>
);
