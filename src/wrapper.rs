/// Defines a wrapper type and implements the appropriate traits.
///
/// The basic syntax is
///
/// ```ignore
/// glib_wrapper! {
///     /// Documentation
///     pub struct $name($kind<$foreign>);
///
///     match fn {
///         $fn_name => /* a closure-like expression */,
///         ...
///     }
/// }
/// ```
///
/// This creates a wrapper named `$name` around the foreign type `$foreign`
/// of $kind (one of `Boxed`, `Object`) using expressions from the `match fn`
/// block to implement type-specific low-level operations. The expression
/// will be evaluated in `unsafe` context.
///
/// ### Boxed
///
/// ```ignore
/// glib_wrapper! {
///     /// Text buffer iterator
///     pub struct TextIter(Boxed<ffi::GtkTextIter>);
///
///     match fn {
///         copy => |ptr| ffi::gtk_text_iter_copy(ptr),
///         free => |ptr| ffi::gtk_text_iter_free(ptr),
///     }
/// }
/// ```
///
/// `copy`: `|*const $foreign| -> *mut $foreign` creates a copy of the value.
///
/// `free`: `|*mut $foreign|` frees the value.
///
/// ### Refcounted
///
/// ```ignore
/// glib_wrapper! {
///     /// Object holding timing information for a single frame.
///     pub struct FrameTimings(Refcounted<ffi::GdkFrameTimings>);
///
///     match fn {
///         ref => |ptr| ffi::gdk_frame_timings_ref(ptr),
///         unref => |ptr| ffi::gdk_frame_timings_unref(ptr),
///     }
/// }
/// ```
///
/// `ref`: `|*mut $foreign|` increases the refcount.
///
/// `unref`: `|*mut $foreign|` decreases the refcount.
///
/// ### Object
///
/// ```
/// glib_wrapper! {
///     /// Object representing an input device.
///     pub struct Device(Object<ffi::GdkDevice>);
///
///     match fn {
///         get_type => || ffi::gdk_device_get_type(),
///     }
/// }
/// ```
///
/// ```
/// glib_wrapper! {
///     /// A container with just one child.
///     pub struct Bin(Object<ffi::GtkBin>): Container, Widget, Buildable;
///
///     match fn {
///         get_type => || ffi::gtk_bin_get_type(),
///     }
/// }
/// ```
///
/// `get_type: || -> GType` returns the type identifier of the class.
#[macro_export]
macro_rules! glib_wrapper {
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Boxed<$ffi_name:path>);

        match fn {
            copy => |$copy_arg:ident| $copy_expr:expr,
            free => |$free_arg:ident| $free_expr:expr,
        }
    ) => {
        glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Refcounted<$ffi_name:path>);

        match fn {
            ref => |$ref_arg:ident| $ref_expr:expr,
            unref => |$unref_arg:ident| $unref_expr:expr,
        }
    ) => {
        glib_refcounted_wrapper!([$($attr)*] $name, $ffi_name, @ref $ref_arg $ref_expr,
            @unref $unref_arg $unref_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, @get_type $get_type_expr, []);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path>): $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, @get_type $get_type_expr,
            [$($implements),+]);
    };
}

/// A wrapper struct.
pub trait Wrapper {
    /// Foreign type represented by the struct.
    type GlibType: 'static;
}
