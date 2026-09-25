# Published PDFium API signatures inspected (not installed or tested)

## Pdfium
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.Pdfium.html
pub fn bind_to_statically_linked_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError>
pub fn bind_to_system_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError>
pub fn bind_to_library(     path: impl AsRef<Path>, ) -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError>
pub fn set_custom_font_provider(     &mut self,     provider: Box<dyn PdfiumCustomFontProvider>, )
pub fn clear_custom_font_provider(&mut self)
pub fn use_platform_default_font_provider(&mut self) -> Result<(), PdfiumError>
pub fn load_pdf_from_byte_slice<'a>(     &'a self,     bytes: &'a [u8],     password: Option<&str>, ) -> Result<PdfDocument<'a>, PdfiumError>
pub fn load_pdf_from_byte_vec(     &self,     bytes: Vec<u8>,     password: Option<&str>, ) -> Result<PdfDocument<'_>, PdfiumError>
pub fn load_pdf_from_file<'a>(     &'a self,     path: &(impl AsRef<Path> + ?Sized),     password: Option<&str>, ) -> Result<PdfDocument<'a>, PdfiumError>
pub fn load_pdf_from_reader<'a, R: PdfiumReader + 'a>(     &'a self,     reader: R,     password: Option<&str>, ) -> Result<PdfDocument<'a>, PdfiumError>
pub async fn load_pdf_from_fetch<'a>(     &'a self,     url: impl ToString,     password: Option<&str>, ) -> Result<PdfDocument<'a>, PdfiumError>
pub async fn load_pdf_from_blob<'a>(     &'a self,     blob: Blob,     password: Option<&str>, ) -> Result<PdfDocument<'a>, PdfiumError>

## PdfPageTextChar
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPageTextChar.html
pub fn unicode_value(&self) -> u32
pub fn unicode_char(&self) -> Option<char>
pub fn unicode_string(&self) -> Option<String>
pub fn scaled_font_size(&self) -> PdfPoints
pub fn unscaled_font_size(&self) -> PdfPoints
pub fn font_name(&self) -> String
pub fn font_weight(&self) -> Option<PdfFontWeight>
pub fn font_is_fixed_pitch(&self) -> bool
pub fn font_is_proportional_pitch(&self) -> bool
pub fn font_is_serif(&self) -> bool
pub fn font_is_sans_serif(&self) -> bool
pub fn font_is_symbolic(&self) -> bool
pub fn font_is_non_symbolic(&self) -> bool
pub fn font_is_cursive(&self) -> bool
pub fn font_is_italic(&self) -> bool
pub fn font_is_all_caps(&self) -> bool
pub fn font_is_small_caps(&self) -> bool
pub fn font_is_bold_reenforced(&self) -> bool
pub fn render_mode(&self) -> Result<PdfPageTextRenderMode, PdfiumError>
pub fn tight_bounds(&self) -> Result<PdfRect, PdfiumError>
pub fn loose_bounds(&self) -> Result<PdfRect, PdfiumError>
pub fn matrix(&self) -> Result<PdfMatrix, PdfiumError>

## PdfPageText
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPageText.html
pub fn segments(&self) -> PdfPageTextSegments<'_>
pub fn segments_subset(     &self,     start: PdfPageTextCharIndex,     count: PdfPageTextCharIndex, ) -> PdfPageTextSegments<'_>
pub fn chars(&self) -> PdfPageTextChars<'_>
pub fn chars_for_object(     &self,     object: &PdfPageTextObject<'_>, ) -> Result<PdfPageTextChars<'_>, PdfiumError>
pub fn chars_for_annotation(     &self,     annotation: &PdfPageAnnotation<'_>, ) -> Result<PdfPageTextChars<'_>, PdfiumError>
pub fn chars_inside_rect<'b>(     &'b self,     rect: PdfRect, ) -> Result<PdfPageTextChars<'a>, PdfiumError>

## PdfPage
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPage.html
pub fn width(&self) -> PdfPoints
pub fn height(&self) -> PdfPoints
pub fn objects(&self) -> &PdfPageObjects<'_>
pub fn objects_mut(&mut self) -> &mut PdfPageObjects<'a>
pub fn fonts(&self) -> Vec<PdfFont<'_>>
pub fn pixels_to_points(     &self,     x: Pixels,     y: Pixels,     config: &PdfRenderConfig, ) -> Result<(PdfPoints, PdfPoints), PdfiumError>
pub fn points_to_pixels(     &self,     x: PdfPoints,     y: PdfPoints,     config: &PdfRenderConfig, ) -> Result<(Pixels, Pixels), PdfiumError>
pub fn render(     &self,     width: Pixels,     height: Pixels,     rotation: Option<PdfPageRenderRotation>, ) -> Result<PdfBitmap<'_>, PdfiumError>
pub fn render_with_config(     &self,     config: &PdfRenderConfig, ) -> Result<PdfBitmap<'_>, PdfiumError>
pub fn render_into_bitmap(     &self,     bitmap: &mut PdfBitmap<'_>,     width: Pixels,     height: Pixels,     rotation: Option<PdfPageRenderRotation>, ) -> Result<(), PdfiumError>
pub fn render_into_bitmap_with_config(     &self,     bitmap: &mut PdfBitmap<'_>,     config: &PdfRenderConfig, ) -> Result<(), PdfiumError>
pub fn apply_matrix_with_clip(     &mut self,     matrix: PdfMatrix,     clip: PdfRect, ) -> Result<(), PdfiumError>
pub fn apply_matrix(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError>

## PdfPageObject
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/enum.PdfPageObject.html
pub fn as_path_object(&self) -> Option<&PdfPagePathObject<'_>>
pub fn as_path_object_mut(&mut self) -> Option<&mut PdfPagePathObject<'a>>
pub fn as_image_object(&self) -> Option<&PdfPageImageObject<'_>>
pub fn as_image_object_mut(&mut self) -> Option<&mut PdfPageImageObject<'a>>
pub fn apply_matrix(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError>
pub fn reset_matrix(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError>
pub fn reset_matrix_to_identity(&mut self) -> Result<(), PdfiumError>
pub fn matrix(&self) -> Result<PdfMatrix, PdfiumError>

## PdfPagePathObject
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPagePathObject.html
pub fn new(     document: &PdfDocument<'a>,     x: PdfPoints,     y: PdfPoints,     stroke_color: Option<PdfColor>,     stroke_width: Option<PdfPoints>,     fill_color: Option<PdfColor>, ) -> Result<Self, PdfiumError>
pub fn new_line(     document: &PdfDocument<'a>,     x1: PdfPoints,     y1: PdfPoints,     x2: PdfPoints,     y2: PdfPoints,     stroke_color: PdfColor,     stroke_width: PdfPoints, ) -> Result<Self, PdfiumError>
pub fn new_bezier(     document: &PdfDocument<'a>,     x1: PdfPoints,     y1: PdfPoints,     x2: PdfPoints,     y2: PdfPoints,     control1_x: PdfPoints,     control1_y: PdfPoints,     control2_x: PdfPoints,     control2_y: PdfPoints,     stroke_color: PdfColor,     stroke_width: PdfPoints, ) -> Result<Self, PdfiumError>
pub fn new_rect(     document: &PdfDocument<'a>,     rect: PdfRect,     stroke_color: Option<PdfColor>,     stroke_width: Option<PdfPoints>,     fill_color: Option<PdfColor>, ) -> Result<Self, PdfiumError>
pub fn new_circle(     document: &PdfDocument<'a>,     rect: PdfRect,     stroke_color: Option<PdfColor>,     stroke_width: Option<PdfPoints>,     fill_color: Option<PdfColor>, ) -> Result<Self, PdfiumError>
pub fn new_circle_at(     document: &PdfDocument<'a>,     center_x: PdfPoints,     center_y: PdfPoints,     radius: PdfPoints,     stroke_color: Option<PdfColor>,     stroke_width: Option<PdfPoints>,     fill_color: Option<PdfColor>, ) -> Result<Self, PdfiumError>
pub fn new_ellipse(     document: &PdfDocument<'a>,     rect: PdfRect,     stroke_color: Option<PdfColor>,     stroke_width: Option<PdfPoints>,     fill_color: Option<PdfColor>, ) -> Result<Self, PdfiumError>
pub fn new_ellipse_at(     document: &PdfDocument<'a>,     center_x: PdfPoints,     center_y: PdfPoints,     x_radius: PdfPoints,     y_radius: PdfPoints,     stroke_color: Option<PdfColor>,     stroke_width: Option<PdfPoints>,     fill_color: Option<PdfColor>, ) -> Result<Self, PdfiumError>
pub fn segments(&self) -> PdfPagePathObjectSegments<'_>
pub fn apply_matrix(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError>
pub fn reset_matrix(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError>
pub fn reset_matrix_to_identity(&mut self) -> Result<(), PdfiumError>
pub fn matrix(&self) -> Result<PdfMatrix, PdfiumError>

## PdfPageImageObject
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPageImageObject.html
pub fn new(     document: &PdfDocument<'a>,     image: &DynamicImage, ) -> Result<Self, PdfiumError>
pub fn new_with_width(     document: &PdfDocument<'a>,     image: &DynamicImage,     width: PdfPoints, ) -> Result<Self, PdfiumError>
pub fn new_with_height(     document: &'a PdfDocument<'a>,     image: &DynamicImage,     height: PdfPoints, ) -> Result<Self, PdfiumError>
pub fn new_with_size(     document: &PdfDocument<'a>,     image: &DynamicImage,     width: PdfPoints,     height: PdfPoints, ) -> Result<Self, PdfiumError>
pub fn get_raw_bitmap(&self) -> Result<PdfBitmap<'_>, PdfiumError>
pub fn get_raw_image(&self) -> Result<DynamicImage, PdfiumError>
pub fn get_processed_bitmap(     &self,     document: &PdfDocument<'_>, ) -> Result<PdfBitmap<'_>, PdfiumError>
pub fn get_processed_image(     &self,     document: &PdfDocument<'_>, ) -> Result<DynamicImage, PdfiumError>
pub fn get_processed_bitmap_with_width(     &self,     document: &PdfDocument<'_>,     width: Pixels, ) -> Result<PdfBitmap<'_>, PdfiumError>
pub fn get_processed_image_with_width(     &self,     document: &PdfDocument<'_>,     width: Pixels, ) -> Result<DynamicImage, PdfiumError>
pub fn get_processed_bitmap_with_height(     &self,     document: &PdfDocument<'_>,     height: Pixels, ) -> Result<PdfBitmap<'_>, PdfiumError>
pub fn get_processed_image_with_height(     &self,     document: &PdfDocument<'_>,     height: Pixels, ) -> Result<DynamicImage, PdfiumError>
pub fn get_processed_bitmap_with_size(     &self,     document: &PdfDocument<'_>,     width: Pixels,     height: Pixels, ) -> Result<PdfBitmap<'_>, PdfiumError>
pub fn get_processed_image_with_size(     &self,     document: &PdfDocument<'_>,     width: Pixels,     height: Pixels, ) -> Result<DynamicImage, PdfiumError>
pub fn get_raw_image_data(&self) -> Result<Vec<u8>, PdfiumError>
pub fn width(&self) -> Result<Pixels, PdfiumError>
pub fn height(&self) -> Result<Pixels, PdfiumError>
pub fn set_image(&mut self, image: &DynamicImage) -> Result<(), PdfiumError>
pub fn set_bitmap(&mut self, bitmap: &PdfBitmap<'_>) -> Result<(), PdfiumError>
pub fn apply_matrix(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError>
pub fn reset_matrix(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError>
pub fn reset_matrix_to_identity(&mut self) -> Result<(), PdfiumError>
pub fn matrix(&self) -> Result<PdfMatrix, PdfiumError>

## PdfBitmap
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfBitmap.html
pub fn empty(     width: Pixels,     height: Pixels,     format: PdfBitmapFormat, ) -> Result<PdfBitmap<'a>, PdfiumError>
pub fn from_bytes(     width: Pixels,     height: Pixels,     format: PdfBitmapFormat,     buffer: &'a mut [u8], ) -> Result<PdfBitmap<'a>, PdfiumError>
pub unsafe fn from_bytes_unchecked(     width: Pixels,     height: Pixels,     format: PdfBitmapFormat,     buffer: &'a mut [u8], ) -> Result<PdfBitmap<'a>, PdfiumError>
pub fn width(&self) -> Pixels
pub fn height(&self) -> Pixels
pub fn format(&self) -> Result<PdfBitmapFormat, PdfiumError>
pub fn as_raw_bytes(&self) -> Vec<u8> ⓘ
pub fn as_rgba_bytes(&self) -> Vec<u8> ⓘ
pub fn as_image(&self) -> Result<DynamicImage, PdfiumError>
pub fn as_image_data(&self) -> Result<ImageData, JsValue>
pub fn bytes_required_for_size(width: Pixels, height: Pixels) -> usize
pub fn bytes_required_for_size_and_format(     width: Pixels,     height: Pixels,     format: PdfBitmapFormat, ) -> usize

## PdfRect
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfRect.html
pub const fn new(     bottom: PdfPoints,     left: PdfPoints,     top: PdfPoints,     right: PdfPoints, ) -> Self
pub const fn new_from_values(     bottom: f32,     left: f32,     top: f32,     right: f32, ) -> Self
pub const fn left(&self) -> PdfPoints
pub const fn right(&self) -> PdfPoints
pub const fn bottom(&self) -> PdfPoints
pub const fn top(&self) -> PdfPoints
pub fn width(&self) -> PdfPoints
pub fn height(&self) -> PdfPoints
pub fn transform(&self, matrix: PdfMatrix) -> PdfRect
pub fn to_quad_points(&self) -> PdfQuadPoints

## PdfPageObjectCommon
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/trait.PdfPageObjectCommon.html


## PdfPageObjectsCommon
https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/trait.PdfPageObjectsCommon.html


https://docs.rs/crate/pdfium-render/0.9.4/features
Feature flags docs.rs About docs.rs Badges Builds Metadata Shorthand URLs Download Rustdoc JSON Build queue Privacy policy Rust Rust website The Book Standard Library API Reference Rust by Example The Cargo Guide Clippy Documentation pdfium-render 0.9.4 A high-level idiomatic Rust wrapper around Pdfium, the C++ PDF library used by the Google Chromium project. Crate Source Builds Feature flags Documentation Feature flags default image_latest pdfium_latest thread_safe image_025 pdfium_7881 image_api bindings console_log core_graphics flatten image_023 image_024 libc++ libstdc++ paragraph pdfium_5961 pdfium_6015 pdfium_6043 pdfium_6084 pdfium_6110 pdfium_6124 pdfium_6164 pdfium_6259 pdfium_6295 pdfium_6337 pdfium_6406 pdfium_6490 pdfium_6555 pdfium_6569 pdfium_6611 pdfium_6666 pdfium_6721 pdfium_6996 pdfium_7123 pdfium_7215 pdfium_7350 pdfium_7543 pdfium_7763 pdfium_enable_v8 pdfium_enable_xfa pdfium_future pdfium_use_skia pdfium_use_win32 static pdfium-render There is currently very little information to present on this page because Docs.rs has only limited support for extracting structured feature metadata from Cargo crates. This issue is tracked in Rust RFC #3416 . Check this library's main docs , readme , and Cargo.toml in case its authors have documentation for features available there instead. This version has 44 feature flags, 6 of them enabled by default . default image_latest (default) pdfium_latest (default) thread_safe (default) image_latest (default) image_025 (default) pdfium_latest (default) pdfium_7881 (default) thread_safe (default) This feature flag does not enable additional features. image_025 (default) dep: image_025 (default) image_api (default) pdfium_7881 (default) This feature flag does not enable additional features. image_api (default) This feature flag does not enable additional features. bindings dep: bindgen console_log This feature flag does not enable additional features. core_graphics static flatten This feature flag does not enable additional features. image_023 dep: image_023 image_api (default) image_024 dep: image_024 image_api (default) libc++ static libstdc++ static paragraph This feature flag does not enable additional features. pdfium_5961 This feature flag does not enable additional features. pdfium_6015 This feature flag does not enable additional features. pdfium_6043 This feature flag does not enable additional features. pdfium_6084 This feature flag does not enable additional features. pdfium_6110 This feature flag does not enable additional features. pdfium_6124 This feature flag does not enable additional features. pdfium_6164 This feature flag does not enable additional features. pdfium_6259 This feature flag does not enable additional features. pdfium_6295 This feature flag does not enable additional features. pdfium_6337 This feature flag does not enable additional features. pdfium_6406 This feature flag does not enable additional features. pdfium_6490 This feature flag does not enable additional features. pdfium_6555 This feature flag does not enable additional features. pdfium_6569 This feature flag does not enable additional features. pdfium_6611 This feature flag does not enable additional features. pdfium_6666 This feature flag does not enable additional features. pdfium_6721 This feature flag does not enable additional features. pdfium_6996 This feature flag does not enable additional features. pdfium_7123 This feature flag does not enable additional features. pdfium_7215 This feature flag does not enable additional features. pdfium_7350 This feature flag does not enable additional features. pdfium_7543 This feature flag does not enable additional features. pdfium_7763 This feature flag does not enable additional features. pdfium_enable_v8 This feature flag does not enable additional features. pdfium_enable_xfa This feature flag does not enable additional features. pdfium_future pdfium_enable_v8 pdfium_enable_xfa pdfium_use_skia pdfium_use_skia This feature flag does not enable additional features. pdfium_use_win32 dep: windows static This feature flag does not enable additional features.

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/trait.PdfPageObjectCommon.html
fn bounds(&self) -> Result<PdfQuadPoints, PdfiumError>

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/trait.PdfPageObjectsCommon.html
fn iter(&'a self) -> PdfPageObjectsIterator<'a> ⓘ
fn bounds(&'a self) -> PdfRect

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPagePathObjectSegments.html
pub fn transform(&self, matrix: PdfMatrix) -> PdfPagePathObjectSegments<'a>
fn iter(&'a self) -> PdfPathSegmentsIterator<'a> ⓘ

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfQuadPoints.html
pub const fn new(     x1: PdfPoints,     y1: PdfPoints,     x2: PdfPoints,     y2: PdfPoints,     x3: PdfPoints,     y3: PdfPoints,     x4: PdfPoints,     y4: PdfPoints, ) -> Self
pub const fn new_from_values(     x1: f32,     y1: f32,     x2: f32,     y2: f32,     x3: f32,     y3: f32,     x4: f32,     y4: f32, ) -> Self
pub fn x1(&self) -> PdfPoints
pub fn y1(&self) -> PdfPoints
pub fn transform(&self, matrix: PdfMatrix) -> PdfQuadPoints

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfMatrix.html
pub fn a(&self) -> PdfMatrixValue
pub fn b(&self) -> PdfMatrixValue
pub fn c(&self) -> PdfMatrixValue
pub fn d(&self) -> PdfMatrixValue
pub fn e(&self) -> PdfMatrixValue
pub fn f(&self) -> PdfMatrixValue
pub fn apply_to_points(     &self,     x: PdfPoints,     y: PdfPoints, ) -> (PdfPoints, PdfPoints)
pub fn transform(     self,     a: PdfMatrixValue,     b: PdfMatrixValue,     c: PdfMatrixValue,     d: PdfMatrixValue,     e: PdfMatrixValue,     f: PdfMatrixValue, ) -> Result<Self, PdfiumError>
pub fn apply_matrix(self, matrix: PdfMatrix) -> Result<Self, PdfiumError>
pub fn reset_matrix(self, matrix: PdfMatrix) -> Result<Self, PdfiumError>
pub fn reset_matrix_to_identity(self) -> Result<Self, PdfiumError>
pub fn matrix(&self) -> Result<PdfMatrix, PdfiumError>

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfDocument.html
pub fn metadata(&self) -> &PdfMetadata<'_>
pub fn pages(&self) -> &PdfPages<'a>
pub fn pages_mut(&mut self) -> &mut PdfPages<'a>
pub fn save_to_writer<W: Write + 'static>(     &self,     writer: &mut W, ) -> Result<(), PdfiumError>

https://docs.rs/lopdf/0.45.0/lopdf/struct.Document.html
pub fn adjust_zero_pages(&mut self)
pub fn is_encrypted(&self) -> bool
pub fn get_pages(&self) -> BTreeMap<u32, ObjectId>
pub fn page_iter(&self) -> impl Iterator<Item = ObjectId> + '_
pub fn get_page_contents(&self, page_id: ObjectId) -> Vec<ObjectId> ⓘ
pub fn get_page_content(&self, page_id: ObjectId) -> Vec<u8> ⓘ
pub fn get_page_content_with_limit(     &self,     page_id: ObjectId,     max_decompressed_size: usize, ) -> Result<Vec<u8>>
pub fn decode_text(encoding: &Encoding<'_>, bytes: &[u8]) -> Result<String>
pub fn encode_text(encoding: &Encoding<'_>, text: &str) -> Vec<u8> ⓘ
pub fn delete_pages(&mut self, page_numbers: &[u32])
pub fn extract_text(&self, page_numbers: &[u32]) -> Result<String>
pub fn replace_text(     &mut self,     page_number: u32,     text: &str,     other_text: &str,     default_str: Option<&str>, ) -> Result<()>
pub fn replace_partial_text(     &mut self,     page_number: u32,     search_text: &str,     replacement_text: &str,     default_char: Option<&str>, ) -> Result<usize>
pub fn load_mem(buffer: &[u8]) -> Result<Document>
pub fn load_mem_with_options(     buffer: &[u8],     options: LoadOptions, ) -> Result<Document>
pub fn load_mem_with_password(buffer: &[u8], password: &str) -> Result<Document>
pub fn load_metadata<P: AsRef<Path>>(path: P) -> Result<PdfMetadata>
pub fn load_metadata_with_password<P: AsRef<Path>>(     path: P,     password: &str, ) -> Result<PdfMetadata>
pub fn load_metadata_from<R: Read>(source: R) -> Result<PdfMetadata>
pub fn load_metadata_from_with_password<R: Read>(     source: R,     password: &str, ) -> Result<PdfMetadata>
pub fn load_metadata_mem(buffer: &[u8]) -> Result<PdfMetadata>
pub fn load_metadata_mem_with_password(     buffer: &[u8],     password: &str, ) -> Result<PdfMetadata>

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPathSegment.html
pub struct PdfPathSegment<'a> { /* private fields */ }
pub fn segment_type(&self) -> PdfPathSegmentType
pub fn is_close(&self) -> bool
pub fn point(&self) -> (PdfPoints, PdfPoints)
pub fn x(&self) -> PdfPoints
pub fn y(&self) -> PdfPoints

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/enum.PdfPathSegmentType.html
pub enum PdfPathSegmentType {
    Unknown = -1,
    LineTo = 0,
    BezierTo = 1,
    MoveTo = 2,
}


https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPagePathObjectSegments.html
pub struct PdfPagePathObjectSegments<'a> { /* private fields */ }
pub fn transform(&self, matrix: PdfMatrix) -> PdfPagePathObjectSegments<'a>
pub fn raw(&self) -> PdfPagePathObjectSegments<'a>
fn len(&self) -> PdfPathSegmentIndex
fn get(     &self,     index: PdfPathSegmentIndex, ) -> Result<PdfPathSegment<'a>, PdfiumError>
fn iter(&'a self) -> PdfPathSegmentsIterator<'a> ⓘ

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPoints.html
pub struct PdfPoints {
    pub value: f32,
}
pub const ZERO: PdfPoints
pub const MAX: PdfPoints
pub const MIN: PdfPoints
pub const fn new(value: f32) -> Self
pub const fn zero() -> Self
pub const fn max() -> Self
pub const fn min() -> Self
pub fn from_inches(inches: f32) -> Self
pub fn from_cm(cm: f32) -> Self
pub fn from_mm(mm: f32) -> Self
pub fn to_inches(&self) -> f32
pub fn to_cm(&self) -> f32
pub fn to_mm(&self) -> f32
pub fn abs(&self) -> PdfPoints

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPageTextChars.html
pub struct PdfPageTextChars<'a> { /* private fields */ }
pub fn first_char_index(&self) -> Option<PdfPageTextCharIndex>
pub fn len(&self) -> PdfPageTextCharIndex
pub fn last_char_index(&self) -> Option<PdfPageTextCharIndex>
pub fn is_empty(&self) -> bool
pub fn get<'b>(     &'b self,     index: PdfPageTextCharIndex, ) -> Result<PdfPageTextChar<'a>, PdfiumError>
pub fn first(&self) -> Result<PdfPageTextChar<'a>, PdfiumError>
pub fn last(&self) -> Result<PdfPageTextChar<'a>, PdfiumError>
pub fn get_char_at_point(     &self,     x: PdfPoints,     y: PdfPoints, ) -> Option<PdfPageTextChar<'_>>
pub fn get_char_near_point(     &self,     x: PdfPoints,     tolerance_x: PdfPoints,     y: PdfPoints,     tolerance_y: PdfPoints, ) -> Option<PdfPageTextChar<'_>>
pub fn iter(&self) -> PdfPageTextCharsIterator<'_> ⓘ

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfQuadPoints.html
pub struct PdfQuadPoints { /* private fields */ }
pub const ZERO: PdfQuadPoints
pub const fn new(     x1: PdfPoints,     y1: PdfPoints,     x2: PdfPoints,     y2: PdfPoints,     x3: PdfPoints,     y3: PdfPoints,     x4: PdfPoints,     y4: PdfPoints, ) -> Self
pub const fn new_from_values(     x1: f32,     y1: f32,     x2: f32,     y2: f32,     x3: f32,     y3: f32,     x4: f32,     y4: f32, ) -> Self
pub const fn zero() -> Self
pub fn from_rect(rect: &PdfRect) -> Self
pub fn x1(&self) -> PdfPoints
pub fn y1(&self) -> PdfPoints
pub fn x2(&self) -> PdfPoints
pub fn y2(&self) -> PdfPoints
pub fn x3(&self) -> PdfPoints
pub fn y3(&self) -> PdfPoints
pub fn x4(&self) -> PdfPoints
pub fn y4(&self) -> PdfPoints
pub fn left(&self) -> PdfPoints
pub fn right(&self) -> PdfPoints
pub fn bottom(&self) -> PdfPoints
pub fn top(&self) -> PdfPoints
pub fn width(&self) -> PdfPoints
pub fn height(&self) -> PdfPoints
pub fn transform(&self, matrix: PdfMatrix) -> PdfQuadPoints
pub fn to_rect(&self) -> PdfRect

https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfMetadata.html
pub struct PdfMetadata<'a> { /* private fields */ }
pub fn len(&self) -> usize
pub fn is_empty(&self) -> bool
pub fn get(     &self,     tag: PdfDocumentMetadataTagType, ) -> Option<PdfDocumentMetadataTag>
pub fn iter(&self) -> Iter<'_, PdfDocumentMetadataTag> ⓘ
