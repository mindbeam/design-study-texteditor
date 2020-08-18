Goals:

- Determine if we can use HashFusion to make the unbase model work
- Determine how we merge this with unbase/mindbase
- Use text as a design ideation on data type API

Critical Path:

- use the most brutally inefficient method possible to render the tree
- render the tree as lines (redundantly render the entire document forward of the cursor for now)
- Presence indication
- STOP

Possible Task Items:

- X leftward cursor traversal
- X leftward pre-traversal regional projection
- X Basic node Ordering
- X document TreeView
- Fully functional cursive UI
- data persistence (sled)
- Tree view
- regional traversal by line
- left-right on-demand regional traversal
- Move basic storage/traversal guts to a storage class, and make document and entity type
- Make DocumentCursor an entity type as well
- X rendered deletion
- cursor indication
- arrow cursor moving (relative)
- Scrolling and positioning
- Modularize projection (create basic SET entity type as a test pilot?)
- Loading flatfiles
- Concurrency
- Saving and loading trees
- Saving flatfiles
- Node consolidation
- Hashfusion vs Merkel strategies - applicability to unbase, etc
- Port HashFusion to rust?
- mouse cursor moving
- TODO - what does this mean? "document section" tree (filtered version of keystroke tree)
