use clap::Parser;
use regex::Regex;
use std::io::{self, Read};
use unicode_width::UnicodeWidthStr;

#[derive(Parser)]
#[command(name = "tablify")]
#[command(about = "Format input text into table format")]
struct Args {
    #[arg(short = 's', long = "separator", help = "Custom character to use as separator")]
    separator: Option<char>,

    #[arg(short = 'p', long = "separator-pattern", help = "Regular expression pattern for splitting input")]
    separator_pattern: Option<String>,

    #[arg(long = "header", help = "Treat first line as header row")]
    header: bool,

    #[arg(long = "columns", help = "Custom column names (comma-separated)")]
    columns: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return;
    }
    
    let rows = parse_input(&lines, &args);
    let table = format_table(rows, &args);
    print!("{}", table);
}

fn parse_input(lines: &[&str], args: &Args) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    
    for line in lines {
        let columns = if let Some(pattern) = &args.separator_pattern {
            let regex = Regex::new(pattern).expect("Invalid regex pattern");
            regex.split(line).map(|s| s.to_string()).collect()
        } else if let Some(sep) = args.separator {
            line.split(sep).map(|s| s.to_string()).collect()
        } else {
            line.split('\t').map(|s| s.to_string()).collect()
        };
        rows.push(columns);
    }
    
    rows
}

fn calculate_display_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

fn format_table(mut rows: Vec<Vec<String>>, args: &Args) -> String {
    if rows.is_empty() {
        return String::new();
    }
    
    let num_cols = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    
    for row in &mut rows {
        while row.len() < num_cols {
            row.push(String::new());
        }
    }
    
    let mut col_widths = vec![0; num_cols];
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            let width = calculate_display_width(cell);
            if width > col_widths[i] {
                col_widths[i] = width;
            }
        }
    }
    
    let mut result = String::new();
    let mut data_start_index = 0;
    
    if args.header || args.columns.is_some() {
        let header_row = if let Some(columns) = &args.columns {
            columns.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            rows[0].clone()
        };
        
        if args.columns.is_some() {
            data_start_index = 0;
        } else {
            data_start_index = 1;
        }
        
        for (i, width) in col_widths.iter_mut().enumerate() {
            if i < header_row.len() {
                let header_width = calculate_display_width(&header_row[i]);
                if header_width > *width {
                    *width = header_width;
                }
            }
        }
        
        result.push_str(&format_row(&header_row, &col_widths));
        result.push_str(&format_separator(&col_widths));
    }
    
    for row in &rows[data_start_index..] {
        result.push_str(&format_row(row, &col_widths));
    }
    
    result
}

fn format_row(row: &[String], col_widths: &[usize]) -> String {
    let mut result = String::from("| ");
    
    for (i, cell) in row.iter().enumerate() {
        if i < col_widths.len() {
            let cell_width = calculate_display_width(cell);
            let padding = col_widths[i] - cell_width;
            result.push_str(cell);
            result.push_str(&" ".repeat(padding));
            result.push_str(" | ");
        }
    }
    
    result.push('\n');
    result
}

fn format_separator(col_widths: &[usize]) -> String {
    let mut result = String::from("+");
    
    for &width in col_widths {
        result.push_str(&"-".repeat(width + 2));
        result.push('+');
    }
    
    result.push('\n');
    result
}
