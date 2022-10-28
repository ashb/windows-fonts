use std::cell::RefCell;
use std::ffi::{c_int, c_void};
use std::slice;

use pyo3::class::basic::CompareOp;
use pyo3::exceptions::{PyIndexError, PyKeyError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::PyString;
use windows::core::HSTRING;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::DirectWrite::{IDWriteFont, IDWriteLocalizedStrings};
use windows::{
    core::{Interface, Result, PCWSTR},
    w,
    Win32::Graphics::DirectWrite::{
        DWriteCreateFactory, IDWriteFactory1, IDWriteFontCollection1, IDWriteFontFamily,
        IDWriteFontFile, IDWriteLocalFontFileLoader, DWRITE_FACTORY_TYPE_SHARED,
    },
};

thread_local! {
    static LOCAL_LOADER: RefCell<IDWriteLocalFontFileLoader> = RefCell::new(_get_local_loader().unwrap());
    static USER_LOCALE: HSTRING = _get_user_locale().unwrap();
}

fn _get_local_loader() -> Result<IDWriteLocalFontFileLoader> {
    // We can't create an instance of LocalFontFileLoader directly, so we have to get a reference to it via loading a local file!
    unsafe {
        let factory: IDWriteFactory1 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;
        // TODO: get the first font filename out of the registry dir!
        let file = factory.CreateFontFileReference(w!(r"C:\Windows\Fonts\Arial.ttf"), None)?;

        let loader = file.GetLoader()?;
        loader.cast()
    }
}

fn _get_user_locale() -> Result<HSTRING> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetUserDefaultLocaleName(lpLocaleName: *mut PCWSTR, cchLocaleName: c_int) -> c_int;
    }

    pub const LOCALE_NAME_MAX_LENGTH: usize = 85;
    let mut buff = Vec::new();
    buff.resize(LOCALE_NAME_MAX_LENGTH + 1, 0);
    unsafe {
        let len = GetUserDefaultLocaleName(
            buff.as_mut_slice() as *mut _ as _,
            LOCALE_NAME_MAX_LENGTH as i32,
        );

        if len <= 0 {
            Err(windows::core::Error::from_win32())
        } else if len == 1 {
            // zero length string! (just the null byte came back) Fallback:
            Ok(w!("en-US").to_owned())
        } else {
            buff.resize((len - 1) as usize, 0);
            Ok(HSTRING::from_wide(buff.as_slice()))
        }
    }
}

mod enums;
mod errors;

use errors::PyWindowsErr;

#[derive(FromPyObject)]
enum IntOrStr<'a> {
    Str(&'a PyString),
    Int(isize),
}

#[pyclass(module = "windows_fonts", unsendable)]
struct FontCollection {
    collection: IDWriteFontCollection1,
}

impl FontCollection {
    fn get_system_font_collection() -> Result<IDWriteFontCollection1> {
        unsafe {
            let factory: IDWriteFactory1 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

            let mut collection: Option<IDWriteFontCollection1> = None;
            factory.GetSystemFontCollection(&mut collection as *mut _ as _, true)?;
            // Panic here is okay, cos we _shouldn't_ have a no error but no collection given back
            Ok(
                collection
                    .expect("GetSystemFontCollection had not error but gave us no collection"),
            )
        }
    }
}

#[pymethods]
impl FontCollection {
    #[new]
    fn __new__() -> PyResult<Self> {
        let collection = Self::get_system_font_collection().map_err(PyWindowsErr::from)?;
        Ok(FontCollection { collection })
    }

    fn __len__(&self) -> usize {
        unsafe { self.collection.GetFontFamilyCount() as usize }
    }

    fn __getitem__(&self, key: IntOrStr) -> PyResult<FontFamily> {
        let index = match key {
            IntOrStr::Str(str) => unsafe {
                let mut exists = BOOL(0);
                let mut i_out = 0;
                let s: Vec<u16> = str.to_string().encode_utf16().collect();
                self.collection
                    .FindFamilyName(&HSTRING::from_wide(s.as_slice()), &mut i_out, &mut exists)
                    .map_err(PyWindowsErr::from)?;
                if !exists.as_bool() {
                    return Err(PyKeyError::new_err(format!(
                        "unknown font family {:?}",
                        str
                    )));
                }
                i_out
            },
            IntOrStr::Int(idx) => idx as u32,
        };

        if index >= unsafe { self.collection.GetFontFamilyCount() } {
            return Err(PyIndexError::new_err("list index out of range"));
        }

        let ifamily = unsafe {
            self.collection
                .GetFontFamily(index)
                .map_err(PyWindowsErr::from)?
        };

        Ok(FontFamily(ifamily))
    }
}
trait BestLocaleName {
    unsafe fn get_best_name(&self) -> Result<String>;
}

impl BestLocaleName for IDWriteLocalizedStrings {
    unsafe fn get_best_name(&self) -> Result<String> {
        let mut index = 0u32;

        USER_LOCALE.with(|locale| -> Result<()> {
            let mut found = BOOL(0);
            let res = self.FindLocaleName(Into::<PCWSTR>::into(locale), &mut index, &mut found);

            if res.is_ok() && !found.as_bool() {
                // Fallback to en-us locale
                _ = self.FindLocaleName(w!("en-us"), &mut index, &mut found);
            }

            if !found.as_bool() {
                // Still not found, get first on the list
                index = 0;
            }

            Ok(())
        })?;

        let len = self.GetStringLength(index)? as usize;

        let mut buff = Vec::new();
        buff.resize(len + 1, 0u16);
        self.GetString(index, buff.as_mut_slice())?;

        Ok(String::from_utf16(slice::from_raw_parts(buff.as_ptr(), len)).unwrap())
    }
}

#[pyclass(sequence, module = "windows_fonts", unsendable)]
#[derive(Clone, Debug)]
struct FontFamily(IDWriteFontFamily);

impl FontFamily {
    /// Get the name from the "best" available locale, or first as a fallback
    unsafe fn _get_best_name(&self) -> Result<String> {
        let names = self.0.GetFamilyNames()?;
        names.get_best_name()
    }
}

#[pymethods]
impl FontFamily {
    #[getter]
    pub fn name(&self) -> PyResult<String> {
        let res = unsafe { self._get_best_name() };
        Ok(res.map_err(PyWindowsErr::from)?)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<FontFamily name={:?}>", self.name()?,))
    }

    pub fn __len__(&self) -> usize {
        unsafe { self.0.GetFontCount() as usize }
    }

    pub fn __getitem__(rc: Py<Self>, mut index: i32, py: Python<'_>) -> PyResult<FontVariant> {
        unsafe {
            let self_ = rc.borrow(py);
            if index < 0 {
                index += self_.0.GetFontCount() as i32;
            }
            match self_.0.GetFont(index as u32) {
                Ok(font) => Ok(FontVariant {
                    font,
                    family: rc.clone(),
                }),
                Err(_) => Err(PyIndexError::new_err(format!(
                    "key {:?} out of range",
                    index
                ))),
            }
        }
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => self.eq(other).into_py(py),
            CompareOp::Ne => self.ne(other).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

impl PartialEq for FontFamily {
    fn eq(&self, other: &Self) -> bool {
        // Best we can do is compare by name. Each time we get the IDWriteFontFamily it will be a different COM Ptr
        let name;
        let other_name: String;
        unsafe {
            if let Ok(n) = self._get_best_name() {
                name = n;
            } else {
                return false;
            }
            if let Ok(n) = other._get_best_name() {
                other_name = n;
            } else {
                return false;
            }
        }
        name == other_name
    }
}

#[pyclass(module = "windows_fonts", unsendable)]
struct FontVariant {
    font: IDWriteFont,
    // Keep the family alive so we can use it in `repr`, but don't create a _rust_ memory cycle
    #[pyo3(get)]
    family: Py<FontFamily>,
}

#[pymethods]
impl FontVariant {
    #[getter]
    pub fn style(&self) -> enums::Style {
        unsafe { ::std::mem::transmute(self.font.GetStyle().0) }
    }

    #[getter]
    pub fn weight(&self) -> enums::Weight {
        unsafe { ::std::mem::transmute(self.font.GetWeight().0) }
    }

    pub fn __repr__(&self, py: Python) -> PyResult<String> {
        let family = self.family.as_ref(py);

        Ok(format!(
            "<FontVariant family={}, style={} weight={}>",
            family.repr()?,
            self.style().into_py(py).as_ref(py).repr()?,
            self.weight().into_py(py).as_ref(py).repr()?,
        ))
    }

    #[getter]
    pub fn filename(&self) -> PyResult<String> {
        let names = self.files()?;
        if names.len() != 1 {
            Err(PyRuntimeError::new_err(
                "FontVariant had more than one name, please use .files()",
            ))
        } else {
            Ok(names[0].to_owned())
        }
    }

    pub fn files(&self) -> PyResult<Vec<String>> {
        let res = unsafe { self._get_files() }.map_err(PyWindowsErr::from)?;
        Ok(res)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => self.eq(other).into_py(py),
            CompareOp::Ne => self.ne(other).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

impl PartialEq for FontVariant {
    fn eq(&self, other: &Self) -> bool {
        // Quick checks first
        if self.weight() != other.weight() || self.style() == other.style() {
            return false;
        }

        Python::with_gil(|py| {
            let family = self.family.as_ref(py);
            let other_family = other.family.as_ref(py);

            family.eq(other_family)
        })
        .unwrap_or(false)
    }
}

impl FontVariant {
    unsafe fn _get_files(&self) -> Result<Vec<String>> {
        let face = self.font.CreateFontFace()?;
        let mut num_files = 0u32;
        face.GetFiles(&mut num_files, None)?;

        let mut filenames: Vec<String> = Vec::with_capacity(num_files as usize);
        let mut font_files: Vec<Option<IDWriteFontFile>> = Vec::with_capacity(num_files as usize);

        face.GetFiles(
            &mut num_files,
            Some(font_files.spare_capacity_mut() as *mut _ as _),
        )?;
        font_files.set_len(num_files as usize);

        for font_file in font_files.iter().flatten() {
            let mut ref_key: *const c_void = std::ptr::null();
            let mut key_size: u32 = 0;
            font_file.GetReferenceKey(&mut ref_key as *mut _ as _, &mut key_size as *mut _ as _)?;

            let filename = LOCAL_LOADER.with(|cell| -> String {
                let loader = cell.borrow();
                let path_len: usize = loader
                    .GetFilePathLengthFromKey(ref_key, key_size)
                    .expect("GetFilePathLengthFromKey failed")
                    as usize;

                let mut buff = Vec::new();
                buff.resize(path_len + 1, 0);

                // let x = path.as_ptr();
                loader
                    .GetFilePathFromKey(ref_key, key_size, buff.as_mut_slice())
                    .expect("GetFilePathFromKey failed");

                String::from_utf16(slice::from_raw_parts(buff.as_ptr(), path_len)).unwrap()
            });
            filenames.push(filename)
        }
        Ok(filenames)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _windows_fonts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FontCollection>()?;
    m.add_class::<enums::Weight>()?;
    m.add_class::<enums::Style>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_loader() {
        // Test that we can actually get a LocalLoader without panicing
        LOCAL_LOADER.with(|f| {
            f.borrow();
            /* no-op */
        })
    }
}
