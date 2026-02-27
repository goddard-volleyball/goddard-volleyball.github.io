use calamine::{open_workbook, Reader, Xlsx, Data};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug)]
struct StandingsRow {
    t: u8,
    w: u8,
    l: u8,
    p: f64,
    b: f32,
}

struct ScheduleRow {
    date: String,
    t1: u8,
    t2: u8,
    t1w: u8,
    t1l: u8
}


fn to_u8(cell: &Data) -> u8 {
    match cell {
        Data::Empty => 0,
        Data::Float(v) => *v as u8,
        Data::Int(v) => *v as u8,
        _ => unreachable!("Expected numeric cell"),
    }
}

fn to_f64(cell: &Data) -> f64 {
    match cell {
        Data::Empty => 0.0,
        Data::Float(v) => *v,
        Data::Int(v) => *v as f64,
        _ => unreachable!("Expected numeric cell"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // read in excel sheet
    let mut workbook: Xlsx<_> = open_workbook("/Users/tylerlee/Code/goddard-volleyball.github.io/docs/VB Standings 2025-2026.xlsx")?;
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
    
    Ok(())
}



