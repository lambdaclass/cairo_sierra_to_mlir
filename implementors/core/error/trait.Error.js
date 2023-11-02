(function() {var implementors = {
"cairo_native":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"struct\" href=\"cairo_native/utils/struct.LayoutError.html\" title=\"struct cairo_native::utils::LayoutError\">LayoutError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"enum\" href=\"cairo_native/error/types/enum.ErrorImpl.html\" title=\"enum cairo_native::error::types::ErrorImpl\">ErrorImpl</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"struct\" href=\"cairo_native/error/libfuncs/struct.Error.html\" title=\"struct cairo_native::error::libfuncs::Error\">Error</a>"],["impl&lt;TType, TLibfunc&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"struct\" href=\"cairo_native/error/compile/struct.Error.html\" title=\"struct cairo_native::error::compile::Error\">Error</a>&lt;TType, TLibfunc&gt;<span class=\"where fmt-newline\">where\n    TType: GenericType,\n    TLibfunc: GenericLibfunc,\n    &lt;TType as GenericType&gt;::Concrete: <a class=\"trait\" href=\"cairo_native/types/trait.TypeBuilder.html\" title=\"trait cairo_native::types::TypeBuilder\">TypeBuilder</a>&lt;TType, TLibfunc&gt;,\n    &lt;TLibfunc as GenericLibfunc&gt;::Concrete: <a class=\"trait\" href=\"cairo_native/libfuncs/trait.LibfuncBuilder.html\" title=\"trait cairo_native::libfuncs::LibfuncBuilder\">LibfuncBuilder</a>&lt;TType, TLibfunc&gt;,\n    <a class=\"enum\" href=\"cairo_native/error/compile/enum.ErrorImpl.html\" title=\"enum cairo_native::error::compile::ErrorImpl\">ErrorImpl</a>&lt;TType, TLibfunc&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + 'static,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,</span>"],["impl&lt;TType, TLibfunc&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"enum\" href=\"cairo_native/error/compile/enum.ErrorImpl.html\" title=\"enum cairo_native::error::compile::ErrorImpl\">ErrorImpl</a>&lt;TType, TLibfunc&gt;<span class=\"where fmt-newline\">where\n    TType: GenericType,\n    TLibfunc: GenericLibfunc,\n    &lt;TType as GenericType&gt;::Concrete: <a class=\"trait\" href=\"cairo_native/types/trait.TypeBuilder.html\" title=\"trait cairo_native::types::TypeBuilder\">TypeBuilder</a>&lt;TType, TLibfunc&gt;,\n    &lt;TLibfunc as GenericLibfunc&gt;::Concrete: <a class=\"trait\" href=\"cairo_native/libfuncs/trait.LibfuncBuilder.html\" title=\"trait cairo_native::libfuncs::LibfuncBuilder\">LibfuncBuilder</a>&lt;TType, TLibfunc&gt;,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"struct\" href=\"cairo_native/error/types/struct.Error.html\" title=\"struct cairo_native::error::types::Error\">Error</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"enum\" href=\"cairo_native/error/libfuncs/enum.ErrorImpl.html\" title=\"enum cairo_native::error::libfuncs::ErrorImpl\">ErrorImpl</a>"],["impl&lt;'de, TType, TLibfunc, D, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"struct\" href=\"cairo_native/error/jit_engine/struct.Error.html\" title=\"struct cairo_native::error::jit_engine::Error\">Error</a>&lt;'de, TType, TLibfunc, D, S&gt;<span class=\"where fmt-newline\">where\n    TType: GenericType,\n    TLibfunc: GenericLibfunc,\n    &lt;TType as GenericType&gt;::Concrete: <a class=\"trait\" href=\"cairo_native/types/trait.TypeBuilder.html\" title=\"trait cairo_native::types::TypeBuilder\">TypeBuilder</a>&lt;TType, TLibfunc&gt;,\n    &lt;TLibfunc as GenericLibfunc&gt;::Concrete: <a class=\"trait\" href=\"cairo_native/libfuncs/trait.LibfuncBuilder.html\" title=\"trait cairo_native::libfuncs::LibfuncBuilder\">LibfuncBuilder</a>&lt;TType, TLibfunc&gt;,\n    D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.188/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,\n    S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.188/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,\n    <a class=\"enum\" href=\"cairo_native/error/jit_engine/enum.ErrorImpl.html\" title=\"enum cairo_native::error::jit_engine::ErrorImpl\">ErrorImpl</a>&lt;'de, TType, TLibfunc, D, S&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + 'static,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,</span>"],["impl&lt;'de, TType, TLibfunc, D, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for <a class=\"enum\" href=\"cairo_native/error/jit_engine/enum.ErrorImpl.html\" title=\"enum cairo_native::error::jit_engine::ErrorImpl\">ErrorImpl</a>&lt;'de, TType, TLibfunc, D, S&gt;<span class=\"where fmt-newline\">where\n    TType: GenericType,\n    TLibfunc: GenericLibfunc,\n    &lt;TType as GenericType&gt;::Concrete: <a class=\"trait\" href=\"cairo_native/types/trait.TypeBuilder.html\" title=\"trait cairo_native::types::TypeBuilder\">TypeBuilder</a>&lt;TType, TLibfunc&gt;,\n    &lt;TLibfunc as GenericLibfunc&gt;::Concrete: <a class=\"trait\" href=\"cairo_native/libfuncs/trait.LibfuncBuilder.html\" title=\"trait cairo_native::libfuncs::LibfuncBuilder\">LibfuncBuilder</a>&lt;TType, TLibfunc&gt;,\n    D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.188/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,\n    S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.188/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()