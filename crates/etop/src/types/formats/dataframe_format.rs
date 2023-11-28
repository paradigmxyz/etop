use crate::{ColumnFormat, ColumnFormatShorthand, EtopError};
use polars::prelude::*;

const DEFAULT_TABLE_HEIGHT: usize = 30;

#[derive(Debug)]
pub struct DataFrameFormat {
    pub column_formats: Option<Vec<ColumnFormatShorthand>>,
    pub column_delimiter: String,
    pub header_separator_delimiter: String,
    pub header_separator_char: char,
    pub include_header_row: bool,
    pub include_header_separator_row: bool,
    pub include_summary_row: bool,
    pub include_summary_separator_row: bool,
    pub render_height: Option<usize>,
    pub max_render_width: Option<usize>,
}

impl Default for DataFrameFormat {
    fn default() -> DataFrameFormat {
        DataFrameFormat {
            column_formats: None,
            column_delimiter: "  │  ".to_string(),
            header_separator_delimiter: "──┼──".to_string(),
            header_separator_char: '─',
            include_header_row: true,
            include_header_separator_row: true,
            include_summary_row: false,
            include_summary_separator_row: false,
            render_height: None,
            max_render_width: None,
        }
    }
}

#[derive(Debug)]
pub struct DataFrameFormatFinal {
    pub column_formats: Vec<ColumnFormat>,
    pub column_delimiter: String,
    pub header_separator_delimiter: String,
    pub header_separator_char: char,
    pub include_header_row: bool,
    pub include_header_separator_row: bool,
    pub include_summary_row: bool,
    pub include_summary_separator_row: bool,
    pub render_height: usize,
    pub max_render_width: usize,
}

impl DataFrameFormat {
    /// format dataframe as String
    pub(crate) fn format(&self, df: DataFrame) -> Result<String, EtopError> {
        let fmt = self.finalize(df.clone())?;
        fmt.format(df)
    }

    /// fill missing format information based on dataframe
    fn finalize(&self, df: DataFrame) -> Result<DataFrameFormatFinal, EtopError> {
        let schema = df.schema();
        let column_formats: Vec<ColumnFormat> = match &self.column_formats {
            Some(cols) => {
                let mut fmts = Vec::new();
                for col in cols.iter() {
                    let dtype = match schema.get_field(col.name.as_str()) {
                        Some(field) => field.dtype,
                        None => {
                            return Err(EtopError::ColumnMissing(format!(
                                "missing column: {}",
                                col.name
                            )))
                        }
                    };
                    fmts.push(col.clone().finalize(&dtype)?);
                }
                fmts
            }
            None => {
                let fmts: Result<Vec<ColumnFormat>, EtopError> = schema
                    .iter()
                    .map(|(name, dtype)| ColumnFormatShorthand::new().name(name).finalize(dtype))
                    .collect();
                fmts?
            }
        };

        let max_render_width = match self.max_render_width {
            Some(value) => value,
            None => {
                let max_render_width = safe_sum_with_max_on_overflow(
                    column_formats.iter().map(|c| c.get_max_width()).collect(),
                );
                safe_sum_with_max_on_overflow(vec![
                    max_render_width,
                    self.column_delimiter.chars().count() * (column_formats.len() - 1),
                ])
            }
        };
        let fmt = DataFrameFormatFinal {
            column_formats,
            column_delimiter: self.column_delimiter.clone(),
            header_separator_delimiter: self.header_separator_delimiter.clone(),
            header_separator_char: self.header_separator_char,
            include_header_row: self.include_header_row,
            include_header_separator_row: self.include_header_separator_row,
            include_summary_row: self.include_summary_row,
            include_summary_separator_row: self.include_summary_separator_row,
            render_height: self.render_height.unwrap_or(DEFAULT_TABLE_HEIGHT),
            max_render_width,
        };
        Ok(fmt)
    }
}

fn safe_sum_with_max_on_overflow(numbers: Vec<usize>) -> usize {
    let mut sum: usize = 0;
    for number in numbers {
        match sum.checked_add(number) {
            Some(s) => sum = s,
            None => return usize::MAX,
        };
    }
    sum
}

// get number of lines in header
impl DataFrameFormatFinal {
    fn n_header_lines(&self) -> usize {
        // TODO: take an n_used_columns parameter, for if only subset of columns used
        self.column_formats
            .iter()
            .map(|f| f.display_name.chars().filter(|&c| c == '\n').count() + 1)
            .max()
            .unwrap_or(0)
    }

    fn n_data_rows(&self) -> usize {
        self.render_height
            - (self.include_header_row as usize)
                * (self.n_header_lines() + (self.include_header_separator_row as usize))
            - (self.include_summary_row as usize)
                * (1 + (self.include_summary_separator_row as usize))
    }

    fn total_rendered_width(&self, used_widths: &Vec<usize>) -> usize {
        used_widths.iter().sum::<usize>()
            + ((used_widths.len() as i64 - 1).max(0) as usize)
                * self.column_delimiter.chars().count()
    }

    fn render_header_rows(&self, used_widths: &[usize], total_width: usize) -> Vec<String> {
        let n_header_lines = self.n_header_lines();
        let mut rows: Vec<String> = (0..n_header_lines)
            .map(|_| String::with_capacity(total_width))
            .collect();
        for (c, width) in used_widths.iter().enumerate() {
            if c != 0 {
                for row in rows.iter_mut() {
                    row.push_str(self.column_delimiter.as_str());
                }
            }
            let name = self.column_formats[c].display_name.as_str();
            let lines: Vec<String> = name.split('\n').map(|s| s.to_string()).collect();
            let bound = n_header_lines - lines.len();
            for row in rows.iter_mut().take(bound) {
                row.push_str(" ".repeat(*width).as_str());
            }
            for (row, line) in rows.iter_mut().skip(bound).zip(lines) {
                row.push_str(format!("{:>width$}", line, width = width).as_str());
            }
        }

        rows
    }

    fn render_header_separator_row(&self, used_widths: &[usize], total_width: usize) -> String {
        let mut row = String::with_capacity(total_width);
        let separator = self.header_separator_char.to_string();
        for (c, width) in used_widths.iter().enumerate() {
            if c != 0 {
                row.push_str(self.header_separator_delimiter.as_str());
            }
            row.push_str(separator.repeat(*width).as_str());
        }
        row
    }

    fn render_columns(&self, df: DataFrame) -> Result<(Vec<usize>, Vec<Vec<String>>), EtopError> {
        // compute global sizes
        let mut column_min_widths: Vec<usize> = vec![];
        let mut column_max_widths: Vec<usize> = vec![];
        for fmt in self.column_formats.iter() {
            let min_width = fmt.header_width().max(fmt.get_min_width());
            let max_width = fmt.get_max_width();
            if min_width > max_width {
                let msg = format!("min_width > max_width for column: {}", fmt.display_name);
                return Err(EtopError::InvalidFormat(msg));
            }
            column_min_widths.push(min_width);
            column_max_widths.push(max_width);
        }

        let total_min_width = column_min_widths.iter().sum::<usize>()
            + self.column_delimiter.chars().count() * (self.column_formats.len() - 1);
        // let total_max_width = column_max_widths.iter().sum::<usize>();

        // compute how many columns to include
        let n_used_columns = if total_min_width >= self.max_render_width {
            let mut n_used_columns = 0;
            let mut used_width = 0;
            for min_width in column_min_widths.iter() {
                if used_width > 0 {
                    used_width += self.column_delimiter.chars().count();
                }
                if used_width + min_width <= self.max_render_width {
                    n_used_columns += 1;
                    used_width += min_width;
                } else {
                    break;
                }
            }
            n_used_columns
        } else {
            self.column_formats.len()
        };
        // let column_min_widths = column_min_widths.into_iter().take(n_used_columns);
        // let column_max_widths = column_max_widths.into_iter().take(n_used_columns);

        // compute used widths
        let mut columns = Vec::with_capacity(n_used_columns);
        let mut used_widths = Vec::with_capacity(n_used_columns);
        let mut spare_room: usize = self.max_render_width
            - column_min_widths.iter().take(n_used_columns).sum::<usize>()
            - self.column_delimiter.chars().count() * ((n_used_columns as i64 - 1).max(0) as usize);
        for (c, column_format) in self.column_formats.iter().take(n_used_columns).enumerate() {
            let min_width = column_min_widths[c];
            let max_width = column_max_widths[c].min(min_width + spare_room);
            let column = column_format
                .clone()
                .min_width(min_width)
                .max_width(max_width)
                .format(df.column(column_format.name.as_str())?)?;
            let used_width = column
                .iter()
                .map(|s| s.chars().count())
                .max()
                .ok_or(EtopError::EmptyData(format!(
                    "empty column: {}",
                    column_format.name
                )))?;
            columns.push(column);
            used_widths.push(used_width);
            spare_room -= used_width - min_width;
        }
        Ok((used_widths, columns))
    }

    fn assemble_rows(&self, columns: Vec<Vec<String>>, rows: &mut Vec<String>, total_width: usize) {
        let n_data_rows = match columns.first() {
            Some(column) => column.len(),
            None => return,
        };
        for r in 0..n_data_rows {
            let mut row = String::with_capacity(total_width);
            for (c, column) in columns.iter().enumerate() {
                if c != 0 {
                    row.push_str(self.column_delimiter.as_str())
                }
                row.push_str(column[r].as_str())
            }
            rows.push(row)
        }
    }

    pub(crate) fn format(&self, df: DataFrame) -> Result<String, EtopError> {
        // clip
        let n_data_rows = self.n_data_rows();
        let df = df.clone().slice(0, n_data_rows);

        // render columns
        let (used_widths, columns) = self.render_columns(df)?;
        let total_width = self.total_rendered_width(&used_widths);

        // assemble rows
        let mut rows = Vec::with_capacity(self.render_height);
        if self.include_header_row {
            for row in self.render_header_rows(&used_widths, total_width) {
                rows.push(row);
            }
            if self.include_header_separator_row {
                rows.push(self.render_header_separator_row(&used_widths, total_width));
            }
        };
        self.assemble_rows(columns, &mut rows, total_width);
        if self.include_summary_row {
            todo!("summary row")
        }

        Ok(rows.join("\n"))
    }
}

// // build header row
// let n_rows = self.n_rows.unwrap_or_else(|| df.height().min(20));
// let widths = determine_widths(&df, &columns)?;
// let total_width = widths.iter().sum();
// let mut header = String::with_capacity(total_width);
// let column_delimiter = self.column_delimiter.clone().unwrap_or(" ".to_string());
// for (i, (column, width)) in columns.iter().zip(widths).enumerate() {
//     header.push_str(format!("{:>width$}", column.display_name, width = width).as_str());
//     if i != columns.len() - 1 {
//         header.push_str(column_delimiter.as_str());
//     }
// }

// // convert numeric fields to float64
// for (name, dtype) in df.schema().iter() {
//     if dtype.is_numeric() {
//         df = df
//             .clone()
//             .with_column(df.column(name)?.to_float()?)?
//             .clone();
//     }
// }

// // print each row
// let mut rows = vec![];
// rows.push(header);
// for r in 0..n_rows {
//     let mut row = String::new();
//     for (c, column_format) in columns.iter().enumerate() {
//         let df = df.clone();
//         let column = df.column(column_format.name.as_str())?;
//         let cell = format_cell(column, column_format, r)?;
//         row.push_str(cell.as_str());
//         if c != columns.len() - 1 {
//             row.push_str(column_delimiter.as_str());
//         }
//     }
//     rows.push(row);
// }

// Ok(rows.join("\n"))

// fn format_cell(
//     column: &Series,
//     column_format: &ColumnFormat,
//     r: usize,
// ) -> Result<String, EtopError> {
//     match column.dtype() {
//         DataType::Binary => match column.binary()?.get(r) {
//             Some(binary) => Ok(column_format.binary_format()?.format(binary)?),
//             None => Ok("-".into()),
//         },
//         DataType::Utf8 => Ok(column.str_value(r)?.to_string()),
//         DataType::Float64 => match column.f64()?.get(r) {
//             Some(number) => Ok(column_format.number_format()?.format(number)?),
//             None => Ok("-".into()),
//         },
//         DataType::Boolean => match column.bool()?.get(r) {
//             Some(true) => Ok("yes".to_string()),
//             Some(false) => Ok("no".to_string()),
//             None => Ok("-".to_string()),
//         },
//         dtype => {
//             let message = format!("column {} has type {}", column.name(), dtype);
//             Err(EtopError::UnsupportedDatatype(message))
//         }
//     }
// }

// pub(crate) fn determine_widths(
//     df: &DataFrame,
//     columns: &Vec<ColumnFormat>,
// ) -> Result<Vec<usize>, EtopError> {
//     let mut widths = Vec::with_capacity(columns.len());
//     for column in columns.iter() {
//         match column.min_width {
//             Some(min_width) => widths.push(min_width),
//             None => match df.schema().get(column.name.as_str()) {
//                 Some(dtype) => widths.push(get_dtype_default_width(dtype)),
//                 None => return Err(EtopError::ColumnMissing(column.name.to_string())),
//             },
//         }
//     }
//     Ok(widths)
// }

// pub(crate) fn get_dtype_default_width(dtype: &DataType) -> usize {
//     match dtype {
//         DataType::Boolean => 12,
//         DataType::UInt8 => 12,
//         DataType::UInt16 => 12,
//         DataType::UInt32 => 12,
//         DataType::UInt64 => 12,
//         DataType::Int8 => 12,
//         DataType::Int16 => 12,
//         DataType::Int32 => 12,
//         DataType::Int64 => 12,
//         DataType::Float32 => 12,
//         DataType::Float64 => 12,
//         // DataType::Decimal(_precision, _scale) => 12,
//         DataType::Utf8 => 12,
//         DataType::Binary => 12,
//         DataType::Date => 12,
//         DataType::Datetime(_, _) => 12,
//         DataType::Duration(_unit) => 12,
//         DataType::Time => 12,
//         // DataType::Array(_datatype, _size) => 12,
//         DataType::List(_datatype) => 12,
//         // DataType::Object(_) => 12,
//         DataType::Null => 12,
//         // DataType::Categorical(_) => 12,
//         DataType::Struct(_fields) => 12,
//         DataType::Unknown => 12,
//     }
// }
