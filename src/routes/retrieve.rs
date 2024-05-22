use std::fs::File;
use std::io::ErrorKind::NotFound;
use std::path::Path;

use crate::get_upload_dir;
use crate::models::paste_id::PasteId;
use crate::models::pretty_syntax::PasteIdSyntax;
use crate::models::response_wrapper::ResponseWrapper;

#[get("/<id>", rank = 2)]
pub async fn retrieve(id: PasteId<'_>) -> ResponseWrapper<File> {
    retrieve_inner(&id.to_string(), None).await
}

// rank 1 here because this would be more oftenly used
#[get("/<id_ext>", rank = 1)]
pub async fn retrieve_ext(id_ext: PasteIdSyntax<'_>) -> ResponseWrapper<File> {
    retrieve_inner(id_ext.get_fname(), Some(id_ext.get_ext())).await
}

pub async fn retrieve_inner(
    id: &str,
    ext: Option<&str>,
) -> ResponseWrapper<File> {
    let filepath = Path::new(&get_upload_dir()).join(id);

    let modified_date =
        match std::fs::metadata(&filepath).and_then(|m| m.modified()) {
            Ok(v) => v,
            Err(e) if e.kind() == NotFound => {
                return ResponseWrapper::not_found(id);
            }
            Err(e) => {
                return ResponseWrapper::server_error(e.to_string());
            }
        };

    let file = match File::open(&filepath) {
        Ok(v) => v,
        Err(e) if e.kind() == NotFound => {
            return ResponseWrapper::not_found(id)
        }
        Err(e) => {
            return ResponseWrapper::server_error(e.to_string());
        }
    };

    let mime = ext.and_then(|ext| mime_guess::from_ext(ext).first());

    if let Some(mime) = mime {
        ResponseWrapper::mime_paste_response(
            file,
            modified_date,
            mime.essence_str().to_string(),
        )
    } else {
        ResponseWrapper::raw_paste_response(file, modified_date)
    }
}
