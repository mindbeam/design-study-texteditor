# The goal of this design study

The goal of this design study is to use the collaborative text editing usecase as a lens into potential APIs for other data types.
At its conclusion, this should loosely resemble a traditional implementation of Operational Transformation, except insofar as it should:

1. Initially record the full history of every keystroke in a an explicit, hashed causal graph to be shared convergently with multiple parties
2. Opportunistically compact portions of that graph into single nodes
3. Calculate the hash of subgraphs based on subgraph content, such that the hash of many fine-grained operations is identical to coarser-grained operations containing their applied outcome.

## A few key implications of this design:

- Operational "Compaction" may not occur with any oustanding / "in flight" concurrencies, else the collaboration will become divergent. We accept that compaction requires coordination.
- Hashing must be based on content rather than lineage. Lineage may be preserved via a separate mechanism if desired
  TODO - Remove tick from hash inputs and determine how to handle the AB[del]B situation

## QUESTIONS:

- Consider how we might potentially refer to subgraphs in a sufficiently versatile fashion as to be robust against pre/post compation
- Consider how we might commutatively merge multi-partite concurrencies such that the same outcome is achieved.

```
  (A + B) + C
  =
  (B + C) + A
  =
  (C + A) + B
```

- Consider how we might detect perform the above idempotently, such that neither A, nor B, nor C is applied more than once, and that this may be calculated without A, B, and C having to be resident on the same host (This is essential for the scaling properties of Unbase)

## NOTES:

Potential strategies:

1. Materialized Chunk identity + index.
   Merge the elements into chunks, and hash those chunks
   in a merkle tree,then element identity chunk + index
2. Hashfusion
   Identify a cryptographically secure hashing algorithm which allows for associative
   merging such that the identity of any fragmentary agglomeration is the same as its materialized version
   https://www.labs.hpe.com/techreports/2017/HPE-2017-08.pdf
3. MaxFrag Merkel Tree
   For any given fragmentary agglomeration, build a merkel tree based on the maximum fragmentation
   In so doing, every agglomerated representation which projects the same output posesses
   the same identity. This is much less efficient than Hashfusion, but is imminently more implementable
   and much more likely to be cryptographically sound
   QUESTIONs:
4. how do HashFusion and MaxFrag approaches work with hidden/erased information? and how do they differ?
5. does every erasure require a materialization and breaking of the concatenative identity?
6. materialized hashing vs "lineage" hashing
   (I thinkk the former can only apply to materializations which are themselves CRDTs, rather than the CRDT being embodied in the lineage)
   A text body fragmented arbitrarily, and edited by a single editor lacks "Hidden" state.
   A set being edited concurrently may not have arbitrarily fragmented operations which hash identically with the materialization
   _however_ a lineage preserving hash may?

7. How does this apply to unbase indexes?
   You will still need to order concurrencies for the hashing process, but not necessarily all at the same time
   TASKS:
   - Implement the MaxFrag Merkel tree approach (explode every document fragment into a character tree)
   - Implement rust version of Hashfusion algo ( moved to https://github.com/mindbeam/design-study-hashfusion )
   - Study potential preimage attacks

# Bibliography

- https://www.figma.com/blog/realtime-editing-of-ordered-sequences/
- Real Differences between OT andn CRDT in Building Co-Editiing Systems and Real world applications Sun et al. https://arxiv.org/abs/1905.01517
- https://github.com/BonsaiDen/cursive_tree_view
  Consider using this for tree vis
- https://en.wikipedia.org/wiki/Operational_transformation
