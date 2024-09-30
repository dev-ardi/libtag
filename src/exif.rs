// use rexif::{ExifData, ExifEntry, TagValue};
// pub fn background_exif(db: &mut SqliteConnection) -> eyre::Result<()> {
//     use crate::diesel::*;
//     use crate::schema::files::dsl::*;
//
//     let r = files
//         .filter(is_exif.is_null())
//         .select((path, id))
//         .load::<(String, i32)>(db)
//         .wrap_err("Error reading exif files from the database")?;
//
//     let mut out = Vec::with_capacity(r.len());
//
//     r.into_par_iter()
//         .map(|(fpath, fid)| (rexif::parse_file(fpath).map(|ex| ex.entries), fid))
//         .collect_into_vec(&mut out);
//
//     let now = select(diesel::dsl::now).get_result::<SystemTime>(conn)?;
//     for (exif, fid) in out {
//         match exif {
//             Ok(exif) => {
//                 #[derive(Debug, Clone, Default, diesel::AsChangeset)]
//                 #[diesel(table_name=files)]
//                 struct UpdFile {
//                     pub description: Option<String>,
//                     pub gps_altitude: Option<i32>,
//                     pub gps_latitude: Option<i32>,
//                     pub x_res: Option<i32>,
//                     pub y_res: Option<i32>,
//                     pub is_exif: Option<bool>,
//                     pub created: Option<i64>,
//                     pub updated_at: Option<i64>,
//                 }
//
//                 let mut file = UpdFile::default();
//                 for entry in exif {
//                     let entry: ExifEntry = entry;
//                     match entry.tag {
//                         rexif::ExifTag::XResolution => {
//                             file.x_res = try_as_i32(entry.value);
//                         }
//                         rexif::ExifTag::YResolution => {
//                             file.x_res = try_as_i32(entry.value);
//                         }
//                         rexif::ExifTag::YResolution => todo!(),
//                         rexif::ExifTag::DateTime => todo!(),
//                         rexif::ExifTag::ImageDescription => {}
//                         rexif::ExifTag::Orientation => todo!(),
//                         rexif::ExifTag::DateTimeOriginal => todo!(),
//                         rexif::ExifTag::DateTimeDigitized => todo!(),
//                         rexif::ExifTag::GPSLatitude => {}
//                         rexif::ExifTag::GPSLongitude => {}
//                         _ => continue,
//                     }
//                 }
//             }
//             Err(e) => {
//                 debug!("Couldn't load exif metadata from file id {fid}: {e}");
//                 todo!("update file's is_exif")
//             }
//         }
//     }
//     Ok(())
// }
//
// fn try_as_i64(data: TagValue) -> Option<i64> {
//     match data {
//         TagValue::Ascii(s) => s.parse().ok(),
//
//         TagValue::U8(vec) => vec.first().map(|&x| x as i64),
//         TagValue::U16(vec) => vec.first().map(|&x| x as i64),
//         TagValue::U32(vec) => vec.first().map(|&x| x as i64),
//         TagValue::I8(vec) => vec.first().map(|&x| x as i64),
//         TagValue::I16(vec) => vec.first().map(|&x| x as i64),
//         TagValue::I32(vec) => vec.first().map(|&x| x as i64),
//
//         TagValue::URational(vec) => vec
//             .first()
//             .map(|rat| (rat.numerator / rat.denominator) as i64),
//         TagValue::IRational(vec) => vec
//             .first()
//             .map(|rat| (rat.numerator / rat.denominator) as i64),
//         _ => None,
//     }
// }
//
// fn try_as_i32(data: TagValue) -> Option<i32> {
//     match data {
//         TagValue::Ascii(s) => s.parse().ok(),
//
//         TagValue::U8(vec) => vec.first().map(|&x| x as i32),
//         TagValue::U16(vec) => vec.first().map(|&x| x as i32),
//         TagValue::U32(vec) => vec.first().map(|&x| x as i32),
//         TagValue::I8(vec) => vec.first().map(|&x| x as i32),
//         TagValue::I16(vec) => vec.first().map(|&x| x as i32),
//         TagValue::I32(vec) => vec.first().map(|&x| x as i32),
//
//         TagValue::URational(vec) => vec
//             .first()
//             .map(|rat| (rat.numerator / rat.denominator) as i32),
//         TagValue::IRational(vec) => vec
//             .first()
//             .map(|rat| (rat.numerator / rat.denominator) as i32),
//         _ => None,
//     }
// }
