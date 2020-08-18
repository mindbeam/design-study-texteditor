# The goal of this design study

The goal of this design study is to use the collaborative text editing usecase as a lens into potential APIs for other data types.
At its conclusion, this should loosely resemble a traditional implementation of Operational Transform, except insofar as it should:

1. Initially record the full history of every keystroke in a an explicit, hashed causal graph to be shared convergently with multiple parties
2. Opportunistically compact portions of that graph into single nodes
3. Calculate the hash of subgraphs based on subgraph content, such that the hash of many fine-grained operations is identical to coarser-grained operations containing their applied outcome.

== A few key implications of this design:

- Operational "Compaction" may not occur with any oustanding / "in flight" concurrencies, else the collaboration will become divergent. We accept that compaction requires coordination.
- Hashing must be based on content rather than lineage. Lineage may be preserved via a separate mechanism if desired
  TODO - Remove tick from hash inputs and determine how to handle the AB[del]B situation

## QUESTIONS:

- Consider how we might potentially refer to subgraphs in a sufficiently versatile fashion as to be robust against pre/post compation
- Consider how we might commutatively merge multi-partite concurrencies such that the same outcome is achieved.
  (A + B) + C
  =
  (B + C) + A
  =
  (C + A) + B

  (See )

- Consider how we might detect perform the above idempotently, such that neither A, nor B, nor C is applied more than once, and that this may be calculated without A, B, and C having to be resident on the same host (This is essential for the scaling properties of Unbase)

# Bibliography

- https://www.figma.com/blog/realtime-editing-of-ordered-sequences/
- Real Differences between OT andn CRDT in Building Co-Editiing Systems and Real world applications Sun et al. https://arxiv.org/abs/1905.01517
- https://github.com/BonsaiDen/cursive_tree_view
  Consider using this for tree vis
- https://en.wikipedia.org/wiki/Operational_transformation
-
