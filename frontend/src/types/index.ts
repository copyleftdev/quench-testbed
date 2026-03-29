export interface Todo {
  id: string;
  title: string;
  description: string;
  status: 'pending' | 'in_progress' | 'done';
  priority: 'low' | 'medium' | 'high' | 'critical';
  assignee: string;
  created_at: string;
  updated_at: string;
  tags: string[];
  audit_trail: AuditEntry[];
}

export interface AuditEntry {
  timestamp: string;
  action: string;
  actor: string;
  details: Record<string, unknown>;
}

export type TodoAction =
  | { type: 'ADD'; payload: Todo }
  | { type: 'TOGGLE'; id: string }
  | { type: 'DELETE'; id: string };
