CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    priority VARCHAR(20) NOT NULL DEFAULT 'medium',
    assignee VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tags JSONB DEFAULT '[]'::jsonb,
    CHECK (status IN ('pending', 'in_progress', 'done', 'archived')),
    CHECK (priority IN ('low', 'medium', 'high', 'critical'))
);

CREATE INDEX idx_todos_status ON todos(status);
CREATE INDEX idx_todos_assignee ON todos(assignee);
CREATE INDEX idx_todos_priority ON todos(priority);
