/*
Copyright (C) 2025 Mateusz Mazur (Mazurel) <mateusz.mazur@e.email>

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, see
<https://www.gnu.org/licenses/>.
*/

use js_sys::{wasm_bindgen::prelude::*, ArrayBuffer, Function, Uint8Array};
use log::*;
use web_sys::{Blob, BlobPropertyBag, FileReader, HtmlAnchorElement, HtmlInputElement, Url};
use yew::Callback;

/// Start file download for user of `data` bytes, named as a `filename`.
///
/// This method interfaces with `web_sys` and `js_sys` to create relevant objects,
/// uses them and cleans up after itself.
pub fn download_binary_data<'a, T>(data: T, filename: &str, mime_type: &str) -> Result<(), String>
where
    T: Into<&'a [u8]>,
{
    // Prepare binary data
    let u8_array = Uint8Array::from(data.into());
    let js_array = js_sys::Array::new();
    js_array.push(&u8_array.buffer());
    let blob_options = BlobPropertyBag::new();
    blob_options.set_type(mime_type);
    let blob = Blob::new_with_u8_array_sequence_and_options(&js_array, &blob_options)
        .map_err(|err| format!("Failed creating blob: {err:?}"))?;

    // Generate URL
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|err| format!("Failed creating URL from blob: {err:?}"))?;

    // Create <a> element in the document
    let window = web_sys::window().ok_or_else(|| "Could not get window".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Could not get document".to_string())?;
    let body = document
        .body()
        .ok_or_else(|| "Could not get document body".to_string())?;

    let a: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|err| format!("Failed to create anchor: {err:?}"))?
        .dyn_into()
        .map_err(|err| format!("Failed to cast anchor: {err:?}"))?;

    a.set_href(&url);
    a.set_download(filename);

    // Click on the anchor -> get the file
    body.append_child(&a)
        .map_err(|err| format!("Failed to append child to the body: {err:?}"))?;

    a.click();

    // Cleanup
    body.remove_child(&a)
        .map_err(|err| format!("Failed to remove child from the body: {err:?}"))?;
    Url::revoke_object_url(&url).map_err(|err| format!("Failed to revoke URL: {err:?}"))?;

    Ok(())
}

pub fn upload_binary_data(
    filename: &str,
    accepts: &str,
    binary_data_cb: Callback<Vec<u8>>,
) -> Result<(), String> {
    // Create <input> element in the document
    let window = web_sys::window().ok_or_else(|| "Could not get window".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Could not get document".to_string())?;
    let body = document
        .body()
        .ok_or_else(|| "Could not get document body".to_string())?;

    let input: HtmlInputElement = document
        .create_element("input")
        .map_err(|err| format!("Failed to create anchor: {err:?}"))?
        .dyn_into()
        .map_err(|err| format!("Failed to cast anchor: {err:?}"))?;
    body.append_child(&input)
        .map_err(|err| format!("Failed to append child: {err:?}"))?;

    input.set_type("file");
    input.set_id(filename);
    input.set_name(filename);
    input.set_accept(accepts);

    let on_input_changed: Function = {
        let input = input.clone();
        Closure::<dyn Fn() -> ()>::new(move || {
            if let Some(files) = input.files() {
                if files.length() > 0 {
                    let file = files.get(0).unwrap();
                    if let Ok(file_reader) = FileReader::new() {
                        // SAFETY: File is essentially a Blob
                        let file_as_blob: Blob = file.unchecked_into();
                        file_reader
                            .read_as_array_buffer(&file_as_blob)
                            .unwrap_or_else(|_| {
                                warn!("Failed `read_as_array_buffer`, blob: {file_as_blob:?}");
                            });

                        let on_file_load_cb: Function = {
                            let binary_data_cb = binary_data_cb.clone();
                            let file_reader = file_reader.clone();
                            Closure::<dyn Fn(web_sys::ProgressEvent) -> ()>::new(move |_| {
                                if let Ok(file_result) = file_reader.result() {
                                    let array_buffer_of_file: ArrayBuffer =
                                        file_result.unchecked_into();
                                    let byte_view_of_buffer =
                                        Uint8Array::new(&array_buffer_of_file);
                                    binary_data_cb.emit(byte_view_of_buffer.to_vec());
                                }
                            })
                            .into_js_value()
                            .unchecked_into()
                        };
                        file_reader.set_onload(Some(&on_file_load_cb));

                        body.remove_child(&input).map(|_| ()).unwrap_or_else(|err| {
                            warn!("Failed to remove input from the body: {err:?}");
                        });
                    }
                }
            }
        })
        .into_js_value()
        .unchecked_into()
    };
    input
        .add_event_listener_with_callback("change", &on_input_changed)
        .map_err(|err| format!("Failed adding input callback to the input element: {err:?}"))?;

    input.click();

    Ok(())
}
