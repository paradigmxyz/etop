use crate::{ColumnFormat, EtopError};
use polars::prelude::*;

const DEFAULT_TABLE_HEIGHT: usize = 30;

#[derive(Debug)]
pub struct DataFrameFormat {
    pub column_formats: Option<Vec<ColumnFormat>>,
    pub column_delimiter: String,
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
            include_header_separator_row: false,
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
    pub include_header_separator_row: bool,
    pub include_summary_row: bool,
    pub include_summary_separator_row: bool,
    pub render_height: usize,
    pub max_render_width: usize,
}

impl DataFrameFormat {
    /// format dataframe as String
    pub(crate) fn format(&self, df: DataFrame) -> Result<String, EtopError> {
        let fmt = self.finalize(df.clone());
        fmt.format(df)
    }

    /// fill missing format information based on dataframe
    fn finalize(&self, df: DataFrame) -> DataFrameFormatFinal {
        let column_formats: Vec<ColumnFormat> = match &self.column_formats {
            Some(c) => c.to_owned(),
            None => df
                .schema()
                .iter_names()
                .map(|name| ColumnFormat::new().name(name))
                .collect(),
        };
        let max_render_width = safe_sum_with_max_on_overflow(
            column_formats.iter().map(|c| c.get_max_width()).collect(),
        );
        DataFrameFormatFinal {
            column_formats,
            column_delimiter: self.column_delimiter.clone(),
            include_header_separator_row: self.include_header_separator_row,
            include_summary_row: self.include_summary_row,
            include_summary_separator_row: self.include_summary_separator_row,
            render_height: self.render_height.unwrap_or(DEFAULT_TABLE_HEIGHT),
            max_render_width,
        }
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
        self.column_formats
            .iter()
            .map(|f| f.display_name.chars().filter(|&c| c == '\n').count())
            .max()
            .unwrap_or(0)
    }

    fn n_data_rows(&self) -> usize {
        self.render_height
            - self.n_header_lines()
            - (self.include_header_separator_row as usize)
            - (self.include_summary_row as usize)
                * (1 + (self.include_summary_separator_row as usize))
    }

    fn total_rendered_width(&self, used_widths: &Vec<usize>) -> usize {
        used_widths.iter().sum::<usize>() + (used_widths.len() - 1) * self.column_delimiter.len()
    }

    fn render_header_row(&self, used_widths: &[usize], total_width: usize) -> String {
        let mut row = String::with_capacity(total_width);
        for (c, width) in used_widths.iter().enumerate() {
            if c != 0 {
                row.push_str(self.column_delimiter.as_str());
            }
            let name = self.column_formats[c].display_name.as_str();
            row.push_str(format!("{:>width$}", name, width = width).as_str());
        }
        row
    }

    fn render_columns(&self, df: DataFrame) -> Result<(Vec<usize>, Vec<Vec<String>>), EtopError> {
        // compute global sizes
        let column_min_widths: Vec<usize> = self
            .column_formats
            .iter()
            .map(|c| c.header_width().max(c.get_min_width()))
            .collect();
        let column_max_widths: Vec<usize> = self
            .column_formats
            .iter()
            .map(|c| c.get_max_width())
            .collect();
        let total_min_width = column_min_widths.iter().sum::<usize>();
        // let total_max_width = column_max_widths.iter().sum::<usize>();

        // compute how many columns to include
        let n_used_columns = if total_min_width >= self.max_render_width {
            let mut n_used_columns = 0;
            let mut used_width = 0;
            for min_width in column_min_widths.iter() {
                if used_width > 0 {
                    used_width += self.column_delimiter.len();
                }
                if used_width + min_width <= self.max_render_width {
                    n_used_columns += 1;
                }
            }
            n_used_columns
        } else {
            self.column_formats.len()
        };

        // compute used widths
        let mut columns = Vec::with_capacity(n_used_columns);
        let mut used_widths = Vec::with_capacity(n_used_columns);
        let mut spare_room: usize = self.max_render_width
            - column_min_widths.iter().take(n_used_columns).sum::<usize>()
            - self.column_delimiter.len() * (n_used_columns - 1);
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
                .map(|s| s.len())
                .max()
                .ok_or(EtopError::EmptyData(format!(
                    "empty column: {}",
                    column_format.name
                )))?;
            columns.push(column);
            used_widths.push(used_width);
            spare_room -= used_width - min_width;
        }
        println!("USED_WIDTHS: {:?}", used_widths);
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
        println!("DATAFRAME_FORMAT: {:?}", self);
        let (used_widths, columns) = self.render_columns(df)?;
        let total_width = self.total_rendered_width(&used_widths);

        // assemble rows
        let mut rows = Vec::with_capacity(self.render_height);
        rows.push(self.render_header_row(&used_widths, total_width));
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
