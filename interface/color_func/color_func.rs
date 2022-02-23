use std::fmt::Debug;
use std::path::Path;

use abi_stable::library::LibraryError;
use abi_stable::library::RootModule;
use abi_stable::package_version_strings;
use abi_stable::sabi_types::VersionStrings;
use abi_stable::std_types::RArc;
use abi_stable::std_types::RStr;
use abi_stable::std_types::{RHashMap, RResult, RString, RVec};
use abi_stable::{
    sabi_trait,
    std_types::{RBox, RSlice},
    StableAbi,
};

pub use fractal_func::RCell;
pub use fractal_func::RChunk;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = "ColorLib_Ref")))]
#[sabi(missing_field(panic))]
pub struct ColorLib {
    #[sabi(last_prefix_field)]
    pub default_color_func: extern "C" fn() -> RColorFuncBox,
}

/// The RootModule trait defines how to load the root module of a library.
impl RootModule for ColorLib_Ref {
    abi_stable::declare_root_module_statics! {ColorLib_Ref}
    const BASE_NAME: &'static str = "example_library";
    const NAME: &'static str = "example_library";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_root_module_in_directory(directory: &Path) -> Result<ColorLib_Ref, LibraryError> {
    ColorLib_Ref::load_from_directory(directory)
}

#[repr(C)]
#[derive(Debug, Clone, StableAbi)]
pub struct RColor {
    pub pos: [u32; 2],
    pub rgb: [u8; 3],
}

pub type ROptionsMap = RHashMap<RString, RString>;

#[sabi_trait]
pub trait RColorFunc: Clone + Debug + Sync + Send + 'static {
    fn compute_colors(&self, chunk: &RChunk) -> RVec<RColor>;

    fn with_option(&self, _name: RStr, _value: RStr) -> RResult<RColorFuncBox, RString> {
        RResult::RErr(RString::from("unimplemented"))
    }
    #[sabi(last_prefix_field)]
    fn get_options(&self) -> ROptionsMap {
        ROptionsMap::default()
    }
}

pub type RColorFuncBox = RColorFunc_TO<RBox<()>>;
pub type RColorFuncArc = RColorFunc_TO<RArc<()>>;

/// re-exports for convenient wildcard-import by users or implementations of this trait
pub mod prelude {
    pub use super::RColor;
    pub use super::ROptionsMap;
    pub use super::{ColorLib, ColorLib_Ref};
    pub use super::{RColorFunc, RColorFuncArc, RColorFuncBox};

    pub use fractal_func::RCell;

    pub use abi_stable::std_types::{
        RHashMap, RResult, RSlice, RStr, RString, RVec, Tuple2, Tuple3,
    };

    pub use abi_stable::erased_types::{TD_CanDowncast, TD_Opaque};
    pub use abi_stable::library::RootModule;
    pub use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait};
}
