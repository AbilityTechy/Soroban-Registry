-- Contract dependencies table (for dependency graph)
CREATE TABLE contract_dependencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    depends_on_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    dependency_type VARCHAR(50) NOT NULL DEFAULT 'calls',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(contract_id, depends_on_id),
    CHECK (contract_id != depends_on_id)
);

CREATE INDEX idx_deps_contract ON contract_dependencies(contract_id);
CREATE INDEX idx_deps_depends_on ON contract_dependencies(depends_on_id);
