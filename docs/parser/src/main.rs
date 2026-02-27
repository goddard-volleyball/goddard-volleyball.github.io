use calamine::{open_workbook, Data, Reader, Xlsx};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

struct StandingsRow {
    t: u8,
    w: u8,
    l: u8,
    p: f64,
    b: f32,
}

struct ScheduleRow {
    date: String,
    ta: u8,
    tb: u8,
    tc: u8,
    td: u8,
    te: u8,
    tf: u8,
    tg: u8,
    th: u8,
}

fn to_u8(cell: &Data) -> u8 {
    match cell {
        Data::Empty => 0,
        Data::Float(v) => *v as u8,
        Data::Int(v) => *v as u8,
        _ => unreachable!("Cell could not be converted to unsigned integer"),
    }
}

fn to_f64(cell: &Data) -> f64 {
    match cell {
        Data::Empty => 0.0,
        Data::Float(v) => *v,
        Data::Int(v) => *v as f64,
        _ => unreachable!("Cell could not be converted to float"),
    }
}

fn to_string(cell: &Data) -> String {
    match cell {
        Data::Empty => "".to_string(),
        Data::Float(v) => v.to_string(),
        Data::Int(v) => v.to_string(),
        Data::String(v) => v.to_string(),
        Data::DateTime(v) => {
            let dt = v.to_ymd_hms_milli();
            format!("{}/{}/{}", dt.1, dt.2, dt.0)
        },
        _ => unreachable!("Cell could not be converted to string"),
    }
}

fn gen_standings() -> Result<(), Box<dyn Error>> {
    // read in excel sheet
    let mut workbook: Xlsx<_> = open_workbook("../2026-standings.xlsx")?;
    let sheet = workbook.sheet_names()[0].clone();
    let ws_range = workbook.worksheet_range(&sheet)?;

    // parse standings data
    let mut rows: Vec<StandingsRow> = Vec::new();
    for i in 1..9 {
        rows.push(StandingsRow {
            t: to_u8(&ws_range[i][29]),
            w: to_u8(&ws_range[i][30]),
            l: to_u8(&ws_range[i][31]),
            p: to_f64(&ws_range[i][32]),
            b: 0.0,
        });
    }

    // sort by percentage
    rows.sort_by(|a, b| b.p.partial_cmp(&a.p).unwrap());

    // update games behind
    for i in 1..8 {
        rows[i].b = ((rows[0].w - rows[i].w) + (rows[i].l - rows[0].l)) as f32 / 2.0;
    }

    // output standings to csv
    let file = File::create("../../static/processed-standings.csv")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Teams,Wins,Losses,Percentage,Behind")?;
    for r in &rows {
        writeln!(writer, "{},{},{},{},{}", r.t, r.w, r.l, r.p, r.b)?;
    }

    Ok(())
}

fn gen_schedule() -> Result<(), Box<dyn Error>> {
    // read in excel sheet
    let mut workbook: Xlsx<_> = open_workbook("../2026-schedule.xlsx")?;
    let sheet = workbook.sheet_names()[0].clone();
    let ws_range = workbook.worksheet_range(&sheet)?;

    // parse standings data
    let mut rows: Vec<ScheduleRow> = Vec::new();
    for i in 3..36 {
        // Check to make sure the row is worth parsing
        match &ws_range[i][2] {
            Data::Float(_) => {
                // println!("{} Valid float {}", i, v);
                
                rows.push(ScheduleRow {
                    date: to_string(&ws_range[i][0]),
                    ta: to_u8(&ws_range[i][2]),
                    tb: to_u8(&ws_range[i][4]),
                    tc: to_u8(&ws_range[i][5]),
                    td: to_u8(&ws_range[i][7]),
                    te: to_u8(&ws_range[i][8]),
                    tf: to_u8(&ws_range[i][10]),
                    tg: to_u8(&ws_range[i][11]),
                    th: to_u8(&ws_range[i][13]),
                });
            },
            _ => {
                // println!("{} Not seeing valid data", i);
                continue // skip row
            },
        };
    }

    // output schedule to csv
    let file = File::create("../../static/processed-schedule.csv")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Date,A,B,C,D,E,F,G,H")?;
    for r in &rows {
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{}",
            r.date, r.ta, r.tb, r.tc, r.td, r.te, r.tf, r.tg, r.th
        )?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    gen_standings().and_then(|_| gen_schedule())
}
