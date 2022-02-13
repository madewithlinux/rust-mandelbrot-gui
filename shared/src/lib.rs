use std::fmt::Debug;
use std::path::Path;

use abi_stable::library::LibraryError;
use abi_stable::library::RootModule;
use abi_stable::package_version_strings;
use abi_stable::sabi_types::VersionStrings;
use abi_stable::std_types::RArc;
use abi_stable::std_types::RStr;
pub use abi_stable::std_types::{RHashMap, RString, RVec, Tuple2, Tuple3};
use abi_stable::{
    sabi_trait,
    std_types::{RBox, RSlice},
    StableAbi,
};

pub mod raw;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = "FractalLib_Ref")))]
#[sabi(missing_field(panic))]
pub struct FractalLib {
    #[sabi(last_prefix_field)]
    pub default_cell_func_for_size: extern "C" fn(width: u32, height: u32) -> RFractalCellFuncBox,
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
    pub pos: Tuple2<u32, u32>,
    pub iter: f32,
    pub rgb: Tuple3<u8, u8, u8>,
    pub data: RVec<u8>,
}

#[sabi_trait]
pub trait RFractalCellFunc: Clone + Debug + Sync + Send + 'static {
    fn clone_self(&self) -> RFractalCellFuncBox;

    fn get_size(&self) -> Tuple2<u32, u32>;

    // fn compute_cell(&self, pos: Tuple2<u32, u32>) -> RCell;
    fn compute_cells(&self, positions: RSlice<Tuple2<u32, u32>>) -> RVec<RCell>;

    fn with_size(&self, size: Tuple2<u32, u32>) -> RFractalCellFuncBox;
    fn with_offset(&self, offset: Tuple2<i32, i32>) -> RFractalCellFuncBox;
    fn add_zoom(&self, zoom_factor: f64) -> RFractalCellFuncBox;

    fn with_option(&self, name: RStr, value: RStr) -> RFractalCellFuncBox;
    #[sabi(last_prefix_field)]
    fn get_options(&self) -> RHashMap<RString, RString>;
}

pub type RFractalCellFuncBox = RFractalCellFunc_TO<RBox<()>>;
pub type RFractalCellFuncArc = RFractalCellFunc_TO<RArc<()>>;
