#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Options {
    pub cumul_plus: bool,
    pub sept_zero: bool,
    pub jump_in: bool,
    pub piocher_jusqua_jouable: bool,
    pub penalite_uno: bool,
    pub contester_plus4: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            cumul_plus: false,
            sept_zero: false,
            jump_in: false,
            piocher_jusqua_jouable: false,
            penalite_uno: true,
            contester_plus4: true,
        }
    }
}
