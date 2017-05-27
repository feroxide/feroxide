Thanks for wanting to contribute! This list states a few rules which we think
are important to keep in mind when changing code.


Before pushing / merging to master, please:
- ALWAYS run tests (Travis)
- ALWAYS run the `precommit.sh` script
- ALWAYS build docs (`yes | precommit.sh` or run `generate_docs` manually)

Every particle type HAS TO implement the `Properties` trait.
Every element type HAS TO implement the `Element` trait.
Every struct SHOULD implement the `Debug` trait unless not possible.

Types SHOULD NOT be explicitly stated unless necessary.

Files SHOULD stay under the ninety (90) character horizontal character limit.

Try to keep your code-style as close to the existing code as possible.

Element number eleven (11) is called aluminium. :blush:
