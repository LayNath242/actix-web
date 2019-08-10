use std::fs::File;
use std::io::Write;
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpResponse};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::time::{Duration, SystemTime};




pub fn images_uploader(multipart: Multipart) -> impl Future<Item = HttpResponse, Error = Error> {
    let mulpar = multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| {
            save_file(field).into_stream()
        })
        .flatten()
        .collect()
        .map(|_| HttpResponse::Ok().json("upload sucess"))
        .map_err(|e| {
            println!("upload multipart failed: {}", e);
            e
        });
    mulpar
}



pub fn save_file(field: Field) -> impl Future<Item = String, Error = Error> {
    let filename = field.content_disposition().unwrap().get_filename().unwrap().replace(' ', "_").to_string();
    let code = match SystemTime::now().duration_since(<std::time::SystemTime>::UNIX_EPOCH) {
        Ok(now) => now,
        Err(_) => Duration::new(0, 0),
    };
    let file = match File::create("storage/images/".to_owned() + &filename + &code.as_millis().to_string() + ".png") {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e)))
    };
    Either::B(
        field
            .fold(file, move |mut file, bytes| {
                web::block(move || {
                    file
                        .write_all(bytes.as_ref())
                        .map_err(|e| {
                            MultipartError::Payload(error::PayloadError::Io(e))
                        })?;
                    Ok(file)
                }) 
                .map_err(|e: error::BlockingError<MultipartError>| {
                    match e {
                        error::BlockingError::Error(e) => e,
                        error::BlockingError::Canceled => MultipartError::Incomplete,
                    }
                })
            })
            .map(|_| filename )
            .map_err(|e| {
                error::ErrorInternalServerError(e)
            }),
    )
}
