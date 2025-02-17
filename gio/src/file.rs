// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Cancellable;
use crate::File;
use crate::FileCreateFlags;
use crate::FileEnumerator;
use crate::FileQueryInfoFlags;
use glib::object::IsA;
use glib::translate::*;
use std::cell::RefCell;
use std::mem;
use std::pin::Pin;
use std::ptr;

pub trait FileExtManual: Sized {
    #[doc(alias = "g_file_replace_contents_async")]
    fn replace_contents_async<
        B: AsRef<[u8]> + Send + 'static,
        R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
        cancellable: Option<&C>,
        callback: R,
    );

    fn replace_contents_async_future<B: AsRef<[u8]> + Send + 'static>(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(B, glib::GString), (B, glib::Error)>>
                + 'static,
        >,
    >;

    #[doc(alias = "g_file_enumerate_children_async")]
    fn enumerate_children_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn enumerate_children_async_future(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<FileEnumerator, glib::Error>> + 'static>>;

    #[doc(alias = "g_file_copy_async")]
    fn copy_async<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        destination: &impl IsA<File>,
        flags: crate::FileCopyFlags,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        progress_callback: Option<Box<dyn FnMut(i64, i64) + Send>>,
        callback: Q,
    );

    fn copy_async_future(
        &self,
        destination: &(impl IsA<File> + Clone + 'static),
        flags: crate::FileCopyFlags,
        io_priority: glib::Priority,
    ) -> (
        Pin<Box<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>,
        Pin<Box<dyn futures_core::stream::Stream<Item = (i64, i64)> + 'static>>,
    );

    #[doc(alias = "g_file_load_partial_contents_async")]
    fn load_partial_contents_async<
        P: FnMut(&[u8]) -> bool + Send + 'static,
        Q: FnOnce(Result<(Vec<u8>, Option<glib::GString>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        read_more_callback: P,
        callback: Q,
    );

    #[doc(alias = "g_file_measure_disk_usage")]
    fn measure_disk_usage(
        &self,
        flags: crate::FileMeasureFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
        progress_callback: Option<Box<dyn FnMut(bool, u64, u64, u64) + 'static>>,
    ) -> Result<(u64, u64, u64), glib::Error>;

    #[doc(alias = "g_file_measure_disk_usage_async")]
    fn measure_disk_usage_async<P: FnOnce(Result<(u64, u64, u64), glib::Error>) + Send + 'static>(
        &self,
        flags: crate::FileMeasureFlags,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        progress_callback: Option<Box<dyn FnMut(bool, u64, u64, u64) + Send + 'static>>,
        callback: P,
    );

    fn measure_disk_usage_async_future(
        &self,
        flags: crate::FileMeasureFlags,
        io_priority: glib::Priority,
    ) -> (
        Pin<Box<dyn std::future::Future<Output = Result<(u64, u64, u64), glib::Error>> + 'static>>,
        Pin<Box<dyn futures_core::stream::Stream<Item = (bool, u64, u64, u64)> + 'static>>,
    );
}

impl<O: IsA<File>> FileExtManual for O {
    fn replace_contents_async<
        B: AsRef<[u8]> + Send + 'static,
        R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
        cancellable: Option<&C>,
        callback: R,
    ) {
        let etag = etag.to_glib_none();
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let user_data: Box<Option<(R, B)>> = Box::new(Some((callback, contents)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, contents_ptr) = {
            let contents = &(*user_data).as_ref().unwrap().1;
            let slice = contents.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe extern "C" fn replace_contents_async_trampoline<
            B: AsRef<[u8]> + Send + 'static,
            R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut user_data: Box<Option<(R, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, contents) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let mut new_etag = ptr::null_mut();
            let _ = ffi::g_file_replace_contents_finish(
                _source_object as *mut _,
                res,
                &mut new_etag,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((contents, from_glib_full(new_etag)))
            } else {
                Err((contents, from_glib_full(error)))
            };
            callback(result);
        }
        let callback = replace_contents_async_trampoline::<B, R>;
        unsafe {
            ffi::g_file_replace_contents_async(
                self.as_ref().to_glib_none().0,
                mut_override(contents_ptr),
                count,
                etag.0,
                make_backup.into_glib(),
                flags.into_glib(),
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn replace_contents_async_future<B: AsRef<[u8]> + Send + 'static>(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(B, glib::GString), (B, glib::Error)>>
                + 'static,
        >,
    > {
        let etag = etag.map(glib::GString::from);
        Box::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.replace_contents_async(
                    contents,
                    etag.as_ref().map(|s| s.as_str()),
                    make_backup,
                    flags,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn enumerate_children_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn create_async_trampoline<
            Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_file_enumerate_children_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = create_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_enumerate_children_async(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                flags.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn enumerate_children_async_future(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<FileEnumerator, glib::Error>> + 'static>>
    {
        Box::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.enumerate_children_async(
                    attributes,
                    flags,
                    io_priority,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn copy_async<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        destination: &impl IsA<File>,
        flags: crate::FileCopyFlags,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        progress_callback: Option<Box<dyn FnMut(i64, i64) + Send>>,
        callback: Q,
    ) {
        let progress_trampoline = if progress_callback.is_some() {
            Some(copy_async_progress_trampoline::<Q> as _)
        } else {
            None
        };

        let user_data: Box<(Q, RefCell<Option<Box<dyn FnMut(i64, i64) + Send>>>)> =
            Box::new((callback, RefCell::new(progress_callback)));
        unsafe extern "C" fn copy_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            ffi::g_file_copy_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<(Q, RefCell<Option<Box<dyn FnMut(i64, i64) + Send>>>)> =
                Box::from_raw(user_data as *mut _);
            callback.0(result);
        }
        unsafe extern "C" fn copy_async_progress_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            current_num_bytes: i64,
            total_num_bytes: i64,
            user_data: glib::ffi::gpointer,
        ) {
            let callback: &(Q, RefCell<Option<Box<dyn FnMut(i64, i64) + Send>>>) =
                &*(user_data as *const _);
            (callback.1.borrow_mut().as_mut().expect("no closure"))(
                current_num_bytes,
                total_num_bytes,
            );
        }

        let user_data = Box::into_raw(user_data) as *mut _;

        unsafe {
            ffi::g_file_copy_async(
                self.as_ref().to_glib_none().0,
                destination.as_ref().to_glib_none().0,
                flags.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                progress_trampoline,
                user_data,
                Some(copy_async_trampoline::<Q>),
                user_data,
            );
        }
    }

    fn copy_async_future(
        &self,
        destination: &(impl IsA<File> + Clone + 'static),
        flags: crate::FileCopyFlags,
        io_priority: glib::Priority,
    ) -> (
        Pin<Box<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>,
        Pin<Box<dyn futures_core::stream::Stream<Item = (i64, i64)> + 'static>>,
    ) {
        let destination = destination.clone();

        let (sender, receiver) = futures_channel::mpsc::unbounded();

        let fut = Box::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.copy_async(
                    &destination,
                    flags,
                    io_priority,
                    Some(cancellable),
                    Some(Box::new(move |current_num_bytes, total_num_bytes| {
                        let _ = sender.unbounded_send((current_num_bytes, total_num_bytes));
                    })),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ));

        (fut, Box::pin(receiver))
    }

    fn load_partial_contents_async<
        P: FnMut(&[u8]) -> bool + Send + 'static,
        Q: FnOnce(Result<(Vec<u8>, Option<glib::GString>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        read_more_callback: P,
        callback: Q,
    ) {
        let user_data: Box<(Q, RefCell<P>)> =
            Box::new((callback, RefCell::new(read_more_callback)));
        unsafe extern "C" fn load_partial_contents_async_trampoline<
            P: FnMut(&[u8]) -> bool + Send + 'static,
            Q: FnOnce(Result<(Vec<u8>, Option<glib::GString>), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut contents = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let mut etag_out = ptr::null_mut();
            let mut error = ptr::null_mut();
            ffi::g_file_load_partial_contents_finish(
                _source_object as *mut _,
                res,
                &mut contents,
                length.as_mut_ptr(),
                &mut etag_out,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((
                    FromGlibContainer::from_glib_full_num(contents, length.assume_init() as usize),
                    from_glib_full(etag_out),
                ))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<(Q, RefCell<P>)> = Box::from_raw(user_data as *mut _);
            callback.0(result);
        }
        unsafe extern "C" fn load_partial_contents_async_read_more_trampoline<
            P: FnMut(&[u8]) -> bool + Send + 'static,
            Q: FnOnce(Result<(Vec<u8>, Option<glib::GString>), glib::Error>) + Send + 'static,
        >(
            file_contents: *const libc::c_char,
            file_size: i64,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            use std::slice;

            let callback: &(Q, RefCell<P>) = &*(user_data as *const _);
            (&mut *callback.1.borrow_mut())(slice::from_raw_parts(
                file_contents as *const u8,
                file_size as usize,
            ))
            .into_glib()
        }

        let user_data = Box::into_raw(user_data) as *mut _;

        unsafe {
            ffi::g_file_load_partial_contents_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(load_partial_contents_async_read_more_trampoline::<P, Q>),
                Some(load_partial_contents_async_trampoline::<P, Q>),
                user_data,
            );
        }
    }

    fn measure_disk_usage(
        &self,
        flags: crate::FileMeasureFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
        progress_callback: Option<Box<dyn FnMut(bool, u64, u64, u64) + 'static>>,
    ) -> Result<(u64, u64, u64), glib::Error> {
        let progress_callback_data: Box<
            Option<RefCell<Box<dyn FnMut(bool, u64, u64, u64) + 'static>>>,
        > = Box::new(progress_callback.map(RefCell::new));
        unsafe extern "C" fn progress_callback_func(
            reporting: glib::ffi::gboolean,
            current_size: u64,
            num_dirs: u64,
            num_files: u64,
            user_data: glib::ffi::gpointer,
        ) {
            let reporting = from_glib(reporting);
            let callback: &Option<RefCell<Box<dyn Fn(bool, u64, u64, u64) + 'static>>> =
                &*(user_data as *mut _);
            if let Some(ref callback) = *callback {
                (&mut *callback.borrow_mut())(reporting, current_size, num_dirs, num_files)
            } else {
                panic!("cannot get closure...")
            };
        }
        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };
        let super_callback0: Box<Option<RefCell<Box<dyn FnMut(bool, u64, u64, u64) + 'static>>>> =
            progress_callback_data;
        unsafe {
            let mut disk_usage = mem::MaybeUninit::uninit();
            let mut num_dirs = mem::MaybeUninit::uninit();
            let mut num_files = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::g_file_measure_disk_usage(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                progress_callback,
                Box::into_raw(super_callback0) as *mut _,
                disk_usage.as_mut_ptr(),
                num_dirs.as_mut_ptr(),
                num_files.as_mut_ptr(),
                &mut error,
            );
            let disk_usage = disk_usage.assume_init();
            let num_dirs = num_dirs.assume_init();
            let num_files = num_files.assume_init();
            if error.is_null() {
                Ok((disk_usage, num_dirs, num_files))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn measure_disk_usage_async<
        P: FnOnce(Result<(u64, u64, u64), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: crate::FileMeasureFlags,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        progress_callback: Option<Box<dyn FnMut(bool, u64, u64, u64) + Send + 'static>>,
        callback: P,
    ) {
        let progress_callback_trampoline = if progress_callback.is_some() {
            Some(measure_disk_usage_async_progress_trampoline::<P> as _)
        } else {
            None
        };

        let user_data: Box<(
            P,
            Option<RefCell<Box<dyn FnMut(bool, u64, u64, u64) + Send + 'static>>>,
        )> = Box::new((callback, progress_callback.map(RefCell::new)));
        unsafe extern "C" fn measure_disk_usage_async_trampoline<
            P: FnOnce(Result<(u64, u64, u64), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut disk_usage = mem::MaybeUninit::uninit();
            let mut num_dirs = mem::MaybeUninit::uninit();
            let mut num_files = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            ffi::g_file_measure_disk_usage_finish(
                _source_object as *mut _,
                res,
                disk_usage.as_mut_ptr(),
                num_dirs.as_mut_ptr(),
                num_files.as_mut_ptr(),
                &mut error,
            );
            let result = if error.is_null() {
                Ok((
                    disk_usage.assume_init(),
                    num_dirs.assume_init(),
                    num_files.assume_init(),
                ))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<(
                P,
                Option<RefCell<Box<dyn FnMut(bool, u64, u64, u64) + Send + 'static>>>,
            )> = Box::from_raw(user_data as *mut _);
            callback.0(result);
        }
        unsafe extern "C" fn measure_disk_usage_async_progress_trampoline<
            P: FnOnce(Result<(u64, u64, u64), glib::Error>) + Send + 'static,
        >(
            reporting: glib::ffi::gboolean,
            disk_usage: u64,
            num_dirs: u64,
            num_files: u64,
            user_data: glib::ffi::gpointer,
        ) {
            let callback: &(
                P,
                Option<RefCell<Box<dyn FnMut(bool, u64, u64, u64) + Send + 'static>>>,
            ) = &*(user_data as *const _);
            if let Some(ref callback) = callback.1 {
                (&mut *callback.borrow_mut())(
                    from_glib(reporting),
                    disk_usage,
                    num_dirs,
                    num_files,
                );
            } else {
                panic!("cannot get closure...")
            }
        }

        let user_data = Box::into_raw(user_data) as *mut _;

        unsafe {
            ffi::g_file_measure_disk_usage_async(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                progress_callback_trampoline,
                user_data,
                Some(measure_disk_usage_async_trampoline::<P>),
                user_data,
            );
        }
    }

    fn measure_disk_usage_async_future(
        &self,
        flags: crate::FileMeasureFlags,
        io_priority: glib::Priority,
    ) -> (
        Pin<Box<dyn std::future::Future<Output = Result<(u64, u64, u64), glib::Error>> + 'static>>,
        Pin<Box<dyn futures_core::stream::Stream<Item = (bool, u64, u64, u64)> + 'static>>,
    ) {
        let (sender, receiver) = futures_channel::mpsc::unbounded();

        let fut = Box::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.measure_disk_usage_async(
                    flags,
                    io_priority,
                    Some(cancellable),
                    Some(Box::new(
                        move |reporting, disk_usage, num_dirs, num_files| {
                            let _ =
                                sender.unbounded_send((reporting, disk_usage, num_dirs, num_files));
                        },
                    )),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ));

        (fut, Box::pin(receiver))
    }
}
