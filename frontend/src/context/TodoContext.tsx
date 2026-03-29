import React, { createContext, useContext, useReducer } from 'react';
import { Todo, TodoAction } from '../types';

const TodoContext = createContext<any>(null);
export const useTodoContext = () => useContext(TodoContext);

function todoReducer(state: Todo[], action: TodoAction): Todo[] {
  switch (action.type) {
    case 'ADD': return [...state, action.payload];
    case 'TOGGLE': return state.map(t => t.id === action.id ? {...t, status: t.status === 'done' ? 'pending' : 'done'} : t);
    case 'DELETE': return state.filter(t => t.id !== action.id);
    default: return state;
  }
}
