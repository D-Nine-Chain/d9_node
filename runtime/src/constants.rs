//! constants module is set for runtime constants.
//! this is not to be used for pallets constants that is to be defined in parameter_types!
pub mod constants {
	use crate::{ Balance, Block };
	pub const MILLISECS_PER_BLOCK: u64 = 3000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
}
