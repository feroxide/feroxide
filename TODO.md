* global
  * Reactions
    * Precipitation reactions
    * Acid/Base reactions
  * Add hydration (ex. CuSO4.5H2O) to molecules
  * Optimise common data(?)
  * Add electrovalence data to atoms
  * Browse structs to see if display has been implemented for structs that need it

* `atoms.rs`
  * Names for special cases (-oxide etc.)

* `reaction.rs`
  * Calculate the correct amount of energy required
  * Remove `is_equilibrium`, as it's not a static value

* `redox.rs`
  * Clean up `elem_reaction`

* 'naming.rs'
  * Implement Display for any n being processed