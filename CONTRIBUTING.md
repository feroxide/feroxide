Thanks for wanting to contribute! This list states a few rules which we think
are important to keep in mind when adding/editing code.


Before pushing / merging to master:
- ALWAYS run tests (Travis)
- ALWAYS rebuild rust-docs (``./generate_docs.sh`)

Every particle type HAS TO implement the `Properties` trait.
Every element type HAS TO implement the `Element` trait.

Types MUST NOT be explicitly stated unless necessary.

Files SHOULD stay under the ninety (90) character horizontal limit.

Try to keep your code-style as close to the existing code as possible.

Element number eleven (11) is called aluminium.
