#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use abi_stable::std_types::{RHashMap, RString, RVec, Tuple2, Tuple3};
use abi_stable::{rvec, StableAbi};
use cglue::cglue_trait;
use std::collections::HashMap;
use itertools::Itertools;
pub struct Cell {
    pub pos: (u32, u32),
    pub iter: f32,
    pub rgb: (u8, u8, u8),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Cell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Cell {
                pos: ref __self_0_0,
                iter: ref __self_0_1,
                rgb: ref __self_0_2,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Cell");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "pos", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "iter", &&(*__self_0_1));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "rgb", &&(*__self_0_2));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Cell {
    #[inline]
    fn clone(&self) -> Cell {
        {
            let _: ::core::clone::AssertParamIsClone<(u32, u32)>;
            let _: ::core::clone::AssertParamIsClone<f32>;
            let _: ::core::clone::AssertParamIsClone<(u8, u8, u8)>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Cell {}
impl Into<RCell> for Cell {
    fn into(self) -> RCell {
        RCell {
            pos: self.pos.into(),
            iter: self.iter.into(),
            rgb: self.rgb.into(),
            data: ::abi_stable::std_types::RVec::from(::alloc::vec::Vec::new()),
        }
    }
}
pub trait FractalCellFunc: Clone {
    fn default_for_size(width: u32, height: u32) -> Self;
    fn get_size(&self) -> (u32, u32);
    fn compute_cell(&self, pos: (u32, u32)) -> Cell;
    fn compute_cells(&self, positions: &[(u32, u32)]) -> Vec<Cell> {
        positions
            .iter()
            .map(|&pos| self.compute_cell(pos))
            .collect_vec()
    }
    fn with_size(&self, size: (u32, u32)) -> Self;
    fn with_offset(&self, offset: (i32, i32)) -> Self;
    fn add_zoom(&self, zoom_factor: f64) -> Self;
    fn with_option(&self, name: &str, value: &str) -> Self;
    fn get_options(&self) -> HashMap<String, String>;
}
#[repr(C)]
pub struct RCell {
    pub pos: Tuple2<u32, u32>,
    pub iter: f32,
    pub rgb: Tuple3<u8, u8, u8>,
    pub data: RVec<u8>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for RCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            RCell {
                pos: ref __self_0_0,
                iter: ref __self_0_1,
                rgb: ref __self_0_2,
                data: ref __self_0_3,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "RCell");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "pos", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "iter", &&(*__self_0_1));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "rgb", &&(*__self_0_2));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "data", &&(*__self_0_3));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for RCell {
    #[inline]
    fn clone(&self) -> RCell {
        match *self {
            RCell {
                pos: ref __self_0_0,
                iter: ref __self_0_1,
                rgb: ref __self_0_2,
                data: ref __self_0_3,
            } => RCell {
                pos: ::core::clone::Clone::clone(&(*__self_0_0)),
                iter: ::core::clone::Clone::clone(&(*__self_0_1)),
                rgb: ::core::clone::Clone::clone(&(*__self_0_2)),
                data: ::core::clone::Clone::clone(&(*__self_0_3)),
            },
        }
    }
}
const _: () = {
    use :: abi_stable;
    #[allow(unused_imports)]
    use ::abi_stable::pmr::{self as __sabi_re, renamed::*};
    const _item_info_const_RCell: abi_stable::type_layout::ItemInfo =
        ::abi_stable::type_layout::ItemInfo::new(
            "shared;0.1.0",
            50u32,
            ::abi_stable::type_layout::ModPath::inside({
                const __STR_NHPMWYD3NJA: ::abi_stable::sabi_types::NulStr<'_> =
                    ::abi_stable::sabi_types::NulStr::from_str("shared\u{0}");
                __STR_NHPMWYD3NJA
            }),
        );
    const _SHARED_VARS_STRINGS_RCell: ::abi_stable::std_types::RStr<'static> =
        abi_stable::std_types::RStr::from_str("pos;iter;rgb;data;");
    pub struct _static_RCell(extern "C" fn());
    unsafe impl __GetStaticEquivalent_ for RCell {
        type StaticEquivalent = _static_RCell;
    }
    #[doc(hidden)]
    const _MONO_LAYOUT_RCell: &'static __sabi_re::MonoTypeLayout =
        &__sabi_re::MonoTypeLayout::from_derive(__sabi_re::_private_MonoTypeLayoutDerive {
            name: abi_stable::std_types::RStr::from_str("RCell"),
            item_info: _item_info_const_RCell,
            data: __sabi_re::MonoTLData::derive_struct(__CompTLFields::new(
                abi_stable::std_types::RSlice::from_slice(&[
                    196608u64,
                    1125899907104772u64,
                    2251799813881865u64,
                    3377699720790029u64,
                ]),
                None,
            )),
            generics: {
                #[allow(unused_parens)]
                let ty_param_range =
                    ::abi_stable::type_layout::StartLenConverter((__StartLen::new(0u16, 0u16)))
                        .to_start_len();
                #[allow(unused_parens)]
                let const_param_range =
                    ::abi_stable::type_layout::StartLenConverter((__StartLen::new(0u16, 0u16)))
                        .to_start_len();
                ::abi_stable::type_layout::CompGenericParams::new(
                    {
                        const __STR_NHPMWYD3NJA: ::abi_stable::sabi_types::NulStr<'_> =
                            ::abi_stable::sabi_types::NulStr::from_str("\u{0}");
                        __STR_NHPMWYD3NJA
                    },
                    {
                        mod __ {
                            pub const __USIZE_CONST: ::core_extensions::__::usize = 0;
                        }
                        __::__USIZE_CONST
                    } as u8,
                    ty_param_range,
                    const_param_range,
                )
            },
            mod_refl_mode: __ModReflMode::Module,
            repr_attr: __ReprAttr::C,
            phantom_fields: abi_stable::std_types::RSlice::from_slice(&[]),
            shared_vars: abi_stable::type_layout::MonoSharedVars::new(
                _SHARED_VARS_STRINGS_RCell,
                abi_stable::std_types::RSlice::from_slice(&[]),
            ),
        });
    impl RCell {
        const __SABI_CONST_PARAMS_A: &'static [&'static __sabi_re::ConstGenericErasureHack<
            dyn ::std::marker::Send,
        >] = &[];
        const __SABI_CONST_PARAMS_B: &'static [__ConstGeneric] = &[];
        const __SABI_SHARED_VARS: &'static __sabi_re::SharedVars =
            &abi_stable::type_layout::SharedVars::new(
                _MONO_LAYOUT_RCell.shared_vars_static(),
                {
                    ::abi_stable::std_types::RSlice::from_slice(&[
                        { __GetTypeLayoutCtor::<Tuple2<u32, u32>>::STABLE_ABI },
                        { __GetTypeLayoutCtor::<f32>::STABLE_ABI },
                        { __GetTypeLayoutCtor::<Tuple3<u8, u8, u8>>::STABLE_ABI },
                        { __GetTypeLayoutCtor::<RVec<u8>>::STABLE_ABI },
                    ])
                },
                __sabi_re::RSlice::from_slice(Self::__SABI_CONST_PARAMS_B),
            );
    }
    unsafe impl __sabi_re::StableAbi for RCell {
        type IsNonZeroType = __sabi_re::False;
        const LAYOUT: &'static __sabi_re::TypeLayout = {
            &__sabi_re::TypeLayout::from_derive::<Self>(__sabi_re::_private_TypeLayoutDerive {
                shared_vars: Self::__SABI_SHARED_VARS,
                mono: _MONO_LAYOUT_RCell,
                abi_consts: Self::ABI_CONSTS,
                data: __sabi_re::GenericTLData::Struct,
                tag: None,
                extra_checks: None,
            })
        };
    }
};
pub trait RFractalCellFunc: Clone {
    fn get_size(&self) -> Tuple2<u32, u32>;
    fn compute_cell(&self, pos: Tuple2<u32, u32>) -> RCell;
    fn compute_cells(&self, positions: &[Tuple2<u32, u32>]) -> RVec<RCell> {
        positions
            .iter()
            .map(|&pos| self.compute_cell(pos))
            .collect_vec()
            .into()
    }
    fn with_size(&self, size: Tuple2<u32, u32>) -> Self;
    fn with_offset(&self, offset: Tuple2<i32, i32>) -> Self;
    fn add_zoom(&self, zoom_factor: f64) -> Self;
    fn with_option(&self, name: &str, value: &str) -> Self;
    fn get_options(&self) -> RHashMap<RString, RString>;
}
#[doc(hidden)]
pub use cglue_rfractalcellfunc::*;
pub mod cglue_rfractalcellfunc {
    use super::*;
    use super::RFractalCellFunc;
    pub use cglue_internal::{
        RFractalCellFuncVtbl, RFractalCellFuncRetTmp, RFractalCellFuncOpaqueObj,
        RFractalCellFuncBaseBox, RFractalCellFuncBaseCtxBox, RFractalCellFuncBaseArcBox,
        RFractalCellFuncBaseMut, RFractalCellFuncBaseCtxMut, RFractalCellFuncBaseArcMut,
        RFractalCellFuncBaseRef, RFractalCellFuncBaseCtxRef, RFractalCellFuncBaseArcRef,
        RFractalCellFuncBase, RFractalCellFuncBox, RFractalCellFuncCtxBox, RFractalCellFuncArcBox,
        RFractalCellFuncMut, RFractalCellFuncCtxMut, RFractalCellFuncArcMut, RFractalCellFuncRef,
        RFractalCellFuncCtxRef, RFractalCellFuncArcRef,
    };
    mod cglue_internal {
        use super::*;
        use super::RFractalCellFunc;
        /// CGlue vtable for trait RFractalCellFunc.
        ///
        /// This virtual function table contains ABI-safe interface for the given trait.
        #[repr(C)]
        pub struct RFractalCellFuncVtbl<
            'cglue_a,
            CGlueC: 'cglue_a + ::cglue::trait_group::CGlueObjBase,
        > {
            get_size: extern "C" fn(cont: &CGlueC) -> Tuple2<u32, u32>,
            compute_cell: extern "C" fn(cont: &CGlueC, pos: Tuple2<u32, u32>) -> RCell,
            compute_cells: extern "C" fn(
                cont: &CGlueC,
                positions: ::cglue::slice::CSliceRef<Tuple2<u32, u32>>,
            ) -> RVec<RCell>,
            with_size: extern "C" fn(cont: &'cglue_a CGlueC, size: Tuple2<u32, u32>) -> CGlueC,
            with_offset: extern "C" fn(cont: &'cglue_a CGlueC, offset: Tuple2<i32, i32>) -> CGlueC,
            add_zoom: extern "C" fn(cont: &'cglue_a CGlueC, zoom_factor: f64) -> CGlueC,
            with_option: extern "C" fn(
                cont: &'cglue_a CGlueC,
                name: ::cglue::slice::CSliceRef<u8>,
                value: ::cglue::slice::CSliceRef<u8>,
            ) -> CGlueC,
            get_options: extern "C" fn(cont: &CGlueC) -> RHashMap<RString, RString>,
            _lt_cglue_a: ::core::marker::PhantomData<&'cglue_a CGlueC>,
        }
        const _: () = {
            use :: abi_stable;
            #[allow(unused_imports)]
            use ::abi_stable::pmr::{self as __sabi_re, renamed::*};
            const _item_info_const_RFractalCellFuncVtbl: abi_stable::type_layout::ItemInfo =
                ::abi_stable::type_layout::ItemInfo::new(
                    "shared;0.1.0",
                    58u32,
                    ::abi_stable::type_layout::ModPath::inside({
                        const __STR_NHPMWYD3NJA: ::abi_stable::sabi_types::NulStr<'_> =
                            ::abi_stable::sabi_types::NulStr::from_str(
                                "shared::cglue_rfractalcellfunc::cglue_internal\u{0}",
                            );
                        __STR_NHPMWYD3NJA
                    }),
                );
            const _SHARED_VARS_STRINGS_RFractalCellFuncVtbl : :: abi_stable :: std_types :: RStr < 'static > = abi_stable :: std_types :: RStr :: from_str ("get_size;cont,compute_cell;cont,pos,compute_cells;cont,positions,with_size;cont,size,with_offset;cont,offset,add_zoom;cont,zoom_factor,with_option;cont,name,value,get_options;cont,_lt_cglue_a;") ;
            pub struct _static_RFractalCellFuncVtbl<'cglue_a, CGlueC: ?Sized>(
                &'cglue_a (),
                extern "C" fn(&CGlueC),
            );
            unsafe impl<'cglue_a, CGlueC: 'cglue_a + ::cglue::trait_group::CGlueObjBase>
                __GetStaticEquivalent_ for RFractalCellFuncVtbl<'cglue_a, CGlueC>
            where
                CGlueC: __StableAbi,
            {
                type StaticEquivalent =
                    _static_RFractalCellFuncVtbl<'static, __GetStaticEquivalent<CGlueC>>;
            }
            #[doc(hidden)]
            const _MONO_LAYOUT_RFractalCellFuncVtbl: &'static __sabi_re::MonoTypeLayout =
                &__sabi_re::MonoTypeLayout::from_derive(__sabi_re::_private_MonoTypeLayoutDerive {
                    name: abi_stable::std_types::RStr::from_str("RFractalCellFuncVtbl"),
                    item_info: _item_info_const_RFractalCellFuncVtbl,
                    data: __sabi_re::MonoTLData::derive_struct(__CompTLFields::new(
                        abi_stable::std_types::RSlice::from_slice(&[
                            1153484454560792576u64,
                            1153484454561054734u64,
                            1153484454561120292u64,
                            1153484454560858177u64,
                            1153484454560989269u64,
                            1153484454560792685u64,
                            1153484454560989319u64,
                            1153484454560989347u64,
                            14073749037580468u64,
                        ]),
                        Some(&{
                            const TLF_A: &[__CompTLFunction] = &[
                                __CompTLFunction::new(
                                    524288u32, 9u16, 0u16, 5u16, 2u16, 1u32, 1025u64,
                                ),
                                __CompTLFunction::new(
                                    786446u32, 27u16, 0u16, 9u16, 3u16, 1u32, 2098178u64,
                                ),
                                __CompTLFunction::new(
                                    852004u32, 50u16, 0u16, 15u16, 5u16, 1u32, 4195330u64,
                                ),
                                __CompTLFunction::new(
                                    589889u32, 75u16, 0u16, 10u16, 7u16, 3u32, 2103298u64,
                                ),
                                __CompTLFunction::new(
                                    720981u32, 97u16, 0u16, 12u16, 7u16, 3u32, 8394754u64,
                                ),
                                __CompTLFunction::new(
                                    524397u32, 118u16, 0u16, 17u16, 7u16, 3u32, 9443330u64,
                                ),
                                __CompTLFunction::new(
                                    721031u32,
                                    147u16,
                                    0u16,
                                    16u16,
                                    7u16,
                                    3u32,
                                    42960164867u64,
                                ),
                                __CompTLFunction::new(
                                    721059u32, 175u16, 0u16, 5u16, 11u16, 1u32, 1025u64,
                                ),
                            ];
                            const TLF_B: __TLFunctions = __TLFunctions::new(
                                __sabi_re::RSlice::from_slice(TLF_A),
                                ::abi_stable::std_types::RSlice::from_slice(&[
                                    65536u32, 65537u32, 65538u32, 65539u32, 65540u32, 65541u32,
                                    65542u32, 65543u32, 8u32,
                                ]),
                            );
                            TLF_B
                        }),
                    )),
                    generics: {
                        #[allow(unused_parens)]
                        let ty_param_range = ::abi_stable::type_layout::StartLenConverter(
                            (__StartLen::new(0u16, 0u16)),
                        )
                        .to_start_len();
                        #[allow(unused_parens)]
                        let const_param_range = ::abi_stable::type_layout::StartLenConverter(
                            (__StartLen::new(0u16, 0u16)),
                        )
                        .to_start_len();
                        ::abi_stable::type_layout::CompGenericParams::new(
                            {
                                const __STR_NHPMWYD3NJA: ::abi_stable::sabi_types::NulStr<'_> =
                                    ::abi_stable::sabi_types::NulStr::from_str("\'cglue_a,\u{0}");
                                __STR_NHPMWYD3NJA
                            },
                            {
                                mod __ {
                                    pub const __USIZE_CONST: ::core_extensions::__::usize = 1;
                                }
                                __::__USIZE_CONST
                            } as u8,
                            ty_param_range,
                            const_param_range,
                        )
                    },
                    mod_refl_mode: __ModReflMode::Opaque,
                    repr_attr: __ReprAttr::C,
                    phantom_fields: abi_stable::std_types::RSlice::from_slice(&[]),
                    shared_vars: abi_stable::type_layout::MonoSharedVars::new(
                        _SHARED_VARS_STRINGS_RFractalCellFuncVtbl,
                        abi_stable::std_types::RSlice::from_slice(&[]),
                    ),
                });
            impl<'cglue_a, CGlueC: 'cglue_a + ::cglue::trait_group::CGlueObjBase>
                RFractalCellFuncVtbl<'cglue_a, CGlueC>
            where
                CGlueC: __StableAbi,
            {
                const __SABI_CONST_PARAMS_A:
                    &'static [&'static __sabi_re::ConstGenericErasureHack<
                        dyn ::std::marker::Send,
                    >] = &[];
                const __SABI_CONST_PARAMS_B: &'static [__ConstGeneric] = &[];
                const __SABI_SHARED_VARS: &'static __sabi_re::SharedVars =
                    &abi_stable::type_layout::SharedVars::new(
                        _MONO_LAYOUT_RFractalCellFuncVtbl.shared_vars_static(),
                        {
                            ::abi_stable::std_types::RSlice::from_slice(&[
                                { __GetTypeLayoutCtor::<extern "C" fn()>::STABLE_ABI },
                                { __GetTypeLayoutCtor::<&CGlueC>::STABLE_ABI },
                                { __GetTypeLayoutCtor::<Tuple2<u32, u32>>::STABLE_ABI },
                                { __GetTypeLayoutCtor::<RCell>::STABLE_ABI },
                                {
                                    __GetTypeLayoutCtor :: < :: cglue :: slice :: CSliceRef < Tuple2 < u32 , u32 > > > :: STABLE_ABI
                                },
                                { __GetTypeLayoutCtor::<RVec<RCell>>::STABLE_ABI },
                                { __GetTypeLayoutCtor::<&'_ CGlueC>::STABLE_ABI },
                                { __GetTypeLayoutCtor::<CGlueC>::STABLE_ABI },
                                { __GetTypeLayoutCtor::<Tuple2<i32, i32>>::STABLE_ABI },
                                { __GetTypeLayoutCtor::<f64>::STABLE_ABI },
                                {
                                    __GetTypeLayoutCtor::<::cglue::slice::CSliceRef<u8>>::STABLE_ABI
                                },
                                { __GetTypeLayoutCtor::<RHashMap<RString, RString>>::STABLE_ABI },
                                {
                                    __GetTypeLayoutCtor::<
                                        ::core::marker::PhantomData<&'cglue_a CGlueC>,
                                    >::STABLE_ABI
                                },
                            ])
                        },
                        __sabi_re::RSlice::from_slice(Self::__SABI_CONST_PARAMS_B),
                    );
            }
            unsafe impl<'cglue_a, CGlueC: 'cglue_a + ::cglue::trait_group::CGlueObjBase>
                __sabi_re::StableAbi for RFractalCellFuncVtbl<'cglue_a, CGlueC>
            where
                CGlueC: __StableAbi,
            {
                type IsNonZeroType = __sabi_re::False;
                const LAYOUT: &'static __sabi_re::TypeLayout = {
                    &__sabi_re::TypeLayout::from_derive::<Self>(
                        __sabi_re::_private_TypeLayoutDerive {
                            shared_vars: Self::__SABI_SHARED_VARS,
                            mono: _MONO_LAYOUT_RFractalCellFuncVtbl,
                            abi_consts: Self::ABI_CONSTS,
                            data: __sabi_re::GenericTLData::Struct,
                            tag: None,
                            extra_checks: None,
                        },
                    )
                };
            }
        };
        impl<'cglue_a, CGlueC: ::cglue::trait_group::CGlueObjBase> RFractalCellFuncVtbl<'cglue_a, CGlueC> {
            /// Getter for get_size.
            ///
            /// Note that this function is wrapped into unsafe, because if already were is an
            /// opaque one, it would allow to invoke undefined behaviour.
            pub fn get_size(&self) -> unsafe extern "C" fn(cont: &CGlueC) -> Tuple2<u32, u32> {
                unsafe { ::core::mem::transmute(self.get_size) }
            }
            /// Getter for compute_cell.
            ///
            /// Note that this function is wrapped into unsafe, because if already were is an
            /// opaque one, it would allow to invoke undefined behaviour.
            pub fn compute_cell(
                &self,
            ) -> unsafe extern "C" fn(cont: &CGlueC, pos: Tuple2<u32, u32>) -> RCell {
                unsafe { ::core::mem::transmute(self.compute_cell) }
            }
            /// Getter for compute_cells.
            ///
            /// Note that this function is wrapped into unsafe, because if already were is an
            /// opaque one, it would allow to invoke undefined behaviour.
            pub fn compute_cells(
                &self,
            ) -> unsafe extern "C" fn(
                cont: &CGlueC,
                positions: ::cglue::slice::CSliceRef<Tuple2<u32, u32>>,
            ) -> RVec<RCell> {
                unsafe { ::core::mem::transmute(self.compute_cells) }
            }
            /// Getter for with_size.
            ///
            /// Note that this function is wrapped into unsafe, because if already were is an
            /// opaque one, it would allow to invoke undefined behaviour.
            pub fn with_size(
                &self,
            ) -> unsafe extern "C" fn(cont: &'cglue_a CGlueC, size: Tuple2<u32, u32>) -> CGlueC
            {
                unsafe { ::core::mem::transmute(self.with_size) }
            }
            /// Getter for with_size.
            ///
            /// This function has its argument lifetime cast so that it's usable with anonymous
            /// lifetime functions.
            ///
            /// # Safety
            ///
            /// This ought to only be used when references to objects are being returned,
            /// otherwise there is a risk of lifetime rule breakage.
            unsafe fn with_size_lifetimed(
                &self,
            ) -> for<'cglue_b> extern "C" fn(cont: &'cglue_b CGlueC, size: Tuple2<u32, u32>) -> CGlueC
            {
                ::core::mem::transmute(self.with_size)
            }
            /// Getter for with_offset.
            ///
            /// Note that this function is wrapped into unsafe, because if already were is an
            /// opaque one, it would allow to invoke undefined behaviour.
            pub fn with_offset(
                &self,
            ) -> unsafe extern "C" fn(cont: &'cglue_a CGlueC, offset: Tuple2<i32, i32>) -> CGlueC
            {
                unsafe { ::core::mem::transmute(self.with_offset) }
            }
            /// Getter for with_offset.
            ///
            /// This function has its argument lifetime cast so that it's usable with anonymous
            /// lifetime functions.
            ///
            /// # Safety
            ///
            /// This ought to only be used when references to objects are being returned,
            /// otherwise there is a risk of lifetime rule breakage.
            unsafe fn with_offset_lifetimed(
                &self,
            ) -> for<'cglue_b> extern "C" fn(
                cont: &'cglue_b CGlueC,
                offset: Tuple2<i32, i32>,
            ) -> CGlueC {
                ::core::mem::transmute(self.with_offset)
            }
            /// Getter for add_zoom.
            ///
            /// Note that this function is wrapped into unsafe, because if already were is an
            /// opaque one, it would allow to invoke undefined behaviour.
            pub fn add_zoom(
                &self,
            ) -> unsafe extern "C" fn(cont: &'cglue_a CGlueC, zoom_factor: f64) -> CGlueC
            {
                unsafe { ::core::mem::transmute(self.add_zoom) }
            }
            /// Getter for add_zoom.
            ///
            /// This function has its argument lifetime cast so that it's usable with anonymous
            /// lifetime functions.
            ///
            /// # Safety
            ///
            /// This ought to only be used when references to objects are being returned,
            /// otherwise there is a risk of lifetime rule breakage.
            unsafe fn add_zoom_lifetimed(
                &self,
            ) -> for<'cglue_b> extern "C" fn(cont: &'cglue_b CGlueC, zoom_factor: f64) -> CGlueC
            {
                ::core::mem::transmute(self.add_zoom)
            }
            /// Getter for with_option.
            ///
            /// Note that this function is wrapped into unsafe, because if already were is an
            /// opaque one, it would allow to invoke undefined behaviour.
            pub fn with_option(
                &self,
            ) -> unsafe extern "C" fn(
                cont: &'cglue_a CGlueC,
                name: ::cglue::slice::CSliceRef<u8>,
                value: ::cglue::slice::CSliceRef<u8>,
            ) -> CGlueC {
                unsafe { ::core::mem::transmute(self.with_option) }
            }
            /// Getter for with_option.
            ///
            /// This function has its argument lifetime cast so that it's usable with anonymous
            /// lifetime functions.
            ///
            /// # Safety
            ///
            /// This ought to only be used when references to objects are being returned,
            /// otherwise there is a risk of lifetime rule breakage.
            unsafe fn with_option_lifetimed(
                &self,
            ) -> for<'cglue_b> extern "C" fn(
                cont: &'cglue_b CGlueC,
                name: ::cglue::slice::CSliceRef<u8>,
                value: ::cglue::slice::CSliceRef<u8>,
            ) -> CGlueC {
                ::core::mem::transmute(self.with_option)
            }
            /// Getter for get_options.
            ///
            /// Note that this function is wrapped into unsafe, because if already were is an
            /// opaque one, it would allow to invoke undefined behaviour.
            pub fn get_options(
                &self,
            ) -> unsafe extern "C" fn(cont: &CGlueC) -> RHashMap<RString, RString> {
                unsafe { ::core::mem::transmute(self.get_options) }
            }
        }
        /// Technically unused phantom data definition structure.
        #[repr(C)]
        pub struct RFractalCellFuncRetTmpPhantom<CGlueCtx: ::cglue::trait_group::ContextBounds> {
            _ty_cglue_ctx: ::core::marker::PhantomData<CGlueCtx>,
        }
        const _: () = {
            use :: abi_stable;
            #[allow(unused_imports)]
            use ::abi_stable::pmr::{self as __sabi_re, renamed::*};
            const _item_info_const_RFractalCellFuncRetTmpPhantom:
                abi_stable::type_layout::ItemInfo = ::abi_stable::type_layout::ItemInfo::new(
                "shared;0.1.0",
                58u32,
                ::abi_stable::type_layout::ModPath::inside({
                    const __STR_NHPMWYD3NJA: ::abi_stable::sabi_types::NulStr<'_> =
                        ::abi_stable::sabi_types::NulStr::from_str(
                            "shared::cglue_rfractalcellfunc::cglue_internal\u{0}",
                        );
                    __STR_NHPMWYD3NJA
                }),
            );
            const _SHARED_VARS_STRINGS_RFractalCellFuncRetTmpPhantom:
                ::abi_stable::std_types::RStr<'static> =
                abi_stable::std_types::RStr::from_str("_ty_cglue_ctx;");
            pub struct _static_RFractalCellFuncRetTmpPhantom<CGlueCtx: ?Sized>(
                extern "C" fn(&CGlueCtx),
            );
            unsafe impl<CGlueCtx: ::cglue::trait_group::ContextBounds> __GetStaticEquivalent_
                for RFractalCellFuncRetTmpPhantom<CGlueCtx>
            where
                CGlueCtx: __StableAbi,
            {
                type StaticEquivalent =
                    _static_RFractalCellFuncRetTmpPhantom<__GetStaticEquivalent<CGlueCtx>>;
            }
            #[doc(hidden)]
            const _MONO_LAYOUT_RFractalCellFuncRetTmpPhantom: &'static __sabi_re::MonoTypeLayout =
                &__sabi_re::MonoTypeLayout::from_derive(__sabi_re::_private_MonoTypeLayoutDerive {
                    name: abi_stable::std_types::RStr::from_str("RFractalCellFuncRetTmpPhantom"),
                    item_info: _item_info_const_RFractalCellFuncRetTmpPhantom,
                    data: __sabi_re::MonoTLData::derive_struct(__CompTLFields::new(
                        abi_stable::std_types::RSlice::from_slice(&[562949954273280u64]),
                        None,
                    )),
                    generics: {
                        #[allow(unused_parens)]
                        let ty_param_range = ::abi_stable::type_layout::StartLenConverter(
                            (__StartLen::new(0u16, 0u16)),
                        )
                        .to_start_len();
                        #[allow(unused_parens)]
                        let const_param_range = ::abi_stable::type_layout::StartLenConverter(
                            (__StartLen::new(0u16, 0u16)),
                        )
                        .to_start_len();
                        ::abi_stable::type_layout::CompGenericParams::new(
                            {
                                const __STR_NHPMWYD3NJA: ::abi_stable::sabi_types::NulStr<'_> =
                                    ::abi_stable::sabi_types::NulStr::from_str("\u{0}");
                                __STR_NHPMWYD3NJA
                            },
                            {
                                mod __ {
                                    pub const __USIZE_CONST: ::core_extensions::__::usize = 0;
                                }
                                __::__USIZE_CONST
                            } as u8,
                            ty_param_range,
                            const_param_range,
                        )
                    },
                    mod_refl_mode: __ModReflMode::Opaque,
                    repr_attr: __ReprAttr::C,
                    phantom_fields: abi_stable::std_types::RSlice::from_slice(&[]),
                    shared_vars: abi_stable::type_layout::MonoSharedVars::new(
                        _SHARED_VARS_STRINGS_RFractalCellFuncRetTmpPhantom,
                        abi_stable::std_types::RSlice::from_slice(&[]),
                    ),
                });
            impl<CGlueCtx: ::cglue::trait_group::ContextBounds> RFractalCellFuncRetTmpPhantom<CGlueCtx>
            where
                CGlueCtx: __StableAbi,
            {
                const __SABI_CONST_PARAMS_A:
                    &'static [&'static __sabi_re::ConstGenericErasureHack<
                        dyn ::std::marker::Send,
                    >] = &[];
                const __SABI_CONST_PARAMS_B: &'static [__ConstGeneric] = &[];
                const __SABI_SHARED_VARS: &'static __sabi_re::SharedVars =
                    &abi_stable::type_layout::SharedVars::new(
                        _MONO_LAYOUT_RFractalCellFuncRetTmpPhantom.shared_vars_static(),
                        {
                            ::abi_stable::std_types::RSlice::from_slice(&[{
                                __GetTypeLayoutCtor :: < :: core :: marker :: PhantomData < CGlueCtx > > :: STABLE_ABI
                            }])
                        },
                        __sabi_re::RSlice::from_slice(Self::__SABI_CONST_PARAMS_B),
                    );
            }
            unsafe impl<CGlueCtx: ::cglue::trait_group::ContextBounds> __sabi_re::StableAbi
                for RFractalCellFuncRetTmpPhantom<CGlueCtx>
            where
                CGlueCtx: __StableAbi,
            {
                type IsNonZeroType = __sabi_re::False;
                const LAYOUT: &'static __sabi_re::TypeLayout = {
                    &__sabi_re::TypeLayout::from_derive::<Self>(
                        __sabi_re::_private_TypeLayoutDerive {
                            shared_vars: Self::__SABI_SHARED_VARS,
                            mono: _MONO_LAYOUT_RFractalCellFuncRetTmpPhantom,
                            abi_consts: Self::ABI_CONSTS,
                            data: __sabi_re::GenericTLData::Struct,
                            tag: None,
                            extra_checks: None,
                        },
                    )
                };
            }
        };
        /// Type definition for temporary return value wrapping storage.
        ///
        /// The trait does not use return wrapping, thus is a typedef to `PhantomData`.
        ///
        /// Note that `cbindgen` will generate wrong structures for this type. It is important
        /// to go inside the generated headers and fix it - all RetTmp structures without a
        /// body should be completely deleted, both as types, and as fields in the
        /// groups/objects. If C++11 templates are generated, it is important to define a
        /// custom type for CGlueTraitObj that does not have `ret_tmp` defined, and change all
        /// type aliases of this trait to use that particular structure.
        pub type RFractalCellFuncRetTmp<CGlueCtx> =
            ::core::marker::PhantomData<RFractalCellFuncRetTmpPhantom<CGlueCtx>>;
        /// Default vtable reference creation.
        impl<
                'cglue_a,
                CGlueC: ::cglue::trait_group::CGlueObjRef<
                        RFractalCellFuncRetTmp<CGlueCtx>,
                        Context = CGlueCtx,
                    > + 'cglue_a,
                CGlueCtx: ::cglue::trait_group::ContextBounds,
            > Default for &'cglue_a RFractalCellFuncVtbl<'cglue_a, CGlueC>
        where
            (CGlueC::ObjType, CGlueCtx): Into<CGlueC>,
            (CGlueC::ObjType, CGlueCtx): Into<CGlueC>,
            (CGlueC::ObjType, CGlueCtx): Into<CGlueC>,
            (CGlueC::ObjType, CGlueCtx): Into<CGlueC>,
            CGlueC::ObjType: RFractalCellFunc,
            CGlueC: ::cglue::trait_group::Opaquable,
            CGlueC::OpaqueTarget: ::cglue::trait_group::GenericTypeBounds,
            RFractalCellFuncVtbl<'cglue_a, CGlueC>: ::cglue::trait_group::CGlueBaseVtbl,
        {
            /// Create a static vtable for the given type.
            fn default() -> Self {
                &RFractalCellFuncVtbl {
                    get_size: cglue_wrapped_get_size,
                    compute_cell: cglue_wrapped_compute_cell,
                    compute_cells: cglue_wrapped_compute_cells,
                    with_size: cglue_wrapped_with_size,
                    with_offset: cglue_wrapped_with_offset,
                    add_zoom: cglue_wrapped_add_zoom,
                    with_option: cglue_wrapped_with_option,
                    get_options: cglue_wrapped_get_options,
                    _lt_cglue_a: ::core::marker::PhantomData,
                }
            }
        }
        impl<'cglue_a, CGlueC: ::cglue::trait_group::CGlueObjBase>
            ::cglue::trait_group::CGlueVtblCont for RFractalCellFuncVtbl<'cglue_a, CGlueC>
        {
            type ContType = CGlueC;
        }
        unsafe impl<
                'cglue_a,
                CGlueC: ::cglue::trait_group::Opaquable + ::cglue::trait_group::CGlueObjBase + 'cglue_a,
            > ::cglue::trait_group::CGlueBaseVtbl for RFractalCellFuncVtbl<'cglue_a, CGlueC>
        where
            CGlueC::OpaqueTarget:
                ::cglue::trait_group::Opaquable + ::cglue::trait_group::CGlueObjBase,
            CGlueC::ObjType: RFractalCellFunc,
            CGlueC::OpaqueTarget: ::cglue::trait_group::GenericTypeBounds,
            RFractalCellFuncVtbl<'cglue_a, CGlueC::OpaqueTarget>: ::abi_stable::StableAbi,
        {
            type OpaqueVtbl = RFractalCellFuncVtbl<'cglue_a, CGlueC::OpaqueTarget>;
            type Context = CGlueC::Context;
            type RetTmp = RFractalCellFuncRetTmp<CGlueC::Context>;
        }
        impl<
                'cglue_a,
                CGlueC: ::cglue::trait_group::CGlueObjRef<
                        RFractalCellFuncRetTmp<CGlueCtx>,
                        Context = CGlueCtx,
                    > + 'cglue_a,
                CGlueCtx: ::cglue::trait_group::ContextBounds,
            > ::cglue::trait_group::CGlueVtbl<CGlueC> for RFractalCellFuncVtbl<'cglue_a, CGlueC>
        where
            CGlueC::OpaqueTarget:
                ::cglue::trait_group::Opaquable + ::cglue::trait_group::CGlueObjBase,
            CGlueC: ::cglue::trait_group::Opaquable,
            CGlueC::OpaqueTarget: ::cglue::trait_group::GenericTypeBounds,
            CGlueC::ObjType: RFractalCellFunc,
        {
        }
        /// Boxed CGlue trait object for trait RFractalCellFunc.
        pub type RFractalCellFuncBaseBox<'cglue_a, CGlueT> = RFractalCellFuncBase<
            'cglue_a,
            ::cglue::boxed::CBox<'cglue_a, CGlueT>,
            ::cglue::trait_group::NoContext,
        >;
        /// CtxBoxed CGlue trait object for trait RFractalCellFunc with context.
        pub type RFractalCellFuncBaseCtxBox<'cglue_a, CGlueT, CGlueCtx> =
            RFractalCellFuncBase<'cglue_a, ::cglue::boxed::CBox<'cglue_a, CGlueT>, CGlueCtx>;
        /// Boxed CGlue trait object for trait RFractalCellFunc with a [`CArc`](cglue::arc::CArc) reference counted context.
        pub type RFractalCellFuncBaseArcBox<'cglue_a, CGlueT, CGlueC> =
            RFractalCellFuncBaseCtxBox<'cglue_a, CGlueT, ::cglue::arc::CArc<CGlueC>>;
        /// By-mut CGlue trait object for trait RFractalCellFunc.
        pub type RFractalCellFuncBaseMut<'cglue_a, CGlueT> =
            RFractalCellFuncBase<'cglue_a, &'cglue_a mut CGlueT, ::cglue::trait_group::NoContext>;
        /// By-mut CGlue trait object for trait RFractalCellFunc with a context.
        pub type RFractalCellFuncBaseCtxMut<'cglue_a, CGlueT, CGlueCtx> =
            RFractalCellFuncBase<'cglue_a, &'cglue_a mut CGlueT, CGlueCtx>;
        /// By-mut CGlue trait object for trait RFractalCellFunc with a [`CArc`](cglue::arc::CArc) reference counted context.
        pub type RFractalCellFuncBaseArcMut<'cglue_a, CGlueT, CGlueC> =
            RFractalCellFuncBase<'cglue_a, &'cglue_a mut CGlueT, ::cglue::arc::CArc<CGlueC>>;
        /// By-ref CGlue trait object for trait RFractalCellFunc.
        pub type RFractalCellFuncBaseRef<'cglue_a, CGlueT> =
            RFractalCellFuncBase<'cglue_a, &'cglue_a CGlueT, ::cglue::trait_group::NoContext>;
        /// By-ref CGlue trait object for trait RFractalCellFunc with a context.
        pub type RFractalCellFuncBaseCtxRef<'cglue_a, CGlueT, CGlueCtx> =
            RFractalCellFuncBase<'cglue_a, &'cglue_a CGlueT, CGlueCtx>;
        /// By-ref CGlue trait object for trait RFractalCellFunc with a [`CArc`](cglue::arc::CArc) reference counted context.
        pub type RFractalCellFuncBaseArcRef<'cglue_a, CGlueT, CGlueC> =
            RFractalCellFuncBase<'cglue_a, &'cglue_a CGlueT, ::cglue::arc::CArc<CGlueC>>;
        /// Base CGlue trait object for trait RFractalCellFunc.
        pub type RFractalCellFuncBase<'cglue_a, CGlueInst, CGlueCtx> =
            ::cglue::trait_group::CGlueTraitObj<
                'cglue_a,
                CGlueInst,
                RFractalCellFuncVtbl<
                    'cglue_a,
                    ::cglue::trait_group::CGlueObjContainer<
                        CGlueInst,
                        CGlueCtx,
                        RFractalCellFuncRetTmp<CGlueCtx>,
                    >,
                >,
                CGlueCtx,
                RFractalCellFuncRetTmp<CGlueCtx>,
            >;
        /// Opaque Boxed CGlue trait object for trait RFractalCellFunc.
        pub type RFractalCellFuncBox<'cglue_a> =
            RFractalCellFuncBaseBox<'cglue_a, ::cglue::trait_group::c_void>;
        /// Opaque CtxBoxed CGlue trait object for trait RFractalCellFunc with a context.
        pub type RFractalCellFuncCtxBox<'cglue_a, CGlueCtx> =
            RFractalCellFuncBaseCtxBox<'cglue_a, ::cglue::trait_group::c_void, CGlueCtx>;
        /// Opaque Boxed CGlue trait object for trait RFractalCellFunc with a [`CArc`](cglue::arc::CArc) reference counted context.
        pub type RFractalCellFuncArcBox<'cglue_a> = RFractalCellFuncBaseArcBox<
            'cglue_a,
            ::cglue::trait_group::c_void,
            ::cglue::trait_group::c_void,
        >;
        /// Opaque by-mut CGlue trait object for trait RFractalCellFunc.
        pub type RFractalCellFuncMut<'cglue_a> =
            RFractalCellFuncBaseMut<'cglue_a, ::cglue::trait_group::c_void>;
        /// Opaque by-mut CGlue trait object for trait RFractalCellFunc with a context.
        pub type RFractalCellFuncCtxMut<'cglue_a, CGlueCtx> =
            RFractalCellFuncBaseCtxMut<'cglue_a, ::cglue::trait_group::c_void, CGlueCtx>;
        /// Opaque by-mut CGlue trait object for trait RFractalCellFunc with a [`CArc`](cglue::arc::CArc) reference counted context.
        pub type RFractalCellFuncArcMut<'cglue_a> = RFractalCellFuncBaseArcMut<
            'cglue_a,
            ::cglue::trait_group::c_void,
            ::cglue::trait_group::c_void,
        >;
        /// Opaque by-ref CGlue trait object for trait RFractalCellFunc.
        pub type RFractalCellFuncRef<'cglue_a> =
            RFractalCellFuncBaseRef<'cglue_a, ::cglue::trait_group::c_void>;
        /// Opaque by-ref CGlue trait object for trait RFractalCellFunc with a context.
        pub type RFractalCellFuncCtxRef<'cglue_a, CGlueCtx> =
            RFractalCellFuncBaseCtxRef<'cglue_a, ::cglue::trait_group::c_void, CGlueCtx>;
        /// Opaque by-ref CGlue trait object for trait RFractalCellFunc with a [`CArc`](cglue::arc::CArc) reference counted context.
        pub type RFractalCellFuncArcRef<'cglue_a> = RFractalCellFuncBaseArcRef<
            'cglue_a,
            ::cglue::trait_group::c_void,
            ::cglue::trait_group::c_void,
        >;
        extern "C" fn cglue_wrapped_get_size<
            CGlueC: ::cglue::trait_group::CGlueObjRef<RFractalCellFuncRetTmp<CGlueCtx>, Context = CGlueCtx>,
            CGlueCtx: ::cglue::trait_group::ContextBounds,
        >(
            cont: &CGlueC,
        ) -> Tuple2<u32, u32>
        where
            CGlueC::ObjType: for<'cglue_b> RFractalCellFunc,
        {
            let (this, ret_tmp, cglue_ctx) = cont.cobj_ref();
            let ret = this.get_size();
            ret
        }
        extern "C" fn cglue_wrapped_compute_cell<
            CGlueC: ::cglue::trait_group::CGlueObjRef<RFractalCellFuncRetTmp<CGlueCtx>, Context = CGlueCtx>,
            CGlueCtx: ::cglue::trait_group::ContextBounds,
        >(
            cont: &CGlueC,
            pos: Tuple2<u32, u32>,
        ) -> RCell
        where
            CGlueC::ObjType: for<'cglue_b> RFractalCellFunc,
        {
            let (this, ret_tmp, cglue_ctx) = cont.cobj_ref();
            let ret = this.compute_cell(pos);
            ret
        }
        extern "C" fn cglue_wrapped_compute_cells<
            CGlueC: ::cglue::trait_group::CGlueObjRef<RFractalCellFuncRetTmp<CGlueCtx>, Context = CGlueCtx>,
            CGlueCtx: ::cglue::trait_group::ContextBounds,
        >(
            cont: &CGlueC,
            positions: ::cglue::slice::CSliceRef<Tuple2<u32, u32>>,
        ) -> RVec<RCell>
        where
            CGlueC::ObjType: for<'cglue_b> RFractalCellFunc,
        {
            let (this, ret_tmp, cglue_ctx) = cont.cobj_ref();
            let ret = this.compute_cells(positions.into());
            ret
        }
        extern "C" fn cglue_wrapped_with_size<
            'cglue_a,
            CGlueC: ::cglue::trait_group::CGlueObjRef<RFractalCellFuncRetTmp<CGlueCtx>, Context = CGlueCtx>,
            CGlueCtx: ::cglue::trait_group::ContextBounds,
        >(
            cont: &'cglue_a CGlueC,
            size: Tuple2<u32, u32>,
        ) -> CGlueC
        where
            (CGlueC::ObjType, CGlueCtx): Into<CGlueC>,
            CGlueC::ObjType: for<'cglue_b> RFractalCellFunc,
        {
            let (this, ret_tmp, cglue_ctx) = cont.cobj_ref();
            let cglue_ctx = cglue_ctx.clone();
            let ret = this.with_size(size);
            let mut conv = |ret| {
                use ::cglue::from2::From2;
                CGlueC::from2((ret, cglue_ctx))
            };
            conv(ret)
        }
        extern "C" fn cglue_wrapped_with_offset<
            'cglue_a,
            CGlueC: ::cglue::trait_group::CGlueObjRef<RFractalCellFuncRetTmp<CGlueCtx>, Context = CGlueCtx>,
            CGlueCtx: ::cglue::trait_group::ContextBounds,
        >(
            cont: &'cglue_a CGlueC,
            offset: Tuple2<i32, i32>,
        ) -> CGlueC
        where
            (CGlueC::ObjType, CGlueCtx): Into<CGlueC>,
            CGlueC::ObjType: for<'cglue_b> RFractalCellFunc,
        {
            let (this, ret_tmp, cglue_ctx) = cont.cobj_ref();
            let cglue_ctx = cglue_ctx.clone();
            let ret = this.with_offset(offset);
            let mut conv = |ret| {
                use ::cglue::from2::From2;
                CGlueC::from2((ret, cglue_ctx))
            };
            conv(ret)
        }
        extern "C" fn cglue_wrapped_add_zoom<
            'cglue_a,
            CGlueC: ::cglue::trait_group::CGlueObjRef<RFractalCellFuncRetTmp<CGlueCtx>, Context = CGlueCtx>,
            CGlueCtx: ::cglue::trait_group::ContextBounds,
        >(
            cont: &'cglue_a CGlueC,
            zoom_factor: f64,
        ) -> CGlueC
        where
            (CGlueC::ObjType, CGlueCtx): Into<CGlueC>,
            CGlueC::ObjType: for<'cglue_b> RFractalCellFunc,
        {
            let (this, ret_tmp, cglue_ctx) = cont.cobj_ref();
            let cglue_ctx = cglue_ctx.clone();
            let ret = this.add_zoom(zoom_factor);
            let mut conv = |ret| {
                use ::cglue::from2::From2;
                CGlueC::from2((ret, cglue_ctx))
            };
            conv(ret)
        }
        extern "C" fn cglue_wrapped_with_option<
            'cglue_a,
            CGlueC: ::cglue::trait_group::CGlueObjRef<RFractalCellFuncRetTmp<CGlueCtx>, Context = CGlueCtx>,
            CGlueCtx: ::cglue::trait_group::ContextBounds,
        >(
            cont: &'cglue_a CGlueC,
            name: ::cglue::slice::CSliceRef<u8>,
            value: ::cglue::slice::CSliceRef<u8>,
        ) -> CGlueC
        where
            (CGlueC::ObjType, CGlueCtx): Into<CGlueC>,
            CGlueC::ObjType: for<'cglue_b> RFractalCellFunc,
        {
            let (this, ret_tmp, cglue_ctx) = cont.cobj_ref();
            let cglue_ctx = cglue_ctx.clone();
            let ret = this.with_option(unsafe { name.into_str() }, unsafe { value.into_str() });
            let mut conv = |ret| {
                use ::cglue::from2::From2;
                CGlueC::from2((ret, cglue_ctx))
            };
            conv(ret)
        }
        extern "C" fn cglue_wrapped_get_options<
            CGlueC: ::cglue::trait_group::CGlueObjRef<RFractalCellFuncRetTmp<CGlueCtx>, Context = CGlueCtx>,
            CGlueCtx: ::cglue::trait_group::ContextBounds,
        >(
            cont: &CGlueC,
        ) -> RHashMap<RString, RString>
        where
            CGlueC::ObjType: for<'cglue_b> RFractalCellFunc,
        {
            let (this, ret_tmp, cglue_ctx) = cont.cobj_ref();
            let ret = this.get_options();
            ret
        }
        pub trait RFractalCellFuncOpaqueObj<'cglue_a>:
            'cglue_a
            + ::cglue::trait_group::GetContainer
            + ::cglue::trait_group::GetVtbl<
                RFractalCellFuncVtbl<
                    'cglue_a,
                    <Self as ::cglue::trait_group::GetContainer>::ContType,
                >,
            >
            + Clone
        {
            type RFractalCellFuncVtbl: ::cglue::trait_group::CGlueVtblCont<
                    ContType = <Self as ::cglue::trait_group::GetContainer>::ContType,
                > + ::abi_stable::StableAbi;
        }
        impl<
                'cglue_a,
                CGlueO: 'cglue_a
                    + ::cglue::trait_group::GetContainer
                    + ::cglue::trait_group::GetVtbl<
                        RFractalCellFuncVtbl<
                            'cglue_a,
                            <Self as ::cglue::trait_group::GetContainer>::ContType,
                        >,
                    >
                    + Clone,
            > RFractalCellFuncOpaqueObj<'cglue_a> for CGlueO
        where
            <CGlueO as ::cglue::trait_group::GetContainer>::ContType: ::abi_stable::StableAbi,
        {
            type RFractalCellFuncVtbl = RFractalCellFuncVtbl<
                'cglue_a,
                <Self as ::cglue::trait_group::GetContainer>::ContType,
            >;
        }
        impl<
                'cglue_a,
                CGlueO: 'cglue_a
                    + ::cglue::trait_group::GetContainer
                    + ::cglue::trait_group::GetVtbl<
                        RFractalCellFuncVtbl<
                            'cglue_a,
                            <Self as ::cglue::trait_group::GetContainer>::ContType,
                        >,
                    >
                    + Clone
                    + RFractalCellFuncOpaqueObj<'cglue_a>,
            > RFractalCellFunc for CGlueO
        where
            RFractalCellFuncVtbl<'cglue_a, <Self as ::cglue::trait_group::GetContainer>::ContType>:
                ::abi_stable::StableAbi,
        {
            #[inline(always)]
            fn get_size(&self) -> Tuple2<u32, u32> {
                let __cglue_vfunc = self.get_vtbl().get_size;
                let cont = self.ccont_ref();
                let mut ret = __cglue_vfunc(cont);
                ret
            }
            #[inline(always)]
            fn compute_cell(&self, pos: Tuple2<u32, u32>) -> RCell {
                let __cglue_vfunc = self.get_vtbl().compute_cell;
                let cont = self.ccont_ref();
                let pos = pos;
                let mut ret = __cglue_vfunc(cont, pos);
                ret
            }
            #[inline(always)]
            fn compute_cells(&self, positions: &[Tuple2<u32, u32>]) -> RVec<RCell> {
                let __cglue_vfunc = self.get_vtbl().compute_cells;
                let cont = self.ccont_ref();
                let mut ret = __cglue_vfunc(cont, positions.into());
                ret
            }
            #[inline(always)]
            fn with_size(&self, size: Tuple2<u32, u32>) -> Self {
                let __cglue_vfunc = unsafe { self.get_vtbl().with_size_lifetimed() };
                let cont = self.ccont_ref();
                let size = size;
                let mut ret = __cglue_vfunc(cont, size);
                self.build_with_ccont(ret)
            }
            #[inline(always)]
            fn with_offset(&self, offset: Tuple2<i32, i32>) -> Self {
                let __cglue_vfunc = unsafe { self.get_vtbl().with_offset_lifetimed() };
                let cont = self.ccont_ref();
                let offset = offset;
                let mut ret = __cglue_vfunc(cont, offset);
                self.build_with_ccont(ret)
            }
            #[inline(always)]
            fn add_zoom(&self, zoom_factor: f64) -> Self {
                let __cglue_vfunc = unsafe { self.get_vtbl().add_zoom_lifetimed() };
                let cont = self.ccont_ref();
                let zoom_factor = zoom_factor;
                let mut ret = __cglue_vfunc(cont, zoom_factor);
                self.build_with_ccont(ret)
            }
            #[inline(always)]
            fn with_option(&self, name: &str, value: &str) -> Self {
                let __cglue_vfunc = unsafe { self.get_vtbl().with_option_lifetimed() };
                let cont = self.ccont_ref();
                let mut ret = __cglue_vfunc(cont, name.into(), value.into());
                self.build_with_ccont(ret)
            }
            #[inline(always)]
            fn get_options(&self) -> RHashMap<RString, RString> {
                let __cglue_vfunc = self.get_vtbl().get_options;
                let cont = self.ccont_ref();
                let mut ret = __cglue_vfunc(cont);
                ret
            }
        }
    }
}
