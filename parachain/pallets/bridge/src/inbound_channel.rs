
use frame_support::dispatch::DispatchResult;
use sp_std::cell::RefCell;

use artemis_core::{AppId, Application, Message, Verifier};

use crate::{
	Event,
	Config,
	InboundChannelData,
	primitives::{self, ChannelId, InboundChannel, InboundChannelData as ChannelData}
};


/// Construct an inbound channel object
pub fn make_inbound_channel<T: Config>(channel_id: ChannelId) -> InboundChannel<T> {
	match channel_id {
		ChannelId::Basic => {
			BasicInboundChannel {
				data: Storage::new()
			}
		},
		ChannelId::Incentivized => {
			IncentivizedInboundChannel {
				data: Storage::new()
			}
		}
	}

}

// Storage layer for a channel
struct Storage<T: Config> {
	cached_data: RefCell<Option<ChannelData>>,
}

impl<T: Config> Storage<T> {
	fn new() -> Self {
		Storage { cached_data: RefCell::new(None) }
	}
}

impl<T: Config> Storage {
	fn data(&self) -> ChannelData {
		match self.cached_data.clone().into_inner() {
			Some(data) => data,
			None => {
				let data = InboundChannelData::<T>::get(&self.lane_id);
				*self.cached_data.try_borrow_mut().expect(
					"we're in the single-threaded environment;\
						we have no recursive borrows; qed",
				) = Some(data.clone());
				data
			}
		}
	}

	fn set_data(&mut self, data: ChannelData) {
		*self.cached_data.try_borrow_mut().expect(
			"we're in the single-threaded environment;\
				we have no recursive borrows; qed",
		) = Some(data.clone());
		InboundChannelData::<T>::set(data)
	}
}

/// Basic Channel
struct BasicInboundChannel<T: Config> {
	data: Storage<T>
}

impl<T: Config> InboundChannel<T> for BasicInboundChannel<T> {
	fn submit(message: &Message) -> DispatchResult {
		// These things are available in this scope:
		//   self.data()  // persistent data for channel
		//   T::Verifier
		//   T::ApplicationRegistry
		//   T::Rewards
		//   Event
	}
}

/// Incentivized Channel
struct IncentivizedInboundChannel<T: Config> {
	data: Storage<T>
}

impl<T: Config> InboundChannel<T> for IncentivizedInboundChannel {
	fn submit(message: &Message) -> DispatchResult {
		// These things are available in this scope:
		//   self.data()  // persistent data for channel
		//   T::Verifier
		//   T::ApplicationRegistry
		//   T::Rewards
		//   Event
	}
}
