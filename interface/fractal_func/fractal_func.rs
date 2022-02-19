use std::fmt::Debug;
use std::path::Path;

use abi_stable::library::LibraryError;
use abi_stable::library::RootModule;
use abi_stable::package_version_strings;
use abi_stable::sabi_types::VersionStrings;
use abi_stable::std_types::RArc;
use abi_stable::std_types::RStr;
use abi_stable::std_types::{RHashMap, RResult, RString, RVec, Tuple2};
use abi_stable::{
    sabi_trait,
    std_types::{RBox, RSlice},
    StableAbi,
};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = "FractalLib_Ref")))]
#[sabi(missing_field(panic))]
pub struct FractalLib {
    #[sabi(last_prefix_field)]
    pub default_fractal_func_for_size: extern "C" fn(width: u32, height: u32) -> RFractalFuncBox,
}

/// The RootModule trait defines how to load the root module of a library.
impl RootModule for FractalLib_Ref {
    abi_stable::declare_root_module_statics! {FractalLib_Ref}
    const BASE_NAME: &'static str = "example_library";
    const NAME: &'static str = "example_library";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_root_module_in_directory(directory: &Path) -> Result<FractalLib_Ref, LibraryError> {
    FractalLib_Ref::load_from_directory(directory)
}

#[repr(C)]
#[derive(Debug, Clone, StableAbi)]
pub struct RCell {
    pub pos: [u32; 2],
    pub data: RVec<u8>,
}

pub type ROptionsMap = RHashMap<RString, RString>;

#[sabi_trait]
pub trait RFractalFunc: Clone + Debug + Sync + Send + 'static {
    fn clone_self(&self) -> RFractalFuncBox;

    fn get_size(&self) -> Tuple2<u32, u32>;

    fn compute_cells(&self, positions: RSlice<[u32; 2]>) -> RVec<RCell>;

    fn with_size(&self, width: u32, height: u32) -> RFractalFuncBox;
    fn with_offset(&self, dx: i32, dy: i32) -> RFractalFuncBox;
    fn add_zoom(&self, zoom_factor: f64) -> RFractalFuncBox;

    fn with_option(&self, _name: RStr, _value: RStr) -> RResult<RFractalFuncBox, RString> {
        RResult::RErr(RString::from("unimplemented"))
    }
    #[sabi(last_prefix_field)]
    fn get_options(&self) -> ROptionsMap {
        ROptionsMap::default()
    }
}

pub type RFractalFuncBox = RFractalFunc_TO<RBox<()>>;
pub type RFractalFuncArc = RFractalFunc_TO<RArc<()>>;

pub mod prelude {
    pub use super::RCell;
    pub use super::ROptionsMap;
    pub use super::{FractalLib, FractalLib_Ref};
    pub use super::{RFractalFunc, RFractalFuncArc, RFractalFuncBox};

    pub use abi_stable::std_types::RResult::{RErr, ROk};
    pub use abi_stable::std_types::{
        RHashMap, RResult, RSlice, RStr, RString, RVec, Tuple2, Tuple3,
    };
    pub use abi_stable::{rtry, rtuple, rstr};

    pub use abi_stable::erased_types::{TD_CanDowncast, TD_Opaque};
    pub use abi_stable::library::RootModule;
    pub use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait};
}
