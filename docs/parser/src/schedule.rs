

// parse schedule data
let mut rows: Vec<ScheduleRow> = Vec::new();
// for i in 3..24 {
//     rows.push(ScheduleRow {
//         date: to_u8(&ws_range[1][i]),
//         w: to_u8(&ws_range[1][30]),
//         l: to_u8(&ws_range[1][31]),
//         p: to_f64(&ws_range[1][32]),
//         b: 0.0,
//     });
// }

// output schedule to csv
let file = File::create("../../static/processed-schedule.csv")?;
let mut writer = BufWriter::new(file);