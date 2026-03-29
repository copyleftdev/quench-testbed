# quench-testbed

> **Automated governance experiment for multi-agent AI-driven Git workflows.**

This repository is a test subject. It exists to be modified, branched, merged, and occasionally broken by autonomous agents under the supervision of the [QUENCH](https://github.com/copyleftdev/quench) control plane.

Nothing here is written by a human. Everything — every PR, every merge, every conflict — is the product of an experiment.

---

## What Happened Here

**90 pull requests.** 80 merged. 10 blocked. 339 CI runs. 8 specialized teams of autonomous agents, all working simultaneously on an over-engineered todo application.

The teams:

| Team | Role | What They Touch |
|------|------|-----------------|
| **Frontend** | UI components, state management | `frontend/src/components/`, `frontend/src/context/` |
| **Backend** | API routes, services, models | `backend/src/api/`, `backend/src/services/` |
| **Database** | Schema migrations | `database/migrations/` |
| **DevOps** | Containers, IaC, orchestration | `infra/docker/`, `infra/terraform/`, `infra/k8s/` |
| **Security** | Auth middleware, scanning config | `security/auth/`, `security/scanning/` |
| **Compliance** | Data retention, access control policies | `compliance/policies/` |
| **QA** | E2E tests, load tests | `tests/e2e/`, `tests/load/` |
| **Platform/SRE** | Dashboards, alert rules | `monitoring/grafana/`, `monitoring/alerts/` |

All 8 teams operated concurrently. When two agents targeted the same file, one was blocked before it could branch. When agents worked on separate files, they proceeded in parallel and merged without conflict.

---

## The Conflict That Was Prevented

The most important event in this repo's history is something that **didn't happen**.

Two backend agents — `backend-carol` and `backend-dave` — both wanted to modify `backend/src/services/todo_service.rs` at the same time:

- **Carol**: adding pagination support
- **Dave**: adding subtask data model

Without governance, both would have branched, pushed conflicting changes, and created a merge conflict requiring manual resolution.

We proved this by letting them race:
- Carol's PR merged first
- Dave's PR was marked `mergeable: false`, `mergeable_state: dirty`
- GitHub returned `HTTP 405: Pull Request is not mergeable`

With governance, Dave was blocked **at intent declaration** — before he branched, before he wrote a single line, before he created a PR. Zero wasted work.

See: [PR #19](../../pull/19) (Carol merged) and [PR #20](../../pull/20) (Dave blocked)

---

## The CI Evidence

Every push to this repo triggers a CI pipeline. The results are visible:

**[View all CI runs](../../actions)**

The pipeline runs `cargo test` and `cargo clippy` on push and PR. Since this is a testbed with placeholder code, CI consistently reports failure — and that's the point. The control plane's CI gate detects this state and blocks merges until checks pass.

During the chaos test, the control plane logged CI status for each PR:
```
frontend-alice       PR #62   MERGED  (CI: failure)
backend-carol        PR #64   MERGED  (CI: failure)
sec-heidi            PR #68   MERGED  (CI: failure)
compliance-judy      PR #70   MERGED  (CI: failure)
```

In a production deployment, these merges would be blocked until CI passes. The testbed runs demonstrate the gate's awareness of CI state — the enforcement mode is what changes.

---

## What We Learned

This experiment is part of a larger study across **19 production repositories** including Kubernetes, React, Terraform, Vault, PyTorch, TensorFlow, VS Code, Rust, Elasticsearch, and others.

### Rules that matter

1. **Declare intent before branching.** Agents must state which files they'll modify. If those files are held by another agent, the request is denied.

2. **CI must pass before merge.** No exceptions. The gate polls until CI reaches a terminal state.

3. **Limit concurrent work per subsystem.** High-contention clusters (like `backend/src/`) get tighter branch limits than low-contention clusters (like `tests/`).

### Rules that don't matter

4. **Human code review adds zero value when CI is active.** Tested across 5 seeds — effect size is exactly 0.000.

5. **Merge queues make things worse.** They create pileup that increases instability (effect: -0.778).

6. **Cooldown windows make things worse.** They throttle merges without reducing conflict risk (effect: -0.808).

### The sharpest cliff

Adding a single required reviewer collapses throughput from 100% to 33%. The cost exceeds the benefit when automated verification is already in place.

### The strongest natural defense

Codebase partitioning. Repositories with 26+ well-separated subsystem clusters show zero merge conflicts under agent pressure. Repositories with 5 concentrated clusters show 9.1% conflict rates.

---

## Repository Structure

```
frontend/
  src/components/       TodoList, TodoItem (React)
  src/context/          State management
  src/types/            TypeScript interfaces

backend/
  src/api/              Routes, handlers (Rust/Actix)
  src/models/           Todo data model
  src/services/         Business logic (the contention hotspot)

database/
  migrations/           PostgreSQL schemas

infra/
  docker/               Dockerfile, compose
  terraform/            AWS ECS config
  k8s/                  Kubernetes deployment

security/
  auth/                 JWT middleware
  scanning/             SAST/DAST config

compliance/
  policies/             SOC2 data retention, access control

tests/
  e2e/                  Playwright tests
  load/                 k6 load scripts

monitoring/
  grafana/              Dashboards
  alerts/               Prometheus alert rules

.github/
  workflows/            CI pipeline
```

This is intentionally over-engineered. A todo app does not need 8 teams, Terraform, Kubernetes, SOC2 compliance policies, and load testing. That's the point — real organizations build systems like this, and the governance problem scales with the number of teams touching shared code.

---

## How to Read the PR History

Browse the [closed PRs](../../pulls?q=is%3Apr+is%3Aclosed) to see the full experiment:

- **PRs #1-6**: Initial E2E validation — single agent lifecycle, parallel merges, conflict detection
- **PRs #7-18**: First chaos run — 13 agents, 8 teams, 12 merged, 1 blocked
- **PRs #19-20**: Contention proof — Carol vs Dave on `todo_service.rs`
- **PRs #21+**: Subsequent chaos runs with different configurations

Each PR title indicates the team and the change. The commit messages show which agent authored the change.

---

## This Repo Is Not Maintained

This is a test artifact. The code doesn't compile, the tests don't pass, and the infrastructure configs point nowhere. That's by design — this repo exists to be governed, not to be run.

The system that governs it is [QUENCH](https://github.com/copyleftdev/quench).
