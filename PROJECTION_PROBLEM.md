# There exists a problem with projection of nodes:

So the issue here is that we don't know when we can stop applying operations
For a given node of any length, we may have to apply ALL of it's child nodes,
regardless of their offset, even if we only want the first character of the node.
This is because deletion operations could substantially shift inserts.

Ideally we would be able to reason about their positional relationships such that we
are able to determine which of them must be applied, and which can be safely skipped.
Until then, we have to apply ALL operations for a given node subgraph in order to
materialize ANY part of said node.
Note on subgraph projection - Later when we enable collaborative editing, we must find a way to
subscribe to subgraph edits, not only descendant of the node in question, but in other ways too
(uncertain how to describe this just now)

```
    struct NodeShadow {
        node_id: NodeId,
        node: Node,
        parents: Vec<NodeId>,
    }
```

OT Ideation:

```
A. Insert(10,"A")    / -> Insert(9,"A")     / -> Insert(10)
B.      *  Delete (5)                      / -> Delete(6)
C.                          * Insert(2,"C")
```

Ideas for how to potentially short circuit nodes when projecting a constrained view:

- Precalculate the sum and minimum offset of all deletions against each parent and use a pessimistic approach
- Index inserts by position and apply only those which might be relevant
- GRR: Transform the operations as they come in, and store their real offsets
  (non-ephemeral state doesn't _completely_ defeat the purpose, but almost)
- Acceptance: efficiency through compaction alone.
- Metadata-only pass - do a first pass which does not actually project the text value,
  but merely transforms the operations sufficient to determine which ones are relevant,
  and feed that list into the projection logic
- Some hybrid of the above which picks a different approach based on the size of each node, and number of nodes.
  Sidebar: Think of how to potentially reduce the number of links to traverse with each approach, and how that might effect long term performance
  Gut Check: How do we get this back on the happy path vis-a-vis mindbase and the creation of meaningful UI?
