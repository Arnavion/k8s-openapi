
{operation_feature_attribute}impl{type_generics_impl} {type_name}{type_generics_type}{type_generics_where} {{
    #[doc(hidden)]
    /// Serializes this object to a [`crate::url::form_urlencoded::Serializer`]
    ///
    /// This function is only exposed for use by the `k8s-openapi-derive` crate and is not part of the stable public API.
    {vis}fn __serialize<T>(
        self,
        __query_pairs: &mut crate::url::form_urlencoded::Serializer<'_, T>,
    ) where T: crate::url::form_urlencoded::Target {{
{fields_append_pair}{watch_append_pair}    }}
}}