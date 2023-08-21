use tzf_rs::DefaultFinder;

fn main() {
    let finder: DefaultFinder = DefaultFinder::new();
    let mut s: redcon::Server<DefaultFinder> = redcon::listen("127.0.0.1:6380", finder).unwrap();
    s.command = Some(|conn, db, args| {
        let name = String::from_utf8_lossy(&args[0]).to_lowercase();
        match name.as_str() {
            "ping" => conn.write_string("PONG"),
            "get_tz" => {
                if args.len() != 3 {
                    conn.write_error("ERR wrong number of arguments");
                    return;
                }
                let lng_result: Result<f64, _> = std::str::from_utf8(&args[1]).unwrap().parse();
                if lng_result.is_err() {
                    conn.write_error("Failed to parse longitude");
                    return;
                }
                let lng = lng_result.unwrap();
                let lat_result: Result<f64, _> = std::str::from_utf8(&args[2]).unwrap().parse();
                if lat_result.is_err() {
                    conn.write_error("Failed to parse latitude");
                    return;
                }
                let lat = lat_result.unwrap();
                conn.write_string(db.get_tz_name(lng, lat));
            }
            "get_tzs" => {
                if args.len() != 3 {
                    conn.write_error("ERR wrong number of arguments");
                    return;
                }
                let lng_result: Result<f64, _> = std::str::from_utf8(&args[1]).unwrap().parse();
                if lng_result.is_err() {
                    conn.write_error("Failed to parse longitude");
                    return;
                }
                let lng = lng_result.unwrap();
                let lat_result: Result<f64, _> = std::str::from_utf8(&args[2]).unwrap().parse();
                if lat_result.is_err() {
                    conn.write_error("Failed to parse latitude");
                    return;
                }
                let lat = lat_result.unwrap();
                // conn.write_string(db.get_tz_name(lng, lat));
                let tzs = db.get_tz_names(lng, lat);
                conn.write_array(tzs.len());
                for tz in tzs {
                    conn.write_bulk(tz.as_bytes());
                }
            }
            _ => conn.write_error("ERR unknown command"),
        }
    });
    println!("Serving at {}", s.local_addr());
    s.serve().unwrap();
}
