// @generated
/// Implement `DataProvider<QuotationMarkV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_qmark_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_QMARK_V1: &'static <icu::properties::provider::QuotationMarkV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\"\0\0\0#\0\0\0'\0\0\0(\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x18 \0\0  \0\09 \0\0; \0\0B.\0\0C.\0\0\x0C0\0\0\x100\0\0\x1D0\0\0 0\0\0A\xFE\0\0E\xFE\0\0\x02\xFF\0\0\x03\xFF\0\0\x07\xFF\0\0\x08\xFF\0\0b\xFF\0\0d\xFF\0\0") }, 30u32)
            });
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::properties::provider::QuotationMarkV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::QuotationMarkV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_QMARK_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::QuotationMarkV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
