
// this is basically an include
// we look for a grid.rs or grid/mod.rs file
pub mod grid;
pub mod vec2;

// this is basically a using to reexport/ shorten the name
pub use grid::Grid;
pub use vec2::Vec2;
