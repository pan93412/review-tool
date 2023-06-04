//! The application state for users.

type StateBit = u64;

/// The max available number of states.
const N: usize = std::mem::size_of::<StateBit>();

/// Nothing state
const STATE_NOTHING: StateBit = 0b0;

/// Saved state
const STATE_SAVED: StateBit = 0b1;

/// Reset (HashMap) state
const STATE_RESET: StateBit = 0b10;

/// The state strucuture.
///
/// `N` is the available number of states.
/// You probably not need to change it.
pub struct State {
    bit: StateBit,
    expired_after: [Option<std::time::Duration>; N],
    triggered_at: [Option<std::time::Instant>; N],
}

impl Default for State {
    fn default() -> Self {
        Self {
            bit: STATE_NOTHING,
            expired_after: [None; N],
            triggered_at: [None; N],
        }
    }
}

macro_rules! register_state {
    ($state_name:ident, $state:expr) => {
        register_state!($state_name, $state, 0);
    };

    ($state_name:ident, $state:expr, $dur_sec:expr) => {
        ::paste::paste! {
            impl State {
                #[doc = concat!("Set the state to `", stringify!($state_name), "`.")]
                #[allow(unused)]
                pub fn $state_name(&mut self) {
                    self.bit |= $state;

                    register_state!(@timeout $state, $dur_sec, self);
                }

                #[doc = concat!("Is the state `", stringify!($state_name), "` (not considering expiration)?")]
                #[allow(clippy::bad_bit_mask)]
                pub const fn [< is_currently_ $state_name >](&self) -> bool {
                    self.bit & $state != 0
                }

                #[doc = concat!("Is the state `", stringify!($state_name), "`?")]
                #[allow(unused)]
                pub fn [< is_ $state_name >](&mut self) -> bool {
                    // Check the expiration and reset the state if it is expired.
                    self.scan_expiration_and_reset();
                    self.[< is_currently_ $state_name >]()
                }
            }
        }
    };

    (@timeout $state:expr, 0, $self:expr) => {
        // no timeout
    };

    (@timeout $state:expr, $dur_sec:expr, $self:expr) => {
        // The triggered_at and expired_after (timer) for
        // every states possibility.
        //
        // For example, the timer of 0b100 (4) state will be
        // stored in the index #4 of the array.
        $self.triggered_at[$state as usize] = Some(std::time::Instant::now());
        $self.expired_after[$state as usize] = Some(std::time::Duration::from_secs($dur_sec));
    };
}

register_state!(nothing, STATE_NOTHING);
register_state!(saved, STATE_SAVED, 3);
register_state!(reset, STATE_RESET, 3);

impl State {
    /// Check all the states and reset the expired state.
    fn scan_expiration_and_reset(&mut self) {
        for i in 0..N {
            if let Some(triggered_at) = self.triggered_at[i] {
                if let Some(expired_after) = self.expired_after[i] {
                    if triggered_at.elapsed() >= expired_after {
                        self.bit ^= i as StateBit;
                        self.triggered_at[i] = None;
                        self.expired_after[i] = None;
                    }
                }
            }
        }
    }
}

impl State {
    /// Get the human-readable text (not considering expiration).
    fn get_currently_human_text(&self) -> &str {
        const STRING_TABLE: &[(StateBit, &str)] = &[
            (
                STATE_SAVED | STATE_RESET,
                "The rank has been saved & reset!",
            ),
            (STATE_SAVED, "The rank has been saved!"),
            (STATE_RESET, "The rank has been reset!"),
            (STATE_NOTHING, ""),
        ];

        for (state, text) in STRING_TABLE.iter() {
            let state = *state;
            if self.bit & state == state {
                return text;
            }
        }

        "Undefined state"
    }

    /// Get the human-readable text.
    pub fn get_human_text(&mut self) -> &str {
        self.scan_expiration_and_reset();
        self.get_currently_human_text()
    }
}
